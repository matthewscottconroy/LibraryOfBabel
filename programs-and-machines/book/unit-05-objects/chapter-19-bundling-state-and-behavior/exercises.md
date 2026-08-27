# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Building a class

**19.1.** Write a `Rectangle` class with `width` and `height`, a constructor that
rejects non-positive values, and methods `area()` and `perimeter()`. State its
representation invariant as a comment.

**19.2.** Write a `Temperature` class storing tenths of a degree as an `int`, with
`celsius()` and `fahrenheit()` accessors. Why store tenths rather than a `double`?
(Chapter 3.)

**19.3. [carries forward]** Write a `Stack` class backed by an array, supporting
`push`, `pop`, `peek`, `size` and `isEmpty`. Write the invariant first, then check
each method against it. What should `pop` do on an empty stack?

**19.4.** Add a `toString` to `Rectangle`. Print one before and after adding it,
and explain the first output.

## Constructors

**19.5.** What happens if you write no constructor at all? What happens to that
behaviour the moment you write one?

**19.6.** Why does a constructor have no return type? What happens if you give it
`void` by mistake?

**19.7. [carries forward]** Write two constructors for `Account` — one taking an
opening balance and one defaulting to zero — with the validation in only one
place. Which construct did you use?

**19.8.** Explain why a constructor is the right place for validation, in terms of
what would otherwise be possible.

## Encapsulation

**19.9.** `private` does not stop reflection and does not stop someone editing the
source. What, then, does it actually accomplish?

**19.10.** For each, say which access level you would choose and why:
- an account's balance field
- a helper method that formats a balance for printing
- the `deposit` method
- a constant for the maximum overdraft

**19.11.** Explain what is wrong with this pair, given the class's invariant:
```java
public long getCents()       { return cents; }
public void setCents(long c) { cents = c; }
```

**19.12. [carries forward]** A method returns the internal `List<Transaction>`.
Give three ways to fix it, and say which you would choose and why.

**19.13.** Rewrite in the "tell, don't ask" style, and say what becomes easier to
change:
```java
if (account.balance() >= price) account.withdraw(price);
```

## static

**19.14.** Predict the output:
```java
Counter a = new Counter(), b = new Counter();
a.bump(); a.bump(); b.bump();
System.out.println(a.mine() + " " + b.mine() + " " + Counter.total());
```

**19.15.** Explain the error `non-static variable n cannot be referenced from a
static context` in terms of what object the field would belong to.

**19.16.** For each, say whether it should be static:
`Math.sqrt`; an account's balance; `MAX_ATTEMPTS`; a method that creates a
default `Account`; a counter of how many accounts exist.

**19.17.** Why is mutable static state usually a mistake? Name two distinct
problems.

## Going further

**19.18.** Go back to your answers to exercise 5.18 — what you believed `public`,
`static`, `void` and `String[]` did. Compare them with Section 19.2.3. Write a
short paragraph on what changed, and on what you had no way of knowing then.

**19.19.** Take the `NameSet` from Chapter 16 — `String[] names` and `int count` —
and turn it into a proper class. Write the invariant as a comment, make the fields
private, and check each operation preserves it. Then write a test that would catch
a violation.

**19.20.** Section 19.2.1 gives an honest exception where public fields are
acceptable. State its three conditions, then find a class in the Java library that
meets them.

**19.21.** A class has fifteen public methods. Argue that this is a problem
without knowing what the class does. Then give a case where it would be
justified.
