# GravelerChallenge
Making a  program to count the number of 1s rolled in 1 billion samples of 231 d4s that is faster than Austin's from Shoddycast.

As a compiled language, rust is much faster than python. In addition, rust
provides great utilities for multithreading, which were nice once I figured out
how to use them.

This program will spread out "samples" of 231 dice rolls across many threads.
I use 2 fewer threads than the total availible so that I can have OBS run in the backgroud
to record it.

My machine has a 7950x3D CPU, which has 32 threads (very fast threads, I might add),
which, when using 30 of those threads each calculating 33,333,334 samples, completed the task in 37 minutes,
and about 24 seconds.

Clearly, this is faster than the original code :)
