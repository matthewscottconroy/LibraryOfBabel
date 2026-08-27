# Recursion and Induction

The last lesson asked you to assume the recursive call works. This one explains
why you are entitled to.

## Induction

**Mathematical induction** proves a statement about all natural numbers in two
steps.

**Base case:** prove it for the smallest value, usually 0 or 1.

**Inductive step:** prove that *if* it holds for *n*, *then* it holds for *n*+1.

Together these give the statement for every natural number. It holds for 0. Since
it holds for 0, the step gives 1. Since it holds for 1, the step gives 2. And so
on — not by checking each, but because the step covers all of them at once.

The standard example: prove that 1 + 2 + … + *n* = *n*(*n*+1)/2.

*Base:* for *n* = 1, the sum is 1 and the formula gives 1(2)/2 = 1. Holds.

*Step:* assume it holds for *n*. Then the sum to *n*+1 is *n*(*n*+1)/2 + (*n*+1),
which factors to (*n*+1)(*n*+2)/2 — the formula with *n*+1 in place of *n*. Holds.

Done. True for all *n*, with two checks.

## The correspondence

Now put a recursive method beside it.

| induction | recursion |
|---|---|
| base case | base case |
| inductive step | recursive case |
| assume it holds for *n* | assume the recursive call is correct |
| prove it for *n*+1 | build this answer from the smaller one |
| conclusion: true for all *n* | conclusion: correct for all inputs |

They are the same argument. When you write:

```java
static int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}
```

you have written the base case of an induction and its inductive step. The
"assumption" of the last lesson is the induction hypothesis, and it is legitimate
for exactly the reason the induction hypothesis is legitimate.

**Writing a correct recursive method is constructing a proof.** That is why the
correctness argument is two sentences: proofs by induction are two steps.

## Back to loops

Chapter 9 said loop invariants are induction too. So loops and recursion are the
same principle in different notation, which is worth making explicit:

| | loop | recursion |
|---|---|---|
| the claim | invariant | the method's contract |
| base | establishment | base case |
| step | preservation | recursive case |
| finishing | exit condition | reaching the base case |
| termination | decreasing variant | shrinking argument |

Two constructs, one underlying justification. Which explains something you may
have noticed: any loop can be rewritten as a recursion and any recursion as a
loop. They are notations for the same thing, and the choice between them is about
which makes the structure of *your particular problem* visible.

Use recursion when the data or the problem is itself recursive — trees, nested
structures, divide-and-conquer. Use iteration when the process is a straight walk
through a sequence. The next section adds a second consideration, which is cost.

## Structural recursion

The most useful case, and worth naming.

When a data structure is defined recursively, a method over it is written
recursively and the shape matches exactly. A binary tree:

> A tree is either empty, or a value with a left tree and a right tree.

That definition has a base case and a recursive case, so a method over it does
too:

```java
static int size(Node t) {
    if (t == null) return 0;                              // empty
    return 1 + size(t.left) + size(t.right);              // value + two trees
}
```

The correctness argument writes itself. *The empty tree has 0 nodes. Assuming the
calls give the sizes of the subtrees, this tree has those plus one for itself.*

Doing the same iteratively means maintaining a stack of nodes yet to visit — which
is to say, building by hand the structure Chapter 12 gave you for free. That is
the clearest argument for recursion there is: **when the problem has the shape of
a stack, use the one the machine already provides.**

Unit IV builds these structures and Unit VI uses them heavily, since a parsed
program is a tree and an interpreter is a recursive walk over it. This chapter is
the groundwork for the centre of the book.

## Strong induction

One variant, mentioned because you will meet the recursive form of it.

Ordinary induction assumes the statement for *n* to prove it for *n*+1. **Strong
induction** assumes it for *all* values less than *n*.

The recursive counterpart is a method that calls itself on inputs that are smaller
but not necessarily by one — binary search halving a range, merge sort splitting a
list. The trust is the same: assume every smaller call is correct, regardless of
how much smaller.

```java
static int gcd(int a, int b) {
    if (b == 0) return a;
    return gcd(b, a % b);
}
```

`gcd(48, 18)` is 6. The second argument shrinks, but not predictably — 18, then
48 % 18 = 12, then 6, then 0. Termination still holds, because a non-negative
integer that strictly decreases must reach the base case. The variant argument of
Chapter 9 does not require the decrease to be by one.

Next: the cost of all this.
