# Kolmogorov Complexity

Two strings, each a thousand characters. One is `abab` repeated five hundred times.
The other came from a random number generator.

Everyone agrees the second is more complicated. Chapter 33's entropy cannot quite
say why — entropy is defined over a *source*, and here there are two strings and no
probabilities in sight. So what is it that we are all agreeing about?

Two strings, each a thousand characters:

```
ababababababababababab ... ab
```

```
4c1j5b2p0cv4w1x8rx2y39umgw5q85s7 ... uhy4
```

The same length. They are not equally complicated, and Chapter 33's entropy cannot
quite say why — entropy is defined over a *source*, and here we have two strings
and no probabilities.

Kolmogorov's answer needs none:

> The **Kolmogorov complexity** $K(s)$ of a string $s$ is the length of the
> shortest program that outputs $s$ and halts.

The first string has a short program:

```java
for (int i = 0; i < 500; i++) System.out.print("ab");
```

About sixty characters, regardless of whether the string is a thousand characters
or a million — the loop bound grows logarithmically.

The second has no such program. The shortest way to produce it is, as far as
anyone can tell, to include it:

```java
System.out.print("4c1j5b2p0cv4w1x8rx2y39umgw5q85s7 ... uhy4");
```

That is about a thousand characters plus a constant.

**A string's complexity is the length of its shortest description.** Which is a
clean definition of information, needing no model of what was likely, and it is a
property of the string rather than of a source.

## The language does not matter much

An obvious objection: the shortest program depends on the language.

It does, and only by a constant. If $L_1$ and $L_2$ are both Turing complete,
there is an interpreter for $L_2$ written in $L_1$, of some fixed length $c$. So
any $L_2$ program can be turned into an $L_1$ program by prepending the
interpreter:

$$K_{L_1}(s) \le K_{L_2}(s) + c$$

and $c$ depends on the two languages and not on $s$. So for long strings the
choice of language is negligible, and $K$ is well defined up to an additive
constant.

That is the **invariance theorem**, and it is what makes the definition
respectable. It is also, in a sense you have already seen, Chapter 25: the reason
languages are interchangeable up to a constant is that you can write one inside
another, which you have done.

## Most strings are incompressible

Chapter 33 proved this by counting and it is worth restating in this language,
because it is the same argument.

How many strings of length $n$ have $K(s) < n - k$? Each such string has a program
shorter than $n - k$ bits, and there are fewer than $2^{n-k}$ such programs. So at
most $2^{n-k}$ strings out of $2^n$ — a fraction of $2^{-k}$.

**Fewer than one string in $2^{10}$ can be described in ten bits fewer than its
length.** Fewer than one in a million in twenty fewer.

So almost every string is incompressible, which is Chapter 33's conclusion in
Kolmogorov's terms. The strings we care about — text, images, programs, this
sentence — are the vanishing exception, and that is why compression works at all
and why it works only on them.

## K is uncomputable

Here is the result, and the proof is a good one.

> **No program can compute $K(s)$.**

Suppose one could. Then write a program that, given $n$, searches all strings in
order and returns the first with $K(s) > n$.

Such a string exists, by the counting above — most strings of length $n + 1$ have
complexity above $n$ — so the search terminates.

Now: that program is short. It contains the code for $K$, a loop, and the number
$n$, which takes about $\log_2 n$ bits to write. Call its total length
$c + \log_2 n$, where $c$ is a constant not depending on $n$.

But the program **outputs a string whose complexity exceeds $n$**, and a program of
length $c + \log_2 n$ that outputs $s$ shows $K(s) \le c + \log_2 n$.

So:

$$n < K(s) \le c + \log_2 n$$

For large enough $n$ that is false, since $\log_2 n$ grows more slowly than $n$.
Contradiction, so $K$ is not computable.

This is the **Berry paradox** made rigorous — "the smallest number not describable
in fewer than twelve words" is a description of it in eleven — and Chaitin turned
that piece of wordplay into a theorem.

Note the shape once more: a short description that produces something which,
by assumption, has no short description. Section 34.1.2's `trouble` was a program
contradicting its own prediction; this is a program contradicting its own length.

## Two definitions of information

Chapter 33 and this chapter give different answers to the same question, and both
are right about different things.

| | Shannon entropy | Kolmogorov complexity |
|---|---|---|
| applies to | a source | a single string |
| needs | probabilities | nothing |
| measures | average surprise per symbol | length of shortest description |
| computable | yes | no |
| used for | channel capacity, coding | theory, foundations |

They agree where both apply: for a source of entropy $H$, the expected Kolmogorov
complexity of an $n$-symbol output is about $nH$. So entropy is, in a precise
sense, the average of $K$.

The trade is exact and it is worth naming. Shannon's measure is computable and
requires a model. Kolmogorov's needs no model and is uncomputable. **You can have
a definition that assumes something, or one you cannot evaluate.**

That trade is not unique to information theory, and recognizing it is one of the
more portable things in this book. It is the same shape as Section 34.1.3's tools:
sound, complete, terminating — pick two.

## Where it is used

$K$ cannot be computed and it is not useless.

**Bounds.** Approximating $K(s)$ by the size of a compressed version — a real
compressor is an upper bound on $K$ — gives a usable measure. **Normalized
compression distance** compares two objects by how much better they compress
together than separately, and it works surprisingly well for clustering texts,
genomes, and even music, with no domain knowledge at all.

**Minimum description length.** A principle for model selection: prefer the model
minimizing the total of the model's description and the data's description given
the model. That is Occam's razor made quantitative, and it is a formal argument
against overfitting — a model with a parameter per data point describes nothing
briefly.

**Proofs.** The incompressibility method proves lower bounds by arguing that if an
algorithm were faster, some incompressible string could be described too briefly.
It is a standard tool in complexity theory.

**Defining randomness**, which is the next lesson.

Next: what it means for a string to be random.
