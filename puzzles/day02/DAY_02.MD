# :magic_wand: AoC 2021 Day 2 :christmas_tree:

---

[task](https://adventofcode.com/2021/day/2)

Second day in terms of difficulty was similar to day 1. 

At first, I was considering to use something fancy for input parsing, like regex or even peg, but my laziness get the 
better of me. :sloth:

Solution for both parts was simple parse input into `Vec<_>` of enums. With each enum holding value. 
Then in for loop count and sum what is needed and return result.

There is not really a need to parse it first. Processing the result would be easy enough in one run, but parsing into enum 
seems so nice and idiomatic, that I couldn't resist. ☺

