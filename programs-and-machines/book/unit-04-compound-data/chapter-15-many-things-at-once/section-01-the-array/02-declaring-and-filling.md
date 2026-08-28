# Declaring and Filling

An index is arithmetic. That was the last lesson, and it is what an array *is*.

This one is how you get hold of one, which is short. Read it slowly anyway — there
are four or five places in here where beginners reliably lose an hour, and they all
look harmless on the way past.

## Creating one

Two ways. Either you know the size and take the defaults:

```java
int[] scores = new int[5];
```

Five elements, every one of them 0. The defaults follow Chapter 7's rule for
fields: 0 for numeric types, `false` for `boolean`, `null` for references.

```java
int[] a    = new int[3];      // [0, 0, 0]
String[] s = new String[3];   // [null, null, null]
```

Stop on that second line, because it is the first of the hours.

An array of objects is created full of `null`. Not full of empty strings, not full
of objects waiting to be used — full of nothing. Every element has to be put there
before it can be used, and forgetting is one of the most common sources of
`NullPointerException` there is. The array exists. Its contents do not.

Or you write the contents out and let Java count:

```java
int[] scores = {87, 92, 78, 95, 88};
```

That short form works only at the moment of declaration. Later on you need the
longer one:

```java
scores = new int[]{1, 2, 3};
```

## Two spellings, one of which is a trap

Java accepts both of these:

```java
int[] a;      // preferred
int a[];      // legal, inherited from C
```

Use the first, and here is the concrete reason rather than an appeal to style.
`int[]` is a type — "array of int" — and putting the brackets on the type says
exactly that. The second form is a C compatibility feature, and it lies to you:

```java
int a[], b;      // a is an array, b is a plain int
```

Read that line again. Two variables declared together, and they have different
types. Nobody writing it intended that, and it does not happen at all if you keep
the brackets attached to the type where they belong.

## Reading and writing

By position, using the arithmetic from the last lesson:

```java
scores[0] = 87;
int first = scores[0];
scores[2] = scores[2] + 5;
```

The index can be any `int` expression at all:

```java
scores[i]
scores[i + 1]
scores[a.length - 1]      // the last element
```

Commit that last one to memory now and save yourself the trouble later. The final
index is `length - 1`, because the indices run from 0 up to `length - 1`. Writing
`a[a.length]` is the off-by-one from Chapter 9, and it throws.

## Walking one

Three ways, and which you choose tells your reader something.

**The counted loop**, when you actually need the index:

```java
for (int i = 0; i < scores.length; i++) {
    System.out.println("Score " + (i + 1) + ": " + scores[i]);
}
```

**The enhanced `for`**, when you do not:

```java
for (int s : scores) {
    total += s;
}
```

Chapter 9 sold you this on the grounds that it removes a whole class of bug by
removing the index. True. It also has a restriction, and this is the next hour
somebody loses:

```java
for (int s : scores) {
    s = 0;              // changes the copy, not the array
}
```

That loop runs. It completes without complaint. And the array is untouched.

`s` is a fresh variable receiving a *copy* of each element — which is Chapter 12's
pass-by-value showing up in a new place. If you want to modify the elements
themselves, you need the index:

```java
for (int i = 0; i < scores.length; i++) {
    scores[i] = 0;
}
```

**A stream**, from Chapter 26, mentioned here only so that you recognize it:

```java
int total = Arrays.stream(scores).sum();
```

## Copying, and the trap inside the trap

Assignment does not copy. Chapter 12 again:

```java
int[] b = a;              // one array, two names
```

To actually copy, you have to ask:

```java
int[] b = a.clone();
int[] c = Arrays.copyOf(a, a.length);
int[] d = Arrays.copyOfRange(a, 1, 4);     // elements 1, 2, 3
```

All three give you a genuinely new array. And now the qualification, which is the
part worth slowing down for. Predict the output:

```java
int[][] deep = { {1, 2} };
int[][] shallow = deep.clone();
shallow[0][0] = 42;
System.out.println(deep[0][0]);      // ?
```

42.

We cloned. We changed the clone. The original changed too.

`clone` copies the *outer* array's elements — and for an array of arrays, those
elements are references. So both outer arrays are now pointing at the very same
inner arrays, and a change made through one is a change seen through the other.

This is a **shallow copy**, and it is the default absolutely everywhere in Java. A
**deep copy** — following the references down and copying the contents too — is
something you write by hand. Chapter 20 returns to this properly. For now, hold on
to one sentence: `clone` on a nested structure copies exactly one level.

## The Arrays utility class

Arrays are not objects with rich methods, so the operations live in
`java.util.Arrays`:

```java
Arrays.toString(a)              // "[3, 1, 4, 1, 5]" — for printing
Arrays.sort(a)                  // sorts in place
Arrays.fill(a, 7)               // sets every element
Arrays.equals(a, b)             // element-by-element comparison
Arrays.binarySearch(a, 4)       // requires a sorted array
```

Adopt `Arrays.toString` today, because printing an array directly does not do what
any reasonable person expects:

```java
System.out.println(a);          // [I@1b6d3586
```

That is the type and a hash code. Not one of your numbers appears in it. Chapter 20
explains where that string comes from; until then, just use `Arrays.toString` and
enjoy being able to see your own data.

`Arrays.equals` deserves the same emphasis for the same kind of reason. `a == b`
compares *references* — it asks whether these are the same array — and has nothing
at all to say about contents:

```java
int[] x = {1, 2};
int[] y = {1, 2};
x == y                  // false: two different arrays
Arrays.equals(x, y)     // true: same contents
```

That distinction is Chapter 20's whole subject, and one of the genuinely hard ideas
in this book. Meeting it here, on arrays, where you can see both objects at once,
is about as gentle an introduction to it as you are going to get.

Next: what happens when the index is wrong.
