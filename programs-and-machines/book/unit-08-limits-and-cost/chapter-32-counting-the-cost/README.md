# Counting the Cost

Chapter 18 said: measure, do not guess. That advice stands and it has a gap.

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

Several debts come due here. Chapter 9 promised that halving a search space
repeatedly gives a logarithm. Chapter 13 promised that naive Fibonacci's growth
had a name. Chapter 15 promised that a `LinkedList` can lose a race the theory
says it should win. Chapter 17 promised that the sum of powers of two would come
back. Chapter 25 promised that its interpreter's measured application counts meant
something. All five are paid.

One warning, since this material invites a particular error. Big-O is a statement
about **large inputs**, and it says nothing about small ones. A program with worse
asymptotic behavior can easily be faster for every input you will ever have.
Section 32.2.3 is largely about that, and about the difference between using this
apparatus and hiding behind it.
