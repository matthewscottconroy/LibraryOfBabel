# Positional Notation

Look at this numeral:

```
742
```

You read it instantly. But what did you actually do?

Not this: you did not look up "742" in a table of number-names. There is no such
table; there could not be, because there are infinitely many numerals and you
have only ever seen finitely many. Whatever you did, it was a *rule* applied to
the parts.

The rule is this. The numeral has three symbols. Each symbol contributes a value
that depends on two things — which symbol it is, and *where it sits*:

```
7 4 2
│ │ └── 2 × 1     =   2
│ └──── 4 × 10    =  40
└────── 7 × 100   = 700
                    ───
                    742
```

The rightmost position is worth 1. The next is worth 10. The next 100. Each
position is worth ten times the one to its right, and a digit's contribution is
the digit multiplied by its position's value.

That is **positional notation**, and it is one of the great inventions. It is
also the thing that has been invisible to you since childhood.

## Why the invisibility matters

Compare Roman numerals. In `MCMXLIV`, the symbols have fixed values — M is
always a thousand, C is always a hundred — and position is used only for a
subtractive trick. You cannot add two Roman numerals by a mechanical procedure
working right to left; there is no such procedure. This is why calculation in
medieval Europe was done on a counting board and only the *result* was written in
Roman numerals. The notation was for recording, not computing.

Positional notation makes computing possible, because it makes arithmetic
*local*. To add two numbers you handle one column at a time, right to left,
carrying when a column overflows. You do not need to comprehend the whole number
at once. You need a small table of single-digit sums and a rule for carrying.

Hold on to that word "local". It is the reason a machine can do arithmetic at
all. A circuit cannot comprehend a number. It can handle one column, and pass a
carry to its neighbour, and that turns out to be enough. Section 2 of this
chapter builds exactly that circuit.

## Ten is arbitrary

Here is the crucial move, and it is a small one.

Nothing in the rule requires the number ten.

The rule says: pick a **base** *b*. The rightmost position is worth 1, the next
*b*, the next $b^{2}$, and so on. Digits run from 0 up to *b* − 1. To find the
value, multiply each digit by its position's value and add.

Ten appears in our version because of an accident of anatomy. Had we evolved with
eight fingers, we would find base eight natural and base ten peculiar, and
schoolchildren would learn a multiplication table with 64 entries instead of 100.
Nothing mathematical would differ. The *numbers* would be the same numbers; only
the *numerals* — the written representations — would change.

This distinction between a number and a numeral is worth pausing on, because it
is Chapter 1's distinction wearing new clothes. The number seven hundred and
forty-two is a quantity. `742` is a pattern of symbols that denotes it under the
base-ten agreement. `2110` denotes the same quantity under the base-seven
agreement. `2E6` denotes it in base sixteen. Same number, three numerals, three
agreements.

## Working a general example

Let us evaluate `2F6` in base sixteen, to show the rule is genuinely mechanical.

Base sixteen needs sixteen digit symbols, and we only have ten, so the convention
borrows letters: A is ten, B eleven, C twelve, D thirteen, E fourteen, F fifteen.

Positions from the right are worth 1, 16, 256:

```
2  F  6
│  │  └── 6  × 1   =   6
│  └───── 15 × 16  = 240
└──────── 2  × 256 = 512
                     ───
                     758
```

Hmm — 758, not 742. I chose those digits to make a point: I asserted above that
`2F6` was 742, and it is not. It is 758. Check my arithmetic; you will find the
rule is fine and my claim was wrong.

I have left the error in deliberately, because this is worth saying once,
early. You should not believe a number in a textbook — including this one —
because it is in a textbook. You should believe it because you checked it or
because you can see why it must be so. The rule is trustworthy; my typing is
not. (742 in base sixteen is `2E6`.)

## Converting into a base

Reading a numeral is straightforward. Going the other way — taking a quantity and
finding its numeral in some base — needs a procedure.

The natural one is repeated division. To write 742 in base seven:

Divide by 7 and keep the remainder. The remainder is the rightmost digit,
because it is precisely the part that does not reach the next position up.

```
742 ÷ 7 = 106 remainder 0     ← rightmost digit
106 ÷ 7 =  15 remainder 1
 15 ÷ 7 =   2 remainder 1
  2 ÷ 7 =   0 remainder 2     ← leftmost digit
```

Read the remainders bottom to top: `2110`.

Check it: 2 × 343 + 1 × 49 + 1 × 7 + 0 = 686 + 49 + 7 = 742. Good.

Which matches the `2110` I claimed above — so that one was fine.

While we are here: there is a faster kind of checking than redoing the
arithmetic. If someone hands you `1338` and calls it a base-seven numeral, you
can reject it without computing anything, because 8 is not a base-seven digit.
Digits always run from 0 to *b* − 1. Learning to spot that sort of impossibility
at a glance is worth more than being quick at conversion.

## Why this generalises

The reason to care is that base two is just this rule with *b* = 2.

The positions become 1, 2, 4, 8, 16, 32 — the powers of two. The digits run from
0 to *b* − 1, which in base two means 0 to 1. Which means each digit is a bit.

Everything else is unchanged. Same rule, same locality, same repeated-division
conversion. If you understand `742`, you already understand binary; you just have
not run the rule with a two in it yet.

That is what we do next.
