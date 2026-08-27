# Parameters and Arguments

A method that always did the same thing would be a poor abstraction. Information
gets in through **parameters**.

```java
static int square(int n) {
    return n * n;
}
```

`n` is a parameter: a variable, declared in the method's header, that receives a
value when the method is called.

```java
int result = square(7);
```

`7` is an **argument**: the actual value supplied at the call.

The distinction is worth keeping. The parameter is the name in the definition;
the argument is the value at the call site. One method has fixed parameters and
sees a different argument every time it is called.

## The parameter is a local variable

This is the mental model to hold, and it settles most questions.

When `square(7)` is called, a fresh variable `n` comes into existence, holding a
**copy** of 7. It lives for the duration of the call — Chapter 7's lifetime,
Chapter 12's call frame — and vanishes on return.

So a parameter behaves exactly like a local variable that was initialized for
you. It can be reassigned inside the method, and doing so affects nothing outside:

```java
static void tryToChange(int n) {
    n = 99;
}

int x = 5;
tryToChange(x);
System.out.println(x);      // 5
```

`x` is unchanged. `n` received a copy, the copy was overwritten, and the copy is
gone.

That is **pass by value**, and Java does it for everything. Chapter 12 will show
what it means for objects, where the answer surprises people; for primitives it
is exactly as straightforward as it looks.

Reassigning a parameter is legal and generally poor practice, because a reader
expects the parameter to hold what the caller passed. If you want a modified
version, make a local:

```java
static int countdown(int start) {
    int n = start;         // clearer than reassigning start
    while (n > 0) { ... }
}
```

## Several parameters

Separated by commas, and matched to arguments **by position**:

```java
static int max(int a, int b) {
    return a > b ? a : b;
}

max(3, 9);      // a is 3, b is 9
```

Position is the whole story, and it is a real hazard once there are several
parameters of the same type:

```java
drawRectangle(10, 20, 5, 3);
```

Which is width and which is height? Which pair is the position? Nothing in the
call says, and passing them in the wrong order compiles cleanly and draws the
wrong thing.

Some languages let the caller name arguments — `drawRectangle(x: 10, y: 20, ...)`
— and Java does not. Three partial defenses:

**Fewer parameters.** A method needing six is often a method needing to be
restructured. Three or four is usually the point at which to worry.

**Distinct types where possible.** The compiler cannot catch a swapped pair of
`int`s and can catch a swapped `int` and `String`. Unit V's custom types make this
much stronger — a `Width` and a `Height` cannot be exchanged.

**Order by convention.** If related methods take `(row, column)`, take
`(row, column)` everywhere. Consistency is the cheapest protection available.

## How many is too many

A rough guide: **zero to two is comfortable, three is fine, four asks a question,
five or more is a signal.**

The signal is usually one of two things. Either the method is doing too much and
wants splitting, or several of the parameters belong together and want to become
one object — a `Rectangle` rather than four numbers. Unit V gives you that second
option, and it is frequently the right answer.

## Variable numbers of arguments

Occasionally a method genuinely takes any number:

```java
static int sum(int... xs) {
    int total = 0;
    for (int x : xs) total += x;
    return total;
}

sum(1, 2, 3, 4);      // 10
sum();                // 0
```

The `...` makes `xs` an array holding whatever was passed. `System.out.printf`
works this way, which is why it accepts any number of values after the format
string.

Use it where the count is genuinely open-ended. It is not a way to avoid deciding
what your parameters are.

## The debt from Chapter 5

We can now pay part of one.

```java
public static void main(String[] args)
```

`String[] args` is a parameter. It is an array of strings — Chapter 15 for
arrays — holding whatever was typed on the command line after the class name.

```
$ java Hello Alice Bob
```

gives `args` two elements, `"Alice"` and `"Bob"`.

So `main` is a method like any other, and the JVM is its caller. It takes a
parameter because the person starting the program may have something to say to
it. That is one word of the incantation explained; `static`, `void`, and `public`
remain, and two of them arrive in the next lesson.

Next: how information gets back out.
