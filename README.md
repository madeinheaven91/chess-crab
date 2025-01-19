# ChessCrab

ChessCrab is a chess engine written in Rust.

## Usage

Running:
```rust
cargo run
```

Commands:
- q - quit
- index [square] - shows a bitboard index of a square (e.g. index e2)
- moves - show available moves
- move [move] - make a move (moves are in algebraic notation, e.g. e2e4)
- eval - show evaluation (NOT DONE YET)
- position - load position from FEN string (NOT DONE YET)

## TODO:
- [x] Board representation 
- [x] Pseudolegal moves
- [] Legal moves
- [] Tests
- [] Evaluation
- [] Search
