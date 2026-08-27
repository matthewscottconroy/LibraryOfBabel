# The Public Surface

Everything `public` about a class is a promise. Deciding what goes there is the
main design decision in writing one.

## What the surface is

The **public surface** — often called the API — is everything a user of the class
can see and rely on: the public methods, their names, their parameters, their
return types, and the behavior they promise.

It is not only what you documented. Chapter 16's warning applies: whatever people
can observe, they come to depend on. If a method happens to return results in a
particular order, someone will rely on the order. If an exception happens to be
thrown for a particular bad input, someone will catch it.

So the surface is larger than the list of methods, and every part of it is a
commitment.

## Designing it

The question to ask is not "what does this class contain" but **"what should a
caller be able to do?"**

For an account: deposit, withdraw, ask the balance, ask the owner. Four
operations, none of which mentions `cents` or `long` or the fact that money is
stored in pence.

Notice what that buys. The representation could change to `BigDecimal`, or to a
currency-plus-amount pair, and none of the four operations changes. That is
Chapter 16's argument — the storage decision is exactly the kind of thing that
changes, so it belongs behind a boundary.

Three habits:

**Start from the caller.** Write the code that uses the class before writing the
class. Awkwardness shows up immediately, which is Chapter 14's point about tests
being the first client.

**Name operations in the domain.** `deposit`, not `addToCents`. The name should
make sense to someone who knows about accounts and nothing about your code.

**Expose the smallest set that does the job.** Every extra method is a commitment
you may want back, and you cannot have it back once people use it.

## Do not return your internals

The trap from Chapter 16, now concrete:

```java
public List<Transaction> history() {
    return transactions;      // hands out the actual list
}
```

The caller can now `history().clear()`. Your invariant is broken by code nowhere
near your class, and no amount of care inside it helps.

Three fixes, in rough order of preference:

```java
return List.copyOf(transactions);                     // an immutable snapshot
return Collections.unmodifiableList(transactions);    // an unmodifiable view
return new ArrayList<>(transactions);                 // a mutable copy
```

The first is usually right. The second is a *view* — it reflects later changes to
the underlying list, which is sometimes what you want and sometimes a surprise.
The third costs a copy and lets the caller do what they like with it.

The same applies to arrays, to dates in older APIs, and to anything mutable you
hold. Chapter 20 gives this a name — defensive copying — and explains when it is
worth its cost.

## toString

One method worth writing on nearly every class:

```java
@Override
public String toString() {
    return owner + ": " + cents + "c";
}
```

Without it, printing an object gives something like `Account@1b6d3586` — the class
name and a hash code, which is what Chapter 15 warned about with arrays.

`toString` is for **humans reading diagnostics**, not for machines. Do not make it
the serialization format, do not include secrets, and keep it short. Chapter 10's
debugging advice depends on being able to print things usefully, and a class
without `toString` makes every log line worse.

The `@Override` is an annotation asserting that this method replaces one from a
supertype. It is optional and you should always write it: if you misspell the
method, the compiler tells you instead of silently defining an unrelated method.
Chapter 21 explains what it is overriding.

## Documenting the surface

Chapter 11's contracts belong here, and the class deserves one of its own:

```java
/**
 * A bank account with an owner and a balance in whole pence.
 *
 * <p>The balance is never negative. Withdrawals that would overdraw
 * are refused rather than throwing.
 *
 * <p>Instances are not safe for use by multiple threads.
 */
public class Account { ... }
```

Three things worth stating at class level: what the thing is, what its invariant
is, and whether it is safe to share between threads. The last one is easy to omit
and Chapter 31 will show what it costs.

Next: the keyword that belongs to the class rather than to any object.
