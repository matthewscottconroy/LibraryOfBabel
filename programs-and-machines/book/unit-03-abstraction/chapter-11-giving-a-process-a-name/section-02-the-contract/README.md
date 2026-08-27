# The Contract

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
