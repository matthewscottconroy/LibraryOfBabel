# Key Concepts

**`String` is immutable.** No operation modifies one; every method returns a new
string. `s.toUpperCase()` with the result discarded changes nothing.

**Four payoffs.** *Sharing is safe*, so identical literals are pooled into one
object. *Passing is safe*, so a method cannot alter your string and no defensive
copy is needed. *Hashing is safe*, so strings are ideal map keys and their hash can
be cached. *Threads are safe*, since nothing writes.

**The cost is allocation.** Nothing for occasional changes; quadratic in a loop.

**The literal pool makes `==` treacherous.** Identical literals are the same
object, so `==` works for literals and fails for anything built at run time — a bug
that passes every test and fails on real input. **Compare with `equals`, always.**

**Concatenation in a loop is quadratic.** Each `+=` allocates and copies
everything, so *n* appends copy about $n^{2}/2$ characters. 40,000 appends measured
79 ms; `StringBuilder` measured 1 ms.

**`StringBuilder`** is a mutable character sequence backed by a doubling array —
Chapter 17's `ArrayList` specialized. `append` is amortized constant.

**The rule is narrow.** The compiler already turns `a + b + c` into a builder. Only
*loops* need one, because the compiler cannot hoist a builder out of a loop.

**`StringBuffer`** is the synchronized older version and is almost never wanted.

**`compareTo` is code point order**, not alphabetical. All uppercase sorts before
all lowercase; `ä` sorts after `z`. Its *sign* is the meaning, not its magnitude.

**Use a `Collator` for order shown to people**, with a locale. `compareTo` is fine
for internal, consistent-but-arbitrary ordering.

**Normalize text from outside before comparing.** `café` as one code point and as
two are not equal. Normalize at the boundary, once, rather than at every
comparison.

**Case conversion is locale-dependent.** Turkish `i` uppercases to `İ`, so
`"title".toUpperCase()` differs by machine. Use `toUpperCase(Locale.ROOT)` for keys
and protocol strings; the user's locale only for display.

**`split` takes a regular expression.** `split(".")` splits on every character.
Trailing empty fields are discarded unless you pass a limit of −1. Middle empties
are kept.

**Parsing expects failure.** `Integer.parseInt` throws on anything that is not an
integer, including empty strings and `"12.5"`. Real data always contains malformed
lines.

**`Scanner` mixes token and line reading badly.** `nextInt` leaves the newline, so
a following `nextLine` returns empty.

**Use a library for any format with a specification.** CSV with quoted fields is a
state machine, not a `split`. The same goes for JSON, XML, dates, and URLs.

**Regular expressions describe shapes** rather than procedures — a small
declarative language. `matches` requires the whole string; `find` searches within;
parenthesized groups are captured and numbered from 1. Every backslash is doubled
in a Java literal.

**Compile patterns once.** `Pattern.compile` does real work; `String.matches`
repeats it on every call.

**Regular expressions cannot parse nesting.** They describe exactly what a finite
state machine can recognize, and Chapter 6 showed such a machine cannot count
without bound. So no HTML, no JSON, no arithmetic expressions — that needs Chapter
24.

**Formatting has two audiences.** Machines want exactness and stability; people
want local conventions. Confusing them is a recurring bug.

**`printf` puts the layout in one place**, which is why it beats concatenation for
anything with structure — and it is the only way to align columns.

**Display rounding does not fix arithmetic.** `%.2f` rounds the display; the
`double` is unchanged and was never exact. If it must be exact, it should have been
`BigDecimal` or integer cents.

**Locale in formatting.** The default comes from the machine, so `%,.2f` gives
`1.234,50` in Germany. Use `Locale.ROOT` for files and protocols; the user's locale
for display.

**ISO 8601 for dates in data.** `2024-01-15` is unambiguous and sorts correctly as
text. `01/02/2024` means two different days depending on the reader.
