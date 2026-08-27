# Tests as Documentation

A test suite is documentation, and it has one property no other documentation has:
**it cannot become wrong without telling you.**

## The problem with comments

A comment says what the author believed at the time of writing. Code changes; the
comment does not, unless someone remembers.

```java
// Returns the largest value, or 0 if the array is empty.
static int largest(int[] a) {
    if (a.length == 0) throw new IllegalArgumentException("empty");
    ...
}
```

The comment is wrong. It was true once. Someone changed the behavior and did not
update it, and now the documentation actively misleads — worse than no comment,
because it is believed.

Nothing detects this. The compiler does not read comments. Reviewers skim them.

A test cannot drift this way:

```java
check("empty array returns 0", 0, largest(new int[]{}));
```

Change the method to throw and this test fails. The documentation is checked on
every run, and stale documentation becomes a build failure rather than a trap.

## Reading tests to learn a method

This is a practical skill worth naming, because it is how experienced programmers
approach unfamiliar code.

Faced with a method you do not understand, **find its tests**. They show you:

- what inputs it accepts, from the values used
- what it returns for them, from the expected values
- what the edge cases are, from which cases someone bothered to write
- what it does on bad input, from the tests for failure
- how it is meant to be called, from the setup

That is the contract, demonstrated with concrete values, which is frequently
clearer than a paragraph of prose. A test that says `check("all negative", -2,
largest(new int[]{-5,-2,-9}))` settles the question of what happens with negatives
in a way no English sentence does as quickly.

Which suggests a way to write tests: **write them so that someone reading them
learns the method.**

## Names that say something

Follow from that. A test's name should describe the case and, ideally, the
expected behavior:

```java
check("empty", ...);                          // weak
check("empty array throws", ...);             // better
check("largest of empty array is rejected", ...);  // better still
```

When a test fails, its name is the first thing you see, and a good one tells you
what broke without opening the file. In JUnit the convention is a descriptive
method name — `largestOfEmptyArrayThrows()` — which reads badly as a method name
and well as a failure report, and the failure report is what matters.

## Structure

A test has three parts, and separating them makes tests readable:

```java
// arrange: set up the input
int[] scores = {5, 3, 9, 1};

// act: call the thing under test
int result = largest(scores);

// assert: check the result
check("finds max in unsorted", 9, result);
```

Sometimes called *arrange, act, assert*. In one-line tests it collapses, and in
anything larger the discipline of keeping the three separate is what stops tests
becoming as hard to read as the code they check.

## One thing per test

The same principle as one job per method.

```java
// less good: several claims, one test
check("stats", ...) {
    // checks mean, max, min, and count
}

// better: four tests
check("mean of {1,2,3} is 2", ...);
check("largest of {1,2,3} is 3", ...);
check("smallest of {1,2,3} is 1", ...);
check("count of {1,2,3} is 3", ...);
```

The reason is diagnostic. When the combined test fails you know something in the
statistics is wrong; when a specific test fails you know what. And a combined test
usually stops at the first failed assertion, hiding whether the others also broke.

## Tests as a design tool

The last observation, and it is the one that changes how people work.

**Difficulty testing something is evidence about its design.**

If a method is hard to test, ask why. Usually one of:

- it does too many things, so a test must set up all of them
- it depends on something external — a file, a clock, a network — that a test
  cannot control
- it requires elaborate setup, which means it is tightly coupled to its context
- it has no return value to check, so its effect is invisible from outside

Every one of those is a design problem that testing revealed. The method was
already awkward to use; writing a test is what made the awkwardness explicit.

This is the argument behind **test-driven development**, where tests are written
before the code. Whether you adopt that discipline is a matter of taste and
context. The underlying observation stands regardless: **the first client of a
method is its test, and if the first client finds it awkward, so will the others.**

## Closing the chapter, and the unit

Decomposition has no algorithm, and it has signals — blank lines, section
comments, repetition, depth, and difficulty naming — that tell you where a
computation wants to divide. A method has one job when it works at a single level
of abstraction and a caller would think of it as one action. Commands do things,
queries answer things, and mixing the two costs you the ability to reason about
either.

Tests are contracts made executable. Choose cases by equivalence class and probe
the boundaries — empty, one, two, duplicates, extremes — because that is where
decisions change and where bugs live. Coverage finds untested regions and makes a
bad target. And a test suite is the only documentation that fails when it becomes
untrue.

Unit III set out to solve the problem Unit II left: a program written as pure
states and transitions is unreadable past a few hundred lines. The remedy was
naming a process. A method is a contract; the stack is the mechanism that makes
calls and returns work, and that makes recursion unremarkable; recursion is
induction written as code; and decomposition is the judgment about where to draw
the lines.

What we still cannot do is give a *name to a collection of values*. Everything so
far has handled one number, one character, one boolean at a time. Real programs
handle thousands at once, and the next unit is about how a heap of values becomes
a structure.
