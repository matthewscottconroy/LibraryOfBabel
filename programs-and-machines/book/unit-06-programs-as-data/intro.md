# Programs as Data

This is the center of the book.

Everything so far has treated a program as a thing you write and a machine
executes. Unit VI takes the other view: a program is **text**, text is data, and
data is something a program can read. Once you accept that, a program can read
another program, and it can read one written in a language you invented an hour
ago.

That is not a stunt. It is the idea underneath compilers, interpreters, browsers,
configuration files, database engines, spreadsheets, regular expressions, and the
`javac` that produced your first `.class` file. It is also, historically, the idea
that turned computing from a collection of machines into a subject.

And here is the surprising part: you are already equipped. Nothing has been held
back for this moment.

You have recursion, which is how a structure that contains smaller copies of itself
gets walked. You have the stack, which is both what makes recursion work and what
every real language runs its own function calls on. You have records and sealed
types, which turn out to be precisely what a syntax tree is made of, as though they
had been designed for it. And you have the design judgment to put them together.
There is nothing new to
learn about Java before starting; what is new is the object of study.

Four chapters.

**Chapter 24 — Languages and Grammars.** A language, formally, is a set of
strings, and a grammar is a finite description of a possibly infinite set. We
write one, discover ambiguity, fix it, and then turn text into a tree with a
parser you can hold in your head. The parser is recursive, and it is recursive
because the grammar is.

**Chapter 25 — An Evaluator.** We write an interpreter for a small language of
our own, in Java, in about two hundred lines. It has numbers, arithmetic,
variables, and procedures, and by the end of the chapter it will run programs you
did not anticipate when you wrote it. The structure is `eval` and `apply` calling
each other, which is the oldest idea in the subject and still the clearest.

**Chapter 26 — Functions as Values.** Having built a language in which procedures
are values, we notice that Java has this too. Lambdas, functional interfaces,
higher-order methods, and the map/filter/reduce vocabulary — introduced here
rather than earlier because the evaluator makes the concept concrete first and the
syntax second.

**Chapter 27 — Programs That Inspect Themselves.** Reflection: a running Java
program examining its own classes. Then erasure, which is where the generics of
Chapter 17 turn out to be a compile-time fiction, and annotations, which is where
`@Override` finally gets explained.

Why an interpreter, specifically? Because it collapses a distinction you have been carrying since Chapter 5.

There is the program, and there is what it means. Until now the second has been
the JVM's business — something that happens elsewhere, correctly, for reasons you
were asked to take on faith. Writing an evaluator makes meaning something you
implement. A variable lookup becomes a hash-map access you typed. A function call
becomes a new environment you allocated. The stack becomes a stack.

Students who write an interpreter tend to report the same thing afterwards: the
language they use every day stops being a set of rules and becomes a set of
decisions, most of which could have gone another way. That is worth more than any
particular technique in this unit.

This unit's structure is borrowed, and it should be said plainly. *Structure and
Interpretation of Computer Programs* — Abelson and Sussman, MIT, 1985 — builds a
Scheme interpreter in Scheme, and the argument that it is the right thing to do
partway through a first course is entirely theirs.

What is different here is the language and therefore the friction. Scheme's
programs are already trees, so their parser is nearly free and the chapter can go
straight to evaluation. Java's syntax is not self-describing, so Chapter 24 has to
earn the tree before Chapter 25 can walk it.

That extra work is not wasted. Parsing is a genuinely useful skill — you will meet
configuration formats, query languages, and log files long before you meet another
interpreter — and doing it by hand once means you will recognize what a parser
generator is doing for you.

A word on what to expect. This unit is harder than Unit V and shorter. The difficulty is not in the Java,
which is all familiar. It is in holding two levels in your head at once: the
program you are writing, and the program it is reading. Confusing them is the
characteristic error, and it happens to everyone.

Two habits help. Keep asking *which level is this?* when a variable name could
belong to either. And run the thing constantly — an evaluator you have run on
twenty small inputs is understood in a way that an evaluator you have read is not.
