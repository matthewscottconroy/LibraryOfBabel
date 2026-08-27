# Testing with JUnit

Chapter 14 argued for tests and wrote them by hand, with a `check` method and a
tally. That was deliberate: a test is an executable claim, and no framework is
needed to make one.

This appendix is the framework. **JUnit** supplies the plumbing Chapter 14 wrote
by hand — the assertions, the discovery of test methods, the reporting — and
changes nothing about what a test *is*.

## The hand-written version, recalled

```java
static int passed = 0, failed = 0;

static void check(String name, int expected, int actual) {
    if (expected == actual) passed++;
    else { failed++; System.out.println("FAIL " + name); }
}
```

Three things are unsatisfactory about it, and JUnit fixes exactly those.

You must call every test yourself from `main`, so a forgotten call is a test that
silently never runs. A failure stops nothing, so a crash in one test loses the
rest. And the reporting is whatever you wrote.

## Getting it

JUnit 5 is a library, so it must be on the classpath. With Maven, in `pom.xml`:

```xml
<dependency>
    <groupId>org.junit.jupiter</groupId>
    <artifactId>junit-jupiter</artifactId>
    <version>5.10.2</version>
    <scope>test</scope>
</dependency>
```

Gradle:

```
testImplementation 'org.junit.jupiter:junit-jupiter:5.10.2'
```

Every modern IDE will also add it for you from a menu, which for a first
encounter is the path of least resistance. Version numbers move; take the current
one from the JUnit site rather than from this page.

## A test class

```java
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class LargestTest {

    @Test
    void largestOfSingleElementIsThatElement() {
        assertEquals(5, Largest.of(new int[]{5}));
    }

    @Test
    void largestFindsMaximumInUnsortedArray() {
        assertEquals(9, Largest.of(new int[]{3, 9, 1}));
    }

    @Test
    void largestHandlesAllNegativeValues() {
        assertEquals(-2, Largest.of(new int[]{-5, -2, -9}));
    }

    @Test
    void largestRejectsEmptyArray() {
        assertThrows(IllegalArgumentException.class,
                     () -> Largest.of(new int[]{}));
    }
}
```

Four things to notice.

**`@Test`** marks a method as a test. This is an **annotation** — Chapter 27's
subject — and it is how JUnit finds your tests without you listing them. A
forgotten call is no longer possible.

**The method names are sentences.** `largestOfSingleElementIsThatElement` reads
badly as a method name and well as a failure report, and the failure report is
what you see when something breaks. This is Chapter 14's argument about test
names, and JUnit's convention follows it.

**Assertions throw.** `assertEquals` throws when the values differ, so the test
method stops and JUnit records a failure and moves to the next test. One broken
test no longer loses the others.

**`assertThrows`** checks that a call fails as promised. The `() ->` is a lambda
— Chapter 26 — and here it means "the code to run and expect an exception from".
Chapter 11 said failure behavior is part of the contract; this is how you test it.

## The assertions worth knowing

```java
assertEquals(expected, actual);
assertEquals(expected, actual, "message shown on failure");
assertNotEquals(unexpected, actual);

assertTrue(condition);
assertFalse(condition);

assertNull(value);
assertNotNull(value);

assertSame(expected, actual);        // identity: ==
assertNotSame(unexpected, actual);

assertArrayEquals(expected, actual);
assertThrows(SomeException.class, () -> risky());

fail("should not reach here");
```

**Expected comes first.** `assertEquals(5, result)`, not the other way round. Get
it backwards and the failure message tells you the opposite of the truth, which
is a genuinely confusing five minutes.

**`assertEquals` uses `equals`, `assertSame` uses `==`.** Chapter 20's
distinction, and the reason there are two.

**Floating point needs a tolerance:**

```java
assertEquals(0.3, a + b, 1e-9);
```

Chapter 3 explained why. `assertEquals(0.3, 0.1 + 0.2)` fails, correctly, and the
third argument is the "close enough" you must decide on explicitly.

## Running them

From a build tool:

```
$ mvn test
$ gradle test
```

From an IDE, a button next to the class or the method.

A passing run reports counts; a failing one reports the test name, the expected
and actual values, and a stack trace pointing at the assertion. That is Chapter
10's material: read the whole thing, note the values, find the topmost frame in
your own code.

## Setup and teardown

When several tests need the same starting state:

```java
class AccountTest {
    private Account account;

    @BeforeEach
    void createAccount() {
        account = new Account("test", 100);
    }

    @Test
    void depositIncreasesBalance() {
        account.deposit(50);
        assertEquals(150, account.balance());
    }

    @Test
    void withdrawDecreasesBalance() {
        account.withdraw(30);
        assertEquals(70, account.balance());
    }
}
```

`@BeforeEach` runs before **every** test, so each gets a fresh `Account`. That
matters: tests must not depend on each other or on the order they run in, and a
shared object mutated by one test is the commonest way that goes wrong.

There is also `@AfterEach` for cleanup, and `@BeforeAll`/`@AfterAll` for
once-per-class setup — the latter must be `static`, and should be used only for
things genuinely expensive to create.

## Parameterized tests

When the same check applies to many inputs:

```java
@ParameterizedTest
@ValueSource(ints = {2, 3, 5, 7, 11, 13})
void primesAreRecognized(int n) {
    assertTrue(Primes.isPrime(n));
}

@ParameterizedTest
@CsvSource({"0, 1", "1, 1", "5, 120", "10, 3628800"})
void factorialIsCorrect(int input, long expected) {
    assertEquals(expected, Factorial.of(input));
}
```

Each value produces a separate test with its own pass or fail, which is Chapter
14's one-claim-per-test rule without the copying.

## What not to do

**Do not test the implementation.** Chapter 14 said test the specification. A test
that reaches inside and checks a private field breaks when you improve the
internals — punishing exactly the change it should protect.

**Do not write tests that depend on order.** JUnit does not promise one, and a
suite that passes only in a particular sequence is a suite that will fail
mysteriously later.

**Do not chase coverage.** Chapter 14's argument: it reports which lines ran, not
whether the assertions meant anything.

**Do not assert nothing.** A test that calls a method and checks no result only
verifies that it did not throw. Sometimes that is the point; usually it is an
oversight.

## The habit

Write a test when you fix a bug. Chapter 14 called this the most valuable
category, and it is the easiest to sustain: you already have the failing input,
you already know the expected result, and the test that would have caught it is
usually three lines.

Over time those accumulate into a suite that reflects what actually goes wrong in
your code, which is worth more than any suite designed in advance.
