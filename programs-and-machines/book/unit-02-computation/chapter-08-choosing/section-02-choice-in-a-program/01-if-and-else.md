# if and else

```java
if (temperature > 30) {
    System.out.println("Hot");
}
```

The condition in parentheses must be a `boolean`. If it is true, the block runs;
if not, it is skipped. In Chapter 6's terms: the machine consults its state and
chooses between two continuations.

## The forms

```java
if (condition) {
    // when true
}

if (condition) {
    // when true
} else {
    // when false
}

if (first) {
    // ...
} else if (second) {
    // ...
} else {
    // none of the above
}
```

The third form is worth reading carefully, because it is not a separate
construct. `else if` is an `if` inside an `else`, and Java lets you omit the
braces around a lone statement. Written out:

```java
if (first) {
    // ...
} else {
    if (second) {
        // ...
    } else {
        // ...
    }
}
```

Which means the branches are tested **in order**, and the first true one wins.
Every later condition is only reached when all earlier ones were false — so you
can rely on that when writing them:

```java
if (score >= 90)      grade = 'A';
else if (score >= 80) grade = 'B';
else if (score >= 70) grade = 'C';
else                  grade = 'F';
```

The second test does not need `score < 90`, because reaching it already
guarantees that. Writing the redundant half is a common beginner habit and it
makes the code longer and no safer.

## The condition must be boolean

```java
if (count) { }        // error: int cannot be converted to boolean
if (count != 0) { }   // fine
```

Some languages accept a number, treating zero as false. Java does not, and it is
a deliberate choice: it makes `if (x = 5)` — assignment where comparison was
meant — a compile error rather than a silent bug, because `x = 5` produces an
`int`.

The protection lapses when the variable is a `boolean`, where `if (flag = true)`
compiles and always takes the branch. Rare, and worth knowing.

While we are here: never write `if (flag == true)`. Write `if (flag)`. The
comparison adds nothing, and it is one keystroke from the bug above.

## Braces

Java allows the braces to be omitted for a single statement:

```java
if (x > 0)
    System.out.println("positive");
```

Do not do this. The reason is that it makes adding a second statement a trap:

```java
if (x > 0)
    System.out.println("positive");
    System.out.println("definitely positive");
```

The indentation says both lines are conditional. The compiler says only the first
is, because a lone statement is what was attached to the `if`. The second prints
always.

This exact class of mistake caused a serious TLS vulnerability in Apple's
software in 2014 — known as "goto fail" — where a duplicated line sat outside a
brace-less conditional and caused certificate validation to be skipped. The
duplicated line was not the whole story, but brace-less conditionals were what
allowed a stray line to change control flow invisibly.

Always use braces. It costs two characters.

## Nesting and its alternative

Conditions inside conditions get hard to read fast:

```java
if (user != null) {
    if (user.isActive()) {
        if (user.hasPermission()) {
            proceed();
        }
    }
}
```

Three levels of indentation, and to know when `proceed()` runs you must hold all
three conditions at once.

Two rewrites help. Combine them, if that reads well:

```java
if (user != null && user.isActive() && user.hasPermission()) {
    proceed();
}
```

Or invert and return early, which is usually better when there is something to
say about each failure:

```java
if (user == null)             return;
if (!user.isActive())         return;
if (!user.hasPermission())    return;
proceed();
```

This is called a **guard clause**. The conditions are handled and dismissed one
at a time, and the main path ends up at the left margin instead of buried. When
you find yourself three levels deep, this is usually the fix.

## The conditional operator

For choosing between two *values*, there is a compact form:

```java
String size = (n > 100) ? "large" : "small";
```

Read as: if `n > 100`, the expression is `"large"`, otherwise `"small"`. It is an
expression, so it produces a value and can go anywhere a value can.

It is good for exactly this — picking one of two values — and bad when nested or
when the branches do work rather than produce values. `a ? b : c ? d : e` is
legal and nobody should have to read it.

Next: the difference between `&&` and `&`, which turns out to matter.
