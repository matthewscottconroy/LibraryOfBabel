# Languages and Grammars

You have written Java for eighteen chapters and never asked what makes a piece of
text *be* Java.

The question has an exact answer. Not a matter of taste, not "whatever the
compiler accepts" — an answer written down in a document, in a notation invented
for the purpose, from which the compiler's behavior follows. That notation is a
**grammar**, and it is the subject of this chapter.

The first section is the theory, and it is short because the theory is small. A
**language** is a set of strings. A **grammar** is a finite set of rules that
generates them, which matters because most interesting languages are infinite and
you cannot list an infinite set. Then **ambiguity**, which is what happens when a
grammar allows a string to be read two ways, and which is the reason `2 + 3 * 4`
is 14 rather than 20.

The second section builds. We take text and produce a tree, in two steps that
every language implementation uses: **tokenizing**, which groups characters into
words, and **parsing**, which finds the structure. The parser is recursive
descent, it is about sixty lines, and its shape is the grammar's shape — each rule
becomes a method, and a rule that refers to itself becomes a method that calls
itself.

That correspondence is the chapter's real content. Once you have seen a grammar
turn directly into code, parsers stop being mysterious, and the mystery is
replaced by something more useful: the knowledge that a notation for describing
structure can be executed.

Two things this chapter is not. It is not a compilers course — there is no lexer
generator, no LALR table, no left-recursion elimination beyond the one case we
need. And it is not about Java's grammar, which runs to several hundred rules. We
build a small language of arithmetic with variables, because it is large enough to
show every idea and small enough to fit in one file.

Chapter 25 then makes it run.

One promise to flag now, since Chapter 13 made it. Recursion was introduced there
with a warning that its natural home was still some chapters off. This is the
home. A grammar is recursive because expressions nest, a tree is recursive because
a branch holds trees, and the parser is recursive because it walks a recursive
grammar to build a recursive tree. Three recursions, and they are the same
recursion.
