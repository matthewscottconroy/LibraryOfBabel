# Returning a Value

So far a method has been a one-way street. You hand it something and it goes off
and does a thing — prints, perhaps.

But most of the methods you actually want are not like that. You want to *ask*,
and get an answer you can hold: store it, compare it, feed it to something else.
`Math.max(a, b)` is not useful for anything it does. It is useful entirely for
what it hands back.

Parameters carry information in. **Return values** carry it out.

```java
static int square(int n) {
    return n * n;
}
```

The `int` before the name is the **return type** — a promise, made in the header,
that whatever else happens in this method, an `int` is coming out. `return`
supplies it and ends the method on the spot.

## return leaves. Immediately.

That word "immediately" does more work than it looks like:

```java
static int firstNegative(int[] a) {
    for (int i = 0; i < a.length; i++) {
        if (a[i] < 0) return i;      // done — out of the loop, out of the method
    }
    return -1;                        // got all the way through, found nothing
}
```

`return i` does not leave the loop. It leaves the *method*, from wherever it is,
however deep. Which is what makes the guard-clause style work, and it is the clean
answer to a problem Chapter 9 left you with — how to escape from the middle of
nested loops without a flag variable and a lot of hoping.

Write anything after a `return` on the same path and Java refuses to compile it. It
knows that line can never run, and it would rather tell you than let you believe
otherwise.

## void

A method that hands nothing back says so:

```java
static void greet(String who) {
    System.out.println("Hello, " + who);
}
```

There is the second word of `main` accounted for. `main` returns `void` because
there is nothing it could usefully hand back — the JVM is not waiting for an
answer, it is waiting for the program to be over.

A `void` method can still leave early. Just `return;`, with nothing after it:

```java
static void process(int[] data) {
    if (data.length == 0) return;      // nothing here to do
    // ...
}
```

## Two kinds of method, and one of them is much easier to live with

A method either **computes a value** or **causes an effect**. `square` computes.
`greet` causes — it prints, which changes something in the world outside itself.

Now look at what you can do with the first kind. Call `square(7)` a hundred times
and you get 49 a hundred times, and nothing else about your program is different
for having called it. So you can test it by comparing what went in with what came
out. You can move the call. You can delete one you did not need. You can reorder
two of them. None of it requires thought.

Try any of that with `greet`. Call it twice and it prints twice. Move it and the
output comes out in a different order. Delete one and something is missing. Test
it and you find you have to capture the output first. Every one of those
operations now requires you to stop and think about consequences.

A method with no effects beyond its return value is called **pure**, and the rule
of thumb worth carrying is this: **prefer pure methods, and when a method has to
have an effect, do not also make it compute something interesting.**

A method that both changes state *and* returns a value is one whose calls cannot
be moved or removed without care, and every reader has to notice both jobs, every
time. When functions themselves become values in Chapter 26 this comes back with
much sharper teeth, and Unit VII is largely about the parts of a program where
effects cannot be avoided and have to be corralled instead.

## Java hands back exactly one thing

Which is occasionally not enough. You want a minimum and a maximum. A quotient and
a remainder. And every option is a little unsatisfying:

**Return an object holding both.** Usually right, and Chapter 22 has a one-line way
to make one that exists for precisely this.

**Return an array.** `return new int[]{min, max};` — works, and now the caller has
to remember which index is which. You have turned a positional-argument problem
around and pointed it at yourself.

**Fill in something the caller passed.** Common in older libraries, generally
worse, because the method's real effect has become invisible at the call site.

**Write two methods.** Often the best answer, particularly when the two results are
useful separately. Two passes over the data is usually a price worth paying for
clarity, and Chapter 32 will give you the tools to know when it is not.

## Say what you mean in the name

Readers lean on the agreement between a return type and a name far more than they
realize:

```java
int  count(...)         // a number
boolean isValid(...)    // true or false
String format(...)      // text
void save(...)          // an action
```

Two conventions are worth adopting outright. A `boolean` method is called
`isSomething` or `hasSomething`, so that `if (isValid(x))` reads as English. A
method that returns a value is named for the value — `largest`, `count`, `total`
— and a method that causes an effect is named for the action — `save`, `print`,
`update`.

Break the convention and you mislead people who trusted you. A method called
`getBalance` that quietly opens a network connection is a small betrayal, and the
code written around it will assume calling it is cheap and safe, because that is
what the name said.

## What a signature tells you, and what it hides

Here is the first line of a method — its **signature** — doing a remarkable amount
of work:

```java
static int largest(int[] values)
```

*Give me an array of `int`, and I will give you back an `int`.* A reader learns
what goes in and what comes out without reading a single line of the body. That is
the abstraction paying for itself.

Now notice what it does *not* say.

It does not say what happens if the array is empty. It does not say whether
`values` comes back modified. It does not even promise the answer is the largest
rather than the smallest, or the first, or 42.

All of that is real, all of it matters, and none of it is in the signature. Those
are the rest of the terms — and the next section is about writing them down.
