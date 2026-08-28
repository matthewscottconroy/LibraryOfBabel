# The Class

A program that models anything real quickly acquires groups of variables that
belong together — a balance and an owner and an account number, three arrays that
must stay the same length, a width and a height that are only meaningful as a
pair. Keeping them together is a convention you hold in your head, and conventions
held in heads are the ones that break.

This section is about writing the grouping down so the language holds it instead.

Three lessons.

An object first, described as what Chapter 6 would call it: a small machine with
its own state and a fixed set of transitions. Then fields and constructors — how
an object comes into existence already satisfying its invariant, which is the
whole reason constructors exist. Then methods that guard state, which is where
the invariant stops being a comment and starts being enforced.
