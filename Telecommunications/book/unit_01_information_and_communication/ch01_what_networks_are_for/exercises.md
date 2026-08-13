# Chapter 1 — Exercises

## A. Recall

**A1.** In your own words, state why *distance* rather than *volume* is the
fundamental problem telecommunications solves. Give one example of a system that
moves enormous quantities of data with no telecommunications involved at all.

**A2.** Identify the six components of Shannon's model for each of the following:
(a) a smoke signal; (b) a courier carrying a USB drive; (c) a Bluetooth mouse;
(d) a smoke detector's chirp when its battery is low.

**A3.** Classify each link as simplex, half duplex, or full duplex, and justify:
(a) FM radio broadcast; (b) a walkie-talkie pair; (c) a modern switched Ethernet
port; (d) a Wi-Fi client associated to an access point; (e) a fibre pair with one
strand per direction.

**A4.** Which of these is a *host*, which is a *node but not a host*, and which is
both in different moments: a laptop; an unmanaged switch; a managed switch being
configured over SSH; a network printer; a fibre media converter?

## B. Apply

**B1.** A fibre route from Sydney to Los Angeles is approximately 12,800 km of
cable. Using a velocity of 204 km/ms, compute (a) the one-way propagation delay,
(b) the round-trip time, and (c) the *minimum* time to complete a TCP three-way
handshake on that path, assuming zero processing delay. (You may take on faith
for now that the handshake costs one and a half round trips before data flows.)

**B2.** A high-frequency trading firm proposes replacing a 1,200 km fibre path
(velocity factor 0.68) with a microwave relay chain along the same route
(velocity factor 0.999, with 14 relay hops each adding 12 µs of processing).
Compute the one-way latency of each and the saving. Then compute how many
kilometres of *additional* microwave path length would erase the advantage.

**B3.** The Chappe optical telegraph could send roughly two symbols per minute,
drawn from a vocabulary of 92 distinguishable arm positions. Estimate its data
rate in bits per second. (Hint: how many bits does it take to identify one of 92
equally likely possibilities? You may leave the answer as log₂92 until Chapter 2
makes it exact.)

**B4.** Explain, using the vocabulary of this chapter, why Whitehouse's increase
in transmit voltage could not have fixed the 1858 cable's problem, even in
principle, if the insulation had been able to withstand it.

**B5.** List the protocols involved in loading a single web page over Wi-Fi, in
the order they are first used. For each, state in one clause what would fail if
it were removed.

## C. Analyse

**C1.** Shannon's model has no notion of a shared channel. Sketch a modification
of the diagram that accommodates *N* transmitters and one receiver, and identify
the new problem that the modified diagram creates but does not solve. (You have
just invented the medium access control problem; Chapter 16 gives the standard
answers.)

**C2.** The chapter claims quantisation error is "noise we manufacture
ourselves." Argue for or against classifying quantisation error as noise in
Shannon's sense. What would it mean, practically, for a receiver design if you
classified it one way rather than the other?

**C3.** Consider a relay network of *k* identical links in series, each of which
independently delivers a message correctly with probability *p*. Derive the
end-to-end success probability. For *p* = 0.99, how many hops before end-to-end
success falls below 90%? What does this imply about where reliability machinery
should live — in each link, or end-to-end? (Keep your answer; Chapter 23 revisits
it as the end-to-end argument.)

## D. Design

**D1.** A field research station in Antarctica needs to send 40 GB of sensor data
per day to a university in Norway, and also needs interactive terminal access for
troubleshooting instruments. Available options: a GEO satellite link (500 ms RTT,
2 Mb/s), a LEO constellation terminal (40 ms RTT, 50 Mb/s but available only
6 hours per day at that latitude), and a fortnightly aircraft that can carry
physical drives.

Design the communications plan. Which traffic goes over which path, and why?
Compute whether each option can carry the bulk load at all. State explicitly which
clause of this book's organising question each of your choices addresses.

## E. Troubleshoot

**E1.** A user reports that "the network is slow." You establish the following:
downloads from a server in the same building complete at 940 Mb/s. Downloads from
a server in Tokyo complete at 3 Mb/s, on a 1 Gb/s Internet circuit that is 4%
utilised. The user's colleague, sitting beside them, sees the same behaviour. A
file transfer from Tokyo using a tool that opens 16 parallel connections achieves
48 Mb/s.

Using only the concepts of this chapter, state what is *not* the problem, and
what category of explanation must be responsible. You do not yet have the
vocabulary to name the mechanism — Chapter 3 will give you the measurement and
Chapter 38 the mechanism — but you should be able to rule out at least three
common wrong answers and say why.
