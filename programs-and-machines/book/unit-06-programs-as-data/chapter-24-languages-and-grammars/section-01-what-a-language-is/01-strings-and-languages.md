# Strings and Languages

Before any machinery, one definition — and I should warn you in advance that it is
going to look as though it has thrown away everything interesting about the
subject.

Read it anyway, and reserve judgment until the paragraph after it.

Here is the vocabulary. Give it a slow minute, because everything in the next two
chapters is built out of these four words.

An **alphabet** is a finite set of symbols. Call it $\Sigma$. For English text
that might be the 26 letters; for Java source it is the Unicode characters of
Chapter 4; for a machine it is $\{0, 1\}$.

A **string** over $\Sigma$ is a finite sequence of its symbols. The empty string,
written $\varepsilon$, is a string of length zero, and it is a real object rather
than an absence — Java writes it `""`.

$\Sigma^*$ is the set of *all* strings over $\Sigma$, including $\varepsilon$.
For any alphabet with at least one symbol this set is infinite, because a string
can be as long as you like.

And then the definition:

> A **language** over $\Sigma$ is a subset of $\Sigma^*$.

That is all. A language is a set of strings — the ones that are *in* it, against
all the others that are not.

## Why throw so much away

Notice everything that definition does not mention. Not meaning. Not grammar. Not
what a program does, or whether it does it well. A language is a set of strings and
nothing else whatsoever.

That is not an oversight, and it is not the definition being lazy before getting to
the real one. It is the move that makes the entire subject tractable — and it is a
move this book has made before. Chapter 1 separated a bit pattern from what the
pattern means. This separates a program's *form* from its *meaning* in exactly the
same way, and for exactly the same reason.

Java, under this definition, is the set of character sequences that are legal Java
programs. `int x = 3;` is in the set. `int x = ;` is not. The question of what
`int x = 3;` *does* is a different question entirely, and Chapter 25 handles it.

Splitting the two is what makes both tractable. A compiler's front end decides
membership in the set. Its back end decides meaning. They are separate programs
with separate theories, and the separation is not an accident of engineering —
it is this definition.

## Small examples

Some languages are finite and can be listed:

$$L_1 = \{\texttt{yes}, \texttt{no}\}$$

Most are not:

$$L_2 = \{\texttt{a}, \texttt{aa}, \texttt{aaa}, \ldots\}$$

$L_2$ is infinite, which is the normal case and the source of the whole problem.
You cannot define a language by listing it, so you need a finite *description* of
an infinite set. There are two standard kinds:

**A recognizer** — a machine that reads a string and answers yes or no. Chapter 6's
Turing machine is the general form.

**A generator** — a set of rules that produce exactly the strings in the language.
That is a grammar, and it is the next lesson.

Both descriptions are finite. The set need not be.

## Not every language has one

Worth knowing now, because Chapter 34 collects the debt.

$\Sigma^*$ is countably infinite — you can list all strings in order of length.
But the *languages* over $\Sigma$ are all the subsets of $\Sigma^*$, and there are
uncountably many of those. Put those two facts together and something falls out that you cannot argue with.
There are strictly more languages than there are finite descriptions to go around
— so there must exist languages that no grammar generates and no program
recognizes.

We have not shown you one. We have shown that they have to be there.

This is not a curiosity about exotic sets. Chapter 34 exhibits a specific,
extremely natural language — the set of programs that halt — and shows no
recognizer exists. The counting argument above is why such a thing has to exist;
the halting proof is why it is one you would actually want.

## The hierarchy

Not every language needs the same power to describe, and the standard
classification is Chomsky's, from 1956:

| type | grammar | recognized by | example |
|---|---|---|---|
| 3 | regular | finite automaton | identifiers, numbers |
| 2 | context-free | pushdown automaton | nested expressions |
| 1 | context-sensitive | linear bounded automaton | rarely used directly |
| 0 | unrestricted | Turing machine | anything computable |

Each contains the ones below it, and each buys power at a price in what you can
decide about it.

Two of the four matter to this chapter, and the split between them is exactly the
split in Section 24.2.

**Regular** languages are what a machine with finite memory can recognize. They
describe the *words* — a number is a run of digits, an identifier is a letter
followed by letters and digits. This is what regular expressions are and it is
what the tokenizer does.

**Context-free** languages require a stack. They describe *nesting* — matched
parentheses, an expression inside an expression. A finite automaton provably
cannot do this: to check that `(((...)))` is balanced you must count, and counting
without bound needs unbounded memory. That is the parser's job, and the stack it
uses is Chapter 12's call stack, arriving through recursion.

So the two-stage design of every language implementation — tokenize, then parse —
is not a convention. It is a consequence of the fact that words are regular and
structure is not.

Next: how a finite set of rules describes an infinite language.
