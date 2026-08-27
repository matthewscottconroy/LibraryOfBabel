# Linear and Tree Recursion

Two recursive methods, superficially similar, with wildly different behavior.

## Linear

```java
static int factorial(int n) {
    if (n <= 1) return 1;
    return n * factorial(n - 1);
}
```

Each call makes **one** further call. The calls form a line:

```
factorial(5) → factorial(4) → factorial(3) → factorial(2) → factorial(1)
```

Five calls for input 5. *n* calls for input *n* — linear in the input. The stack
reaches depth *n*, which for `factorial` is fine, since anything above 20
overflows a `long` anyway.

## Tree

```java
static int fib(int n) {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
}
```

Each call makes **two** further calls. The calls form a tree:

```
                fib(5)
               /      \
         fib(4)        fib(3)
         /    \        /    \
    fib(3)  fib(2)  fib(2)  fib(1)
     /  \
 fib(2) fib(1)          ... and so on
```

And now count. Here are the actual call counts:

| `fib(n)` | result | calls |
|---:|---:|---:|
| 10 | 55 | 177 |
| 20 | 6,765 | 21,891 |
| 30 | 832,040 | 2,692,537 |

Ten more in the input multiplies the work by more than a hundred. `fib(50)` would
take something like two hundred million times the work of `fib(10)` — hours,
for a number a loop produces instantly.

## Why it explodes

Because the same subproblems are computed repeatedly.

`fib(5)` needs `fib(4)` and `fib(3)`. But `fib(4)` also needs `fib(3)`. So
`fib(3)` is computed twice — and each of those recomputes `fib(2)`, and so on. The
tree contains enormous duplication, and nothing remembers that a value has already
been found.

Roughly, the number of calls grows like the golden ratio to the *n*. That is
exponential, and Chapter 32 will make the vocabulary precise; the practical point
is available now: **a tree recursion that recomputes shared subproblems is
unusable beyond small inputs.**

## Two fixes

**Remember answers.** Keep a table of already-computed values and consult it
first. This is called **memoization**, and it turns the exponential into a linear
process because each value is computed once. Unit IV gives you the map to store
them in.

**Work upwards instead.** Compute `fib(2)`, then `fib(3)`, and so on, keeping only
the last two:

```java
static int fibIter(int n) {
    if (n < 2) return n;
    int prev = 0, curr = 1;
    for (int i = 2; i <= n; i++) {
        int next = prev + curr;
        prev = curr;
        curr = next;
    }
    return curr;
}
```

Linear, constant space, and no duplication — because working upwards means each
value is available when needed rather than being recomputed on demand.

## The lesson is not "avoid recursion"

It would be easy to take the wrong conclusion here. Tree recursion is not the
problem; **recomputing shared subproblems** is.

Plenty of tree recursions are perfectly efficient, because their branches are
genuinely independent:

```java
static int size(Node t) {
    if (t == null) return 0;
    return 1 + size(t.left) + size(t.right);
}
```

Two calls per node, and the tree of calls exactly matches the tree of data. Each
node is visited once. This is linear in the number of nodes and could not be
improved.

The difference is that a tree's left and right subtrees share nothing, while
`fib(n-1)` and `fib(n-2)` overlap almost entirely.

So the question to ask of any tree recursion is: **do the branches overlap?** If
they do not, it is fine. If they do, you are recomputing, and you need memoization
or a different approach.

## Divide and conquer

The productive form of tree recursion, worth naming because you will meet it
constantly.

Split the problem in half, solve both halves, combine. Merge sort and binary
search do this, and so does the quicksort of Chapter 32.

The branches do not overlap — the two halves are disjoint — so there is no
recomputation. And because the input halves each time, the depth is
about $\log_{2} n$ rather than *n*: a million elements is twenty levels deep, not a
million.

That is the good case, and it is why divide-and-conquer is one of the small number
of genuinely important algorithmic ideas.

Next: a distinction between the shape of the code and the shape of the process.
