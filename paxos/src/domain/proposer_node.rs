use std::collections::{HashMap, HashSet};

use super::{id::ProposalId, proposal::Proposal};
use crate::network::Network;

/// Node that broadcast proposals to all the acceptors. All the information stored in
/// this struct is ephemeral, being erased once the round completes.
pub struct ProposerNode {
    pub id: u64,
    /// Buffer that stores temporarily the id and value of the latest proposal set to
    /// be accepted by any acceptor.
    pub latest_proposal: Option<Proposal>,
    /// History of proposals sent by this proposer, and their respective values.
    pub proposal_history: HashMap<ProposalId, u64>,
    /// Nodes that replied to the prepare request.
    pub prepared_nodes: HashSet<u64>,
    /// Nodes that replied to the accept request.
    pub accepted_value_nodes: HashSet<u64>,
    /// Interface to communicate with other nodes.
    pub network_interface: Box<dyn Network + Send + Sync>,
}

impl ProposerNode {
    pub fn new(network_interface: Box<dyn Network + Send + Sync>) -> Self {
        let id = 1; // TODO: change when there's more than one proposer
        let proposal_history = HashMap::new();
        let prepared_nodes = HashSet::new();
        let accepted_value_nodes = HashSet::new();

        Self {
            id,
            network_interface,
            latest_proposal: None,
            proposal_history,
            accepted_value_nodes,
            prepared_nodes,
        }
    }
}
