# RROLL

A simple command-line utility to roll lots of dice, Dungeons and Dragons style.

## Installation

Clone this repository and run `cargo install --path .` in the project root.

## Usage

`rroll [NdN+N...]`

for example, `rroll 1d20+4 2d4 3d10+1` might produce the following output:

```
[1] + 4 : 5
[4, 1] : 5
[9, 1, 10] + 1 : 21
Total: 31
```

Each line is the roll for one "group" of dice. The third line in our example output shows `[9, 1, 10] + 1 : 21`, meaning that the results of the three dice rolls were 9, 1, and 10 (`3d10`), and after adding those numbers plus the "delta" of 1 (`+1`), we get 21.

`rroll` is not restricted to rolling "standard" dice - it can roll dice with any number of sides - for example, `rroll 1d76` is valid, and will roll a 76-sided die. 
