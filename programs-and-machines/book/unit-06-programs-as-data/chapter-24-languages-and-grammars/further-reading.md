# Further Reading

**Robert Nystrom, *Crafting Interpreters* (2021).** Free online, and the best
book there is for what this unit does. Nystrom builds two complete interpreters
for the same language — a tree-walker in Java, then a bytecode VM in C — and
explains every decision. The chapters on scanning and parsing cover Section 24.2
in more depth and with better error handling. If this chapter interested you, read
this next, not instead.

**Aho, Lam, Sethi, and Ullman, *Compilers: Principles, Techniques, and Tools*,
second edition.** The Dragon Book. Comprehensive, formal, and heavy going; the
right reference once you have written a parser by hand and want to know what the
generated ones do. Chapters 3 and 4 are lexing and parsing.

**Peter Naur, ed., "Report on the Algorithmic Language ALGOL 60" (1960).** Worth
looking at as an artifact. It is the first language definition precise enough to
implement from, the grammar is readable sixty years later, and comparing it with
any language manual written before it shows what the notation bought.

**Noam Chomsky, "Three Models for the Description of Language" (1956).** Short,
and readable if you skip the linguistics arguments. The hierarchy is here, in its
original setting, and seeing it as an argument about English rather than about
compilers is clarifying.

**Terence Parr, *The Definitive ANTLR 4 Reference*.** ANTLR is the parser generator
you would actually use, and it generates recursive-descent parsers — the same
technique as Section 24.2.3, mechanically produced with better error recovery.
Reading its output after writing a parser by hand is instructive.

**John Backus, "Can Programming Be Liberated from the von Neumann Style?" (1977).**
The Turing Award lecture. Provocative, occasionally overstated, and the source of
several ideas Chapter 26 takes up. Read it for the argument, not the language he
proposes.

**Peter Naur, "Programming as Theory Building" (1985).** Not about parsing at all.
An essay arguing that the valuable product of programming is a theory held by the
programmers, of which the code is an incomplete record. Short, unusual, and it
will change how you think about handing over a codebase.
