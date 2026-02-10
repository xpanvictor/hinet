## Overview

`hinet` is a peer‑to‑peer messaging system written in Rust. It focuses on:

- Fully distributed communication built on top of modern P2P networking (libp2p).
- First‑class support for direct messages and group conversations.
- An internal message bus that decouples services such as networking, storage, and higher‑level features.
- A storage layer for local durability and eventual consistency without a central mailbox server.

The repository is organized as a small workspace of focused crates and an example CLI application that exercises the runtime.

## High‑Level Goals

- **Decentralized messaging core**: Nodes discover and talk to each other over a P2P network (no central server required).
- **Direct & group messaging**: Support for one‑to‑one chats and group chats with room to extend to richer message types.
- **Offline delivery & wake flows**: Allow messages to be queued locally and delivered once peers come online, with the option to "wake" a peer via relay when possible.
- **Extensible protocols**: Encode all messages as protobufs, making it easy to extend the protocol without breaking compatibility.
- **Pluggable clients**: Expose clear boundaries so that multiple clients (CLI, desktop, mobile) can share the same core.

In the long term, the project aims to offer a messaging experience comparable to mainstream chat platforms while remaining self‑hostable and privacy‑oriented.

## Architecture

### Workspace Layout (current)

- `apps/cli`: Example command‑line client and entrypoint to the system runtime.
- `crates/common`: Shared infrastructure primitives (message bus, service trait, tracing helpers).
- `crates/message`: Protobuf definitions and message‑level utilities.
- `crates/net`: Networking stack built on libp2p plus a legacy TCP implementation.
- `crates/root`: System runtime, orchestration, and metrics.
- `crates/storage`: Persistent storage (SQLite via `sqlx`) and DB types.
- `docs/`: Design notes and deeper dives into specific components.

### Runtime & Services (`crates/root` and `crates/common`)

- The **runtime** (`crates/root/src/runtime.rs`) initializes tracing, constructs a shared `MsgBus`, spawns services, and coordinates shutdown.
- A lightweight **service trait** (`crates/common/src/service.rs`) defines a common `run(shutdown_rx)` entrypoint for long‑running tasks.
- The **message bus** (`crates/common/src/bus.rs`) is a typed publish/subscribe hub:
	- Services subscribe to specific message types.
	- Publishers send strongly‑typed events without knowing who will consume them.
	- This decouples networking, storage, and higher‑level logic.

### Messaging Model (`crates/message` and `docs/message.md`)

- Protobuf definitions under `crates/message/src/pb` describe the on‑wire message format.
- `docs/message.md` outlines the conceptual model:
	- **Direct messages** between two peers.
	- **Group messages** for multi‑party chats.
	- Multiple transport patterns depending on peer availability (online/online, wake via relay, queued for later).
- The design explicitly targets eventual consistency without a centralized "mailbox" service by leveraging local storage and P2P relaying.

### Networking (`crates/net` and `docs/swarm_structure.md`)

- The primary networking implementation is the **P2P swarm** built on libp2p:
	- `crates/net/src/behavior.rs` defines `MsgBehaviour`, a custom `NetworkBehaviour` combining:
		- Kademlia DHT for discovery.
		- mDNS for local peer discovery.
		- Identify for exchanging peer information.
		- Request/response for direct messaging.
		- Gossipsub for group chat topics.
		- Ping, AutoNAT, DCUtR, and relay client support.
	- `crates/net/src/swarm.rs` constructs a `Swarm<MsgBehaviour>` with proper transports, multiplexing, and authentication.
	- `crates/net/src/handler.rs` contains `P2pSwarmHandler`, which reacts to swarm events (discovery, connections, gossipsub messages, etc.) and will be the place to emit and consume higher‑level events via the `MsgBus`.
- An earlier **TCP networking** implementation exists in `crates/net/src/tcp.rs`:
	- Exposes a `Network` service that listens on a TCP port, reads framed messages, and publishes them onto the bus.
	- This is marked as deprecated in favor of the libp2p‑based swarm.

For a brief conceptual description of the swarm, see `docs/swarm_structure.md`.

### Storage (`crates/storage` and `storage/src/db_types`)

- The **storage crate** wraps SQLite through `sqlx`:
	- `crates/storage/src/db.rs` manages connection setup, environment configuration, and migrations.
	- Migrations live in `crates/storage/migrations` (for example, `20260205_messages.sql`).
- Additional DB type definitions, such as `Message`, live under `storage/src/db_types`. These represent how messages are persisted locally and are intended to act as converters from P2P message forms (DM, group) into durable records.

### CLI Application (`apps/cli`)

- `apps/cli` contains a thin command‑line interface over the runtime:
	- `main.rs` wires together:
		- The system runtime (`root::runtime::Runtime::run`).
		- CLI parsing (via `clap`) and command dispatch.
		- A user command handler loop (`user_handler`) that listens for stdin input while the runtime is running.
- This layer is primarily for local development, debugging, and manual interaction with the node.

## Current Status

The project is under active development. At the moment:

- The **runtime** can start up, initialize tracing and metrics counters, and launch network services.
- The **message bus** is in place and used by the networking layer; it supports typed publish/subscribe semantics.
- The **libp2p swarm behaviour** is largely wired:
	- Discovery (Kademlia + mDNS) and basic event handling are implemented.
	- Gossipsub and direct message protocols are defined and structurally integrated, but the high‑level application flow for sending and receiving chat messages is still being built out.
- The **storage** crate has:
	- A migration pipeline and SQLite setup.
	- Early work on DB types for messages.
	- Error types and wiring that still need refinement and more usage throughout the runtime.
- The **CLI** can start the system and run an input loop but is minimal and not yet a full interactive chat client.

Broadly, the foundations for networking, messaging, and storage exist, but the end‑to‑end message flow (from user input, through P2P network, into storage, and back to UI) is still in progress.

## Future Plans

### Core Messaging & Protocol

- Finalize the protobuf schema for direct and group messages, including:
	- Text content, typing indicators, delivery/read acknowledgements.
	- Room for future media attachments.
- Implement the full request/response flow for direct messaging:
	- Map bus events to outbound P2P requests.
	- Route inbound P2P messages into the bus and storage.
- Implement group chat over gossipsub:
	- Topic management for groups.
	- Consistent ordering and de‑duplication strategies.
- Solidify offline and wake behaviour:
	- Use local storage to persist undelivered messages.
	- Implement the wake‑via‑relay path before falling back to queued delivery.

### Storage & Data Model

- Complete the DB types for messages, contacts, and groups.
- Implement repository APIs in `crates/storage/db_repos` for:
	- Writing incoming and outgoing messages.
	- Querying conversation history by peer or group.
	- Tracking delivery/read status and timestamps.
- Add migrations for identity and key management, if those are stored locally.

### Identity, Security & Privacy

- Integrate libp2p identities with user‑facing identities (usernames / handles) and DHT records.
- Add end‑to‑end encryption for messages where appropriate.
- Design key backup and device onboarding flows (e.g., recovery, multi‑device).

### Observability

- Expand `crates/root/src/metrics.rs` to include:
	- Per‑service task counts.
	- Network metrics (connections, bytes sent/received, failure rates).
	- Storage metrics (DB latency, errors).
- Provide basic dashboards or CLI commands to introspect node health.

### Cross‑Platform Client Layer (Tauri + Rust)

In addition to the CLI, the plan is to build a cross‑platform client layer using Tauri and Rust. This client will sit on top of the existing runtime and expose a modern chat experience, while keeping the P2P core and storage logic within Rust.

Planned capabilities include:

- **Multi‑conversation interface**: Chats list with unread counts and previews.
- **Direct and group chats**: Rich conversation view powered by the P2P message flow.
- **Delivery semantics**: Indicators for sent, delivered, and read states.
- **Presence & typing states**: Surface online status and typing indicators where protocol allows.
- **Media & attachments (later phase)**: Support for sending files, images, and other media types.
- **Notifications**: OS‑level notifications for new messages and mentions.
- **Cross‑device support**: Reuse the same core logic on desktop and, in the future, mobile targets where Tauri is supported.

The Tauri layer will primarily:

- Bind to the Rust runtime as a local node.
- Use the `MsgBus` and storage APIs to read/write data.
- Render a UI that feels comparable to modern messaging apps but powered by the decentralized `hinet` core.

### Tooling, Tests & Developer Experience

- Add more integration tests that spin up multiple nodes and validate:
	- Peer discovery and connection establishment.
	- Direct and group message delivery.
	- Persistence and re‑delivery across restarts.
- Improve ergonomics around running local multi‑node test setups (scripts, Docker, or just multiple CLI instances).
- Document common workflows in the `docs/` folder (e.g., how to run a two‑node demo, how to inspect the DB, how to extend the protocol).

## Getting Started (Early)

> Note: The project is still evolving; expect breaking changes. Currently, things might break.

1. Ensure you have a recent Rust toolchain installed.
2. From the workspace root, run the CLI app:

	 ```bash
	 cargo run -p cli -- start
	 ```

3. Open another terminal and start a second node (once multi‑node workflows are documented).
4. Explore the source in `crates/net`, `crates/message`, and `crates/storage` to understand the current flow.

As the project matures, this section will be expanded with concrete multi‑node examples and client usage instructions.
