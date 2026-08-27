# The World Outside the Program

Every program in this book so far has been a closed system. It took no input it
did not construct, wrote nothing but to the console, ran once, and finished. Given
the same source it did the same thing every time.

Real programs are not like that, and this unit is about the difference.

A real program reads a file that might not be there. It writes data that must
still be correct after the process is gone. It waits for a user who may click
anything in any order. It talks to a machine on the other side of the world that
may not answer. And it does several of these at once.

The unifying difficulty is that **the world does not cooperate**. Inside your
program you control everything: a variable holds what you put in it, a method
returns what it computed. Outside, the disk is full, the file is malformed, the
network is slow, the user closed the window, and another thread changed the value
between your reading it and your using it.

Four chapters, each about one form of that.

**Chapter 28 — When Things Go Wrong.** Exceptions: what they are, why returning
`-1` was never good enough, and the design question of where a failure should be
handled. Also Java's checked exceptions, which are a genuinely contested feature
and worth understanding as an argument rather than a rule.

**Chapter 29 — Persistence.** Files and streams. Storage outlives the process,
which sounds obvious and has consequences — a format is a promise to your future
self, encoding is Chapter 4's problem returning, and a half-written file is a
state your program can be interrupted into.

**Chapter 30 — Events and Interfaces.** A graphical program inverts control: you
do not call the toolkit, it calls you. That inversion changes the shape of a
program, and understanding it is more useful than any particular widget library.

**Chapter 31 — Many Things at Once, Really.** Concurrency. Chapter 26 showed a
parallel stream producing three different wrong answers; here is why, what to do
about it, and why almost everyone finds this hard. Then sockets, where the other
program is on a different machine and nothing is shared but bytes.

Two things change, and they run through all four chapters.

**State becomes the central difficulty again.** Unit V introduced state and Unit
VI mostly avoided it — an evaluator is a tree walk, a lambda is best kept pure.
Here state is unavoidable: a file has contents, a window has a position, a
connection has a status, and a thread is a second reader of everything. Chapter
20's immutability argument, which may have seemed like taste, becomes the main
line of defense.

**Correctness becomes conditional.** Until now a program was correct or it was
not. Now it is correct *if* the file exists, *if* the encoding matches, *if* the
network responds, *if* the interleaving is favourable. Programs at this level are
built out of assumptions, and the skill is naming them, checking the ones you can,
and failing usefully on the rest.

A word on what this unit is not. It is not an API tour. Java's I/O libraries alone would fill a book, the GUI
toolkits have changed three times, and the concurrency utilities are enormous.
Committing any of it to memory is a poor use of your attention when the
documentation is a keystroke away.

What is worth learning is the shape of each problem: why streams are the
abstraction they are, why an event loop must not be blocked, what a race
condition actually is at the level of memory. Those transfer to every language
you will use. The method names do not.

The examples are correspondingly small. They run, every output shown is real, and
they are meant to be typed rather than admired.
