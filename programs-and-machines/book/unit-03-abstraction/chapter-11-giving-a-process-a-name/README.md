# Giving a Process a Name

You have been calling methods since Chapter 5. `System.out.println` is one.
`Integer.toBinaryString` is one. This chapter is about writing your own, and
about the judgment involved in deciding what they should be.

The mechanics take about two pages. The judgment takes the rest of the unit and,
honestly, a career.

## The mechanics, in brief

```java
static int square(int n) {
    return n * n;
}
```

A name, a list of inputs, a type of output, and a body. Called as `square(7)`,
which evaluates to 49.

That is nearly everything mechanical about methods, and if you have programmed
before it is familiar. The rest of this chapter is about the parts that are not
mechanical.

## The part that is not mechanical

Here is the question this chapter is really about: **what should be a method?**

Any program can be written as one enormous `main`. It will work. Any program can
also be written as two hundred three-line methods, and it will work, and it will
be worse — because now understanding anything requires following a chain of names
through twenty files.

Between those extremes there is a set of good decompositions, and finding one is
a design activity with real principles. This chapter introduces them and Chapter
14 works them properly.

The principles are worth previewing, because they explain what the chapter is
selecting for:

**A method should do one thing**, and its name should say what.

**A method should be describable in a sentence** that does not contain "and".

**A method should keep its promise for every input it accepts**, or else state
clearly which inputs it accepts.

That third one is the contract, and the second half of the chapter is about it.

## What is here

**The Method** covers why naming a process is valuable, how parameters carry
information in, how return values carry it out, and the debts from Chapter 5 —
`static`, `void`, and the shape of `main` — which we can now start to settle.

**The Contract** is about the promise. Preconditions, postconditions, and what to
do when a caller violates them. This is the material that makes the difference
between a method you can rely on and a method you have to remember things about.
