# The Debugger

Printing works, and you should keep doing it. A **debugger** does the same job
without editing the program, and adds things printing cannot do.

## What it gives you

A debugger runs your program under supervision, and lets you:

**Set a breakpoint** — mark a line, and execution pauses when it is reached.

**Inspect everything.** Every variable in scope, with its current value. Not the
ones you thought to print — all of them.

**Step.** Advance one line at a time. *Step over* runs a method call and stops
after it; *step into* enters the method and stops at its first line; *step out*
finishes the current method and stops at the caller.

**See the call stack.** The same information as a stack trace, live, and you can
click a frame to inspect the variables *in that frame*. This is the feature people
underuse most.

**Change values.** Set a variable to something else and continue, to test what
would happen without editing and recompiling.

**Conditional breakpoints.** Pause only when a condition holds — `i == 4999`, or
`name.equals("Smith")`. This is the killer feature for loops: printing a million
iterations is useless, and stopping at the one that matters is exactly right.

## Using it well

**Put the breakpoint before the problem, not at it.** By the time the failure
happens the evidence is often gone. Stop earlier and step forward.

**Watch the call stack.** When a method receives a wrong value, the mistake is
usually in the caller. Click up a frame and look.

**Step over by default, into when suspicious.** Stepping into everything drops you
into library code you did not want to read. Step over until a call returns
something wrong, then rerun and step into that one.

**Use conditional breakpoints for loops.** If iteration 4,999 is the problem, say
so. Clicking continue five thousand times is not a plan.

## When printing is better

The debugger is not always the right tool, and the cases where it is not are
worth knowing.

**Concurrency.** Pausing a thread changes the timing, and timing is frequently the
bug — the act of observing alters what you are observing. Chapter 31 discusses
this. Logging is usually the only workable approach.

**Anything not on your machine.** A failure in production, in a container, on a
customer's system. Logs are what you have.

**Intermittent failures.** You cannot sit at a breakpoint waiting for something
that happens once a day. Log, and read the log afterwards.

**Seeing a pattern across many iterations.** A debugger shows one moment well. A
printed table of a hundred iterations shows a trend, and trends are sometimes what
you need.

The two are complementary. Printing is a record over time; the debugger is a
complete picture at one moment. Which you want depends on whether your question
is "how did this evolve" or "what exactly is true right now".

## The habit that survives both

Whichever tool you use, the method from this chapter does not change:

1. State what you expect.
2. Observe what is actually the case.
3. Find where they first diverge.
4. Ask what would have to be true for that divergence to occur.

The debugger makes step 2 cheap. It does not do steps 1, 3, or 4, and a debugger
in the hands of someone with no expectation is a very efficient way to look at
values that mean nothing.

## Closing the chapter, and the unit

This chapter was about a skill that is usually left to be picked up: finding out
what a program is actually doing.

Trace by hand until the machine holds no surprises. Desk check by predicting
before observing, choosing inputs at the boundaries, and being most suspicious
where you feel most sure. Read errors completely — the exception type, the
values, the topmost frame in your own code. Bisect rather than guess, in the
program, in the input, and in the history. And use the debugger for the questions
it answers well, while keeping printing for the questions it does not.

Unit II is finished. We began with a model of computation — state, transition,
and a starting point — and found it enough to describe everything a machine does.
We gave state a name with variables, and found that naming introduces a
distinction between a name and a value that will matter enormously later. We
built choice from Boole's algebra and Shannon's observation, and derived the
adder of Chapter 2 from two logic gates. We built repetition from a backward
jump, and learned to prove a loop correct rather than test it. And we finished
with how to find out what is really happening when the proof was not written and
the test did not pass.

What is missing is a way to stop thinking about all of it at once.

Every program in this unit has been one block of statements. Real programs are
too large for that, and the constraint is not the machine's — Chapter 6 showed a
three-rule tape machine can compute anything. The constraint is yours: a human
cannot hold ten thousand lines of state and transitions in mind.

Unit III is about the first and most important tool for that. Giving a process a
name.
