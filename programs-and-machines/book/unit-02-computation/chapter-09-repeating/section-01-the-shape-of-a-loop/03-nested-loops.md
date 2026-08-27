# Nested Loops

A loop's body is statements, and statements may be loops:

```java
for (int row = 1; row <= 3; row++) {
    for (int col = 1; col <= 4; col++) {
        System.out.print(row * col + "\t");
    }
    System.out.println();
}
```

```
1	2	3	4	
2	4	6	8	
3	6	9	12	
```

The inner loop runs **completely** for each iteration of the outer one. Outer
takes its first value, inner runs all four of its values, outer moves on, inner
runs all four again.

If tracing that is not immediate, do it on paper once. Almost everyone needs to,
and once done it is permanent.

## The multiplication

Three outer iterations, four inner each, is twelve inner iterations. The counts
multiply.

That is the important fact about nesting, and it is not a small one. A loop over
*n* items inside another loop over *n* items runs $n^{2}$ times:

| *n* | $n^{2}$ |
|---:|---:|
| 10 | 100 |
| 100 | 10,000 |
| 1,000 | 1,000,000 |
| 10,000 | 100,000,000 |
| 100,000 | 10,000,000,000 |

The last row is ten billion iterations, which on a modern machine is seconds to
minutes rather than the microseconds you might have assumed from the fact that
the code is six lines long.

This is Chapter 32's subject, and the reason to raise it here is that nesting is
where beginners first write something accidentally unusable. Code that is
instant on the ten-item test data and unusable on the hundred-thousand-item real
data is a rite of passage, and the multiplication in the table is why.

**Whenever you nest a loop, notice that you have multiplied.** Sometimes the
multiplication is necessary — comparing every pair genuinely requires examining
every pair — and sometimes there is a way to avoid it, which Unit IV's data
structures are largely about.

## Depth

Three levels multiply three ways, and $n^{3}$ gets bad quickly: a thousand items
gives a billion iterations.

Deep nesting is also hard to read, and it is usually a signal that the inner part
wants to be a method with a name — which is Chapter 11's subject and one of its
best arguments.

## break and continue in nests

Here is a trap.

```java
for (int i = 0; i < 3; i++) {
    for (int j = 0; j < 3; j++) {
        if (found) break;      // leaves the INNER loop only
    }
}
```

`break` exits the innermost enclosing loop, not all of them. The outer loop
carries on, and if the intent was to stop searching entirely, it does not.

Three ways out. A flag:

```java
boolean done = false;
for (int i = 0; i < 3 && !done; i++) {
    for (int j = 0; j < 3; j++) {
        if (found) { done = true; break; }
    }
}
```

A labelled break, which Java supports for exactly this:

```java
outer:
for (int i = 0; i < 3; i++) {
    for (int j = 0; j < 3; j++) {
        if (found) break outer;
    }
}
```

Or — usually best — put the nest in a method and `return`:

```java
static int[] find(int[][] grid, int target) {
    for (int i = 0; i < grid.length; i++) {
        for (int j = 0; j < grid[i].length; j++) {
            if (grid[i][j] == target) return new int[]{i, j};
        }
    }
    return null;
}
```

`return` leaves everything, the intent is unmistakable, and the search has
acquired a name. Labels are the one place Java retains something like a `goto`,
and they are legal and rare; if you find yourself reaching for one, the method
version is usually available and clearer.

## Reading a nested loop

The question that unlocks one is: **what does the inner loop do, for a single
value of the outer variable?**

Answer that first, in a sentence, treating the outer variable as fixed. Then ask
what the outer loop does with that.

For the multiplication table: *the inner loop prints one row of products for a
fixed `row`*. The outer loop does that for each row. Two sentences, and the
structure is clear without tracing twelve iterations.

That two-step reading is the same move as a loop invariant, which the next
section makes precise — describe what is true rather than what happens, and the
number of iterations stops mattering.
