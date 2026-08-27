# Important Researchers

**Alonzo Church** (1903–1995) invented the lambda calculus in the 1930s, and the
`->` in every lambda expression in this chapter descends from his `λ`. His system
had exactly three things — variables, function definition, and function
application — and it turned out to be able to compute anything computable, which
is Chapter 6's Church–Turing thesis. Church appeared in Chapter 13 as Turing's
doctoral supervisor and as the author of the other half of that thesis; this
chapter is where his notation shows up in code you can run. Everything in Section
26.1 is his idea with types added.

**John Backus** (1924–2007) returns from Chapter 24 for the argument rather than
the notation. His 1977 Turing Award lecture, "Can Programming Be Liberated from
the von Neumann Style?", claimed that assignment and iteration were the reason
programs were hard to reason about, and that a language built from composable
functions would be better. He was overstating it, the language he proposed found
no users, and the diagnosis was right — every mainstream language has since grown
the features in this chapter. It is a good example of an argument being valuable
while its proposed remedy is not.

**Guy Steele** (born 1954) returns from Chapter 25. Besides Scheme, he wrote the
Java Language Specification, and was on the committee that added lambdas in Java
8 — thirty-three years after arguing in the Lambda Papers that a well-implemented
function call needs no more machinery than a jump. The decision that a Java lambda
is a functional interface implementation rather than a new kind of value is
consistent with that view: nothing new was added to the runtime, only a notation
for something the language could already express.

**Brian Goetz** (born 1968) returns from Chapter 22 as the architect of Java 8's
lambdas and streams. His design writing on the subject is unusually candid about
the constraints — why function types were rejected, why `invokedynamic` was used
rather than generating classes, why the primitive stream specializations exist
despite being ugly. Reading it is a good corrective to the assumption that
awkward parts of an API are oversights.

**Barbara Liskov** appears once more, briefly, because Section 26.1.3's argument
for pure functions is her argument for abstraction reaching its natural limit: a
function whose behavior depends on nothing but its arguments is the smallest
possible unit you can reason about in isolation.

**Philip Wadler** (born 1956) did the work that made functional programming
practical rather than merely elegant, and Java's generics — Chapter 17 — are
partly his: he co-authored the GJ proposal that became them. He also introduced
monads to programming from category theory, which is the machinery behind
`Optional`, streams, and the `flatMap` operation that both share. If you ever
wonder why `Optional.map` and `Stream.map` feel like the same operation, the
answer is that they are, and Wadler is why anyone noticed.
