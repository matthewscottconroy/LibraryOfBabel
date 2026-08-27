# Numbers That Do Not End

The last chapter dealt with whole numbers, and the news there was mostly good.
Whole numbers come one after another, so a fixed-width box holds a contiguous run
of them with no gaps. Every value in the range is exactly representable. The only
casualty was the values outside the range, and we learned exactly what happens to
those.

Fractions are worse, and it is worth being honest about how much worse.

Between 0 and 1 there are infinitely many real numbers. Between 0.1 and 0.2 there
are infinitely many. Between any two you care to name there are infinitely many
more. A 64-bit box holds about 18.4 quintillion patterns — an enormous number,
and still infinitely too few. Whatever agreement we adopt, almost every number we
might want will not be in it.

So the question is not "how do we store fractions". It is "which fractions do we
keep, and what do we do about the rest". That is a design problem with no clean
answer, and the answer the world settled on — floating point — is a bargain with
real terms. This chapter is about reading the contract before you sign it.

Here is the thing you have probably already heard about, and which we will
derive rather than memorize. In Java, as in nearly every language you will meet:

```
0.1 + 0.2  →  0.30000000000000004
```

That is not a rounding display artifact and it is not a bug in Java. The two
values being added are not 0.1 and 0.2, because neither can be stored. Their sum
is not 0.30000000000000004 by accident, either — it is the exact and predictable
result of adding the two particular numbers that *were* stored. By the end of the
chapter you will be able to work out that digit sequence yourself.

The first section, **Fractions in Finite Space**, asks what a binary fraction is
and which fractions can be written exactly in base two. The answer turns out to
be a fact about prime factors that you already half-know from decimal: one third
does not terminate in base ten, and for the same underlying reason one tenth does
not terminate in base two. Then we look at the floating-point bargain — how
scientific notation, moved into binary, buys enormous range by spending precision.

The second section, **Living with Approximation**, is practical. Why `0.1` is not
0.1, what goes wrong when you compare two floating-point values with `==`, how
error accumulates when you add many small numbers, and — the part I most want you
to take away — how to recognize the situations where floating point is the wrong
tool and you should refuse it outright.

A note before we start. Almost every treatment of this topic I have seen falls
into one of two failure modes. Either it says "floats are inaccurate, be careful"
and leaves you superstitious, or it opens with the IEEE 754 bit layout and loses
you in fields and biases before you know what problem they solve.

I want to avoid both. Floating point is not inaccurate; it is *exactly* what it
says it is, and every result it produces is the correctly rounded answer to a
question you may not have realized you were asking. The confusion is never in the
arithmetic. It is in the gap between the number you wrote and the number that got
stored — and that gap is completely knowable.
