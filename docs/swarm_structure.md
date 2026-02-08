# Swarm structure

The swarm behaviour is defined in `/crates/net/src/behavior.rs`. This includes the 
following protocols and their respective functionalities:

### 1. Kademlia DHT (Kad)
This is the layer for discovery. Discovery will follow the following patterns:
- `User Identity`: Record managed on kad for username fetch & interact. 
