# Java Passes by Value, Always

Here is the question, and it is worth answering carefully because the wrong answer
is widespread.

> Does Java pass objects by reference?

**No.** Java is pass-by-value, always, for every type. What confuses people is that
when the argument is an object, *the value being copied is a reference*.

Let us establish it with two demonstrations that appear to contradict each other,
and then show they do not.

## Demonstration one: a swap that fails

```java
static void swap(int x, int y) {
    int t = x; x = y; y = t;
}

int a = 1, b = 2;
swap(a, b);
System.out.println("a=" + a + " b=" + b);     // a=1 b=2
```

Nothing happened. `swap` received copies, swapped the copies, and the frame was
discarded. Exactly what Chapter 11 said about primitives.

## Demonstration two: a change that sticks

```java
static void swapInArray(int[] p) {
    int t = p[0]; p[0] = p[1]; p[1] = t;
}

int[] arr = {1, 2};
swapInArray(arr);
System.out.println(arr[0] + " " + arr[1]);    // 2 1
```

This time the change survived. Which looks like pass-by-reference, and is where
most explanations stop and conclude that objects are passed by reference.

## The third demonstration, which settles it

```java
static void tryToReplace(int[] p) {
    p = new int[]{7, 7, 7};
}

int[] arr = {1, 2, 3};
tryToReplace(arr);
System.out.println(arr[0]);      // 1
```

`arr` is unchanged.

**If Java passed by reference, this would print 7.** Under genuine
pass-by-reference the parameter *is* the caller's variable, so assigning to it
would replace what the caller holds. That is what pass-by-reference means, and it
is what happens in languages that have it.

It printed 1. So Java does not pass by reference.

## What actually happened

One rule explains all three.

**The value in the caller's box is copied into the parameter's box.**

In demonstration one, the box held 1. A copy of 1 went into `x`. Changing `x`
changed a copy.

In demonstration two, the box held a reference. A copy of that reference went into
`p`. Now `arr` and `p` are two boxes holding the same reference — aliases for one
array. `p[0] = ...` does not touch `p`; it **follows** the reference and modifies
the array at the other end. There is only one array, and both names see the
change.

In demonstration three, `p = new int[]{7,7,7}` assigns to `p` **itself** —
overwriting the copied reference with a different one. The caller's box is
untouched, because it was never shared; only its contents were copied. `p` now
points at a new array, `arr` still points at the old one, and when the frame is
discarded the new array becomes garbage.

```
before:   arr ──▶ [1,2,3]        p ──▶ [1,2,3]     (same array)
after p = new:
          arr ──▶ [1,2,3]        p ──▶ [7,7,7]     (different arrays)
```

## The rule stated properly

> **Java copies the contents of the variable into the parameter. Modifying the
> parameter never affects the caller. Following a copied reference and modifying
> what it points to affects everything that shares that reference.**

Two distinct operations that look similar in source:

```java
p = something;        // assigns to the parameter — invisible to the caller
p[0] = something;     // follows the reference — visible to everyone
p.setName("x");       // follows the reference — visible to everyone
```

The first writes to the box. The second and third write to the thing at the far
end of the arrow.

Once you see that distinction, every case is predictable and you never need
another rule.

## Why the wrong answer persists

Because it usually predicts correctly. "Objects are passed by reference" gets
demonstration two right, and demonstration two is the common case. You can hold
the belief for a long time before meeting demonstration three.

And when you do meet it, the belief offers no explanation — only surprise. That is
the mark of a rule rather than a model, and this book's argument throughout has
been that models survive where rules do not.

## Practical consequences

**You cannot write a method that swaps two `int` variables.** Not a limitation to
work around; it follows from the semantics. Return the swapped values in an object,
or swap them at the call site.

**A method receiving an object can modify it.** Which is useful and dangerous. If a
method should not modify its argument, say so in the contract — and consider
making the object immutable, which Chapter 20 recommends and Chapter 18 shows Java
doing for `String`.

**Reassigning a parameter is invisible to the caller**, for every type. Chapter 11
advised against it on readability grounds; now you know it also accomplishes
nothing outward.

**Returning is the way to send information back.** The stack discipline means the
frame is discarded, so anything the caller should see must be returned or must be
a change to something the caller can still reach.

Next: several methods with the same name.
