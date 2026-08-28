# Values and References

Chapter 7 handed you a picture — a variable is a labelled box with a value in it —
and quietly added that it would stop being true later.

This is later.

## Two kinds of variable

For a primitive, the box holds the value itself:

```java
int x = 5;
```

```
x: ┌───┐
   │ 5 │        the number is in the box
   └───┘
```

For an object, the box holds a **reference** — where the object is, not the object
itself:

```java
int[] a = {1, 2, 3};
```

```
a: ┌────────┐         ┌───┬───┬───┐
   │ ref ───┼────────▶│ 1 │ 2 │ 3 │
   └────────┘         └───┴───┴───┘
   on the stack        on the heap
```

The variable is on the stack, in a frame. The array is on the heap. The variable
holds the information needed to find it.

Why the difference? Because a frame is fixed-size and created on every call. An
`int` is 4 bytes and fits. An array of a million elements does not, and its size
is not known when the method is compiled. So objects live on the heap and
variables hold references, which are small and fixed-size.

That is Chapter 1's fixed-width argument, deciding a language's design.

## What this changes

Assignment copies **the contents of the box**. For primitives that is the value;
for objects it is the reference.

Predict the last line before you read it. We never mention `a` after the second
statement.

```java
int[] a = {1, 2, 3};
int[] b = a;              // copies the reference
b[0] = 99;
System.out.println(a[0]); // 99
```

99. We wrote to `b` and read from `a`.

`a` and `b` are two boxes holding the same reference, so there is one array with
two names:

```
a: ┌────────┐
   │ ref ───┼───┐      ┌────┬───┬───┐
   └────────┘   ├─────▶│ 99 │ 2 │ 3 │
b: ┌────────┐   │      └────┴───┴───┘
   │ ref ───┼───┘
   └────────┘
```

Two names for one thing is **aliasing**, and it is the source of a large fraction
of the confusion in the rest of this book. Chapter 20 is devoted to it.

Compare the primitive case from Chapter 7:

```java
int x = 5;
int y = x;      // copies the value
y = 99;
System.out.println(x);    // 5
```

Two boxes, two independent values, and no surprise at all.

Now put the two examples side by side and notice that the rule did not change
between them. *Assignment copies the contents of the box*, both times. It produced
results that look like opposites purely because the contents were different in
kind — a number in one case, a reference in the other.

**One rule, not two.** That is the thing worth carrying, and it will keep you out
of trouble in every chapter after this one.

## Java does not have addresses

A clarification, since "reference" sounds like "pointer".

In C, a pointer is a number you can inspect, print, and do arithmetic on. In Java
a reference is opaque: you cannot see its numeric value, add to it, or construct
one. The only things you can do are follow it (`a[0]`, `s.length()`), copy it,
compare it with `==`, and set it to `null`.

That restriction is what makes Java memory-safe. Without pointer arithmetic there
is no way to construct a reference to somewhere you should not be — which removes
the buffer overflow of Chapter 6 and an entire category of security defect.

The cost is a loss of control that matters in systems programming and almost
nowhere else.

## null

A reference variable may hold `null`, meaning *refers to nothing*.

```java
int[] a = null;
System.out.println(a.length);     // NullPointerException
```

Following a reference that points nowhere fails, and Chapter 10 showed the error.
The concept is worth meeting properly here: `null` is not an object, not an empty
array, and not zero. It is the absence of a reference, and Chapter 16 discusses
whether it should have existed at all.

## The box picture, updated

The revision is small and its consequences are not:

> A variable is a box. For a primitive, the box holds the value. For an object,
> the box holds a reference, and the object is elsewhere.

Every result in the next lesson follows from that sentence.
