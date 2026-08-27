# Truth Tables

A **boolean** value is one of exactly two things: true or false. Chapter 1 called
this one bit, and noted that we would sometimes call the two states 0 and 1 and
sometimes false and true, depending on what we wanted to do with them. Here we
want to do logic.

## The operators

Because there are only two values, an operator taking two booleans has only four
possible input combinations. So we can specify it *completely* by listing all
four. That listing is a **truth table**, and it is an unusually strong kind of
definition — nothing is left to interpretation, and there are no cases we forgot.

**AND** is true when both inputs are true.

| A | B | A AND B |
|---|---|---|
| false | false | false |
| false | true | false |
| true | false | false |
| true | true | **true** |

**OR** is true when at least one input is true.

| A | B | A OR B |
|---|---|---|
| false | false | false |
| false | true | **true** |
| true | false | **true** |
| true | true | **true** |

Note that OR here is *inclusive*: true when both are true. English "or" is often
exclusive — "you may have soup or salad" does not usually mean you may have both
— and this mismatch is a genuine source of bugs when translating a specification
into code. If you want exclusive or, you must ask for it.

**NOT** takes one input and reverses it.

| A | NOT A |
|---|---|
| false | true |
| true | false |

**XOR**, exclusive or, is true when the inputs *differ*.

| A | B | A XOR B |
|---|---|---|
| false | false | false |
| false | true | **true** |
| true | false | **true** |
| true | true | false |

XOR is worth knowing separately because it has a use that is not obvious: it is
"is this different?", and it is its own inverse. Apply XOR with the same value
twice and you are back where you started, which is the basis of a simple cipher
and of several bit-manipulation tricks.

## Java's notation

| Logic | Java |
|---|---|
| AND | `&&` |
| OR | `||` |
| NOT | `!` |
| XOR | `^` |

```java
boolean canVote = age >= 18 && isCitizen;
boolean isWeekend = day == SATURDAY || day == SUNDAY;
boolean isWeekday = !isWeekend;
boolean exactlyOne = a ^ b;
```

## How many operators are there?

A question worth asking, because the answer is finite and small.

An operator on two booleans is a function from four input combinations to a
result. Each of the four rows can produce true or false independently, so there
are $2^{4}$ = **16** possible two-input boolean operators. All sixteen. That is
the complete list, and we have named four of them.

The others include implication (`A` implies `B`, false only when `A` is true and
`B` is false), NAND (not-and), NOR (not-or), and some trivial ones like "always
true" and "ignore B and return A".

That we can count them at all is a consequence of both the inputs and the outputs
being finite — Chapter 1's counting argument, applied to functions rather than
values.

## One operator is enough

Now something genuinely surprising, and it matters for the next lesson.

**Every one of the sixteen can be built from NAND alone.**

NAND is AND followed by NOT: true unless both inputs are true.

| A | B | A NAND B |
|---|---|---|
| false | false | true |
| false | true | true |
| true | false | true |
| true | true | **false** |

Watch. NOT first — feed the same value into both inputs:

```
NOT A  =  A NAND A
```

Check it: if `A` is true, `true NAND true` is false. If `A` is false,
`false NAND false` is true. That is NOT.

Now AND, which is NAND with the result negated, and we just built NOT:

```
A AND B  =  (A NAND B) NAND (A NAND B)
```

And OR, using De Morgan's law from the next lesson:

```
A OR B  =  (A NAND A) NAND (B NAND B)
```

With NOT, AND, and OR you can construct any truth table whatsoever: write down
the rows where the output is true, express each as an AND of inputs and negated
inputs, and OR those together. So NAND alone is sufficient for all of logic.

This property is called **functional completeness**, and it is not a curiosity. A
chip manufacturer who can fabricate one reliable gate type can build every
circuit from it, which simplifies manufacturing enormously. NAND and NOR are both
functionally complete and both are used this way.

Next: what a gate physically is.
