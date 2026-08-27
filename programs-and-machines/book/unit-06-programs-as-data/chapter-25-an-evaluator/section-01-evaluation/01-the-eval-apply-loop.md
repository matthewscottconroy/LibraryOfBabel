# The eval/apply Loop

Every interpreter ever written has the same skeleton. Two functions:

> **`eval`** takes an expression and an environment, and produces a value.
>
> **`apply`** takes a procedure and a list of argument values, and produces a
> value.

And they call each other. `eval`, meeting a call, evaluates the arguments and
hands them to `apply`; `apply`, having bound the parameters, calls `eval` on the
procedure's body.

That mutual recursion is the whole architecture. Everything else is detail.

## Why two functions

Because there are two kinds of thing to interpret, and they are genuinely
different.

An **expression** is syntax — a piece of the tree — and evaluating it means
looking at its shape and deciding what to do. `eval` is a switch over the node
types, which is exactly the tree walk of Section 24.2.2.

A **procedure** is not syntax. It is a *value* the interpreter constructed: a
parameter list and a body, stored somewhere. Applying it means setting up an
environment and then evaluating something. `apply` does no case analysis at all.

Keeping them apart matters because the two halves scale differently. Adding a new
kind of expression — a conditional, a comparison — adds a case to `eval` and
touches nothing else. Adding a new *calling* mechanism — recursion, closures,
tail calls — changes `apply` and touches no expression case.

The design is a separation of concerns, and it is the reason an interpreter can be
extended repeatedly without becoming tangled. Section 25.2 extends it three times
and you can watch which half each change lands in.

## In signatures

```java
int eval(Expr e, Env env)
int apply(Procedure p, List<Integer> args)
```

`eval` needs the environment because an expression can mention names, and a name
means nothing without one. `apply` does not take an environment — it *makes* one,
which is the single most important fact about how procedure calls work and the
subject of the next lesson.

The return type is `int` because our language has one type of value. That is a
simplification and a real one; a language with several value types returns some
`Value` and every operation checks. Section 25.2.3 accounts for it.

## Where it came from

John McCarthy, 1960, *Recursive Functions of Symbolic Expressions and Their
Computation by Machine, Part I*. The paper defines Lisp, and about half way
through it defines a function called `eval` that takes a Lisp expression —
represented as a Lisp list — and computes its value.

McCarthy's intent was theoretical. He wanted a small, precise definition of the
language, in the language, to show that it was well defined: a **metacircular
evaluator**. His student Steve Russell then observed that this definition could
be *implemented* as it stood, hand-compiled it into machine code, and Lisp had an
interpreter. McCarthy reportedly did not expect this to work.

The idea that survived is worth stating on its own. **A program is data.** Lisp's
expressions are lists, lists are Lisp's data structure, and so a Lisp program can
be built, taken apart, and evaluated by another Lisp program with no parsing
whatsoever. Our language is not self-representing — the tree is a Java data
structure, not a value in our language — but the tree walk is the same walk.

That is Chapter 16's promise coming due, quoted in full because it is the unit's
thesis:

> a data structure becomes a program purely because our evaluator agrees to treat
> it as one.

Nothing about a `Bin('+', Num(2), Num(3))` makes it a program. It is a record with
three fields. It becomes an addition because `eval` has a case for it, and it
would become something else if `eval` said something else. The meaning is in the
evaluator, not in the tree.

## Two levels

The confusion this material reliably produces, named early so you can watch for
it.

There are two programs. The **interpreter** is Java: it has its own variables, its
own stack, its own types. The **interpreted program** is in our language: it has
*its* variables, which are entries in a `HashMap` the interpreter allocated.

When our program says `x = 5`, no Java variable called `x` comes into existence.
A `String` `"x"` and an `Integer` `5` go into a map. Our program's variable is the
interpreter's data.

The same doubling applies everywhere:

| in the interpreted program | in the interpreter |
|---|---|
| a variable | an entry in a `HashMap` |
| a scope | an `Env` object |
| a procedure | a `Procedure` record |
| a call | a Java call to `apply` |
| the call stack | the JVM's call stack |
| an error | a Java exception |

Read that table again after Section 25.2.3. Every row is a decision that could
have been made differently, and some of them are the difference between one
language and another.

## The shape of what follows

```
eval(expression, environment)
    number       -> its value
    variable     -> look it up in the environment
    operation    -> eval both sides, combine
    conditional  -> eval the test, then eval one branch
    call         -> eval the arguments, then apply

apply(procedure, arguments)
    make a new environment
    bind the parameters to the arguments
    eval the body in that environment
```

Eleven lines of pseudocode, and it is a complete description of the interpreter.
The Java in the rest of this chapter is that outline with types on it.

Next: the environment, which is the piece doing the most work.
