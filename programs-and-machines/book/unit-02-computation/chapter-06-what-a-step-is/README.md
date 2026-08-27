# What a Step Is

We are going to build a model of computation, and we are going to build it small
enough that nothing is hidden.

The temptation at this point is to start with Java — `if`, `while`, the things
you came here to learn. I want to resist that for one chapter, for the same
reason Unit I resisted starting with `int`. If your first model of computation is
"the things Java lets me write", then computation and Java become the same thing
in your head, and you will have no way to think about what Java's constructs
*are*.

So this chapter builds the model from underneath. It has three ingredients and no
more:

- a **state**, which is everything the machine currently holds
- a **transition rule**, which says what the next state is
- a **starting state**, and a way of recognizing when to stop

That is a complete model of computation. Everything in Java — every loop, every
method call, every object — is a way of describing states and transitions more
conveniently than writing them out. Nothing in the language adds power to this
model. It only adds expressiveness, which is a different thing and which Unit III
is about.

The first section builds the model and shows a small machine actually computing.
The second looks at two historical machines that made the model concrete: Turing's
abstract machine from 1936, which was invented to prove something about the limits
of computation rather than to compute anything, and the stored-program design of
1945, which is the architecture of the device you are reading this on.

By the end you should be able to answer a question that sounds trivial and is
not: what is the difference between a program and the machine that runs it? The
answer will turn out to be less clear-cut than you expect, and that lack of
clarity is the most productive idea in the whole book — it is what Unit VI is
built on.

There is no Java in this chapter. Chapter 7 resumes it.

I realize that is the second time this book has asked you to wait, and I will not
do it again after this. But the model in this chapter takes about twenty pages to
build and pays for itself for the remaining seven units, and mixing it with syntax
would obscure both.
