# Boolean Algebra

Here is a condition. It is not a made-up one; conditions like this are everywhere.

```java
if (!(age >= 18 && hasLicence)) {
    refuse();
}
```

Work out when it fires. Not roughly — exactly. Who gets refused?

You can do it. It takes a few seconds of holding a negation over a conjunction in
your head, and by the end you are fairly confident and not completely certain.
Somewhere in a codebase near you there is a condition three times that size, and
somebody wrote it at the end of a long day.

Boole's contribution was to notice that logic obeys **algebraic laws** — that a
logical expression can be transformed the way an arithmetic one can, with a
guarantee that the meaning survives the transformation. Which means the tangle
above is not something you have to be clever about. It is something you can
*calculate* your way out of.

## The laws

Writing AND as `&&`, OR as `||`, NOT as `!`:

**Commutative** — order does not matter.
```
A && B   ==   B && A
A || B   ==   B || A
```

**Associative** — grouping does not matter.
```
(A && B) && C   ==   A && (B && C)
```

**Distributive** — like multiplication over addition, except for one bonus:
unlike arithmetic, it works in both directions.
```
A && (B || C)   ==   (A && B) || (A && C)
A || (B && C)   ==   (A || B) && (A || C)
```

**Identity and annihilation.**
```
A && true  == A          A && false == false
A || false == A          A || true  == true
```

**Double negation.**
```
!!A  ==  A
```

**Idempotence** — saying it twice adds nothing.
```
A && A  ==  A
A || A  ==  A
```

**Complement.**
```
A && !A  ==  false
A || !A  ==  true
```

Every one of these can be proved by writing out the truth table for both sides and
checking that the columns match. Pick one and do it by hand before moving on — it
takes a minute, and the thing to appreciate is not the particular law but the
**finiteness**. There are only so many rows. When you have checked them all there
is nothing left to check, and you are not confident, you are certain. Very little
else in programming offers you that.

## De Morgan's laws

Two more, named for Augustus De Morgan, a contemporary of Boole, and by some
distance the most useful in daily work:

```
!(A && B)   ==   !A || !B
!(A || B)   ==   !A && !B
```

In words: **negating an AND gives you an OR of the negations, and the other way
round.** The negation moves inward, and the operator flips as it passes.

Now go back and apply that to the condition we started with:

```java
if (age < 18 || !hasLicence) {
    refuse();
}
```

"Too young, or no license."

That is what a human being would have said in the first place, and now the code
says it too. Nothing about the behavior changed. What changed is that a reader can
tell at a glance what it means, and could not before.

One detail in that rewrite is worth stopping on, because it is where people
actually get hurt. The inner comparison flipped as well: `!(age >= 18)` became
`age < 18`. Negating a comparison inverts it — and the opposite of `>=` is `<`,
**not** `<=`. Get that wrong and you have an off-by-one error that fires on exactly
one input value in the entire range, which is the hardest kind to find and the
easiest kind to ship.

## Three habits worth having

**Push negations inward.** A `!` sitting on a compound expression is harder to read
than negations sitting on the pieces. De Morgan lets you move it and guarantees you
have not changed anything.

**Name intermediate conditions.** The algebra tells you a rewrite is *safe*; a name
tells the reader what it *means*, and those are different services.

```java
// hard
if (!(user != null && user.isActive() && !user.isSuspended())) { ... }

// better
boolean canProceed = user != null && user.isActive() && !user.isSuspended();
if (!canProceed) { ... }
```

Notice that nothing was logically reduced there. Not one operator was removed. It
became readable, which was the actual problem all along.

**Be suspicious of the English word "and".** A specification says: "reject users
under 18 and users without a license." That "and" is describing two rules. The
condition that implements it uses an OR.

This is a place where requirements quietly turn into bugs, and it is where this
chapter's algebra earns its keep. When a specification and a condition seem to
disagree about "and", a truth table on a piece of paper is usually the fastest
route to being sure.

## Making things smaller

The laws also let you shrink an expression, with each step justified by name:

```
(A && B) || (A && !B)
  = A && (B || !B)      distributive
  = A && true           complement
  = A                   identity
```

So `(A && B) || (A && !B)` was `A` the whole time.

Find that condition in real code and you could delete two thirds of it — and here
is the part that matters — you would **know** the behavior was unchanged. Not
believe. Not have tested. Know, by the same kind of argument that says $2x - x$ is
$x$.

This is the identical activity to simplifying an algebraic expression, and it pays
the same dividend: fewer terms means fewer places to be wrong.

In hardware the benefit is direct, since every operator you remove is a gate you do
not have to build, and there are systematic methods for finding minimal forms —
Karnaugh maps, the Quine–McCluskey algorithm. In software the benefit is
comprehension instead of cost, which over the life of a program is worth at least
as much.

Next: what Java actually gives you to write conditions with.
