# Representation

Pick up the device you are reading this on and consider a genuinely strange
question: where, physically, is the last message you received?

You know the answer is "in the phone", but that answer is doing a lot of work.
Somewhere inside there is a region of silicon in which some number of tiny
structures are holding electrical charge and some number are not. That is all
that is there. There is no text in the phone. There is no picture of your friend
in the phone. There is charge, arranged in a pattern, and an enormous stack of
agreements about how to read that pattern.

This unit is about those agreements.

Most introductions to programming start with a program. You type something,
something appears, and the mystery of what happened in between is deferred —
often permanently. That approach gets you writing code faster, and I understand
its appeal. But it builds on sand, and you find out that it built on sand at the
worst possible moment: when something goes wrong in a way the rules you were
given do not explain.

Consider three things that will happen to you if you program for any length of
time. You will add one to a positive number and get a negative number. You will
add `0.1` and `0.2` and get `0.30000000000000004`. You will read a file that
someone else wrote and find `café` has become `cafÃ©`.

Each of these looks like a malfunction. None of them is. In every case the
machine did exactly what it agreed to do, and the surprise lives entirely in the
gap between what it agreed to and what you assumed. If you understand
representation, all three become predictable — not memorized as quirks, but
*derivable* from what a finite machine can hold. That is the difference this
unit is trying to make.

Five chapters get us there.

**Chapter 1 — Two Voltages and an Agreement** settles what a bit is, and why the
choice of two states rather than ten is an engineering decision rather than a
mathematical necessity. Then the central idea of the unit: an encoding is a
convention, and the same pattern of bits means whatever we have agreed it means.

**Chapter 2 — Numbers That End** covers positional notation, counting in binary,
hexadecimal as a shorthand for human eyes, and then arithmetic inside a
fixed-size box — including two's complement, the trick that makes subtraction
free, and overflow, which is its price.

**Chapter 3 — Numbers That Do Not End** takes on fractions in finite space.
Floating point is a bargain: enormous range in exchange for the guarantee that
most numbers you write cannot be stored exactly. We look at what that bargain
costs and when you should refuse it.

**Chapter 4 — Text and Other Agreements** follows how letters became numbers, how
that worked adequately for English and badly for everyone else, and how Unicode
attempts to repair it. Then a wider look: color, sound, and images turn out to be
the same question asked again.

**Chapter 5 — Your First Instrument** is where Java arrives. Not as the subject —
as a tool for looking at the bits we have spent four chapters discussing. You
will write your first program, and it will be a program that shows you what is
actually in memory.

You may have noticed that this is a Java book in which Java does not appear for
four chapters. That is deliberate, and it is the most unusual decision in the
book, so let me defend it.

If I show you Java first, every idea in this unit becomes a footnote to a
language feature. Overflow becomes "a thing `int` does". Floating point becomes
"a thing `double` does". You would learn them as trivia attached to keywords.

If we build the ideas first, the language features arrive as *answers*. By the
time you meet `int` in Chapter 5, you will already know why a fixed-width integer
type must wrap around, and `int` will be a particular instance of something you
understand rather than a rule you were handed. That is worth four chapters.

It also means the ideas transfer. Nothing in Chapters 1 through 4 is about Java.
It is about what any digital machine can hold, which is why it will still be true
in whatever language you are writing in ten years from now.
