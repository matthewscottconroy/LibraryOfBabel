# Two-Dimensional Data

A chessboard, a spreadsheet, an image, a table of results — grids are everywhere,
and Java handles them with arrays whose elements are arrays.

```java
int[][] grid = new int[3][4];      // 3 rows, 4 columns
grid[1][2] = 7;
```

## What that actually creates

Not a rectangle of twelve integers. **Four objects**: one outer array of three
references, and three inner arrays of four `int`s each.

```
grid: ┌────────┐        ┌───┬───┬───┬───┐
      │ ref ───┼───┐   ▶│ 0 │ 0 │ 0 │ 0 │   row 0
      └────────┘   │   │ └───┴───┴───┴───┘
                   ▼   │
              ┌─────┬──┴──┬─────┐
              │ ref │ ref │ ref │            the outer array
              └──┬──┴─────┴──┬──┘
                 │           │   ┌───┬───┬───┬───┐
                 │           └──▶│ 0 │ 0 │ 0 │ 0 │   row 2
                 │               └───┴───┴───┴───┘
                 │               ┌───┬───┬───┬───┐
                 └──────────────▶│ 0 │ 0 │ 0 │ 0 │   row 1
                                 └───┴───┴───┴───┘
```

So `grid[1][2]` is two lookups: follow the outer array to element 1, getting a
reference; follow that to element 2. Two memory accesses rather than one
multiplication.

Languages with genuine two-dimensional arrays — Fortran, C with a fixed second
dimension — store the whole grid consecutively and compute
`base + (row × cols + col) × size`, which is one access. Java's arrangement is
more flexible and slightly slower, and the flexibility is the next lesson.

## Dimensions

```java
grid.length         // 3 — the number of rows
grid[0].length      // 4 — the length of row 0
```

There is no `grid.length` for columns, because columns are not a thing the outer
array knows about. Each row is an independent array with its own length, and
`grid[0].length` asks row 0 how long *it* is.

That distinction becomes important the moment rows differ in length.

## Literals

```java
int[][] grid = {
    {1, 2, 3, 4},
    {5, 6, 7, 8},
    {9, 10, 11, 12}
};
```

Readable, and worth laying out with one row per line so the shape is visible.

## Walking

Nested loops, outer over rows, inner over columns:

```java
for (int row = 0; row < grid.length; row++) {
    for (int col = 0; col < grid[row].length; col++) {
        System.out.print(grid[row][col] + " ");
    }
    System.out.println();
}
```

Note `grid[row].length` rather than `grid[0].length`. Using row 0's length assumes
every row is the same length, which Java does not guarantee.

The enhanced form is cleaner when the indices are not needed:

```java
for (int[] row : grid) {
    for (int value : row) {
        System.out.print(value + " ");
    }
    System.out.println();
}
```

Read the outer loop carefully: it iterates over **arrays**, so `row` is an
`int[]`. That is the structure made visible in the type.

## Three dimensions and beyond

```java
int[][][] cube = new int[3][4][5];
```

Arrays of arrays of arrays. Legal, and beyond three dimensions you should
generally stop and reconsider — usually there is a structure that would be
clearer, and Unit V provides the means to build one.

## Rows and columns are not symmetric

The most important consequence of the arrays-of-arrays design, and the source of
several bugs.

Getting a row is trivial — it is an object:

```java
int[] firstRow = grid[0];
```

Getting a *column* is not. There is no object representing column 2; the values
are scattered across three different arrays, and you must collect them:

```java
int[] column = new int[grid.length];
for (int row = 0; row < grid.length; row++) {
    column[row] = grid[row][2];
}
```

So the two dimensions have genuinely different costs, and which one you make the
outer index is a design decision. If your program mostly works with rows, index by
row first. If it mostly works with columns, consider storing it transposed.

That asymmetry is invisible in the notation — `grid[1][2]` looks symmetric — and
it is real underneath. Which is a good example of an abstraction that leaks, in
Chapter 11's sense: the convenient notation hides a structure that you
nevertheless have to know about.

Next: what happens when the rows are not all the same length.
