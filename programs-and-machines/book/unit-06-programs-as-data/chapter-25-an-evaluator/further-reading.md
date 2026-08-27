# Further Reading

**Harold Abelson and Gerald Jay Sussman, *Structure and Interpretation of Computer
Programs*, second edition.** Free online. Chapter 4 builds the metacircular
evaluator: the same interpreter as this chapter, in Scheme, for Scheme. Reading it
after writing yours is a different and better experience than reading it first,
because you will recognize every decision. Then read section 4.2, which changes
the evaluator to be lazy, and 4.3, which adds nondeterminism — both by editing
`eval`, and both startling.

**Robert Nystrom, *Crafting Interpreters* (2021).** Free online. Recommended in
Chapter 24 and even more relevant here. Part II builds a complete tree-walking
interpreter in Java for a language with closures, classes, and inheritance; Part
III rebuilds it as a bytecode VM in C. The two halves together are the best
available illustration of what Section 25.2.3 means by a spectrum.

**John McCarthy, "Recursive Functions of Symbolic Expressions and Their
Computation by Machine, Part I" (1960).** Read it now, having built one. It is
about fifteen pages, the notation takes some adjusting to, and `eval` is on page
13. Chapter 13's further reading told you to wait for this chapter; this is it.

**Peter Landin, "The Next 700 Programming Languages" (1966).** Short, funny, and
sharper than most things written about language design since. The argument that
differences between languages are mostly notational will read differently now that
you have made a language by choosing notation.

**Niklaus Wirth, *Compiler Construction* (1996).** Free online. Wirth designed
Pascal, Modula-2 and Oberon, and wrote compilers that were famously small and fast.
The book is under 200 pages and covers the whole pipeline. A useful corrective to
the Dragon Book's weight.

**Guy Steele and Gerald Jay Sussman, "Lambda: The Ultimate Imperative" (1976) and
its companions.** The Lambda Papers. The argument that function calls, message
sends, and jumps are one mechanism, made by implementing each in terms of the
others. Demanding, and worth returning to after Chapter 26.

**Andrew Appel, *Modern Compiler Implementation in Java*.** If Exercise 25.9's
static checker interested you, this is where that thread continues — type checking,
intermediate representations, register allocation, and the rest of what happens
between the tree and the machine code.
