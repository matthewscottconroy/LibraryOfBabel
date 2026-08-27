# Key Concepts

**The gap this chapter closes.** An array of values is a pile of values. That they
are scores, that they are bounded, that −1 means absent — none of that is in the
data. It lives in your head, and anything that can reach the array can violate it.

**Abstract data type.** A set of operations with contracts, considered apart from
any particular storage. Every implementation answers *what can I do with this* the
same way and *how is it stored* differently.

**Interface versus representation.** The interface is what a user must know; the
representation is what a user must *not* need to know.

**If a user can observe the representation, you cannot change it.** Not "should
not" — cannot, without breaking their code. Whatever people can observe, they come
to depend on.

**Why it pays.** Fifty call sites using three operations means a change of storage
costs one file. Fifty call sites reaching into the array means fifty edits.
`String` changed its internal storage in Java 9 and no source anywhere needed
altering.

**How representations leak.** Public fields; returning the internal array (a
reference, not a copy); behavior people notice and depend on; and exposed limits.

**Representation invariant.** The claim about the fields that must hold whenever
anyone outside can observe. It is what turns two variables that sit near each
other into a *set of names*.

**The invariant is what lets operations be written.** `size()` returns `count`
because the invariant says `count` is the number of names. Like a precondition, an
invariant is *a way of not handling cases*.

**The obligation.** Each operation may assume the invariant on entry and must
restore it on exit. Between those points it may legitimately be false — the
invariant must hold when outsiders can observe, not at every instant.

**Three scales, one technique.** Loop invariant (Chapter 9), method contract
(Chapter 11), representation invariant (here). State what stays true; check that
each step preserves it.

**Where invariants break.** A missed case; an escaping reference, which is worst
because the damage happens outside the unit meant to guarantee the property; and
concurrency, which interleaves during the window where the structure is legitimately
broken.

**What a class is for.** `private` puts a boundary around an invariant so that the
set of code which could break it is small enough to check. That is the sentence to
carry into Unit V.

**The primitive/object split.** Primitives are fixed-width values, not objects, with
no methods and no `null`. Collections hold objects only. The split exists for speed.

**Wrapper classes.** `Integer`, `Double`, `Character` and the rest hold one
primitive each — and are where the library keeps everything useful about that
primitive: `MAX_VALUE`, `parseInt`, `isNaN`, `isDigit`.

**Wrappers cost.** Allocation, indirection, a header (sixteen bytes to hold four),
and garbage. Summing three million values measured about 27× slower with `Long`
than with `long`.

**The `Integer` cache.** `valueOf` reuses instances for −128 to 127, which makes
object identity observable.

**Autoboxing hides a distinction that still matters**, and the traps are that
distinction resurfacing:

- **Identity.** `==` on wrappers compares references. It accidentally works below
  128, which is worse than never working. Use `equals`.
- **Unboxing null.** `int x = someInteger` throws when the wrapper is null — from
  a line containing no visible method call. Map lookups are the usual source.
- **Overload resolution.** `list.remove(1)` removes by index; exact match beats
  boxing.
- **Silent cost.** One capital letter turns a loop into three million
  allocations.
- **Typed equality.** `Integer(1).equals(Long(1L))` is false; wrappers are objects
  with types, not numbers.

**Rules.** Primitives unless a collection forces otherwise; never `==` on
wrappers; guard unboxing of anything nullable; watch types in hot loops.

**`null`.** Hoare introduced it in 1965 and called it his billion-dollar mistake.
It defeats the type system — every reference type is secretly itself-or-nothing and
the compiler will not say which. It fails remotely, far from where it entered. And
it has no meaning of its own: not-found, not-initialized, not-applicable, and error
are indistinguishable.

**Living with it.** Return empty collections, not null. Use `Optional` for
possibly-absent *return values* only. `Objects.requireNonNull` on parameters to
fail fast. State nullability in the contract. Prefer values that cannot be null.

**Better is possible.** Kotlin's `String` versus `String?` makes nullability part
of the type and checks it at compile time. Java cannot follow, because thirty
years of libraries assume otherwise.
