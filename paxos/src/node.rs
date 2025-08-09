//! A node is a participant in the Paxos distributed consensus protocol.
//! Each node can act as a proposer (suggesting values for consensus),
//! an acceptor (voting on proposed values), or both. Nodes communicate
//! with each other through message passing to eventually agree on a
//! single value across the distributed system.
// TODO
