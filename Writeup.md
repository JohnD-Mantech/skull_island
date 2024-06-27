# Skull Island Writeup
By John Devost

---

The lore of the challenge is that captain Mack Sparrow has landed on an island to placate a diety that lives there (that is displeased with the pirates). The player must find the correct message for captain Mack to give to a diety that will placate it.

In order to do so they have been given rust code. This code has been heavily obfuscated, with variable names changed, to make figuring out what it does more difficult. There are many comments in the code, written to look like a software dev was trying to figure out how it worked, but most of them are not useful.

When creating the challenge, I was inspired by the game of life, and thought it would be interesting to use it as a novel encryption method.

More specifically, the program uses a block cellular automota simulation to encrypt the user's input. It then compares it to a file with the correct encryption and, if it is correct, then it will give them the flag.

## Objectives

This challenge aims to:
- Get players used to analysing and reversing code written in languages they are not familiar with.
- Write code to reverse programs they have debugged.
- Recognize and research patterns and algorithms found while reversing.

## Program

First, it prints out the intro, then prompts for input.

Once it has the input, it aes encrypts it then sends runs the cellular automota simulation on it for 150 iterations.

Then, it compares the final state of this to the data stored in `./lava`.

If they are equal, then it reads and prints the flag from `flag.txt`

The challenge should be run on a server with an actual flag, while players are just given the contents of `skull_island.7z`.