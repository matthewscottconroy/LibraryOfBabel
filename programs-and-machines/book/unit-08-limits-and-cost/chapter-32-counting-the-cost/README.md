# Counting the Cost

*Measure, do not guess.* That was the rule in Chapter 18 and it still holds. It
also has a hole in it, and this chapter is what goes in the hole.

A measurement tells you what a program cost on one machine, with one input, on one
day. It does not tell you what happens when the input is ten times larger, and
that is usually the question you actually have.

This chapter is about answering it in advance. The tool is a way of describing how
cost **grows** with input size, ignoring everything that does not affect the
growth — the machine, the language, the constant factors. What remains is a
property of the algorithm, and it is what lets you say that a program fine on a
thousand records will not survive a million, without waiting to find out.

Section 32.1 is the apparatus. Counting operations rather than seconds. Big-O
notation, which is a statement about growth and nothing else. And the handful of
growth classes that cover nearly everything you will write, with the difference
between them made visible by measurement rather than assertion.

Section 32.2 is where it meets a machine. Searching and sorting, which is where
the classes become concrete. Space, which is a cost too and is more often the
binding one than people expect. And then a lesson on measuring honestly, which
includes a case where this chapter's own prediction and this chapter's own
measurement do not agree, and the disagreement is left in.

This is also the chapter where a number of loose threads get tied off, and you may
have been carrying some of them for a while.

Halving a search space over and over gives you a logarithm — you were asked to take
that on trust in Chapter 9. Naive Fibonacci grew alarmingly and we said the growth
had a name without saying what it was. A `LinkedList` lost a race the theory
insisted it should win, and the explanation was postponed. The sum of powers of two
turned up in Chapter 17 and was told it would be needed again. And the interpreter
of Chapter 25 counted its own applications, on the promise that the count would
eventually mean something.

Every one of those is settled here.

One warning, since this material invites a particular error. Big-O is a statement
about **large inputs**, and it says nothing about small ones. A program with worse
asymptotic behavior can easily be faster for every input you will ever have.
Section 32.2.3 is largely about that, and about the difference between using this
apparatus and hiding behind it.
