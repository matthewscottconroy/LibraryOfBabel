# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Mechanics

**15.1.** Declare an array of 10 `int`s. What are its contents? Declare an array
of 10 `String`s. What are its contents, and why is that more dangerous?

**15.2.** Give three ways to create an array containing 1, 2, 3.

**15.3.** What is the last valid index of an array of length *n*? Write the
expression for the last element.

**15.4.** Predict and explain:
```java
int[] a = {1, 2, 3};
int[] b = a;
b[0] = 99;
System.out.println(a[0]);
```

**15.5.** Predict and explain:
```java
int[] a = {1, 2, 3};
int[] b = a.clone();
b[0] = 99;
System.out.println(a[0]);
```

**15.6. [carries forward]** Predict and explain:
```java
int[][] a = { {1, 2} };
int[][] b = a.clone();
b[0][0] = 99;
System.out.println(a[0][0]);
```

## Understanding the mechanism

**15.7.** Explain, using the address formula, why array elements must all be the
same type.

**15.8.** Explain why an array cannot grow.

**15.9. [carries forward]** Why do indices start at 0? Give the arithmetic
argument and the half-open-range argument.

**15.10.** `a.length`, `s.length()`, `list.size()`. Which is a field and which are
methods? Why can this not be fixed?

**15.11.** Why does `System.out.println(a)` print something like `[I@1b6d3586`?
What should you use instead?

**15.12.** For `int[] x = {1,2}; int[] y = {1,2};` — what do `x == y` and
`Arrays.equals(x, y)` give, and why do they differ?

## Bounds

**15.13.** What would happen without bounds checking if you wrote to `a[5]` of a
five-element array? Describe both the immediate effect and the possible security
consequence.

**15.14.** Bounds checking costs a comparison per access. Give two reasons the
cost is usually much smaller than that suggests.

**15.15.** For each error, say what mistake it most likely indicates:
`Index 5 out of bounds for length 5`; `Index -1 out of bounds for length 10`;
`Index 1000 out of bounds for length 10`.

## Grids

**15.16.** How many objects does `new int[3][4]` create? Draw them.

**15.17.** Write a loop that prints a ragged array correctly. Then explain the
bug in a version that uses `grid[0].length`.

**15.18.** Write a method returning column *c* of a grid as a new array. Explain
why no such method is needed for rows.

**15.19. [carries forward]** Write a method that makes a genuine deep copy of an
`int[][]`.

**15.20.** Row-major traversal was 3× faster than column-major on a 4000 × 4000
grid. Explain why, using cache lines. Would the ratio be larger or smaller for a
10 × 10 grid, and why?

## Going further

**15.21.** Write a method `boolean isRectangular(int[][] g)`. Then write
`transpose`, stating its precondition.

**15.22.** Implement Conway's Game of Life on a fixed grid: a cell with two or
three live neighbours survives, a dead cell with exactly three becomes alive.
Use the neighbour pattern from Section 15.2.3, and say how you handle the edges.

**15.23.** Section 15.2.1 calls the symmetric notation `grid[1][2]` a leaky
abstraction. Explain what leaks, and name one other leaky abstraction you have
met in this book so far.
