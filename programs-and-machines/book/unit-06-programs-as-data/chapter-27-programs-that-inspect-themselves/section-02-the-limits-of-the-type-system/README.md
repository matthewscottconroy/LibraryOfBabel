# The Limits of the Type System

Two things you have been told are about to turn out to be less than the whole
truth.

Chapter 17 said a `List<String>` is a list of strings and the compiler enforces it.
Chapter 20 said `@Override` catches a mistake. Both are true; neither is quite what
you would guess from the wording, and the gaps are where this section lives.

Two lessons about the boundary between what the compiler knows and what the
running program knows.

Erasure first: the generics of Chapter 17 are enforced during compilation and
almost entirely absent afterwards, which explains a set of restrictions that look
arbitrary until you know why. Then annotations, which run the other way — extra
information attached to code specifically so that another program can read it,
whether at compile time or at run time. That is where `@Override` finally gets
its explanation.
