# RUST API Helper

A simple rust program to remove body of functions from the source code.

Main idea is to check if it's helpful to do so with local LLMs as their tokens budget is limited.
I.e. if we have `src/main.rs` and `src/helper.rs` and want tell DeepSeek to poke around `main.rs`, 
we don't need implementations of functions inside of `helper.rs`, we need to know api only.
So prompt can be simply constructed by concatenating `helper.rs` (with implementations removed) and `main.rs` without any changes


