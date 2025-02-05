# ChessCrab

ChessCrab is a chess engine written in Rust.

## Usage

Building:
```rust
cargo build
```

Implemented commands:
- `q` - quit
- `i [square]` - shows a bitboard index of a square (e.g. index e2)
- `moves` - show available moves
- `moves a` - show available moves in algebraic notation
- `m [move]` - make a move (moves are in algebraic notation, e.g. e2e4)
- `r` - make a random move

## TODO:
- [x] Board representation 
- [x] Pseudolegal moves
- [x] Legal moves
- [x] Castling, promotions, en passant, special rules
- [ ] Tests + perft
- [ ] Evaluation
- [ ] Search
