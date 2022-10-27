#RUST CPU SIMULATOR

### This readme is still a WIP

An 8-bit cpu simulated with Rust!

![cpu sim running](https://github.com/jasonhilder/jcpu-rust/blob/main/showcase.png)

> As a note this code base is not optimized, there are definitely a ton of better more efficient ways to do whats here.
> The idea was more to learn about lower level computing and getting more familiar with Rust.

## How to use it

Create a main.jsm file and write your 'jasm'.
or
Use the compile to compile an example.

Run the compiler.

Run the sim.


## Whats generated

- boot.img: What the sim reads, it is the binary instructions that have been compiled.
- instructions.d: A debug file for the sim, to show the deassembled instructions.
