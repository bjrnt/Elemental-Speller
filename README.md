# Elemental Speller

Small Rust program that tries to spell a word using elements from the period table, as per this [/r/dailyprogrammer challenge](https://www.reddit.com/r/dailyprogrammer/comments/5seexn/20170206_challenge_302_easy_spelling_with/). Done as an exercise to get more familiar with Rust.

## Instructions

1. Make sure you have `cargo` and `rust` available.
2. Navigate to the checked out directory and run `cargo run word_to_spell`.

## Example Output
```
poison -> PoISON (polonium, iodine, sulfur, oxygen, nitrogen)
bacon -> BaCoN (barium, cobalt, nitrogen)
sickness -> SiCKNeSS (silicon, carbon, potassium, neon, sulfur, sulfur)
ticklish -> TiCKLiSH (titanium, carbon, potassium, lithium, sulfur, hydrogen)
```

Thanks to Shepmaster for [providing a lot of helpful feedback](http://codereview.stackexchange.com/questions/154700/spelling-a-word-with-chemical-elements-in-rust/154717).
