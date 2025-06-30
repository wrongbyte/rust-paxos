-- Core nodes table
CREATE TABLE nodes (
    id INTEGER PRIMARY KEY,
    role TEXT CHECK (role IN ('proposer', 'acceptor', 'learner')), -- TODO: enum?
    last_activity DATETIME DEFAULT CURRENT_TIMESTAMP,
    restart_count INTEGER DEFAULT 0 -- debugging?
);

CREATE TABLE node_paxos_state (
    node_id INTEGER PRIMARY KEY,
    current_proposal_number INTEGER DEFAULT 0,
    promised_proposal_number INTEGER,
    accepted_proposal_number INTEGER,
    accepted_value TEXT,
    FOREIGN KEY (node_id) REFERENCES nodes(id)
);

-- Indexes for performance
CREATE INDEX idx_nodes_status ON nodes(status);
