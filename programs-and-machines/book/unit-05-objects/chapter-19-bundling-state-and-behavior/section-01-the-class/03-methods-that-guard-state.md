# Methods That Guard State

The constructor establishes the invariant. Every method must preserve it — which
is Chapter 16's obligation, now with somewhere to live.

```java
public void deposit(long amount) {
    if (amount <= 0)
        throw new IllegalArgumentException("deposit must be positive");
    cents += amount;
}

public boolean withdraw(long amount) {
    if (amount <= 0)
        throw new IllegalArgumentException("withdrawal must be positive");
    if (amount > cents) return false;
    cents -= amount;
    return true;
}
```

Run it:

```
withdraw 2000 -> false, balance 1500
withdraw 200  -> true, balance 1300
```

## What the guards are doing

Each method assumes the invariant on entry — the balance is non-negative — and
must leave it true.

`deposit` adds a positive amount to a non-negative balance, so the result is
non-negative. Preserved.

`withdraw` refuses when the amount exceeds the balance, so the subtraction cannot
go below zero. Preserved.

Neither method could be written without knowing the invariant, and neither is
correct without its guard. Delete the `amount > cents` check and the class no
longer means what it says.

You have met these two obligations twice already, in loops and then in data
structures, and here they are a third time wearing a class: establishment is the
constructor's job, preservation is every
method's.

## Two ways to refuse

Go back and look at the two methods again. `deposit` throws an exception when it
refuses. `withdraw` returns `false`.

That inconsistency is not an oversight, and working out why is worth more than the
rule I am about to give you.

A negative deposit is a **programming error** — no sensible caller ever wants one,
and it means something upstream is wrong. Throwing is right: it fails fast and
names the mistake.

An overdraft is an **expected outcome**. A caller may legitimately try to withdraw
more than is available, and wants to know it did not work. That is not an error;
it is an answer, so it is returned.

The rule that follows: **throw for what should never happen, return for what might
reasonably happen.** Chapter 28 develops this properly. Getting it wrong in either
direction is unpleasant — exceptions used for ordinary outcomes make normal code
look like error handling, and error codes used for real bugs get ignored.

## Accessors

```java
public long balance() { return cents; }
public String owner()  { return owner; }
```

A method that reports state without changing it. Chapter 14 called this a
**query**, and the command–query rule says it should do nothing else.

Java convention names these `getBalance` and `getOwner`. I have used the shorter
form here, which is increasingly common and which records use. Either is fine;
consistency within a project is what matters.

The important question is not the name but whether the accessor should exist at
all.

## The getter-and-setter habit

Many people are taught to write, for every field, a getter and a setter:

```java
public long getCents()          { return cents; }
public void setCents(long c)    { cents = c; }
```

Look hard at that pair, because it is taught everywhere and it undoes this entire
chapter.

`setCents` accepts any value at all, negative ones included. So the invariant is
gone. You made the field private, and then you supplied a public method that does
precisely what direct field access would have done — with an extra step.

The class now has all the ceremony of encapsulation and none of the protection,
which is the worst of both: the cost is paid and the benefit is not received.

The useful test: **does this accessor correspond to something a user of the class
actually wants to do?** `balance()` does — asking an account its balance is a
sensible operation. `setCents(-500)` does not correspond to anything; what a
caller wants is `deposit` or `withdraw`, which are operations in the domain and
which can enforce the rules.

So the guidance is:

**Write a getter when callers legitimately need to read that value.** Often they
do.

**Write a setter only when there is a real operation it represents**, and give it
the operation's name and its checks. `setBalance` is almost never right;
`deposit` is.

**Expose behavior, not fields.** A class is a set of things you can *do*, not a
bag of values with lids on.

## Objects that answer, rather than being interrogated

The habit that follows, and it takes practice.

```java
// asking the object for its state, then deciding elsewhere
if (account.balance() >= price) {
    account.withdraw(price);
}
```

```java
// asking the object to do the thing
if (account.withdraw(price)) {
    // succeeded
}
```

Read both versions and ask where the *rule* lives in each.

In the second, it lives inside `Account`. In the first, it has leaked out to the
call site — and to every other call site that does the same check.

Now imagine the bank decides to permit a small overdraft. The second version needs
one edit, in one file. The first version needs you to find every place anybody
compared a balance to a price, which is a search you cannot do reliably, because
nothing marks those lines as related.

This is sometimes stated as "tell, don't ask", and it is the practical form of
Chapter 16's argument. The point of a boundary is that decisions about the data
happen inside it.

Next: the keyword that makes any of this enforceable.
