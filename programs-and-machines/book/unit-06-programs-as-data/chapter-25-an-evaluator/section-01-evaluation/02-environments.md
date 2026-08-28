# Environments

`x` is three pixels wide and carries no value.

Something, somewhere, has to hold the association between that name and a number,
and every question you have ever had about scope — why a parameter hides a field,
why a local vanishes when a method returns, why two calls to the same method do not
tread on each other — is a question about how that something is arranged.

It is twelve lines.

A name means nothing by itself. `x` is three pixels wide and carries no value.
Something has to hold the association, and that something is an **environment**.

```java
static final class Env {
    private final Map<String, Integer> values = new HashMap<>();
    private final Env parent;

    Env(Env parent) { this.parent = parent; }

    void define(String name, int value) { values.put(name, value); }

    int lookup(String name) {
        Integer v = values.get(name);
        if (v != null) return v;
        if (parent != null) return parent.lookup(name);
        throw new EvalError("undefined variable: " + name);
    }
}
```

Twelve lines. They implement scope, shadowing, nesting, and undefined-variable
errors, and it is worth going through what each piece is doing because each
corresponds to something you have taken for granted since Chapter 7.

## The map

A `HashMap<String, Integer>` from names to values. This is what a variable *is*
in our language — not a memory location, not a box, but a key in a map the
interpreter owns.

And there goes the box metaphor from Chapter 7 — not wrong, but revealed as an
implementation choice rather than a fact about machines. Our language could have used an array with names resolved to
indices at parse time, which is faster and is roughly what a compiled language
does. The map is chosen because it is obvious, and Section 25.2.3 says what it
costs.

## The parent

The `parent` field is the entire idea.

An environment does not have to contain everything. If a name is not here, ask
the enclosing environment; if that one does not have it either, ask its parent.
The chain ends at the global environment, whose parent is `null`, and if the name
is not there it does not exist.

That chain is a **scope chain**, and following it is exactly what Java does when
it resolves a name inside a method inside a class. You have been relying on this
mechanism since Chapter 7 and it is nine lines long.

## Lookup, in order

`lookup` searches locally first, then upward. That order is not arbitrary — it is
**shadowing**, and it is the whole of shadowing.

```
x = 100;
def f(x) = x + 1;
print f(1);
print x;
```

Verified output:

```
2
100
```

Inside `f`, the name `x` finds the parameter, because `apply` put it in the local
environment and `lookup` looks there first. The global `x` is not modified, not
consulted, and not damaged — it is merely not the nearest `x`.

Reverse the two clauses in `lookup` — parent first, then local — and parameters
would stop working. That is how small the mechanism is.

## The error

```java
throw new EvalError("undefined variable: " + name);
```

Verified:

```
print y;   ==>  undefined variable: y
```

Here is the moment the last chapter promised you. A grammar cannot express "every
name must be defined" — it is not a property of the string's shape — so the parser accepted
`y` happily and the check had to happen somewhere else.

Here it happens at run time, when the lookup fails. Java checks the same thing at
compile time, by walking the tree before running it and consulting a table of
declarations. That difference — when the check happens — is the difference between
a statically checked language and a dynamically checked one, and it is a decision
made in this method.

We could make ours static. Walk the tree once after parsing, tracking which names
are in scope, and report any `Var` that is not. It is about forty lines, it is
Exercise 25.9, and doing it makes the compile-time/run-time distinction concrete
in a way no explanation does.

## Environments and the call stack

`apply` creates one:

```java
int apply(Procedure p, List<Integer> args) {
    Env local = new Env(global);
    for (int i = 0; i < p.params().size(); i++)
        local.define(p.params().get(i), args.get(i));
    return eval(p.body(), local);
}
```

Three lines with a great deal in them.

**A new `Env` per call.** Not per procedure — per *call*. Two calls to the same
procedure get two environments, which is why a recursive procedure's parameter has
a different value at each depth. `fact(5)` calling `fact(4)` has two live
environments, each with its own `n`.

That is Chapter 12's stack frame, and the correspondence is exact. A frame holds a
call's local variables; an `Env` holds a call's local variables. The JVM allocates
frames on a stack; we allocate `Env` objects on the heap and let the JVM's own
stack keep them alive through the chain of `eval` calls.

**The parameters are bound by position.** `params.get(i)` to `args.get(i)`. That
is what positional arguments *are*, and it is why Chapter 11 warned about long
parameter lists: nothing in this loop can tell you that the caller swapped two.

**The parent is `global`, not the caller's environment.** This is a real language
design decision and it is easy to miss.

## Lexical and dynamic scope

Choosing what `apply` passes as the parent decides how names resolve, and there
are two answers.

**Lexical scope** — the parent is the environment where the procedure was
*defined*. Names in a procedure body mean what they meant where the body was
written. Our language does this: procedures are defined at the top level, so the
parent is `global`.

**Dynamic scope** — the parent would be the environment of whoever *called*. Names
would mean whatever they happen to mean at the call site.

Consider:

```
x = 1;
def f() = x;
def g() = 2;
```

Under lexical scope, `f()` is 1 always. Under dynamic scope, if some caller had a
local `x`, `f()` would see that one instead.

Every mainstream language is lexically scoped, and the reason is that lexical
scope lets you read a procedure and know what its names mean, from the text
alone. Under dynamic scope you cannot: the meaning of the body depends on every
call site, including ones written later by other people.

Early Lisp was dynamically scoped — partly by accident of implementation, since
the caller's environment is already at hand and using it is less work. Scheme
introduced lexical scope to Lisp in 1975, and the fix is one argument in `apply`.

Nested procedures make the choice harder. If a procedure is defined *inside*
another, lexical scope requires the procedure to remember the environment it was
created in, which means storing an `Env` inside the `Procedure` record. That
stored pair — code plus its defining environment — is a **closure**, and Chapter
26 shows it in Java, where the same mechanism is what lets a lambda use a variable
from the method that created it.

Our `Procedure` has no `Env` field, so our language has no closures. That is the
single largest thing it is missing, and Exercise 25.11 adds it in about six lines.

Next: making arithmetic actually happen.
