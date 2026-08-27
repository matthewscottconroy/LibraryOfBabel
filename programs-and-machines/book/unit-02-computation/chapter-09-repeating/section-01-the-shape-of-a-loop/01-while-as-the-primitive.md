# while as the Primitive

```java
int i = 1;
int total = 0;
while (i <= 5) {
    total += i;
    i++;
}
System.out.println(total);      // 15
```

Read it as a machine. Evaluate the condition. If true, run the body, then go back
and evaluate the condition again. If false, skip past.

In Chapter 6's terms: a transition rule that consults the state, and on one
outcome sets the program counter *backwards*. That backward jump is the entire
novelty. Everything else we have seen moves forward.

## Why backwards matters

A program without loops executes each of its lines at most once, so its running
time is bounded by its length. Ten lines, at most ten steps.

Once the program counter can move backwards, that bound is gone. A three-line
loop can run for a billion steps, or forever. **The size of the program stops
telling you anything about the size of the computation**, and that is what makes
loops powerful and what makes them dangerous.

It is also, precisely, what took us from the parity machine to the Turing machine
in Chapter 6. The parity machine consumed one input symbol per step and stopped
when the input ran out; its running time was the input's length. A Turing machine
can revisit the tape, and cannot in general be predicted to stop at all.

## The three parts

Every loop has three moving parts, and if any is missing or wrong the loop
misbehaves:

```java
int i = 1;              // 1. initialization
while (i <= 5) {        // 2. condition
    total += i;
    i++;                // 3. progress toward the condition failing
}
```

**Initialization** puts the state where the loop expects it. **The condition**
decides whether to continue. **Progress** is the change that eventually makes the
condition false.

Omit the progress and the loop never ends:

```java
int i = 1;
while (i <= 5) {
    total += i;         // i never changes
}
```

The condition is true, stays true, and the program hangs. Nothing crashes. The
machine is working perfectly and will continue to for as long as you let it —
Chapter 6's second outcome, a cycle.

If a program of yours ever appears to freeze, this is the first thing to suspect,
and the fix is to find the variable the condition depends on and ask what is
supposed to change it.

## Infinite loops on purpose

Sometimes forever is right:

```java
while (true) {
    Event e = waitForNextEvent();
    handle(e);
}
```

That is an event loop, and Chapter 30 builds one. A program that services
requests has no natural end, and the loop expresses that honestly. The exit, when
it comes, is by `break`, by `return`, or by the process ending.

## break and continue

Two ways to interfere with the normal flow:

```java
while (condition) {
    if (somethingBad) break;      // leave the loop entirely
    if (skipThisOne) continue;    // jump to the next iteration
    // ...
}
```

Both are useful and both cost something: they add exits, so the reader can no
longer assume the loop ends only when the condition fails. That matters for the
invariant reasoning of the next section, where every exit is a place the claim
must hold.

Use them where they genuinely simplify — searching a collection and stopping at
the first match is clearer with `break` than with a flag variable — and be
suspicious of a loop with several.

## do-while

The variant that tests afterwards:

```java
int n = 0;
do {
    n++;
} while (n < 0);
System.out.println(n);      // 1
```

The condition `n < 0` was false from the start, and the body still ran once,
because `do`-`while` checks at the bottom.

Use it when the body must run at least once — prompting for input, for instance,
where you must ask before you can know whether the answer is acceptable. It is
uncommon, and when you meet one in code it is worth checking that the
run-at-least-once behavior was intended rather than accidental.

## The claim about primitiveness

`while` is the primitive because the others reduce to it. Here is `for` written
as `while`, and we will meet the abbreviation properly in the next lesson:

```java
for (int i = 1; i <= 5; i++) { body }

// is exactly
{
    int i = 1;
    while (i <= 5) {
        body
        i++;
    }
}
```

Note the outer braces: they exist because `i`'s scope in the `for` version is the
loop, and the rewrite must reproduce that.

`do`-`while` reduces too — run the body once, then loop. Every repetition in Java
is a conditional backward jump, which is the one mechanism Chapter 6 needed.

Next: the abbreviation, and why it is worth having.
