# When Recursion Is Wrong

This chapter has been advocating. Here is the other side, because recursion is
frequently the wrong choice and knowing when is part of using it well.

## When a loop is better

**When the process is a straight walk.** Summing an array, printing each element,
finding a maximum — these are sequences, not nested structures, and a loop says so:

```java
// clear
int total = 0;
for (int x : a) total += x;

// works, and says less
static int sum(int[] a, int i) {
    if (i == a.length) return 0;
    return a[i] + sum(a, i + 1);
}
```

The recursive version needs an extra parameter that exists only for bookkeeping,
and it will exhaust the stack on a large array. Nothing was gained.

**When the depth could be large.** Chapter 12 gave the limit — tens of thousands
of frames. A recursion over a collection whose size is controlled by user input is
a `StackOverflowError` waiting for a large enough input, and that is a real
failure mode rather than a theoretical one.

**When the recursive version needs a helper.** If you find yourself writing
`sum(a, 0)` because the recursion needs an index the caller should not care about,
the structure is fighting you. Sometimes it is still worth it; often the loop was
the right shape.

**When it recomputes.** The Fibonacci case. If the branches overlap, the naive
recursion is not slow — it is unusable, and no amount of elegance compensates.

## When recursion is better

**When the data is recursive.** Trees, nested directories, JSON documents,
expressions. The method's shape matches the data's shape, and the alternative is
maintaining a stack by hand.

**When the problem divides.** Merge sort, quicksort, binary search. Split, solve
the parts, combine. The recursive statement is the algorithm's description.

**When backtracking is needed.** Puzzle solving, maze walking, constraint search
— try a possibility, recurse, and if it fails undo and try the next. The stack
handles the "undo and try the next" for free, and the iterative version is
substantially harder to get right.

**When the recursive definition is the specification.** Sometimes the recursion
*is* the clearest statement of what the thing means, and writing it any other way
obscures it.

## The honest summary

**Match the code to the structure of the problem.** A sequence wants a loop. A
tree wants recursion. When the problem is genuinely recursive, the recursive
solution is shorter, easier to verify, and easier to modify. When it is not,
recursion adds parameters, stack depth, and indirection for nothing.

The failure mode in both directions is using the technique you are more
comfortable with rather than the one that fits.

## A word about elegance

Recursive solutions are often called elegant, and it is worth being careful with
the word.

Elegance that helps is code whose shape reveals the problem's shape — where the
structure of the solution teaches you something about the structure of the
question. `size(tree)` is elegant in this sense.

Elegance that does not help is cleverness admired for its own sake: a solution
that is shorter but requires more thought to verify, or that fails on large inputs
in exchange for looking neat.

The naive Fibonacci is the standard example of the second kind. It is the textbook
illustration of recursion, it is beautiful, it matches the mathematical definition
exactly, and it is catastrophically wrong as an implementation. That it is still
taught as an exemplar is a small scandal, and I have included it here as a warning
rather than a model.

## Closing the chapter

A method may call itself, and Chapter 12's machinery makes it unremarkable —
frames belong to executions, so two executions of one method are as separate as
two executions of different methods.

Every recursion needs a base case and a recursive case that moves strictly towards
it. Write the base case first; it settles the domain and the termination argument
before you need either.

To think about recursion, **trust the recursive call**. Assume it returns the right
answer for its smaller input, and check only whether you build the right answer
from it. Two sentences, regardless of input size — which is legitimate because the
argument is mathematical induction, with the base case as the base case and the
recursive case as the inductive step. Chapter 9's loop invariants are the same
principle, which is why loops and recursions interconvert.

The costs are real. Linear recursion is proportional to the input; tree recursion
with overlapping branches is exponential and unusable. A recursive *procedure* may
generate an iterative *process* if it is tail-recursive — though Java does not
reward this, since it does not eliminate tail calls.

And the judgment: match the shape of the code to the shape of the problem. That
is a design decision, which is the subject of the last chapter of the unit.
