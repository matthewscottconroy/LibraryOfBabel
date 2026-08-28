# The Contract

`static int largest(int[] values)` — what does that promise?

It is a fair question and the obvious answer is incomplete in ways that will bite
somebody. What if the array is empty? What if it is null? Is the result guaranteed
to be an element of the array? The signature answers none of that, and the answers
are still part of what the method means.

Two lessons on what a method promises.

A signature says what goes in and what comes out. It leaves out everything else:
which inputs are acceptable, what is guaranteed about the result, and whether
anything outside the method changes. Those are the contract's real terms, and
stating them is what turns a method into something you can rely on without
reading.

The first lesson introduces preconditions and postconditions. The second is about
what to do when a caller breaks the deal — which is a design decision with
consequences, and one where the wrong choice produces bugs that surface far from
their cause.
