# Exercises

**34.1** Implement the five-instruction machine from Section 34.1.1 with a step
budget. Run the three example programs on inputs 0, 3 and 10, and reproduce the
table. Then explain why `countdown` does not halt on input 0.

**34.2** Run `countdown` from 500 with a budget of 100 and then of 2000. State
precisely what the first answer tells you and what it does not.

**34.3** Implement the Collatz loop with a step budget. Report the step counts for
6, 7, 27, 97, 871 and 6171, and find the longest-running start under 100,000. Then
state what would follow if someone wrote a correct `halts` procedure.

**34.4** Write out the halting-problem proof in your own words, without looking at
Section 34.1.2. Then check it. The two things to get right are that `trouble` is an
ordinary program and that both cases contradict.

**34.5** Build a finite version of the diagonal table: six programs, six inputs,
arbitrary entries, and the flipped diagonal. Confirm the flipped row differs from
every row, and say what the infinite version needs that the finite one does not.

**34.6** Show by reduction that "does this program ever print `hello`?" is
undecidable. Then do the same for "does this program ever reach line 42?".

**34.7** State Rice's theorem and then classify each of these as decidable or not,
with a one-sentence reason: does the source contain a `goto`; does it ever divide
by zero; is every variable declared before use; does it terminate on all inputs;
is it longer than 100 lines.

**34.8** Write a Java method that the compiler rejects for possibly-uninitialized
use of a variable, where you can see the variable is always assigned. Explain which
of sound, complete and terminating the compiler chose to give up.

**34.9** For each of a type checker, an antivirus scanner, and a bounded model
checker, say which two of sound, complete and terminating it has, and what the
consequence is for its users.

**34.10** Estimate the Kolmogorov complexity of: a million `a` characters; the
first million digits of pi; a million bytes from `SecureRandom`; this exercise.
Justify each in a sentence.

**34.11** Explain why $K$ depends on the choice of language only up to an additive
constant. Your explanation should mention an interpreter, and should connect to
Chapter 25.

**34.12** Reconstruct the proof that $K$ is uncomputable. Then say which step
would fail if the search for a high-complexity string might not terminate.

**34.13** Compute what fraction of 1000-bit strings can be compressed by 20 bits or
more. Then state, in one sentence, what that says about how special the files on
your computer are.

**34.14** *Measurement.* Generate a megabyte from `new Random(42)` and compress it.
Report the ratio. Then explain why the ratio is what it is, and why the string's
Kolmogorov complexity is nevertheless only a few hundred bits.

**34.15** *Design, no code.* Your team wants a tool that reports every possible
null dereference in a codebase, with no false positives and no false negatives, and
always finishes. Explain why they cannot have it, and propose which property to
give up and why.

**34.16** *Reading, no code.* [carries forward] Look up the Busy Beaver function. State what it
computes, and explain — using this chapter — why it grows faster than any
computable function.
