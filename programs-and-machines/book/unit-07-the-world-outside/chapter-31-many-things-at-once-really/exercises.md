# Exercises

**31.1** Print `Runtime.getRuntime().availableProcessors()` on your machine. Then
compute, with Amdahl's law, the best possible speedup on that many cores for a
program that is 90% parallel and for one that is 99% parallel.

**31.2** Reproduce the race: eight threads each incrementing a shared `int`
200,000 times. Run it five times and report all five totals. Then explain the
lowest one in terms of read-add-write.

**31.3** Repeat 31.2 with the field marked `volatile`. Report the result and
explain why `volatile` did not help.

**31.4** Write a loop in one thread that spins until a non-volatile `boolean` flag
is set by another. Run it. Then add `volatile` and run again. Describe both
outcomes and say what the JIT was entitled to do.

**31.5** Fix the race three ways — `synchronized`, `AtomicInteger`, and a local
accumulator per thread summed at the end. Time all three plus the unsynchronized
version. Report four numbers and rank the approaches.

**31.6** Use a `ConcurrentHashMap` from eight threads with `merge`. Confirm the
count. Then replace `merge` with `if (!map.containsKey(k)) map.put(k, 1); else
map.put(k, map.get(k) + 1)` and confirm it is wrong. Explain why a concurrent
collection did not save you.

**31.7** Write two threads that acquire two locks in opposite orders and observe
the deadlock. Use `jstack` or a thread dump to confirm what each thread is waiting
for. Then fix it by ordering, and confirm.

**31.8** Submit five tasks each sleeping 100 ms to a fixed pool of four threads and
time the whole thing. Predict the answer first. Then vary the pool size from 1 to
8 and plot or tabulate the times.

**31.9** *Measurement.* Submit 10,000 tasks that each sleep 100 ms to (a) a fixed
pool of 200 platform threads and (b) a virtual-thread-per-task executor. Report
both times and explain the difference.

**31.10** Write the echo server and client from Section 31.2.1. Use port 0 and
report the port the OS gave you. Then make the client send two lines in one
`write` with no newline between them, and describe what the server does. Name the
protocol property that was missing.

**31.11** Add framing to your protocol with a four-byte length prefix instead of a
newline. Confirm it survives a message containing a newline.

**31.12** Add a read timeout to your client and point it at a server that never
replies. Report the exception. Then remove the timeout and describe what happens
instead, and why that is worse.

**31.13** *Design, no code.* You are calling a payment service. The request times
out. Describe exactly what your program should do, including what it must have
sent in the original request for your answer to be safe.

**31.14** *Longer.* [carries forward] Turn Chapter 25's interpreter into a
network service: it accepts connections, reads a program, evaluates it, and
returns the output. Handle several clients at once. State whether your interpreter
has any shared mutable state, and if it does, say what you did about it.

**31.15** *Reading, no code.* Find a concurrency bug report in an open-source
project — search an issue tracker for "race condition". Read the discussion and
write a paragraph on how it was diagnosed, given that it could not be reproduced
on demand.
