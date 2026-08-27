# Important Concepts

**eval and apply** — two mutually recursive functions that are, between them, an
interpreter. `eval` takes an expression and an environment and produces a value;
`apply` takes a procedure and argument values and produces a value.

**Why two** — an expression is syntax and needs case analysis; a procedure is a
value and needs none. Adding an expression form touches only `eval`; adding a
calling mechanism touches only `apply`.

**The metacircular evaluator** — McCarthy's 1960 `eval`, defining Lisp in Lisp as
a specification. Steve Russell hand-compiled it and Lisp had an interpreter,
which McCarthy had not expected.

**A program is data** — nothing makes `Bin('+', Num(2), Num(3))` a program except
that `eval` has a case for it. The meaning lives in the evaluator, not the tree.

**Two levels** — the interpreter is Java; the interpreted program is in our
language. Its variables are entries in a map the interpreter allocated. Confusing
the levels is the characteristic error of this material.

**Environment** — a map from names to values plus a pointer to a parent. Twelve
lines implementing scope, shadowing, nesting, and undefined-variable errors.

**The scope chain** — look locally, then ask the parent, until the global
environment's `null` parent ends the search. This is what Java does to resolve a
name, and it is nine lines.

**Shadowing is a search order** — local before parent. Reverse the two clauses and
parameters stop working.

**A new environment per call, not per procedure** — which is why a recursive
procedure has a different parameter value at each depth. An `Env` is Chapter 12's
stack frame.

**Lexical versus dynamic scope** — whether a procedure's parent environment is
where it was defined or where it was called. Lexical lets you read a body and know
what its names mean; every mainstream language chose it, and it is one argument in
`apply`.

**Closure** — a procedure stored together with the environment it was created in.
Our `Procedure` has no such field, which is why the language has no closures.

**Static versus dynamic checking** — our undefined-variable and wrong-arity errors
happen at run time; Java's happen at compile time. The information for both is
available after parsing, so the difference is when you choose to look.

**eval is the tree walk** — the same skeleton as `depth` from Chapter 24, with the
operator in place of `max`. An interpreter is a recursive function over a tree.

**Depth in the tree is order of evaluation** — `eval` contains nothing about
precedence, because the parser settled it by building `(+ 2 (* 3 4))`.

**Errors must not leak** — an interpreter that lets `ArithmeticException` escape
has shown its user a Java stack trace. Every failing operation needs a check that
converts the failure into the language's own terms.

**What you do not decide, you inherit** — our numbers wrap at $2^{31}$ because
Java's do. A language designer chooses arbitrary precision, or doubles, or exact
rationals, in the equivalent of that switch.

**Expression and statement** — one has a value, the other has an effect. Some
languages erase the distinction; Java mostly keeps it.

**Terminator versus separator** — whether the final statement needs its semicolon.
A small decision with a long history of arguments.

**Keywords recognized by text** — our `print` is a `NAME` the parser checks the
spelling of, so it is not reserved and cannot be used as a variable. Real
tokenizers emit a distinct kind per reserved word.

**Eager versus lazy** — `Define` evaluates the right side immediately. Storing the
expression and evaluating on use is lazy, is what Haskell does, and is one line's
difference here.

**Procedure as stored syntax** — `DefFun` does not evaluate the body; it puts an
unevaluated tree somewhere retrievable. It becomes a computation when `apply`
hands it to `eval`.

**Call by value** — arguments are evaluated before `apply` is called. Call by name
passes the expression instead, and changes what an ignored argument costs.

**Binding by position** — `params.get(i)` to `args.get(i)`. Nothing in that loop
can notice that the caller swapped two arguments.

**Conditional evaluation makes recursion possible** — `If` evaluates the test and
exactly one branch. If it evaluated both, no recursion could reach a base case.
This is also why `&&` short-circuits and why it cannot be a method.

**Turing complete** — with arithmetic, conditionals and recursion, the language can
compute anything computable, despite having no loops and no data structures.
Chapter 6's claim, built rather than asserted.

**Our stack overflow is Java's** — an infinite recursion in our language exhausts
the JVM's stack, because our recursion is implemented by its recursion.

**Tree-walking is the slowest reasonable approach** — typically 10 to 100 times
slower than compiled code. Remedies: compile to bytecode, resolve names to slot
indices at compile time, compile to machine code.

**Compiled and interpreted describe implementations, not languages** — Java is all
three at once, and C's machine code is itself interpreted by microcode.
