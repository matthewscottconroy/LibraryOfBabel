# Overflow Is Not a Bug

Here is the fact that started the chapter. In Java:

```
2147483647 + 1  ==  -2147483648
```

The largest `int` plus one is the smallest `int`. No exception is thrown. No
warning is printed. The program continues with a number that is off by about
four and a quarter billion.

You now have everything needed to see why, and — more usefully — to see that
nothing went wrong.

## Working it out

An `int` is 32 bits, two's complement. The largest value, 2,147,483,647, is
$2^{31}$ − 1, and its pattern is a 0 followed by thirty-one 1s:

```
0111 1111 1111 1111 1111 1111 1111 1111
```

Add one. The rightmost column becomes 0 and carries. The next becomes 0 and
carries. Every column of 1s does the same, all the way along, until the carry
reaches the leading 0 and turns it into a 1:

```
1000 0000 0000 0000 0000 0000 0000 0000
```

Now read that pattern back under two's complement. The leading bit is set, so
this is a negative value, and its weight is −$2^{31}$. Every other position is zero.
The value is −2,147,483,648.

Which is exactly what Java reports.

Nothing malfunctioned. The adder added, one column at a time, exactly as
designed. The result is the correct pattern for "one more than the largest
representable value" — it is that the pattern in question means something
far away, because we agreed to give the top bit a negative weight.

## Two kinds of "too big"

It helps to separate two things that both get called overflow.

**Unsigned overflow** is the carry falling off the left end, as in 200 + 100 = 44.
The true answer needed a ninth column and there wasn't one.

**Signed overflow** is what we just saw: the carry did *not* fall off the end.
It landed in the top bit — the one carrying negative weight — and flipped the
result's sign. Nothing was discarded; the answer wrapped around the
circle.

Processors detect these with two separate status flags, usually named "carry" and
"overflow". Java exposes neither, which is a deliberate language decision we will
come back to.

The rule for signed overflow is worth knowing because it is short: it can only
happen when you add two numbers of the *same* sign and get a result of the
opposite sign. Adding a positive and a negative can never overflow — the result
lies between them, so it is already representable.

## Why Java does not stop you

You may reasonably ask why the language does not just throw an exception.

Speed is the honest first answer. Addition is the most frequently executed
operation in a processor, and checking every one for overflow costs something on
every single add — a test and a branch. The languages Java descends from decided
this was not worth it, and Java followed the convention.

The deeper answer is that wrapping is genuinely wanted often enough. Hash
functions rely on it; a hash is *supposed* to churn its bits and wrap freely.
Random number generators rely on it. Checksums rely on it. If arithmetic threw an
exception on wrap, all of these would need special handling.

Java does give you a choice, and it is worth knowing before you need it. The
`Math` class provides `addExact`, `subtractExact`, and `multiplyExact`, which
perform the same arithmetic but throw an `ArithmeticException` on overflow. When
you are computing something where a wrapped answer would be a disaster — a
financial total, an array size, a timeout — those are the methods to use. They
cost a little; the cost is almost never the thing that matters.

## Where this actually bites

Three real cases, so this does not stay abstract.

**The midpoint bug.** To find the middle of a range you might write
`(low + high) / 2`. For arrays of ordinary size this is fine. When `low` and
`high` are both large, `low + high` overflows to a negative number, and the
midpoint lands outside the array. This bug lived in the binary search in the Java
standard library for around nine years, and Joshua Bloch wrote it up in 2006 in a
piece titled "Extra, Extra — Read All About It: Nearly All Binary Searches and
Mergesorts Are Broken". The fix is `low + (high - low) / 2`, which cannot
overflow because the difference is always in range.

If a bug of that shape can survive nine years in the standard library of a major
language, written and reviewed by very good engineers, it is not a sign of
carelessness. It is a sign that the underlying arithmetic does not behave the way
intuition expects, which is the entire argument of this unit.

**Ariane 5.** In 1996 the maiden flight of the European Ariane 5 rocket was
destroyed about forty seconds after launch. The cause, per the official inquiry
board chaired by Jacques-Louis Lions, was a conversion of a 64-bit floating-point
value — horizontal velocity — into a 16-bit signed integer. Ariane 5 flew faster
than Ariane 4, the value did not fit, and the resulting exception led both
inertial reference systems to shut down. The number was too large for the box it
was being put into.

**The 2038 problem.** Much software records time as the number of seconds since
1 January 1970, in a signed 32-bit integer. That counter reaches $2^{31}$ − 1 on 19
January 2038, and the next second wraps it to the most negative value —
placing the time in December 1901. This is the same arithmetic as
`2147483647 + 1`, with a calendar attached. Systems using 64-bit time values are
unaffected, which is most modern ones, but not all of them.

## The habit to build

There is a question worth asking whenever you write arithmetic that matters:

*What is the largest value this can take, and does it fit?*

Not "will it usually fit". The Ariane 4 code was correct for Ariane 4. The binary
search was correct for the array sizes anyone tested. Overflow bugs are
characteristically dormant — they lie quiet through every test you thought to
write, and fire when the inputs finally get large enough.

You will not ask this question about every line, and you should not. But when the
value is a size, a total, a duration, or anything that grows with your program's
success, ask it.

## Closing the chapter

We began with the claim that a machine holds patterns, not numbers. This chapter
built the most important agreement layered on those patterns — positional
notation in base two — and then followed it honestly to its consequences.

Fixed width means finitely many patterns. Finitely many patterns means a bounded
range. A bounded range means some sums have no representable answer, and
something must happen anyway. Two's complement makes the something be a wrap, and
in exchange gives us subtraction for free.

Every part of that is a consequence of the previous part. None of it is arbitrary
and none of it needs memorizing.

Next we ask the harder version of the same question. Whole numbers at least come
one after another, so a finite range holds all of them within it. Between any two
fractions there are infinitely many more. What can a finite machine possibly do
with that?
