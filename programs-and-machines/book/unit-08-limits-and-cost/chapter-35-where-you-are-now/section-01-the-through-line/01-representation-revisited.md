# Representation Revisited

Here is a test of whether the preface's claim was worth making.

Below is a list of eight bugs from eight different chapters — an unsigned integer
misread, a `double` comparison, a wrong charset, a mutable map key. They look like
eight things to memorize. If the claim was any good, they are one thing, and
knowing which one replaces eight rules with a single question.

> A computer holds patterns and changes them. Everything else is an agreement we
> have layered on top.

That was the preface's claim. Here is where it appeared.

## The climb

**Chapter 1.** A voltage is high or low. That this means 1 or 0 is a convention,
and the same eight bits are 200 or $-56$ depending on which agreement you are
using. Nothing in the bits decides.

**Chapters 2 and 3.** Two's complement is an agreement chosen because it makes
subtraction into addition, and it is why the `int` range is asymmetric — the fact
the preface said you had been handed without a reason. Floating point is an
agreement that trades exactness for range, and it is why one tenth does not exist.

**Chapter 4.** Text is the same move again, with more parties to the agreement.
UTF-8 gives common characters short encodings and rare ones long, which Chapter 33
showed was not arbitrary but optimal, and Chapter 29 showed what happens when two
programs disagree about which agreement is in force: `Gödel` becomes `GÃ¶del` and
nothing throws.

**Chapters 6 to 9.** A step is an agreement about what changes. A `switch` on an
enum is a set of possibilities agreed to be closed. Chapter 8's four-entry parity
table knows nothing about parity — the knowledge is in the arrangement, not in any
part, which is the same claim in a smaller room.

**Chapters 15 to 18.** An array is a contiguous block plus the agreement that
`base + i × size` finds element $i$. A collection is a heap of values plus a
representation invariant, and Chapter 16 argued the invariant *is* the data
structure.

**Chapters 19 to 23.** An object is a pattern in memory. That it is an `Account`
is an agreement; that its balance is never negative is an agreement; that two
accounts with the same owner are not the same account is an agreement. Chapter 20's
distinction between identity and equality is the sharpest form of the claim in the
book, because both answers are defensible and the language cannot choose for you.

**Chapters 24 to 27.** The largest instance. A tree of records is a program because
`eval` agreed to treat it as one — Chapter 16's promise, paid in Chapter 25. And
Chapter 27 showed the agreement running in the other direction: generics are an
agreement enforced at compile time and absent at run time, so a `List<String>` is
a `List` with a promise attached.

**Chapters 28 to 31.** A file is bytes plus a format. A protocol is bytes plus
framing, encoding, grammar and sequence. A network connection is a fiction
maintained by two endpoints over a system that knows nothing about it.

**Chapters 32 to 34.** And at the end, the limits of representation itself.
Chapter 33: almost every string has no shorter description, so the things we can
represent compactly are a vanishing corner. Chapter 34: almost every string is
random and no program can identify one, and almost every language has no
recognizer.

## What the claim was for

It was never philosophy, and here is the practical test.

Every one of these is the same bug:

- reading an `int` as unsigned and getting a negative number
- comparing two `double`s with `==` and getting false for equal quantities
- a `String` read in the wrong charset
- `==` on two `Integer`s above 127
- a mutable object used as a map key and then modified
- a `List<String>` containing an `Integer`, smuggled through a raw type
- a CSV field with a comma in it
- ten socket writes arriving as one read

Eight bugs, eight chapters, one cause: **the reader and the writer were operating
under different agreements.** Once you see that they are one bug, you stop
learning them one at a time and start looking for the general question, which is
always *what did the other side assume?*

That is what the claim was for.

## And the asymmetry

One thing worth adding at the end, which could not have been said at the start.

Agreements are not arbitrary. Two's complement was chosen because it makes one
adder do both operations. UTF-8 was designed to be backward compatible with ASCII
and self-synchronizing. Left-associativity in Chapter 24's parser was chosen
because subtraction works that way.

So there are better and worse agreements, and the difference is usually in what
becomes possible afterward. A good representation makes the operations you need
cheap and the errors you fear impossible, and Chapter 22's records and enums are
the purest example: they give up openness and get compiler-enforced guarantees in
exchange.

**Choosing a representation is design**, and it is usually the decision with the
longest reach. Chapter 23's flashcard design turned on separating a card from its
review history; Chapter 29 pointed out that a file format cannot be refactored
once someone has a file. The agreement you pick is the one you and everyone after
you will live inside.

Next: the other idea.
