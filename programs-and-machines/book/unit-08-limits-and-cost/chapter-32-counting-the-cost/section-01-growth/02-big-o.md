# Big-O

The counts in the last lesson were exact, and exactness turns out to be the wrong
thing to want.

An exact count depends on how you chose to count, and it is machine-specific the
moment you care what an operation costs. What survives all of that — what is a
property of the algorithm rather than of your afternoon — is coarser, and the
notation for it is built out of throwing two things away on purpose.

The counts in the last lesson were exact: $n$, $\log_2 n$, $n(n-1)/2$. Exact
counts are more precision than the question needs, and the extra precision is
misleading — it depends on how you counted, and it is machine-dependent the moment
you care about how long an operation takes.

Big-O throws away everything that does not affect growth.

## The definition

> $f(n)$ is $O(g(n))$ if there exist constants $c > 0$ and $n_0$ such that
> $f(n) \le c \cdot g(n)$ for all $n \ge n_0$.

In words: **beyond some point, $f$ grows no faster than $g$, up to a constant
factor.**

Two things are being discarded, deliberately.

**Constant factors.** $c$ absorbs them, so $n$ and $100n$ and $n/2$ are all
$O(n)$. A program twice as fast is the same class.

**Small inputs.** $n_0$ excludes them, so behavior below some threshold does not
count.

Both exclusions are the point and both are where the notation is misused. Big-O
is a statement about *large* inputs *up to a constant*, and using it as though it
said which of two programs is faster is a mistake this chapter will keep
returning to.

## Simplifying

Two rules follow directly.

**Drop constant factors.** $3n^2 + 5n$ is $O(n^2)$.

**Drop lower-order terms.** $n^2 + n$ is $O(n^2)$, because for large $n$ the $n^2$
dominates. At $n = 1000$, $n^2$ is a million and $n$ is a thousand — one tenth of
one percent.

So $n(n-1)/2 = n^2/2 - n/2$ is $O(n^2)$. The $1/2$ goes, the $-n/2$ goes.

That is why the `c = r + 1` idiom, which genuinely halves the work, does not change
the class. It is a real improvement — half the running time is half the running
time — and it is invisible to this notation, which is measuring something else.

## Related notation

Three symbols, and the distinction is worth knowing because the third is what
people usually mean.

$O(g)$ — grows **no faster than** $g$. An upper bound.
$\Omega(g)$ — grows **no slower than** $g$. A lower bound.
$\Theta(g)$ — both. A tight bound.

Linear search is $O(n)$ and also $\Theta(n)$. It is also, correctly, $O(n^2)$ and
$O(2^n)$ — those are true upper bounds and useless ones.

Almost everyone writes $O$ where they mean $\Theta$, and this is harmless in
practice because the intent is understood. It is worth knowing that "this is
$O(n^2)$" is technically compatible with the algorithm being linear.

## Reading code

Common shapes, and recognizing them is most of the skill.

**A simple statement** — $O(1)$.

**A loop over the input** — $O(n)$.

**Two loops in sequence** — $O(n) + O(n) = O(n)$. Sequential work adds, and adding
does not change the class. Chapter 14's remark that three passes over an array are
usually a cost worth paying for clarity is this fact: three passes and one pass
are both $O(n)$.

**Nested loops over the input** — $O(n^2)$. Nested work multiplies.

**Halving each step** — $O(\log n)$.

**A loop with a halving inside it** — $O(n \log n)$.

**Two recursive calls per level** — $O(2^n)$, unless the recursion tree is shallow.

The general procedure: **find the innermost expensive operation and count how many
times it runs.** Everything else is arithmetic.

Watch for costs hidden inside method calls. This looks linear:

```java
for (String s : list)
    if (otherList.contains(s)) count++;
```

and is $O(n \cdot m)$, because `List.contains` is itself a linear scan. Using a
`HashSet` for `otherList` makes it $O(n)$, and that substitution is one of the
highest-value changes available in ordinary code.

Similarly, string concatenation in a loop is $O(n^2)$ because each `+` copies the
whole accumulated string — Chapter 18's measured 84-times `StringBuilder`
improvement, now with a reason attached.

## Why logarithms have no base

$\log_2 n$ and $\log_{10} n$ differ by a constant factor:

$$\log_2 n = \frac{\log_{10} n}{\log_{10} 2} \approx 3.32 \log_{10} n$$

Constant factors are discarded, so all logarithms are the same in big-O and the
base is conventionally omitted. Writing $O(\log n)$ says nothing about which base,
because it cannot.

## What it does not tell you

The list is worth having explicitly, because every item is a real mistake people
make.

**It says nothing about small inputs.** Verified, searching 16 elements: linear
took 16 comparisons and binary took 5. A three-fold difference, not the
four-hundred-thousand-fold difference at ten million. And below some size the
simpler algorithm wins outright once the constants are included, which is why real
sort implementations switch to insertion sort for small subarrays.

**It says nothing about constants.** Two $O(n \log n)$ sorts can differ by a factor
of ten. That factor is your whole runtime and big-O cannot see it.

**It says nothing about memory access.** Chapter 15 measured a three-times
difference between row-major and column-major traversal of the same array, doing
the same number of operations. Both are $O(n^2)$. Section 32.2.3 develops this.

**It says nothing about which operations.** An $O(n)$ algorithm doing $n$ disk
reads is far slower than an $O(n^2)$ algorithm doing $n^2$ register operations, up
to a substantial $n$.

The honest summary: **big-O tells you how a program will behave as the input grows,
and nothing else.** That is a genuinely valuable thing to know in advance, and it
is one input to a decision rather than the decision.

Next: the classes themselves, and what each one costs.
