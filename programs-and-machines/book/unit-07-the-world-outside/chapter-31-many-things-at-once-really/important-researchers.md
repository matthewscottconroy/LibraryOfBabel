# Important Researchers

**Edsger W. Dijkstra** (1930–2002) returns for the fourth time, and this is
arguably his largest contribution. In 1965 he posed and solved the mutual
exclusion problem, invented the **semaphore**, and gave the field the vocabulary it
still uses — critical section, mutual exclusion, deadlock. The Dining Philosophers
problem is his, invented as an exam question to make the deadlock and starvation
issues concrete, and Section 31.1.3's lock ordering is one of the standard
solutions to it. He also identified the banker's algorithm for deadlock avoidance.
That an entire subject's core vocabulary comes from one person over a few years is
unusual even for Dijkstra.

**Leslie Lamport** (born 1941) made distributed systems a subject with theory
rather than a collection of tricks. His 1978 paper "Time, Clocks, and the Ordering
of Events in a Distributed System" is one of the most cited in computing: it
observes that without a shared clock, the only meaningful ordering is causal, and
gives the logical clocks that make it precise. He posed the Byzantine Generals
problem in 1982, designed Paxos — the consensus algorithm underlying most
distributed databases — and, incidentally, wrote LaTeX. Turing Award, 2013.

**C. A. R. Hoare** (born 1934) appears again for **monitors**, which he proposed
in 1974 with Per Brinch Hansen: an object that bundles data with the lock
protecting it, so that synchronization is a property of a type rather than a
discipline callers must follow. Java's `synchronized` is a direct implementation.
He also wrote *Communicating Sequential Processes* in 1978, arguing that
concurrency is better expressed by processes exchanging messages than by threads
sharing memory — the model Erlang and Go adopted, and Section 31.1.3's last
recommendation.

**Per Brinch Hansen** (1938–2007) developed monitors alongside Hoare and built the
first operating system structured around them. He argued that concurrent
programming should be safe by construction — that a language should make it
impossible to touch shared data without holding its lock — rather than relying on
convention. Java did not go that far, and most concurrency bugs in Java are cases
of what he wanted the compiler to prevent.

**Doug Lea** (born 1955) wrote `java.util.concurrent`. The executors, the atomics,
`ConcurrentHashMap`, the fork/join framework, the locks — all of it is his, and it
turned Java from a language with `synchronized` and hope into one with a serious
concurrency library. He also co-authored the Java Memory Model revision that made
`volatile` and `final` mean something precise. Section 31.1.3's advice to use the
library rather than write your own synchronization is, in effect, advice to use
his code rather than yours.

**Gene Amdahl** (1922–2015) designed the IBM System/360 and, in a 1967 conference
paper arguing against parallel processing, stated the law that bears his name. He
was wrong about the conclusion and right about the arithmetic, which is a better
outcome than the reverse. The paper is two pages.

**Rob Pike** (born 1956) returns from Chapter 28 for the concurrency/parallelism
distinction and for Go's channels, which put Hoare's CSP into a mainstream
language. His talk "Concurrency is not Parallelism" is the clearest statement of
Section 31.1.1's difference, and it is twenty minutes.
