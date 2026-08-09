
# Scarlet Chess Engine

Scarlet is an open source chess engine that implements most of the UCI protocol.  The engine includes an efficient bitboard representation of the gamestate based on the pext and pdep instructions, meaning that zen2 is unsupported.

## Strength

Scarlet v1 is rated 2640 on the CCRL blitz rating list as of August 9th, 2026.

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
- History heuristic using [type][from][to] w/ gravity, penalties, and reductions for LMR
- Killer moves heuristic
- Hard/Soft time management
- Check extension
- Aspiration window
- Early cycle/repetition detection
- [PeSTO evaluation](https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function)

## Usage

Find and download the binary from releases most fitting your system, then run that executable. <br>
Alternatively, you can build the project yourself. Ensure both rust and cargo are installed, enter the main directory of the project, and run
`cargo build --release`.  After this, the executable for the chess engine will be in target/release. 
