# Networking crate

# A Rust crate for handling networking operations.

Copyright (c) 2025 xpanvictor. All Rights Reserved.
This project is licensed under the MIT License. See the LICENSE file for details.

## Features

- Initially uses `TCP` for communication, this is to review the implementation logic of
  networking operations.

## Interaction with Other Components

- Uses `MsgBus` and uses two message types:
  - `NetworkSendMsg`: Listens to requests to send data over the network.
  - `MsgReceived`: Notifies when data is received over the network.

## Metrics & Diagnostics

The network crate provides the following metrics for monitoring and diagnostics:

1. **Active Connections**: The current number of active network connections.
2. **Data Sent**: The total amount of data sent over the network (in bytes).
3. **Data Received**: The total amount of data received over the network (in bytes).
