# Key Concepts

**Debugging is the experimental method.** You hold a belief about what the
program does; the program disagrees; find where belief and behavior first
diverge. Changing plausible things and rerunning is a random search with no
memory and no termination condition.

**The machine is not capricious.** Chapter 6's determinism means a surprise
identifies a false belief of yours, and a specific one that can be found.

**State table.** One row per state change, one column per variable in scope, the
value recorded after the line executes. The written form of Chapter 6's sequence
of states.

**Trace to find, invariant to generalize.** A trace shows what happens for one
input; a loop invariant states what is true for all of them. Trace until you can
write the invariant, then stop.

**Tracing reveals mechanism.** A print statement tells you what happened; a hand
trace tells you what you *expected*, and the disagreement locates the false
belief. Tracing also works on code you cannot run.

**Desk checking.** Predict, choose inputs at the boundaries, trace while
predicting each step, compare with the prediction. The hazard is reading what you
meant rather than what you wrote — so evaluate expressions as written, and be
most suspicious where you feel most confident.

**Boundary inputs.** Empty, one element, and the ends of ranges. Ordinary
mid-range input is handled correctly by almost anything, including wrong code.

**Read the whole error.** A Java failure reports the exception *type*, the
offending *values*, the *line*, and the *call path*. `Index 3 out of bounds for
length 3` names the off-by-one and quantifies it.

**Stack trace.** Top line is where it broke; lines below are how execution got
there. In a long trace, find the topmost frame naming a file you wrote — frames
above it are usually a library correctly rejecting what you passed.

**Compile-time versus run-time.** Compile errors concern what you *wrote*: a name
that does not exist, a type that does not fit. Run-time errors concern what the
program *met*: an out-of-range value, a null, a missing file. Output printed
before a crash is evidence of how far execution got.

**Bisection.** Check the middle, discard the half that was still correct, repeat.
Each observation halves the search, so a thousand steps take about ten questions.
Requires an expectation at each checkpoint — without a prediction, an observation
tells you nothing.

**Check what you are certain of.** The commonest result of a good bisection is
discovering the input was not what you assumed. Unexamined beliefs are where bugs
live.

**Bisecting history.** The same method over commits rather than over execution.
A hundred commits take about seven checkouts; `git bisect` automates it.

**Minimization.** Narrow the *input* rather than the location: remove things while
the failure persists. The smallest failing input is usually close to
self-explaining, and producing one does most of the diagnostic work.

**The shared move.** Bisection, minimization, and boundary selection all make
observations that eliminate a *fraction* of the possibilities rather than testing
candidates one at a time — logarithmic instead of linear.

**Debugger.** Breakpoints, inspection of every variable in scope, stepping
over/into/out, a live call stack whose frames can be inspected, value
modification, and conditional breakpoints — the last being what makes a failure at
iteration 4,999 tractable.

**Set breakpoints before the problem.** By the time the failure occurs the
evidence is often gone.

**When printing wins.** Concurrency, where observation perturbs timing; anything
not on your machine; intermittent failures; and seeing a trend across many
iterations. A log is a record over time; a debugger is a complete picture at one
moment.

**The tool does not replace the method.** A debugger makes observation cheap. It
does not supply the expectation, locate the divergence, or ask what would have to
be true — and without those it is an efficient way to look at meaningless values.
