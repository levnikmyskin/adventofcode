# adventofcode
Advent of code 2018, solved in Rust http://adventofcode.com/

This is a repo containing solutions for the adventofcode 2018, written in Rust. Please notice this is a low-effort project, which I used to learn some Rust:  
*  The first days are a bit messed up and there's no tracking of the first solutions;  
*  Sometimes, for simplicity, I simply hardcoded the game input data into the code;  
*  Sometimes the solutions are not very clean or extremely performant;  

## Building and running  
All the code is written using Rust 2018 which, at the time of writing, is not on the stable channel yet, thus you need to install
the nightly build. You can easily do this with [rustup](https://rustup.rs/).  
That said, compiling and running any *day* should be pretty straight forward:  
*  Move into the *day* you want to run;  
*  Build with `cargo build`;  
*  Run the executable found in target/debug/day_{1,2,3,etc};  
