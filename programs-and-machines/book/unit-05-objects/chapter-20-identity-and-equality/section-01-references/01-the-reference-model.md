# The Reference Model

Chapter 12 established this and Chapter 19 relied on it. Here it is once more,
because everything in this chapter is a consequence.

```java
Account a = new Account("Ada", 1000);
```

```
a: ┌────────┐          ┌─────────────────────┐
   │ ref ───┼─────────▶│ owner: "Ada"        │
   └────────┘          │ cents: 1000         │
   stack               └─────────────────────┘
                        heap
```

The variable holds a **reference** — where the object is. The object is on the
heap.

## The one rule

**Assignment copies the contents of the variable.**

For a primitive the contents are the value. For an object the contents are a
reference. One rule, two very different-looking outcomes, and Chapter 12 showed
that keeping it as one rule is what makes every case predictable.

```java
int x = 5;
int y = x;          // y holds 5; independent

Account a = new Account("Ada", 1000);
Account b = a;      // b holds the same reference; one object
```

## Why objects are not copied

A reasonable question: why does `b = a` not make a second account?

Because copying is expensive, usually unwanted, and frequently ambiguous.

**Expensive** — an object may hold a list holding objects holding lists. Copying
on every assignment would make passing an argument arbitrarily costly.

**Unwanted** — most of the time you want to refer to the same thing. Passing an
account to a method that records a transaction should record it against *that*
account.

**Ambiguous** — if `Account` held a `List<Transaction>`, should copying it copy
the list? The transactions? There is no answer the language can give, which is
precisely why Section 20.1.3 exists.

So Java copies the reference, which is cheap and unambiguous, and leaves real
copying to you when you want it.

## What you can do with a reference

The complete list, from Chapter 12:

- **follow it** — `a.balance()`, `a.owner`
- **copy it** — `b = a`
- **compare it** — `a == b`, asking whether they point at the same object
- **set it to null** — `a = null`

You cannot inspect its numeric value, do arithmetic on it, or construct one. That
restriction is what makes Java memory-safe, and it is why the word "reference" is
used rather than "pointer".

## null, once more

```java
Account a = null;
a.balance();      // NullPointerException
```

A reference that points at nothing. Chapter 16 discussed whether it should have
existed; here the practical point is that **`null` is a legitimate value of every
reference type**, so any variable of object type may be holding it, and the
compiler will not tell you which.

Note what `null` is not. It is not an `Account` with empty fields. It is not zero.
It is not an object at all, so there is nothing to call a method on — which is why
the error says *cannot invoke because it is null* rather than reporting a missing
method.

## Where this is going

Three consequences, one per lesson in this section and the next.

Two variables can refer to one object, so a change through one is visible through
the other. That is **aliasing**.

Copying a reference is not copying an object, and copying an object is not
copying what it contains. That is **shallow versus deep**.

`==` compares references, so it asks whether two names denote one object — which
is almost never the question you meant. That is **identity versus equality**.

Every one of them follows from the picture at the top of this page.
