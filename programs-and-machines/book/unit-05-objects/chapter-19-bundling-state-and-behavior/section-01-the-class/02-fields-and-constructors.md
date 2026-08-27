# Fields and Constructors

A **field** is a variable belonging to an object. Chapter 7 met them briefly and
noted the odd rule: unlike locals, fields are given a default automatically — 0,
`false`, or `null`.

That default is a problem, and constructors are the answer to it.

## The problem with defaults

```java
public class Account {
    private String owner;
    private long cents;
}

Account a = new Account();
```

`a` now exists with `owner` null and `cents` zero. It satisfies nothing. Its
invariant — an account has an owner and a non-negative balance — is not
established, and any method called on it either fails or quietly does the wrong
thing.

Worse, there is no way to tell from outside whether an `Account` has been properly
set up. A half-initialized object is indistinguishable from a real one until it
misbehaves.

## What a constructor is for

A **constructor** runs when an object is created and its job is to leave the
object satisfying its invariant. That is the whole purpose, and stating it that
way makes the design decisions follow.

```java
public Account(String owner, long cents) {
    if (owner == null || owner.isBlank())
        throw new IllegalArgumentException("owner required");
    if (cents < 0)
        throw new IllegalArgumentException("balance cannot start negative");
    this.owner = owner;
    this.cents = cents;
}
```

Now `new Account("Bad", -1)` fails immediately:

```
rejected: balance cannot start negative
```

and there is no such thing as an `Account` with a negative balance anywhere in the
program. Not "we try not to create one" — there is no way to obtain one.

That is a genuinely strong guarantee, and it is Chapter 11's fail-fast argument
placed where it does the most good: **at the boundary where an object comes into
being.** A bad value rejected here never enters the system at all.

## The rules

A constructor has the **class's name** and **no return type** — not even `void`.
That is not a style choice; it is how the compiler tells a constructor from a
method.

```java
public Account(String owner, long cents) { ... }     // constructor
public void Account(String owner) { ... }            // a method, confusingly named
```

The second compiles and is never called by `new`. If a constructor appears not to
run, check that you did not accidentally give it a return type.

If you write no constructor, Java supplies a **default constructor** taking no
arguments and setting everything to defaults. The moment you write any
constructor, that default disappears — so adding a two-argument constructor makes
`new Account()` stop compiling, which is usually exactly what you want.

## Several constructors

Overloading, from Chapter 12, applies:

```java
public Account(String owner, long cents) { ... }

public Account(String owner) {
    this(owner, 0);          // delegate to the other one
}
```

`this(...)` calls another constructor of the same class and must be the first
statement. Delegating like this means the validation lives in one place. Writing
the checks twice is how two constructors drift apart and one of them stops
enforcing something.

## final fields

```java
private final String owner;
private long cents;
```

`final` on a field means it must be assigned exactly once — in the constructor —
and never again. An account's owner does not change; its balance does.

This is worth doing wherever it is true, for the reason Chapter 7 gave: it removes
a field from the set of things a reader must track. It also makes a class safer to
share between threads, which Chapter 31 will explain.

A field that is `final` and whose type is immutable is a field you can stop
thinking about entirely. Note the second condition — `final` on a reference stops
reassignment, not modification of the object it points at:

```java
private final List<String> items = new ArrayList<>();
items = new ArrayList<>();     // error
items.add("x");                // fine — the list is not final, the field is
```

That distinction catches people, and Chapter 20 returns to it.

## What the constructor must not do

Two things worth knowing now.

**Do not let a reference to a half-built object escape.** If a constructor passes
`this` to something else before it has finished, that something can observe an
object that does not yet satisfy its invariant.

**Do not do heavy work.** A constructor should establish the invariant, not open
files or contact networks. An object that cannot be created without a working
network is an object you cannot test, which is Chapter 14's argument about
testability revealing a design problem.

Next: the methods that keep the invariant true once it has been established.
