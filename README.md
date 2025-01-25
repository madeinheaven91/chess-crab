# ChessCrab

ChessCrab is a chess engine written in Rust.

## Usage

Building:
```rust
cargo build
```

Implemented commands:
- q - quit
- index [square] - shows a bitboard index of a square (e.g. index e2)
- moves - show available moves
- move [move] - make a move (moves are in algebraic notation, e.g. e2e4)

## TODO:
- [x] Board representation 
- [x] Pseudolegal moves
- [x] Legal moves
- [ ] Castling, promotions, en passant, special rules
- [x] Tests
- [ ] Evaluation
- [ ] Search
