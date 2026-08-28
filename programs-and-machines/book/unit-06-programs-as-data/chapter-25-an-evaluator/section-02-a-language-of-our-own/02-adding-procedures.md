# Adding Procedures

Two additions, and after them the language can compute anything computable.

That is a large claim for a small amount of code, and it is exact rather than
rhetorical. The second addition is the one to watch: it is a single line of `eval`,
it looks like the least significant thing in the chapter, and without it every
recursive definition runs forever.

## Defining one

```java
record Procedure(List<String> params, Expr body) { }
```

A parameter list and a body. That is a procedure, entirely.

Notice what is absent: there is no name in there. The name is how you *found* the
procedure, not part of what it is — the same distinction as between a variable and
the value sitting in it.

`DefFun` stores one:

```java
case DefFun f -> procs.put(f.name(), new Procedure(f.params(), f.body()));
```

Note what does *not* happen: the body is not evaluated. It is stored as a tree, to
be walked later, possibly many times, possibly never. A procedure definition is
the act of putting a piece of unevaluated syntax somewhere retrievable.

Stay with that, because it is the sharpest form this unit's claim ever takes.

`x * x`, sitting inside a `Procedure`, is a data structure. Three fields and a
pointer. It is no more a computation than a recipe is a dinner — and it becomes one
at the exact moment `apply` hands it to `eval`, and not one instant before.

## Calling one

The tree needs a fourth shape, and Chapter 24 told you this one was coming:

```java
sealed interface Expr permits Num, Var, Bin, Call, If { }

record Call(String name, List<Expr> args) implements Expr { }
```

The moment I added it, every `switch` over `Expr` in the whole program stopped
compiling until I had handled it — which is the exhaustiveness check of Chapter 22
earning its keep on a real change rather than a toy one.

The parser distinguishes a call from a variable by looking one token ahead:

```java
case NAME: {
    if (match(LPAREN)) {
        List<Expr> args = new ArrayList<>();
        if (!check(RPAREN)) {
            do { args.add(expression()); } while (match(COMMA));
        }
        expect(RPAREN);
        return new Call(tok.text(), args);
    }
    return new Var(tok.text());
}
```

`square` is a variable. `square(` is a call. The whole distinction is one
character, and the parser resolves it by peeking exactly one token ahead — which
is the LL(1) property of the last chapter, being used rather than described.

## eval and apply meet

```java
case Call c -> {
    Procedure p = procs.get(c.name());
    if (p == null) throw new EvalError("undefined procedure: " + c.name());
    if (p.params().size() != c.args().size())
        throw new EvalError(c.name() + " expects " + p.params().size()
            + " arguments but got " + c.args().size());
    List<Integer> vals = new ArrayList<>();
    for (Expr a : c.args()) vals.add(eval(a, env));
    yield apply(p, vals);
}
```

```java
int apply(Procedure p, List<Integer> args) {
    Env local = new Env(global);
    for (int i = 0; i < p.params().size(); i++)
        local.define(p.params().get(i), args.get(i));
    return eval(p.body(), local);
}
```

And there is the loop the whole chapter was named for. `eval` evaluates the
arguments **where the caller stands**, `apply` binds them **somewhere new**, and
`eval` runs the body there.

Now look at three details in those ten lines. Not one of them is forced. Each is a
decision somebody made about what a language should be, and you are making all
three right now by writing this code rather than different code.

**Arguments are evaluated before the call.** Call by value, and it means
`square(2 + 3)` computes 5 and passes 5. The alternative — passing the unevaluated
expression and evaluating it if the body uses it — is call by name, and it changes
what `f(expensive())` costs when `f` ignores its argument.

**The values are `int`, so nothing is shared.** Our language has no references,
which means aliasing cannot occur in it — the entire difficulty of Chapter 20
cannot arise here at all. Enjoy that while it lasts. Add one mutable value type and
all of Chapter 20 walks in at once, uninvited.

**Arity is checked here, at run time.** Verified:

```
def g(a) = a; print g(1, 2);   ==>  g expects 1 arguments but got 2
```

Java catches that before your program ever starts, because it knows every
signature at compile time. We could too — our procedure table is complete the
moment parsing finishes, and nothing stops us walking the tree once and checking.

Notice that this is the *second* time the same choice has come up, after the
undefined-variable check. That is not a coincidence. These decisions arrive as a
set: a language tends to be early-checking or late-checking throughout, because
the machinery you build for one gives you the other for free.

Verified:

```
def square(x) = x * x;
def hyp2(a, b) = square(a) + square(b);
print square(7);      ->  49
print hyp2(3, 4);     ->  25
```

`hyp2` calls `square` twice. Nothing special was needed for that: `eval` met a
`Call`, called `apply`, which called `eval` on `square`'s body, which met another
`Call`. The mutual recursion nests as deeply as the program does.

## Conditionals

Procedures alone are not enough. A procedure that always evaluates its whole body
can never stop recursing, so the language needs a way to *not* evaluate something.

```java
record If(Expr test, Expr then, Expr otherwise) implements Expr { }
```

An expression, not a statement — it has a value, like Java's `?:`.

```java
case If f -> eval(f.test(), env) != 0
        ? eval(f.then(), env) : eval(f.otherwise(), env);
```

One line, and it is the most important line in the chapter.

**Only one branch is evaluated.** That is what makes `if` different from every
other node type. `Bin` evaluates both children always; `If` evaluates the test,
then exactly one of the other two.

If it evaluated both — say by computing them and selecting — then
`if n < 2 then 1 else n * fact(n - 1)` would call `fact` at every level including
the base case, and the recursion would never terminate. Conditional evaluation is
not an optimization. It is the thing that makes recursion possible.

This is also why `&&` and `||` short-circuit in Java, as Chapter 8 said, and why
they cannot be ordinary methods: a method's arguments are evaluated before the
call, and short-circuiting requires not evaluating one.

Our language has no booleans, so the test is an `int` and zero is false. That is C's
convention, and Chapter 8 explained why Java rejected it — `if (x = 0)` compiles
under it. We have inherited the bug, deliberately, so it can be seen.

## Recursion

Everything is now in place, and nothing further was added:

```
def fact(n) = if n < 2 then 1 else n * fact(n - 1);
def fib(n)  = if n < 2 then n else fib(n - 1) + fib(n - 2);
def gcd(a, b) = if b < 1 then a else gcd(b, a - b * (a / b));
print fact(10);
print fib(15);
print gcd(1071, 462);
```

Verified:

```
3628800
610
21
```

Factorial, Fibonacci, and Euclid's algorithm, running in a language that did not
exist an hour ago. `gcd` is doing modulo by hand — `a - b * (a / b)` — because the
language has no `%`, and that it works at all is a small demonstration that the
arithmetic is real.

Recursion required nothing new because it was already there. `fact`'s body
contains a `Call` to `fact`; `eval` looks it up in `procs`, which by then contains
it; `apply` makes a fresh environment with a fresh `n`. The nesting is handled by
the JVM's own stack, which is holding one `eval` frame per level of our recursion.

Which means:

```
def loop(n) = loop(n);
print loop(1);
```

Verified: `StackOverflowError`.

Our language's infinite recursion is Java's stack overflow. Chapter 12's error,
arriving from a program written in a language Chapter 12 had never heard of, and
it is a fair summary of what an interpreter is — our language's resources are the
implementation's resources, wearing our names.

## What we have, formally

With conditionals, recursion, and arithmetic, this language is **Turing complete**.
Chapter 6's claim — that a very small set of operations suffices for anything
computable — is now something you have built rather than been told.

The language has no loops. It does not need them: `fact` iterates by recursing,
and any loop can be rewritten as a recursive procedure, which is Chapter 13's
equivalence. It has no data structures, no strings, no input. It is still enough,
in the precise sense that any function a Turing machine can compute can be written
in it, though a great many of them would be unbearable to write.

That gap — between *possible* and *bearable* — is what the rest of a language is
for, and it is the subject of the next lesson.
