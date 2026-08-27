# The Limits of the Type System

Two lessons about the boundary between what the compiler knows and what the
running program knows.

Erasure first: the generics of Chapter 17 are enforced during compilation and
almost entirely absent afterwards, which explains a set of restrictions that look
arbitrary until you know why. Then annotations, which run the other way — extra
information attached to code specifically so that another program can read it,
whether at compile time or at run time. That is where `@Override` finally gets
its explanation.
