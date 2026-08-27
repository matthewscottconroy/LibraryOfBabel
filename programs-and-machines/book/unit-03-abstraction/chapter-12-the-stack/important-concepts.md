# Key Concepts

**Call frame.** The state belonging to *one execution* of *one method*: its
parameters, its local variables, the return address, and working space. Chapter
6's state, scoped to a call.

**One execution, not one method.** Three calls to the same method mean three
frames with independent variables. This is why a local is fresh on every call and
remembers nothing, and it is what makes recursion possible.

**Push on call, pop on return.** Because calls nest, the most recently started is
always the first to finish. The stack discipline is not a design choice but a
recognition of the shape the problem already has.

**Suspended frames.** At any instant one method is executing and the rest of the
stack is a record of how the program got there, each frame stopped at a call and
holding its variables.

**A stack trace is the stack, printed.** Top line is the executing frame; each
line below is its caller. This is why Chapter 10's reading rule works and why a
debugger can show you a caller's variables — the frame is suspended, not gone.

**Allocation is a pointer move.** Pushing a frame subtracts from the stack
pointer; popping adds it back. One arithmetic operation, no searching — which is
why method calls are cheap and why breaking code into small methods costs
essentially nothing.

**The stack is bounded.** A fixed region, typically under a megabyte per thread,
giving room for something in the order of tens of thousands of nested calls.

**Stack versus heap.** The stack holds frames, tied to call structure, freed in
strict order by moving a pointer. The heap holds objects, freed when nothing
refers to them. A local variable of object type lives on the stack and holds a
reference to a heap object — which is how an object outlives the method that
created it.

**`StackOverflowError`.** Frames exhausted. Classified as an `Error` because
recovery would require calling a method, and frames are what ran out. Nearly
always an unintended infinite recursion: a missing base case, a base case never
approached, or accidental mutual recursion. The repeating pattern in the trace is
the diagnosis.

**Java does not eliminate tail calls.** A deliberate decision, defended partly on
the value of complete stack traces, and a real limitation next to languages that
do.

**Reference.** For an object, the variable holds *where the object is*, not the
object. Objects go on the heap because a frame is fixed-size and an object's size
is not known when the method is compiled.

**One assignment rule.** *Assignment copies the contents of the box.* For a
primitive that is the value; for an object it is the reference. The rule is the
same in both cases and produces opposite-looking results because the contents
differ in kind.

**Aliasing.** Two variables holding the same reference are two names for one
object. Changing it through either is visible through both.

**References are not pointers.** Opaque, with no arithmetic and no way to
construct one. That restriction is what makes Java memory-safe and removes buffer
overflows.

**`null`.** The absence of a reference — not an object, not an empty array, not
zero. Following one throws `NullPointerException`.

**Java is pass-by-value, always.** The contents of the caller's variable are
copied into the parameter. For an object, the copied value is a reference, so the
parameter becomes an alias.

**The two operations that look alike.** `p = something` writes to the parameter's
box and is invisible to the caller. `p[0] = something` or `p.setName(...)` follows
the reference and is visible everywhere. Every apparent contradiction resolves
here.

**The decisive test.** If Java passed by reference, assigning a new object to a
parameter would replace the caller's object. It does not — which is why "objects
are passed by reference" is a rule that predicts correctly until it does not, and
then offers no explanation.

**Consequences.** You cannot write a method that swaps two `int` variables. A
method receiving an object can modify it, so say in the contract whether it does.
Reassigning a parameter accomplishes nothing outward. Returning is how information
comes back.

**Overloading.** Several methods sharing a name, distinguished by the number,
types, and order of parameters. The return type is *not* part of the signature,
because a call that discards the result would be ambiguous.

**Resolution order.** Exact match or smallest widening first, then boxing, then
varargs — always preferring the most specific. Resolution is **static**: it uses
the declared type, not the runtime type, which is the contrast with overriding in
Chapter 21.

**Overloading hazards.** `f(null)` is ambiguous when several reference overloads
exist. `list.remove(1)` selects `remove(int)` and removes by index rather than by
value.

**When not to overload.** When the methods do different things, when related types
mean different operations, or when a good distinct name exists.
