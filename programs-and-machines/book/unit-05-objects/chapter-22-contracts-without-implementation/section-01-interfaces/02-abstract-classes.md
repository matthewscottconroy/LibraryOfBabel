# Abstract Classes

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

`deposit` was written once. The 5 that `Percent` withheld came from a method
`Account` calls but does not implement.

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
the steps and does not control the order. That inversion is the value: the
sequence lives in one place, and a subclass cannot get it wrong, because it never
sees it.

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

If you find yourself unsure, choose the interface. It constrains implementers
less, it composes with other interfaces, and Chapter 23 will argue that the
inheritance an abstract class requires is a commitment worth avoiding when you
can.

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
