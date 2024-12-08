To run, first make sure you have `direnv` installed and hooked into your shell. Then, in the root of the repository, run `direnv allow`. Now, you will have a nix development shell set up every time you cd into this directory.

### Architecture
In this implementation, we simulate a varying number of nodes that communicate with each other. The goal is to simulate a "distributed" fibonacci sequence.
In each round, proposers are going to send a value `n` that must be added to the current value `n` of the nodes. 
After the protocol phases are completed, we expect that all nodes will have the same value. This is tested by running the algorithm a few times and checking the value for each node.

The storage used is SQLite.
