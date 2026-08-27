# The Syntax Tree

A token list is flat. `2 + 3 * 4` is five tokens in a row, and nothing in the row
says that `3 * 4` belongs together.

The structure is a **tree**:

```
        +
       / \
      2   *
         / \
        3   4
```

This is an **abstract syntax tree** — abstract because it keeps the structure and
discards everything that was only notation. The parentheses in `(2 + 3) * 4`
change the tree's shape and then vanish; nothing in the tree records that they
were typed. The whitespace is long gone. What remains is what the expression
*is*.

## Declaring it

```java
sealed interface Expr permits Num, Var, Bin { }

record Num(int value)                     implements Expr { }
record Var(String name)                   implements Expr { }
record Bin(char op, Expr left, Expr right) implements Expr { }
```

Four lines, and they say something precise: **an expression is exactly one of
three shapes** — a number, a variable, or a binary operation with two
sub-expressions.

This is the construct Chapter 22 closed with, and this is what it is for. The
`sealed` keyword lists the permitted implementations, so the set is closed the way
an enum's is. The records carry each shape's data. Together they are an
**algebraic data type**, and a syntax tree is the example every language with the
feature was designed for.

Look at `Bin` for a moment. Its `left` and `right` are `Expr`, so a `Bin` can
contain a `Bin` can contain a `Bin`. That is the recursion Chapter 13 described,
now in a data structure rather than a method — and it is the direct image of the
grammar rule `expression := expression '+' expression`.

The grammar is recursive, so the type is recursive. That correspondence is not a
coincidence and it is not a metaphor; the type is a transcription of the grammar.

## Walking it

Consuming a tree means asking which shape a node has and handling each:

```java
static String show(Expr e) {
    return switch (e) {
        case Num n -> String.valueOf(n.value());
        case Var v -> v.name();
        case Bin b -> "(" + b.op() + " " + show(b.left()) + " " + show(b.right()) + ")";
    };
}
```

Two things to notice, and both are payoffs from earlier chapters.

**No `default` clause.** The interface is sealed, so the compiler knows there are
exactly three cases and that all three are covered. Add a fourth shape — say
`Call` for function calls, which Chapter 25 does — and every `switch` like this
one becomes a compile error listing the places to update. That is Section 22.2.1's
exhaustiveness, and on a tree walker it is the difference between a refactor that
is safe and one that is a search.

**The method calls itself on the children.** `show` handles a `Bin` by showing its
parts, which are smaller trees. The base cases, `Num` and `Var`, do not recurse.
That is precisely Chapter 13's structure — recurse on the sub-problems, stop at
the atoms — and it is how every operation on a tree is written.

Verified:

```
2 + 3 * 4      -> (+ 2 (* 3 4))
(2 + 3) * 4    -> (* (+ 2 3) 4)
2 - 3 - 4      -> (- (- 2 3) 4)
1 + 2 + 3 + 4  -> (+ (+ (+ 1 2) 3) 4)
width * 2 + 1  -> (+ (* width 2) 1)
```

The output is in prefix form — operator first — which makes the nesting explicit
and is worth reading carefully. `(+ 2 (* 3 4))` is the 14 tree from Section
24.1.3. `(* (+ 2 3) 4)` is what parentheses did to it. The last two show
left-associativity: the accumulated tree is always the left child.

Incidentally, that prefix notation is Lisp's actual syntax. Lisp programs are
written as parenthesized prefix lists, which means Lisp source *is* a tree with
the parentheses drawn in — which is why Lisp needs almost no parser and why
Chapter 25 can be shorter in Scheme than in Java. McCarthy did not choose that
notation to be difficult.

## Another walk

Once the shape is declared, every operation over it has the same skeleton:

```java
static int depth(Expr e) {
    return switch (e) {
        case Num n -> 1;
        case Var v -> 1;
        case Bin b -> 1 + Math.max(depth(b.left()), depth(b.right()));
    };
}
```

Verified: `1 + 2 + 3 + 4` has depth 4, and every other example above has depth 3.
The left-leaning chain of three additions is one level deeper than the trees with
a single nesting, which is the associativity showing up as a measurement.

That skeleton — switch on the shape, base cases return, recursive cases combine —
is every tree operation you will write. Chapter 25's evaluator is this method with
arithmetic in place of `max`. A pretty-printer is this method with strings. A
type checker is this method with types.

Once you see that, an interpreter stops looking like a large program and starts
looking like one you have already written twice.

## Concrete versus abstract

Some parsers build a **concrete syntax tree** — a parse tree — with a node for
every grammar rule applied, including the `term` and `factor` levels that existed
only to encode precedence, and nodes for the parentheses.

For `2 + 3 * 4` the concrete tree has a dozen nodes where the abstract one has
five. The extra structure is faithful to the derivation and useless for
evaluation.

The rule of thumb: **build an abstract tree unless you need to reproduce the
original text.** Compilers and interpreters want abstract. Code formatters,
refactoring tools, and syntax highlighters want concrete, because they must put
the whitespace and the parentheses back exactly as the programmer wrote them.

The parser in the next lesson builds abstract trees. It applies the `term` rule
and then does not create a node for it, which is why the layering that
fixed the ambiguity leaves no trace in the output.

Next: the parser itself.
