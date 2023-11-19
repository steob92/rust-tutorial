# Day 2 of Advent of Code

[Advent of Code Day 6](https://adventofcode.com/2022/day/6) presents a faulty communication device. The input is a data stream, and the task is to detect when the message starts.

"To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the data stream. In the protocol used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different."


We're given an example:
```mjqjpqmgbljsphdztnvjfqwrcgsmlb```

In this example, we're searching for the first instance of four non-repeating characters. The goal is to determine how many characters need to be parsed before the start of the message. We're provided with several examples and their respective answers:

- `bvwbjplbgvbhsrlpgdmjqwftvncz`: first marker after character 5
- `nppdvjthqldpwncqszvftbrmjlhg`: first marker after character 6
- `nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg`: first marker after character 10
- `zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw`: first marker after character 11

Hence, the task involves searching over a block of characters with the number of characters to search over being fixed. This scenario seems suitable for utilizing a [RingBuffer](https://en.wikipedia.org/wiki/Circular_buffer) or a double-ended vector.
