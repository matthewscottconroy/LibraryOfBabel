# Important Researchers

**John McCarthy** (1927–2011) is this chapter's author in every sense that
matters. His 1960 paper *Recursive Functions of Symbolic Expressions and Their
Computation by Machine, Part I* introduced Lisp, and with it: `eval`, the idea
that a program can be represented as the language's own data, garbage collection,
conditional expressions as a construct, and recursion as a normal way to compute
rather than a curiosity. Any one of those would be a career. He also coined
"artificial intelligence" and named the field.

McCarthy's own view of `eval` was that it was a *definition* — a way of saying
precisely what Lisp meant, in Lisp, so that the language did not have to be
explained in English. That his student Steve Russell could implement it as written,
and thereby produce an interpreter nobody had planned to build, is the most quoted
anecdote in the field's history and deserves the quoting. Turing Award, 1971.
Chapter 13 introduced him for recursion; this is what he wanted the recursion for.

**Steve Russell** (born 1937) hand-compiled `eval` into IBM 704 machine code
because he did not accept that it was only a specification. He also wrote
*Spacewar!*, one of the first computer games, and later worked on the systems Bill
Gates and Paul Allen learned to program on. He is a good example of a figure whose
contribution was to take an idea literally when its author did not.

**Peter Landin** (1930–2009) built the bridge from Church's lambda calculus —
Chapter 13's — to how languages are actually implemented. His 1964 paper described
the SECD machine, the first abstract machine for evaluating lambda expressions,
and the ancestor of every bytecode virtual machine including the JVM. He coined
"syntactic sugar", introduced the `let` construct, and in 1966 wrote *The Next 700
Programming Languages*, which argued that most language differences are notational
and that the underlying evaluation model is nearly always the same one. This
chapter is a small piece of evidence for that claim.

**Christopher Strachey** (1916–1975) and **Dana Scott** (born 1932) gave meaning a
mathematical treatment: denotational semantics, in which a program is mapped to a
mathematical object rather than to a sequence of steps. Our `eval` is *operational*
semantics — it says what happens — and the denotational alternative says what a
program *is*, independent of any machine. The distinction matters when you want to
prove two programs equivalent, which is impossible to do by running them. Scott
received the Turing Award in 1976.

**Gerald Jay Sussman** (born 1947) and **Guy Steele** (born 1954) designed Scheme
in 1975, whose first contribution was fixing Lisp's dynamic scope — the decision
Section 25.1.2 describes as one argument in `apply`. Their "Lambda Papers" of the
following years argued that procedure calls and message sends and `goto` are the
same operation seen from different angles, and that a well-implemented function
call needs no more machinery than a jump. Sussman later co-wrote *Structure and
Interpretation of Computer Programs*, which this unit is modelled on and which
builds this chapter's interpreter in Scheme.

**Harold Abelson** (born 1947) co-wrote SICP with Sussman and taught the course it
came from. The book's central argument — that a first course should treat
programs as objects of study, and should culminate in writing an evaluator for the
language you are learning — is the reason this unit exists in the shape it does.
Chapter 35 returns to it.
