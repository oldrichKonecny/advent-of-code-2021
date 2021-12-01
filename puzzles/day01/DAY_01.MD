# AoC 2021 Day 1 🎄

---

[task](https://adventofcode.com/2021/day/1)

Day one is over, and I'm glad the task wasn't that harder. 

First part seemed easy. After parsing input into `Vec<i32>` I went with the good old for cycle while remembering count
and previous value. But of course I only updated the previous value when incrementing count, which took me like 10 
minutes to realize... After that it was easy-peasy. :lemon:

Second part was easier than first part. Only I have to google how to use sliding window with rust.
With that comparing first and fourth element in window was no-brainer.

Also I wrote solution for first part using sliding window and then bench it against for loop. To my great surprise, 
sliding window won pretty convincingly.

> sliding window: 682 ns/iter (+/- 30) :rocket:
> 
> for loop:  1,030 ns/iter (+/- 34)

Not sure what magic rust/llvm is doing, but it feels great to know that short, elegant solution is 
also the faster one.

Can't wait for day 2 🙂
