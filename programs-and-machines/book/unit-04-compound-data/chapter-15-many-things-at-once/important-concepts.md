# Key Concepts

**Array.** A fixed-size sequence of same-typed values stored consecutively and
reached by position.

**The index is arithmetic.** `address = base + i × elementSize`. One
multiplication and one addition, so access costs the same for element 47 as for
element 4,700,000 — **constant-time access by position**.

**Every constraint follows from that formula.** Same type, because the formula
needs one `elementSize`. Consecutive storage, because it adds a simple offset.
Fixed size, because the space after the array belongs to something else.

**Indices start at 0** because an index is an *offset*, not a count. Element 0 is
at `base + 0 × size`. One-based indexing would add a subtraction to every access
ever performed.

**An array variable holds a reference.** The array is on the heap, because a stack
frame is fixed-size and an array's size is not known at compile time. `b = a`
copies the reference: one array, two names.

**`a.length` is a field**, `s.length()` and `list.size()` are methods. A historical
inconsistency nobody can now fix.

**Defaults.** `new int[5]` is five zeros; `new String[3]` is three `null`s. An
array of objects starts empty of objects.

**Copying is explicit and shallow.** `clone`, `Arrays.copyOf`, `Arrays.copyOfRange`
make a new array, but copy only one level — `clone` on an `int[][]` shares the
inner arrays.

**`==` versus `Arrays.equals`.** `==` compares references — is it the same array.
`Arrays.equals` compares contents.

**`Arrays.toString`.** Printing an array directly gives type and hash code, not
contents.

**The enhanced `for` cannot modify.** `for (int s : a) s = 0;` assigns to a copy.
Use an index to write.

**Bounds checking.** Java compares the index against the length on every access.
Without it, an out-of-range write silently corrupts unrelated memory — the
mechanism behind **buffer overflow** attacks, which account for a large share of
security defects in languages that omit the check.

**The cost is usually near zero.** Branch prediction handles it, and the JIT
performs **bounds check elimination** when the loop condition already proves the
index safe — so the idiomatic loop pays nothing and a clever hand-optimized one
often defeats the optimizer.

**Java has no two-dimensional arrays.** `new int[3][4]` creates four objects: an
outer array of three references and three inner arrays. `grid[1][2]` is two memory
accesses, not one multiplication.

**Rows are objects; columns are not.** `grid[0]` is an array you can pass around.
Column 2 has no object and must be collected. The notation `grid[r][c]` looks
symmetric and the structure is not — a leaky abstraction.

**Ragged arrays.** Rows may differ in length. Useful for triangular data, genuinely
varying data, and rows that arrive separately. **Always write `grid[row].length`,
never `grid[0].length`** — correct for rectangular arrays too.

**Rectangularity is a precondition, not a guarantee.** Nothing in `int[][]` promises
it. State it, and check it in public methods.

**Locality of reference.** Memory is fetched in cache lines of about 64 bytes.
Consecutive access gets fifteen or so elements nearly free; scattered access pays
full price each time. Row-major traversal of a 4000 × 4000 grid measured 3× faster
than column-major for identical work.

**Walk data in the order it is stored** — vary the last index in the innermost
loop.

**Traversal idioms.** The diagonal `grid[i][i]`; the upper triangle with
`c = r + 1`, which visits each unordered pair once; and the eight-neighbor offset
pattern, where the bounds check at the edges is most of the work.

**What arrays cannot do.** Grow; insert or remove without shifting; find by content
in less than linear time; or say what they mean. The next three chapters address
these.
