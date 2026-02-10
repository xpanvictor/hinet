# Message

## Structure
`Message` represents widely any form of exchange between two nodes 
intended to serve as communication on the p2p chat structure. 
Widely, *2* forms of messages are supported:
- Direct Message
- Group Message
However, these messages will be sent across different protocols depending
on the state of communication eg:
1. both peers of a "DM" being both "online" .i.e swarm both dialed
2. one peer of a "DM" being online can initiate a "wake" protocol using a
dumb relay, to revert back to format 1. 
3. if form 2 fails (can't wake inactive peer), active peer can keep
message (undelivered) pending successful wake (sqlite eventual consistency). This is to avoid a mail box structure.
4. group message using gossipsub to keep message buffering with some
ordered layer of consistency.
5.  

## Types of messages
1. DM
    - Text
    - Typing
    - Ack (Seen)
2. Group
    - Text 
    - 
