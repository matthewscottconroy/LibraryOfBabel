# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## State

**6.1.** You are halfway through multiplying 47 by 36 using the long
multiplication you learned at school. List everything a stranger would need in
order to finish the calculation. Which items are data, and which are position
within the procedure?

**6.2.** A vending machine accepts 5p, 10p and 20p coins and dispenses when 30p
is reached. What is its state? How many states does it need? Draw the table.

**6.3.** A program uses three `boolean` variables and one `byte`. How many
distinct states can it be in? Show the arithmetic.

**6.4. [carries forward]** The chapter claims an intermittent bug is a machine
behaving deterministically on a state larger than you thought. Give three
concrete things that might be in the state without your having noticed.

## Finite state machines

**6.5.** Trace the parity machine on `110`, `0000`, and the empty string. Give
the final state and whether each is accepted.

**6.6.** Build a table for a machine that accepts strings containing an even
number of `0`s *and* an even number of `1`s. How many states do you need, and
why that many?

**6.7. [carries forward]** Build a machine that accepts strings ending in `01`.
(Hint: the state should record what you have seen most recently that could still
become a match.)

**6.8.** Explain why no finite state machine can accept exactly the strings of
the form *n* zeros followed by *n* ones. Your explanation should refer to the
number of states being fixed in advance.

**6.9.** The parity machine tracks parity rather than a count. Describe a
question about a `0`/`1` string that a finite machine *can* answer, and one it
cannot, and say what distinguishes them.

## Turing machines

**6.10.** Trace the add-one machine from Section 6.2.1 on the tape `111`, with
the head on the rightmost digit. What is on the tape at the end, and is the
arithmetic right?

**6.11.** Write the rule table for a Turing machine that moves right until it
finds a blank, then halts. Two states are enough.

**6.12.** The add-one machine has three rules. Sketch what a subtract-one machine
would need, and say which case makes it harder.

**6.13.** Java can do everything a Turing machine can and no more. Given that,
what exactly does Java give you? Answer in your own words, in three sentences.

## Stored programs

**6.14.** The chapter says being a program is not a property of a pattern but of
how it is used. Restate this in terms of Chapter 1's argument about encodings,
and say what plays the role of "the agreement" here.

**6.15.** List four things that become possible when instructions live in the
same memory as data. For each, say briefly why it is impossible otherwise.

**6.16.** Buffer overflow attacks exploit the same property that makes compilers
possible. Explain the connection. Then say why the defenses named in the chapter
are described as partial rather than complete.

## Going further

**6.17.** Section 6.2.3 claims each layer of the stack is a machine whose state
is data to the layer below. Take one concrete item — the JVM's program counter —
and trace what it is at each of the four layers.

**6.18.** The Church–Turing thesis is called a thesis rather than a theorem.
Explain precisely why it cannot be proved, and say what kind of evidence supports
it instead. Is there any observation that would refute it?
