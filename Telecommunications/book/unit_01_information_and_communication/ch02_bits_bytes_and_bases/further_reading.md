# Chapter 2 — Further Reading

## Primary sources

**Shannon, C. E. (1938). "A Symbolic Analysis of Relay and Switching Circuits."
*Transactions of the AIEE* 57(12): 713–723.**
The published version of the 1937 master's thesis. Readable, and startling in
retrospect: you can watch digital design being invented in real time. The thesis
itself is in the MIT archives and is also worth a look for the hand-drawn circuit
diagrams.

**Cohen, D. (1980). "On Holy Wars and a Plea for Peace." IEN 137; reprinted in
*IEEE Computer* 14(10), October 1981.**
Four pages, funny, and it settled network byte order. Read it. It is also a
model for how to argue that a decision matters more than which decision you make —
a rhetorical move you will need if you ever sit on a standards committee.

**Pike, R. & Thompson, K. (1993). "Hello World, or Καλημέρα κόσμε, or こんにちは
世界." Proceedings of the Winter 1993 USENIX Conference.**
The UTF-8 paper. Short, and the design rationale section is a compact lesson in
what "self-synchronising" buys you.

**ANSI X3.4-1963, *American Standard Code for Information Interchange*.**
Worth skimming for the control character definitions, which are almost entirely
telecommunications concepts (SOH, STX, ETX, ACK, NAK, SYN, DLE). ASCII was
designed for wires.

**RFC 1700, *Assigned Numbers* (1994), §"Data Notations".**
The canonical statement of network byte order for the Internet protocol suite.
The RFC as a whole is obsolete, but this section is where the convention is
formally pinned.

**RFC 3629, *UTF-8, a transformation format of ISO 10646* (2003).**
The current specification. Four pages of substance. Note especially §3's table of
byte patterns and §10's security considerations, which cover overlong encodings —
an attack class that has produced real vulnerabilities in path-traversal filters.

## Books

**Petzold, C. (1999, 2nd ed. 2022). *Code: The Hidden Language of Computer
Hardware and Software.* Microsoft Press.**
The best book ever written for someone who wants to genuinely understand binary,
Boolean logic, and how bits become computers. It starts with two children
signalling with flashlights and ends with a working computer, and it never skips a
step. If §2.1 or §2.2 felt fast, read this. If they felt slow, read it anyway.

**Gleick, J. (2011). *The Information.* Pantheon.**
Chapters 5–7 cover Morse code, Baudot, and the emergence of the bit, with more
historical texture than any technical source.

**Seife, C. (2006). *Decoding the Universe.* Viking.**
On the relationship between information and thermodynamics, including Landauer's
principle and Maxwell's demon. Popular-level, occasionally overreaching, but the
Landauer chapters are sound and connect this chapter to Chapter 4.

**Kernighan, B. W. & Pike, R. (1999). *The Practice of Programming.* Addison-
Wesley.**
Chapter 8 on portability covers byte order and data representation with the
authority of people who got it wrong first. Relevant if you will ever write code
that touches a socket.

## Reference and practice

**Unicode Consortium, *The Unicode Standard*, current version.**
Not for reading. For knowing it exists, and for looking up why a particular
character behaves strangely. Chapter 2 of the standard, "General Structure," is
the readable part.

**Spolsky, J. (2003). "The Absolute Minimum Every Software Developer Absolutely,
Positively Must Know About Unicode and Character Sets (No Excuses!)."**
A widely circulated essay, and still the fastest way to internalise §2.4's
principle that bytes without an encoding are meaningless. Its central slogan —
"it does not make sense to have a string without knowing what encoding it uses" —
is exactly the *location plus agreement* idea generalised.

**`subnet_practice.py`** in this book's [tools/](../../../tools/) directory.
Generates unlimited binary-conversion and masking drills with worked solutions.
Use it until §2.2's operations are automatic; the investment is repaid in
Chapter 26 with interest.

## For the certification-minded

Hexadecimal, binary conversion, and the powers of two are not examinable topics in
themselves on N10-009. They are the *prerequisites* for objective 1.7 (IPv4
addressing, subnetting, VLSM, CIDR), which is heavily examined and which candidates
most commonly fail. Every hour here is an hour not spent memorising subnet charts
that will desert you under time pressure.
