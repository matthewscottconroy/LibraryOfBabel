# Adding Names and Definitions

A calculator computes and forgets. A program remembers — which is what lets its
second line depend on its first, and is most of the difference between the two
things.

Remembering requires a construct that has an *effect* rather than a value, and Java
draws that line too, in a place worth noticing before you draw it yourself.

So far the language evaluates one expression. A language needs to *remember*
things, which means a second category of construct.

## Expressions and statements

An **expression** has a value. `2 + 3` is 5.

A **statement** has an effect. `width = 8` does not evaluate to anything; it
changes the environment.

Some languages erase this distinction — in Scheme everything is an expression, and
assignment returns some unspecified value — and there are good arguments for
doing so. Java keeps it, mostly: `if` is a statement, `?:` is an expression, and
Java 14 added `switch` in both forms because the distinction turned out to be
inconvenient.

We keep it, because it makes the tree types clearer:

```java
sealed interface Stmt permits Define, DefFun, Print { }

record Define(String name, Expr value)                     implements Stmt { }
record DefFun(String name, List<String> params, Expr body) implements Stmt { }
record Print(Expr value)                                   implements Stmt { }
```

A second sealed interface, sitting beside `Expr`. And look at the relationship
between them: a `Define` *contains* an `Expr`. The two are not parallel, they are
nested, which is exactly right, because
that a statement can contain an expression and not the reverse.

## The grammar grows

```
program   := statement (';' statement)* ';'
statement := 'print' expression
           | 'def' NAME '(' params ')' '=' expression
           | NAME '=' expression
```

And the parser grows to match, one method per rule as before:

```java
List<Stmt> program() {
    List<Stmt> out = new ArrayList<>();
    while (!check(END)) { out.add(statement()); expect(SEMI); }
    return out;
}
```

The semicolon is a **terminator** here — every statement ends with one, including
the last. Java does the same. Some languages make it a *separator*, required
between statements but not after the final one, which is a surprisingly annoying
distinction to implement and the source of trailing-comma arguments in every
configuration format.

## Keywords that are not keywords

```java
if (check(NAME) && peek().text().equals("print")) { ... }
```

Our tokenizer has no notion of a keyword. `print` arrives as `NAME("print")` and
the parser recognizes it by its text.

That means `print` is not reserved: a program could name a variable `print`, and
the parser — checking for the keyword first — would reject the assignment. This is
a small bug, and it is the kind that a language design produces by omission rather
than by mistake.

Real tokenizers keep a set of reserved words and emit a distinct token kind for
each, so that the parser tests kinds rather than strings and the ambiguity cannot
arise. Exercise 25.6 does this, and it takes four lines.

Some languages deliberately have no reserved words at all — PL/I famously allowed
`IF IF = THEN THEN THEN = ELSE` — and every such language regrets it.

## Executing a statement

```java
void exec(Stmt s, Env env) {
    switch (s) {
        case Define d -> env.define(d.name(), eval(d.value(), env));
        case DefFun f -> procs.put(f.name(), new Procedure(f.params(), f.body()));
        case Print p  -> System.out.println(eval(p.value(), env));
    }
}
```

Three cases, three lines, and the first is the interesting one.

**`Define` evaluates before it binds.** `area = width * height` computes the value
first, then stores it. So the right-hand side sees the environment as it was — and
`x = x + 1` works, reading the old `x` and storing a new one.

This is **eager** evaluation, and it is what nearly every language does. The
alternative is to store the *expression* and evaluate it when the name is used,
which is **lazy** evaluation, and it means `x = expensive()` costs nothing until
someone reads `x`. Haskell works this way. The consequences are large — it changes
what infinite data structures mean and it makes performance hard to predict — and
it is one line's difference here.

## Running it

```
width = 8;
height = 5;
area = width * height;
print area;
print area / 2;
```

Verified:

```
40
20
```

Three definitions and two prints. `area` was computed from two names that were
themselves defined by earlier statements, which means the environment persisted
across statements — the `Interp` holds one global `Env` and every statement runs
in it.

That persistence is what makes a sequence of statements a *program* rather than a
list of unrelated calculations, and it is the entire reason statements exist.

## Redefinition

`values.put(name, value)` overwrites. So `x = 1; x = 2;` leaves `x` as 2, and
there is no distinction between declaring a variable and assigning to one.

Java distinguishes them: `int x = 1;` declares, `x = 2;` assigns, and `int x = 2;`
twice is an error. The check catches typos — a misspelled assignment becomes a new
variable in our language and a compile error in Java — and it is why Chapter 7
insisted on declarations.

Our language could add it: keep a set of declared names and require `var` before
the first assignment. That is Exercise 25.7, and the exercise is really about
noticing that Java's rule is a choice made for the programmer's benefit rather
than a technical necessity.

Next: procedures, and the language becomes capable of anything.
