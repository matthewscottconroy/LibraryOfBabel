# People

## Stephen Cole Kleene (1909–1994)

An American mathematician, a student of Alonzo Church, who introduced **regular
expressions** in 1951 — not for text processing, but as a notation for describing
the behavior of the neural-net models McCulloch and Pitts had proposed.

His result, now called Kleene's theorem, is that three things describe exactly the
same set of languages: regular expressions, finite state machines, and regular
grammars. That equivalence is why Section 18.2.2 can assert that a regular
expression cannot match balanced brackets — it is Chapter 6's finite machine
limitation, transferred by the theorem.

The `*` in `a*`, meaning zero or more, is called the Kleene star.

He is one more instance of this book's recurring pattern: a mathematician working
on a question with no application in sight, whose notation becomes a daily tool
twenty years later.

## Ken Thompson (born 1943)

Thompson appeared in Chapter 4 for UTF-8. He appears here for putting Kleene's
notation into a text editor.

His 1968 paper "Regular Expression Search Algorithm" described compiling a regular
expression into machine code that searches efficiently — the first practical
implementation. He built it into the editor `ed`, and from there it spread to
`grep`, `sed`, `awk`, and eventually every programming language.

The command name `grep` comes from the `ed` command `g/re/p` — globally search for
a regular expression and print. That an editor command became a verb tells you how
useful the idea turned out to be.

Worth noting that Thompson's original construction has good worst-case behavior,
and that most modern implementations — including Java's — use a backtracking
approach that is more expressive and can be exponentially slow. The ReDoS
vulnerability of Section 18.2.2 is a consequence of abandoning his algorithm.

## Joshua Bloch (born 1961)

Bloch's fifth appearance, for the argument this chapter's first section makes.

*Effective Java* Item 17, "Minimize mutability", is the general case of `String`'s
design. His five rules for an immutable class — no mutators, no subclassing, all
fields private and final, and defensive copies of any mutable component — are what
Chapter 20 will formalize.

His summary is the sentence worth carrying: *classes should be immutable unless
there is a very good reason to make them mutable.* That is a stronger claim than
most people expect, and the four payoffs in Section 18.1.1 are the case for it.

## Mark Davis (born 1952)

Co-founder of the Unicode Consortium and its president for many years, and a
principal author of the Unicode collation algorithm and the normalization forms
that Section 18.1.3 depends on.

The problems he worked on are the ones this chapter can only warn you about: what
it means for two strings to be "the same" when the same visible text has several
encodings; how to sort when every language has different rules; how to compare
case-insensitively when case is itself language-dependent.

The Turkish-I problem is documented in the Unicode standard precisely because
solving it in general required someone to enumerate every such case across every
writing system. That work is invisible when it succeeds, which is most of the time,
and it is the reason a Java program can handle text in a language its author has
never seen.
