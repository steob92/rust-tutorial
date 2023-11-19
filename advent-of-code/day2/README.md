# Day 2 of Advent of Code

[Advent of Code Day 2](https://adventofcode.com/2022/day/2) presents an encrypted strategy guide for winning games of Rock-Paper-Scissors. The provided file format looks like this:

```
A Y
B X
C Z
```


The first column denotes the opponent's move: `A` for Rock, `B` for Paper, and `C` for Scissors. While the full explanation for the second column isn't given, it's implied that it represents our moves: `X` for Rock, `Y` for Paper, and `Z` for Scissors.

Each move grants us points: `1` for Rock, `2` for Paper, and `3` for Scissors. Additionally, the outcome of the game earns us points: `0` if we lose, `3` for a draw, and `6` for a win.

The task is to calculate the total score following the provided strategy. Using the given example:


```
A Y
B X
C Z
```


The expected score is 15 points, serving as our test case.

Given that Rock, Paper, or Scissors are exclusive options, this might be an opportunity to use an `enum`.
