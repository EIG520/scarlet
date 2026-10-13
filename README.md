
# Scarlet Chess Engine

Scarlet is an open source chess engine that implements most of the UCI protocol.  The engine includes an efficient bitboard representation of the gamestate based on the pext and pdep instructions, meaning that zen2 is unsupported.

## Strength

Scarlet v1 is rated 2640 on the CCRL blitz rating list as of August 9th, 2026.
Scarlet v2 is not yet tested on CCRL, but based on internal tests should be ~3400 elo.

## Engine Features

- Principle Variation Search
- Quiescence Search
- Transposition Table (move ordering and cutoffs)
- Internal iterative reductions if no TT move
- Time Management with Hard/Soft bounds
- Null move pruning
- Reverse futility pruning
- Futility pruning
- Late move pruning
- Late move reductions
- Static Exchange Evaluation (SEE) pruning and move ordering
- Singular Extensions (SE) with multicut
- Improving heuristic
- History heuristic using [type][from][to] w/ gravity, penalties, and reductions for LMR
- Capture history
- Continuation history (1+2 ply)
- Killer moves heuristic
- Hard/Soft time management
- Check extension
- Aspiration windows
- Early cycle/repetition detection
- Efficiently updatable neural network (NNUE) evaluation using 768->512x2->1 architecture, trained on 2GB of selfplay data in viriformat.

## Usage

Find and download the binary from releases most fitting your system, then run that executable. <br>
Alternatively, you can build the project yourself. Ensure both rust and cargo are installed, enter the main directory of the project, and run
`cargo build --bin scarlet --release`.  After this, the executable for the chess engine will be in target/release. 
