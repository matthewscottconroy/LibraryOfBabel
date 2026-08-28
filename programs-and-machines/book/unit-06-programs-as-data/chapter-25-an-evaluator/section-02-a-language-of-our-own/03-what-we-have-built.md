# What We Have Built

It runs. Before moving on, it is worth an honest accounting — because a list of
what a language *lacks* describes it at least as well as a list of what it has,
and because the missing pieces are where the next three chapters come from.

Roughly two hundred lines of Java. A tokenizer, a parser, a tree, an environment,
`eval`, and `apply`. It runs programs.

This lesson is the accounting: what that is, what it is not, and which of the
differences are interesting.

## The parts

| component | job | lines |
|---|---|---|
| `tokenize` | characters to tokens | ~35 |
| `Parser` | tokens to trees | ~70 |
| `Expr`, `Stmt` | the tree types | ~10 |
| `Env` | names to values | ~12 |
| `eval` | expressions to values | ~35 |
| `apply` | procedures to values | ~6 |

The two most conceptually loaded components — the environment and `apply` — are
the two smallest. That is normal, and it is a decent argument for the idea that
understanding is not proportional to code size.

## What it is missing

An honest list, because a list of absences describes a language as well as a list
of features.

**Closures.** Procedures cannot be defined inside procedures, and a `Procedure`
does not remember where it was created. Six lines fixes it: store the defining
`Env` in the record and use it as the parent in `apply`. Chapter 26 shows what
Java does with the same idea.

**Data.** No strings, no lists, no records. Every value is an `int`. Adding a
second type means `eval` returns a `Value` rather than an `int`, and every
operation must check what it got — which is the entire subject of type checking,
arriving as soon as there is more than one kind of thing.

**Mutation inside procedures.** `define` on a local environment adds to it; there
is no assignment to an existing binding. Adding one raises immediately the
question of *which* binding — local or global — and that question is Chapter 20's
aliasing in a new costume.

**Loops, input, output beyond `print`, comments, floating point, a module system.**
Each is a few lines to a few hundred, and the ordering of that list is roughly the
ordering in which a real language grows.

**Good errors.** No line numbers, no recovery, no context. Section 24.2.3 said
this is what separates a teaching parser from a usable one, and it is at least as
true of the evaluator.

## What it does have

It is worth being equally clear about this, because the list is not short.

Correct precedence and associativity. Parenthesization to any depth. Lexical
scope with shadowing that works. Recursion to the limit of the host stack.
Run-time errors that name the actual problem in the language's own terms rather
than Java's. And Turing completeness, which is not a small property to have by
accident.

## Tree-walking, and what it costs

Our interpreter walks the tree every time. `fib(25)` re-walks the same body a
quarter of a million times, re-deciding at every step that this node is a `Bin`
and that operator is `+`.

Measured, by counting calls to `apply`:

```
fib(5)  took     15 applications
fib(10) took    177 applications
fib(15) took   1973 applications
fib(20) took  21891 applications
fib(25) took 242785 applications
```

Each step of `n` by 5 multiplies the work by about eleven — the growth is
exponential, which is a property of the naive Fibonacci algorithm rather than of
our interpreter, and Chapter 32 will name it. But every one of those 242,785
applications allocates a `HashMap`, and every node visit performs a type switch
that could have been decided once.

**A tree-walking interpreter is the slowest reasonable way to run a program**,
typically 10 to 100 times slower than compiled code. The three standard remedies:

**Compile to bytecode.** Walk the tree once, emit a flat instruction sequence, and
run that in a loop. No repeated type dispatch, no pointer chasing. This is what
CPython does, and what `javac` plus the JVM does — Chapter 5's `.class` file is
exactly this.

**Resolve names at compile time.** Replace each `Var` with a slot index during a
pass over the tree, so lookup becomes an array access instead of a hash-map
search up a chain. This is one of the largest single wins available, and it is why
compiled languages do not need a scope chain at run time.

**Compile to machine code.** Either ahead of time, or at run time when a function
proves hot enough to be worth it. Chapter 21 measured the JVM doing the second.

Our interpreter does none of these, which is the right choice for two hundred
lines whose purpose is to be understood.

## The distinction that dissolves

Compiled and interpreted are usually presented as two kinds of language. Having
written one, you can see that they are two ends of a spectrum and that the middle
is crowded.

Our language: parse, then walk the tree. Purely interpreted.

Java: compile to bytecode ahead of time, interpret the bytecode, then compile the
hot parts to machine code while running. All three at once.

C: compile to machine code ahead of time — and the machine code is then
interpreted by the processor's microcode, which is a detail Chapter 6 raised and
which makes even C's position less absolute than it sounds.

**A language is not compiled or interpreted. An implementation is**, and most
serious implementations are both. There have been C interpreters and Python
compilers, and neither changed what the language was.

## What changed

The claim made at the top of this chapter was that writing an evaluator changes
how you read your own language. Concretely, here is what should now be different.

When you write `int x = 5;` you can see the map entry. When a variable resolves to
a parameter rather than a field, you can see the order of the two lookups. When
`javac` reports an undefined symbol you know it walked a tree and consulted a
table, because you wrote the version that does it at run time and could have moved
it. When a recursive method overflows the stack you know what ran out. When `&&`
short-circuits you know why it could not have been a method.

And the largest one: you know that every rule in Java is a decision. Someone chose
lexical scope, eager evaluation, static arity checking, reserved words, 32-bit
wrapping arithmetic, left-to-right evaluation order. You have now made six of those
choices yourself, mostly by writing one line rather than another, and the ones you
made by omission are the ones worth noticing.

Chapter 26 takes the idea that a procedure is a value — which our `Procedure`
record made concrete — and finds it in Java, where it has been available since
Java 8 and is the basis of a style of programming this book has not yet used.
