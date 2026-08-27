# Key Concepts

**Variable.** A name bound to a cell of state holding a value. All three parts —
name, cell, value — are separately real. In Chapter 6's terms, the cell is state
and the name is a convenience that disappears at compile time.

**The box picture.** A labeled box holding a value. Accurate for primitives, and
it expires in Unit V: for objects the box holds a *reference*, and two boxes can
refer to one object.

**Declaration, initialization, assignment.** Reserving the cell and binding the
name; writing the first value; writing any later value.

**Definite assignment.** Java refuses to compile a read of a local variable that
might not have been assigned. A compile-time proof, and deliberately conservative
— it will occasionally reject a program that would have worked.

**Locals versus fields.** Locals must be assigned before use; fields default to
0, `false`, or `null`. The rules differ because the compiler can see every path
through a method but not every path to a field.

**Assignment is a command, not a claim.** `=` evaluates the right side and stores
the result in the left. It is not symmetric: `n = 5` is legal and `5 = n` is not,
because the left side must have a location and the right side a value. This is
why `n = n + 1` is sensible — the two `n`s refer to different moments.

**`=` versus `==`.** Assignment versus comparison. Java's requirement that `if`
take a `boolean` catches most confusions, but not when the variable is itself a
`boolean`.

**Copy semantics for primitives.** `y = x` copies the value. Later changes to `x`
do not reach `y`. For objects this is not true, which is Chapter 20.

**Compound assignment.** `+=`, `-=`, `*=`, `/=`, `%=`, and `++`/`--`. Note that
`+=` performs an implicit narrowing cast, so `byte b = 10; b += 300;` compiles
and silently gives 54.

**Prefix and postfix.** `n++` yields the old value, `++n` the new one. Legal in
expressions and best avoided there.

**Type as promise.** A declaration commits you to what the cell may hold, and the
compiler enforces it. This buys early errors, lets the compiler select the right
operation for `/` and `+`, and fixes the size for memory layout.

**Static versus dynamic typing.** Java fixes types at compile time. The
alternative defers checking to run time. Both are defensible; Java's choice costs
verbosity and buys early failure and speed.

**`var` is inference, not dynamic typing.** The type is still fixed and enforced;
you merely did not write it.

**`final`.** A promise that a variable is never reassigned. Cheap to write, and
it removes a name from the set of things a reader must track.

**Scope.** Where a name is visible: from its declaration to the closing brace of
its block. Declaring in the smallest workable scope is a way of limiting how much
a reader must hold in mind.

**Lifetime.** When the cell exists at run time. A local's cell lives in the call
frame, so it is fresh on every call and remembers nothing between them. Scope and
lifetime coincide for primitives and diverge for objects, which may outlive the
name that created them.

**Mutation.** The cost of a changing variable is that its value at any line
depends on which assignment last executed. Scattered mutation makes code hard to
read; the accumulator pattern — many changes in one place, following one rule —
does not, because a single sentence describes it. That sentence is a loop
invariant.
