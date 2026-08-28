# Declaring and Filling

The last lesson argued that an index is arithmetic. That argument is what an
array *is*; this lesson is how you get one, which is short, and worth reading
slowly anyway — two of these four lines are where beginners lose an hour.

## Creating one

Two ways. With a known size, and default values:

```java
int[] scores = new int[5];
```

Five elements, all 0. The defaults follow Chapter 7's rule for fields — 0 for
numeric types, `false` for `boolean`, `null` for references:

```java
int[] a    = new int[3];      // [0, 0, 0]
String[] s = new String[3];   // [null, null, null]
```

That second one matters. An array of objects is created full of `null`, not full
of objects. Every element must be filled before it can be used, and forgetting is
a common source of `NullPointerException`.

Or with the contents written out:

```java
int[] scores = {87, 92, 78, 95, 88};
```

The size comes from the count. This form works only at the point of declaration;
later you need the longer version:

```java
scores = new int[]{1, 2, 3};
```

## The declaration syntax

Java accepts two spellings:

```java
int[] a;      // preferred
int a[];      // legal, inherited from C
```

Use the first. `int[]` is a type — "array of int" — and putting the brackets with
the type says so. The second form is a C compatibility feature and it misleads:

```java
int a[], b;      // a is an array, b is a plain int
```

That is a genuine trap, and it does not arise if the brackets are on the type.

## Reading and writing

By position, using the arithmetic of the last lesson:

```java
scores[0] = 87;
int first = scores[0];
scores[2] = scores[2] + 5;
```

An index may be any `int` expression:

```java
scores[i]
scores[i + 1]
scores[a.length - 1]      // the last element
```

That last one is worth committing to memory. The last index is `length - 1`,
because indices run from 0 to `length - 1`. Writing `a[a.length]` is Chapter 9's
off-by-one and throws.

## Walking one

Three ways, and the choice communicates something.

**The counted loop**, when you need the index:

```java
for (int i = 0; i < scores.length; i++) {
    System.out.println("Score " + (i + 1) + ": " + scores[i]);
}
```

**The enhanced for**, when you do not:

```java
for (int s : scores) {
    total += s;
}
```

Chapter 9 said this removes a class of bug by removing the index. It does have a
restriction worth knowing: you cannot assign to the array through it.

```java
for (int s : scores) {
    s = 0;              // changes the copy, not the array
}
```

`s` is a fresh variable receiving a copy of each element — Chapter 12's
pass-by-value in a new setting. To modify elements you need the index:

```java
for (int i = 0; i < scores.length; i++) {
    scores[i] = 0;
}
```

**A stream**, from Chapter 26. Mentioned so you know it exists:

```java
int total = Arrays.stream(scores).sum();
```

## Copying

Assignment does not copy — Chapter 12 again:

```java
int[] b = a;              // one array, two names
```

To copy, ask explicitly:

```java
int[] b = a.clone();
int[] c = Arrays.copyOf(a, a.length);
int[] d = Arrays.copyOfRange(a, 1, 4);     // elements 1, 2, 3
```

All three make a new array. But note the qualification, because it is the trap:

```java
int[][] deep = { {1, 2} };
int[][] shallow = deep.clone();
shallow[0][0] = 42;
System.out.println(deep[0][0]);      // 42
```

`clone` copies the *outer* array's elements — which for an array of arrays are
references. Both outer arrays now point at the same inner arrays, so a change
through one is visible through the other.

This is a **shallow copy**, and it is the default everywhere in Java. A **deep
copy** — copying the contents recursively — has to be written by hand. Chapter 20
returns to this properly; for now, know that `clone` on a nested structure copies
one level.

## The Arrays utility class

`java.util.Arrays` holds the operations arrays do not have as methods:

```java
Arrays.toString(a)              // "[3, 1, 4, 1, 5]" — for printing
Arrays.sort(a)                  // sorts in place
Arrays.fill(a, 7)               // sets every element
Arrays.equals(a, b)             // element-by-element comparison
Arrays.binarySearch(a, 4)       // requires a sorted array
```

`Arrays.toString` is worth adopting immediately, because printing an array
directly does not do what you want:

```java
System.out.println(a);          // [I@1b6d3586
```

That is the type and a hash code, not the contents. Chapter 20 explains where it
comes from; for now, use `Arrays.toString`.

And `Arrays.equals` deserves the same emphasis. `a == b` compares *references* —
whether they are the same array — not contents:

```java
int[] x = {1, 2};
int[] y = {1, 2};
x == y                  // false: two different arrays
Arrays.equals(x, y)     // true: same contents
```

That distinction is Chapter 20's subject and one of the genuinely hard ideas in
the book. Meeting it here, on arrays, is a gentle introduction.

Next: what happens when the index is wrong.
