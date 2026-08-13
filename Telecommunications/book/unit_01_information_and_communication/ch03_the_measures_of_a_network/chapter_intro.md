# Chapter 3 — The Measures of a Network

There is a conversation that happens in every organisation on Earth, and it goes
like this.

*"The network is slow."*

*"I'll get you a bigger circuit."*

And then the circuit is upgraded from 100 Mb/s to 1 Gb/s at considerable expense,
and the users report that the network is still slow, and everyone concludes that
the network is mysterious and that networking people are unreliable. The
particular tragedy of this exchange is that it was entirely predictable, and that
the prediction required nothing more than one measurement taken before the money
was spent.

The problem is that "slow" is not a measurement. It is a symptom, and it maps onto
at least four distinct and independent physical quantities, only one of which is
fixed by buying a bigger pipe. Confusing them is the most common error in
practical networking, it is committed by experienced people daily, and eliminating
it is worth more to your career than any protocol in this book.

The four quantities are **bandwidth**, **latency**, **jitter**, and **loss**. They
are independent. A link can have enormous bandwidth and terrible latency (a
satellite feed). It can have low bandwidth and excellent latency (a serial cable
between two adjacent machines). It can have both and still be unusable for voice
because of jitter. It can look perfect on every graph and be dropping 0.5% of
packets, which is enough to reduce a TCP transfer to a crawl while barely
registering as an error rate.

## The one story to keep in mind

In 1996, the physicist and networking researcher **Stuart Cheshire** wrote an
essay whose title has become the standard shorthand for this entire chapter:
*It's the Latency, Stupid*. His central observation is a piece of arithmetic
anyone can check.

Suppose you must move a 1 GB file. On a 10 Mb/s link with 10 ms round trip, and
on a 10 Mb/s link with 500 ms round trip, the *bandwidth* is identical. But TCP
cannot send more than one window of data before waiting for an acknowledgement,
and if the window is 64 KB — as it was on essentially every operating system in
1996 — then the maximum achievable throughput is

$$\frac{65{,}536 \times 8 \ \text{bits}}{0.5 \ \text{s}} \approx 1.05 \ \text{Mb/s}$$

on the high-latency link, no matter how much bandwidth you buy. Ten megabits of
capacity delivering one megabit of throughput, and every additional megabit
purchased delivering exactly nothing. Cheshire's essay was written in irritation at
a modem vendor, and it is still, thirty years later, the thing most worth
understanding about network performance.

That calculation is the **bandwidth–delay product**, and §3.4 derives it properly.
It explains the exercise you were given at the end of Chapter 1 — the transfer
from Tokyo that ran at 3 Mb/s on a 4%-utilised gigabit circuit, and reached
48 Mb/s when split across sixteen parallel connections. Sixteen connections, each
limited by the same window, sixteen times the throughput. Nothing about the
circuit changed.

## What this chapter does

We define the four quantities carefully, one section at a time, and — more
importantly — we define what *distinguishes* them, so that a symptom can be
attributed to the right one.

§3.1 separates **bandwidth** (capacity), **throughput** (achieved rate), and
**goodput** (achieved rate of useful payload), and computes the overhead that
separates the last two, because a "1 Gb/s" link never delivers a gigabit of file
and it is worth knowing exactly where the missing bits went.

§3.2 decomposes **latency** into its four independent components — propagation,
transmission, processing, and queueing — of which only one is fixed by physics and
only one grows under load. Being able to say which component a delay lives in is
the difference between a diagnosis and a guess.

§3.3 covers **jitter** and **loss**, the two quantities that averages conceal, and
explains why a link with excellent mean latency can be unusable for a voice call.

§3.4 assembles them into the bandwidth–delay product, the "long fat pipe" problem,
and the reason window scaling had to be invented.

## By the end you will be able to

- Compute throughput, goodput, and protocol overhead for a real frame size.
- Decompose a measured round-trip time into its four components and say which one
  a given change would affect.
- Predict the maximum single-stream TCP throughput on a path from its RTT and
  window size, and compute the window needed for a target rate.
- Explain, with numbers, why adding bandwidth does not fix a latency-bound
  transfer, and identify from a symptom description which of the four quantities
  is the binding constraint.
- Distinguish jitter from latency and explain why a jitter buffer trades one for
  the other.

## Where this sits in the argument

Chapter 2 made information countable. This chapter makes its *movement*
measurable. Chapter 4 then proves there is a hard ceiling on the first quantity,
bandwidth, that no engineering can exceed.

Everything in Unit XIII — the entire troubleshooting discipline — is built on the
distinctions drawn here. If you read only one chapter of Unit I before starting
practical work, read this one.
