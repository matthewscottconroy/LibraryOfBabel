# Ragged Arrays

Because each row is a separate array, the rows need not be the same length.

```java
int[][] triangle = {
    {1},
    {1, 2},
    {1, 2, 3}
};
```

Three rows, of lengths 1, 2 and 3. This is a **ragged** or **jagged** array, and a
language with genuine rectangular arrays cannot express it.

## Building one

The rows can be created independently:

```java
int[][] triangle = new int[3][];      // note: second dimension omitted
for (int i = 0; i < 3; i++) {
    triangle[i] = new int[i + 1];
}
```

`new int[3][]` creates the outer array of three references, all `null`. Each row
is then created and assigned. Miss one and it stays `null`, and touching it throws
`NullPointerException` rather than an index error — a different message for a
different mistake.

## Where it is useful

**Triangular data.** A table of distances between cities needs only half the
grid, since the distance from A to B is the distance from B to A. Pascal's
triangle is naturally triangular. Storing the full rectangle wastes half the
memory and invites inconsistency between the two copies of each value.

**Genuinely varying data.** A list of exam scores per student, where students sat
different numbers of exams. Padding to a rectangle means inventing a value for
"no exam", and every piece of code then has to know and respect that convention —
which is Chapter 16's argument for making the structure say what is true rather
than encoding it in a sentinel.

**Rows that arrive separately.** Lines of a file, each split into fields. Nothing
guarantees they have the same number of fields, and forcing them to would lose
information.

## Where it bites

The bug it causes is always the same one:

```java
for (int row = 0; row < grid.length; row++) {
    for (int col = 0; col < grid[0].length; col++) {      // wrong
        // ...
    }
}
```

`grid[0].length` assumes every row matches row 0. On a ragged array this either
skips elements or throws, depending on which row is longer.

**Always use `grid[row].length`.** It is correct for rectangular arrays too, so
there is no reason to write anything else, and making it a habit costs nothing.

## Rectangular is a convention, not a guarantee

Worth stating plainly, because it affects how you write methods.

```java
static int sum(int[][] grid) { ... }
```

Nothing in that signature promises the grid is rectangular. If your method needs
it to be, that is a **precondition** in Chapter 11's sense, and it should be
stated:

> **Requires:** `grid` is non-null and every row has the same length.

And if the method is public and the caller is not under your control, checking it
is cheap:

```java
for (int[] row : grid) {
    if (row.length != grid[0].length)
        throw new IllegalArgumentException("grid must be rectangular");
}
```

This is Chapter 11's fail-fast argument in a specific setting. A ragged array
passed to a method expecting a rectangle produces either a confusing index error
deep inside, or — worse — a wrong answer computed from a partially-visited grid.

## Rows are objects

One more consequence, which is Chapter 12's aliasing arriving in two dimensions.

Rows are arrays, so they are objects, so they can be shared:

```java
int[] shared = {1, 2, 3};
int[][] grid = { shared, shared };

grid[0][0] = 99;
System.out.println(grid[1][0]);      // 99
```

Both rows are the same array. Changing one changes "both", because there is only
one.

This is rarely done deliberately and it happens by accident when a row is copied
without care:

```java
int[][] copy = original.clone();      // shallow: rows are shared
```

which is the trap from Section 15.1.2. A real copy of a grid means copying each
row:

```java
int[][] copy = new int[original.length][];
for (int i = 0; i < original.length; i++) {
    copy[i] = original[i].clone();
}
```

Chapter 20 gives this a name — deep versus shallow copying — and explains why Java
does not do it for you.

Next: the order you walk a grid, and why it matters more than it should.
