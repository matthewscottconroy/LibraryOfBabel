# Evaluating Arithmetic

This is the moment the tree stops being a picture and produces a number.

Watch for one thing as it does. The code coming up contains no table of operator
precedences, no comparison of which operator binds tighter, nothing about
precedence at all — and `2 + 3 * 4` comes out 14. Where the precedence went is the
most useful observation in the lesson.

Here is `eval` for the three node types Chapter 24 built.

```java
int eval(Expr e, Env env) {
    return switch (e) {
        case Num n -> n.value();
        case Var v -> env.lookup(v.name());
        case Bin b -> {
            int l = eval(b.left(), env);
            int r = eval(b.right(), env);
            yield switch (b.op()) {
                case '+' -> l + r;
                case '-' -> l - r;
                case '*' -> l * r;
                case '/' -> {
                    if (r == 0) throw new EvalError("division by zero");
                    yield l / r;
                }
                default -> throw new EvalError("unknown operator " + b.op());
            };
        }
    };
}
```

That is a working arithmetic interpreter. Verified:

```
print 2 + 3 * 4;      ->  14
print (2 + 3) * 4;    ->  20
print 100 / 7;        ->  14
print 0 - 5;          ->  -5
```

## It is the tree walk again

Compare with `depth` from Section 24.2.2:

```java
case Num n -> 1;
case Var v -> 1;
case Bin b -> 1 + Math.max(depth(b.left()), depth(b.right()));
```

Same skeleton. Base cases return; the recursive case recurses on the children and
combines. `depth` combines with `max`; `eval` combines with the operator. That is
the only difference between measuring a tree and running it.

This is worth dwelling on because it demystifies the word *interpreter*. An
interpreter is a recursive function over a tree. You have written several.

## Where precedence went

`2 + 3 * 4` gives 14, and there is nothing in `eval` about precedence. No table,
no comparison of operator strengths, no special case.

Precedence was settled by the parser. Section 24.1.3's layered grammar built the
tree `(+ 2 (* 3 4))`, and `eval` only walks what it is given. To compute the
addition it must first evaluate both children, and the right child is the
multiplication, so the multiplication happens first — because it is deeper.

**Depth in the tree is order of evaluation.** That is the whole relationship
between syntax and execution, and it is why getting the grammar right in Chapter
24 mattered.

Parentheses likewise. `(2 + 3) * 4` gives 20 because the parser built a different
tree. `eval` cannot tell that brackets were involved and does not need to.

## Order of evaluation, left to right

```java
int l = eval(b.left(), env);
int r = eval(b.right(), env);
```

Two statements, so the left is evaluated first. Java specifies the same for its
own operators, and it matters as soon as evaluation can have side effects — if
evaluating an expression can print something or change a variable, the order is
observable.

Our expressions have no side effects yet, so the order is invisible. C and C++
leave it unspecified for exactly this reason, which is a well-known source of
code that works on one compiler and not another. Java specifies left-to-right, and
that specification is a line of documentation backing a decision that looks like
this one.

## Errors

```
print 1 / 0;   ==>  division by zero
```

Two things worth noting about that check.

It is in `eval`, not in the parser, because it is not a property of the shape.
`1 / 0` is a perfectly well-formed expression; it is only its *meaning* that
fails. This is Section 24.1.2's boundary again, and every check falls on one side
or the other of it.

And it is checked explicitly rather than left to Java. Without the check, `l / r`
would throw `ArithmeticException`, our language's user would see a Java stack
trace, and the abstraction would leak. **An interpreter must not let its
implementation's errors escape into its language.** Every operation that can fail
needs a check that converts the failure into an error in *our* terms.

That principle costs a few lines per operation and it is what distinguishes an
interpreter from a demonstration. It is also why our `EvalError` exists as its own
type.

## What is not checked

Integer overflow. `print 2000000000 + 2000000000` gives a negative number,
silently, because our `+` is Java's `+` and Java's `int` wraps at $2^{31}$.

Our language inherited Chapter 2's arithmetic without deciding to. This is the
general hazard of writing an interpreter in a high-level language: whatever you do
not decide, you inherit. Our numbers are 32-bit and two's complement, our division
truncates toward zero, our `/` on negatives rounds the way Java rounds — none of
which we chose.

A language designer would decide each of these. Python's integers are arbitrary
precision; JavaScript's numbers are all doubles; Scheme's are exact rationals
where possible. Each is a decision someone made in the equivalent of this switch,
and each has consequences that reach every program written in the language.

Exercise 25.4 changes ours to `long` and then to `BigInteger`, which takes about
five minutes and makes the point better than another paragraph would.

## The default clause

```java
default -> throw new EvalError("unknown operator " + b.op());
```

This one is unavoidable and slightly unsatisfying. `Bin`'s operator is a `char`,
so the compiler cannot know the set is closed and cannot check exhaustiveness —
the `default` is required.

Had the operator been an enum, Section 22.2.1's exhaustive switch would apply and
the `default` would be unnecessary, with adding an operator becoming a compile
error instead of a run-time one. That is the better design, it is Exercise 25.5,
and the reason the code here uses a `char` is that it keeps the parser shorter for
a first reading.

It is a fair example of a real trade: the version in the book optimizes for being
read once, and the version you would ship optimizes for being changed.

Next: turning this into a language with names, definitions, and procedures.
