# Why 0.1 Is Not 0.1

We have all the pieces. Let us take the most famous result in floating point and
derive it completely, so that nothing about it remains mysterious.

```
0.1 + 0.2  →  0.30000000000000004
```

## Step one: what actually got stored

When you write `0.1` in source code, the compiler converts that decimal text into
the nearest available `double`. From the last lesson, that value is exactly:

```
0.1  →  0.1000000000000000055511151231257827021181583404541015625
```

Slightly *more* than one tenth. The rounding went up.

Do the same for `0.2`. It is one tenth doubled, and doubling in binary is an
exponent change that costs no precision, so the error doubles too:

```
0.2  →  0.200000000000000011102230246251565404236316680908203125
```

Also slightly more than two tenths.

Neither of these is an approximation *of* what the machine holds. Each is exactly
what the machine holds, written out in decimal. A binary fraction with 52
fraction bits is a perfectly definite rational number.

## Step two: add them exactly

Add those two exact values as ordinary rationals:

```
  0.1000000000000000055511151231257827021181583404541015625
+ 0.200000000000000011102230246251565404236316680908203125
  ─────────────────────────────────────────────────────────
  0.3000000000000000166533453693773481063544750213623046875
```

Both inputs were a little high, so the true sum is a little high too.

## Step three: round the result

That sum needs more than 52 fraction bits, so it cannot be stored either. IEEE
754 requires the result to be **correctly rounded**: the arithmetic must produce
the representable value nearest the exact answer, as if the exact sum had been
computed and then rounded once.

The two candidate doubles either side of our exact sum are:

```
0.299999999999999988897769753748434595763683319091796875   ← the double called "0.3"
0.3000000000000000444089209850062616169452667236328125     ← the next one up
```

Our exact sum, 0.30000000000000001665…, sits between them. Which is nearer?

Work out both distances and something unexpected happens:

```
distance down to 0.29999999999999998889…  =  2.77555756156289135105907917022705078125e-17
distance up   to 0.30000000000000004440…  =  2.77555756156289135105907917022705078125e-17
```

They are not merely close. They are **exactly equal**. Our sum lands precisely
halfway between two representable doubles, and "round to nearest" has no nearest
to choose.

IEEE 754 settles ties with a rule called **round half to even**: when a value is
exactly midway, pick the candidate whose last mantissa bit is 0. Of our two, the
lower one ends in a 1 and the upper one ends in a 0, so the upper one wins.

The rule is not arbitrary. Always rounding halves up — the convention you were
taught in school — biases every tie in the same direction, and across a long
computation those biases accumulate into a drift. Rounding to even splits ties
between up and down roughly evenly, so the errors tend to cancel instead of
compounding.

So the result is the upper value, and when Java prints it, it shows the shortest
decimal string that uniquely identifies that particular double — which is
`0.30000000000000004`.

## Step four: the comparison

Meanwhile, when you write the literal `0.3`, you get the *other* one — the
nearest double to three tenths, which is the lower of the two.

So `0.1 + 0.2` and `0.3` are two different doubles, one representable step apart.
They are not equal, and

```
0.1 + 0.2 == 0.3   →  false
```

is correct. Every operation in that chain did the right thing. There is no
inaccuracy anywhere in the arithmetic: each step produced the correctly rounded
result of an exactly specified operation. The mismatch came entirely from the
conversion at the very beginning, when three decimal literals were mapped onto a
grid that does not contain them.

## Why the printed form misleads

One more piece, because it is the thing that makes this feel like trickery.

If the stored value for `0.1` is really 0.10000000000000000555…, why does printing
it show `0.1`?

Because Java's default `double` printing does not show the exact value. It shows
the *shortest decimal string that would round-trip back to the same double*.
Since `0.1` is enough to identify that particular bit pattern uniquely — no other
double is nearer to one tenth — that is what gets printed.

This is a genuinely good design. Showing all 55 digits would be unreadable and
would suggest a precision that is not there. But it has the side effect of hiding
the discrepancy until an operation makes it visible, which is why the error seems
to appear out of nowhere at the moment of addition. It was there all along.

If you want to see the exact value, you have to ask for it — `BigDecimal`
constructed from a `double` will show you every digit, and
`Double.toHexString(0.1)` gives `0x1.999999999999ap-4`, in which you can read the
repeating `9`s of the mantissa and the rounded-up final `a`.

## The general shape

Strip away the specifics and the pattern is one you should expect everywhere:

1. A decimal literal in your source is converted to the nearest representable
   binary value. **This is where error enters.**
2. Arithmetic on representable values is correctly rounded — the best possible
   answer at every step.
3. Printing hides the discrepancy by showing the shortest round-tripping form.
4. A comparison, a subtraction of near-equal values, or a long accumulation makes
   the hidden discrepancy visible, and it looks like the arithmetic went wrong.

It did not go wrong. Step 1 went wrong, quietly, before any arithmetic happened.

Keep that ordering in mind and floating-point surprises stop being surprises.
They become a question with a definite answer: *what was actually stored, and how
far is that from what I wrote?*
