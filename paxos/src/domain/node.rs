use super::id::ProposalId;
use crate::network::Network;

pub struct AcceptorNode {
    /// Identifier of the node.
    // TODO: this should probably be an uuid, that will be stored in non-volatile
    // memory to keep track of nodes, especially those thay may die and then restart.
    pub id: u64,
    /// Interface to communicate with other nodes.
    pub network_interface: Box<dyn Network + Send + Sync>,
    /// Buffer that stores temporarily the id of the latest proposal set to be
    /// accepted in this node.
    pub buffer: Option<ProposalId>,
}

impl AcceptorNode {
    pub fn new(id: u64, network_interface: Box<dyn Network + Send + Sync>) -> Self {
        Self {
            id,
            network_interface,
            buffer: None,
        }
    }
}
