# Objects as Little Machines

Think about a vending machine for a moment.

At any instant it is in some condition: how much money you have put in, what is in
stock, whether it is waiting for a selection. That condition is not visible from
outside except through what the machine lets you do — insert a coin, press a
button, take the change. And each of those actions moves it from one condition to
the next.

You cannot reach in and set the credit to five pounds. Not because it would be
philosophically wrong, but because the panel does not offer it, and the panel is
all you have.

That is this chapter, and you have been carrying the idea since Chapter 6.

Chapter 6 described a machine as state plus transitions that change it. An
**object** is exactly that, at a scale you choose.

```java
Account a = new Account("Ada", 1000);
a.deposit(500);
a.withdraw(200);
System.out.println(a.balance());     // 1300
```

`a` has state — an owner and a balance — and a fixed set of transitions:
`deposit`, `withdraw`, `balance`. Nothing else can touch it. That is a machine in
Chapter 6's sense, and the only novelty is that you get to define what its states
and transitions are.

## Class and object

Two words, and confusing them causes trouble.

A **class** is a description. It says what fields an object will have and what
methods it will support. It is written once, in a file, and it is not a thing that
exists while the program runs, any more than a blueprint is a house.

An **object** — or **instance** — is a particular thing built to that description,
living on the heap, with its own copy of the fields.

```java
Account ada   = new Account("Ada", 1000);
Account grace = new Account("Grace", 50);
```

One class, two objects. Each has its own `owner` and its own `cents`. Calling
`ada.deposit(500)` changes one of them and leaves the other alone, because the
fields belong to the object.

`new` is what makes one. It asks the heap for space, initializes the fields, runs
the constructor, and returns a reference — which, per Chapter 12, is what the
variable actually holds.

## Why bundle at all

The honest question: what does putting the balance and the operations in one place
buy that two loose variables and some methods do not?

**The invariant gets a home.** "The balance is never negative" is a claim about
one field, and now there is exactly one place that field can be changed from. You
can read the class and know.

**The state cannot be separated from its meaning.** With loose variables you can
pass `cents` to a method that has no idea it is money in pence. With an `Account`
you pass the thing, and the thing carries its own operations.

**Related things stay related.** If a balance needs a currency later, one class
changes. With parallel arrays of names, balances, and currencies, every piece of
code that walks them changes.

**You can have many.** This is the mundane one and it matters most in practice.
Two loose variables describe one account. A class describes a kind of thing, and
you can make ten thousand.

## The shape of one

```java
public class Account {

    private final String owner;      // fields: the state
    private long cents;

    public Account(String owner, long cents) {      // constructor
        this.owner = owner;
        this.cents = cents;
    }

    public void deposit(long amount) {              // methods: the transitions
        cents += amount;
    }

    public long balance() {
        return cents;
    }
}
```

Four parts, and every one of them is discussed in this chapter: the class header,
the fields, the constructor, and the methods.

Look at how the methods refer to `cents` — no prefix, no ceremony, just the name.
Inside an instance method the fields of *this particular object* are already in
scope, and that is the thing that makes
`deposit` mean "deposit into the account I was called on".

## this

When there is ambiguity — a parameter with the same name as a field — `this`
means the current object:

```java
public Account(String owner, long cents) {
    this.owner = owner;      // field = parameter
    this.cents = cents;
}
```

Without `this`, `owner = owner` would assign the parameter to itself and leave the
field null, which compiles cleanly and is a real bug. Some people avoid it by
naming parameters differently; the `this.x = x` form is the common convention and
worth being comfortable with.

`this` is available in any instance method and refers to the object the method was
called on. `a.deposit(500)` runs `deposit` with `this` bound to `a`.

## Where the object lives

Chapter 12's picture, now with a name:

```
a: ┌────────┐          ┌─────────────────────┐
   │ ref ───┼─────────▶│ owner: ref ──▶ "Ada"│
   └────────┘          │ cents: 1500         │
   stack               └─────────────────────┘
                        heap
```

The variable is a reference. The object is on the heap and outlives the method
that created it, if a reference escapes. Two variables can refer to one object,
and then a change through either is visible through both — which is aliasing, and
Chapter 20's subject.

Everything Chapter 12 said about references now applies constantly, because from
here almost everything is an object.

Next: how an object comes into existence already correct.
