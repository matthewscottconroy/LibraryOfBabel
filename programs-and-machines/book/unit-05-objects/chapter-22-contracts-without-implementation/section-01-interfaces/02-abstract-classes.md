# Abstract Classes

Sometimes neither extreme fits. You have four kinds of account that genuinely share
a balance and genuinely share the logic for depositing into it — but they differ on
one step, the fee, and you cannot write that step without knowing which kind you
have.

You want to write the algorithm once and leave a hole.

An interface has no implementation. A class has all of it. An **abstract class**
is the position between: some methods supplied, some left as holes.

```java
abstract class Account {
    private long balance;

    Account(long start) { this.balance = start; }

    long balance() { return balance; }

    void deposit(long amount) {
        balance += amount - fee(amount);
    }

    abstract long fee(long amount);      // the hole
}
```

`Account` has state, a constructor, an invariant it maintains, and a complete
implementation of `deposit` — except for one decision it refuses to make. Each
subclass fills it:

```java
class Free    extends Account { long fee(long a) { return 0;       } }
class Percent extends Account { long fee(long a) { return a / 100; } }
```

Verified, depositing 500 into each starting at 1000:

```
Free 1500
Percent 1495
```

Look at what just happened there. `deposit` was written exactly once, in
`Account`, and it produced two different answers.

The 5 that `Percent` withheld came out of a method that `Account` calls and does
not implement — a hole in the parent, filled by the child, at a moment the parent
chose.

## What abstract means

`abstract` on a method declares it without a body. `abstract` on a class means the
class cannot be instantiated — `new Account(0)` does not compile, and correctly,
because there would be no `fee` to call.

A class must be declared abstract if it has any abstract method. It may be
declared abstract without one, which is a way of saying *this is not a complete
thing*, and it is occasionally useful.

## The template method

The shape above has a name — Gamma and colleagues called it the **template
method**, and it is one of the few design patterns that earns its keep.

The parent writes the algorithm and calls out to the steps. The subclass supplies
the steps and has no say at all in when they happen.

That inversion is the entire value, and it is worth saying the consequence out
loud: **the subclass cannot get the sequence wrong, because it never sees the
sequence.** You have not asked subclass authors to be careful. You have arranged
things so that carefulness is not required of them.

Compare with what you would otherwise write — each subclass implementing
`deposit` in full — and the difference is that in the abstract version the
balance-updating logic exists once. If it is wrong, it is wrong in one place.

The caution attached is Chapter 21's constructor trap, in stronger form. The
parent is calling a method the child overrides, deliberately. That is fine after
construction and dangerous during it, so an abstract class's constructor must not
call its own abstract methods.

## Choosing between the two

The distinction has narrowed since Java 8 gave interfaces default and static
methods. What remains is real:

| | interface | abstract class |
|---|---|---|
| fields (state) | no | yes |
| constructors | no | yes |
| how many per class | any number | one |
| method bodies | `default` and `static` only | any |
| private helpers | since Java 9 | yes |
| the relationship | *can do* | *is a kind of* |

The practical rule is the last row.

**Reach for an interface** when unrelated things share a capability, when you want
callers to depend on a promise rather than a class, or when implementers already
have a superclass. This should be the default.

**Reach for an abstract class** when the implementations genuinely share state and
code, when they are honestly kinds of one thing, and when you control them all.
`Account` qualifies: the balance is common, `deposit` is common, and a savings
account really is a kind of account.

And if you are genuinely unsure, choose the interface. It constrains your
implementers less, it composes with other interfaces where an abstract class cannot,
and Chapter 23 is going to argue that the inheritance an abstract class demands is a
commitment worth avoiding whenever you have the choice.

## Both at once

The standard library frequently does both, and it is a useful idiom to recognize:

```java
public interface List<E> extends Collection<E> { ... }
public abstract class AbstractList<E> implements List<E> { ... }
public class ArrayList<E> extends AbstractList<E> { ... }
```

The interface is the type callers use. The abstract class is a convenience for
implementers — it supplies everything derivable from a few core methods, so a new
list type need only write `get` and `size` to get `contains`, `indexOf`, and
`iterator` free.

Nothing forces an implementer to use it. A class that already has a superclass can
implement `List` directly and write all the methods. That is the arrangement
working as intended: the contract is mandatory, the help is optional.

Next: the convention that follows from all this.
