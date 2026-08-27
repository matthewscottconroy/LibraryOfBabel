# Important Concepts

**Alphabet, string, $\Sigma^*$** — a finite set of symbols, a finite sequence of
them, and the infinite set of all such sequences. The empty string $\varepsilon$
is a real string, not an absence.

**A language is a subset of $\Sigma^*$** — a set of strings, with no reference to
meaning. Java is the set of character sequences that are legal Java programs; what
they *do* is a separate question.

**Recognizer and generator** — the two kinds of finite description of a possibly
infinite language. A machine that answers yes or no, and a set of rules that
produces exactly the members.

**More languages than descriptions** — $\Sigma^*$ is countable but its subsets are
not, so some languages have no grammar and no recognizer. Chapter 34 exhibits a
natural one.

**The Chomsky hierarchy** — regular, context-free, context-sensitive,
unrestricted, each needing more machine than the last. Two matter here: words are
regular, nesting is context-free.

**Why tokenizing and parsing are separate** — not a convention but a consequence.
Finite memory suffices for words; nesting provably requires a stack.

**Grammar** — a finite set of rules generating a language. Terminals appear in the
output; non-terminals exist to be replaced; one non-terminal is the start symbol.

**Derivation** — beginning at the start symbol and replacing non-terminals by
their rules until only terminals remain. A string is in the language exactly when
some derivation produces it.

**Recursion in a grammar** — a non-terminal appearing in its own rule, which is
what lets four lines describe infinitely many strings. It needs a non-recursive
alternative as a base case, for the same reason a recursive method does.

**BNF and EBNF** — the standard notations. EBNF's `*`, `+` and `?` are shorthand
for extra recursive rules, and `*` becomes a loop in a recursive-descent parser.

**Grammars describe syntax only** — a grammar generates `1 / 0` and an undeclared
variable happily. Checking meaning is a separate pass, and the boundary is a real
design decision.

**Ambiguity** — a grammar permitting two distinct trees for one string. Fatal,
because meaning then depends on which derivation the parser found.

**Fixing ambiguity by layering** — one grammar level per precedence tier, loosest
binding at the top. `expression → term → factor`. Precedence comes out of the
structure, not from an annotation.

**Associativity** — which way a chain of same-precedence operators groups.
Left-associativity comes from the parser's loop putting the accumulated tree on the
left; right-associativity comes from recursion instead.

**The dangling else** — the classic ambiguity, resolved by rule rather than by
grammar in nearly every language: the `else` binds to the nearest unmatched `if`.
Braces make it moot.

**Token** — a classified piece of text, with a kind and its original characters. An
enum and a record, and a real compiler adds line and column.

**Sentinel** — an `END` token after the last real one, so the parser can always
ask for a next token. A special case turned into a normal one.

**Maximal munch** — take the longest character run that could be a token, which is
why `123` is one number.

**Discarding whitespace** — the line that makes a language free-format. Python's
tokenizer does not, which is the whole difference.

**Abstract syntax tree** — the structure with the notation removed. Parentheses
change its shape and leave no node behind.

**Sealed interface plus records** — the declaration that an expression is exactly
one of a fixed set of shapes. The syntax tree is the example algebraic data types
were designed for.

**The grammar's recursion becomes the type's recursion** — `Bin` holds two `Expr`
because the rule mentions `expression` on both sides. The type transcribes the
grammar.

**Walking a tree** — switch on the shape, base cases return, recursive cases
combine results from the children. Every operation on a tree has this skeleton,
including the evaluator of Chapter 25.

**Exhaustive switch on a sealed type** — no `default`, so adding a shape turns
every walker into a compile error listing what to update.

**Concrete versus abstract trees** — a concrete tree records every rule applied and
every bracket, and is what a formatter or refactoring tool needs. Interpreters want
abstract.

**Recursive descent** — one method per grammar rule, descending through the
precedence levels, recursing where the grammar does. The method most hand-written
parsers use.

**Lookahead and LL(1)** — deciding which branch to take by examining one token.
More lookahead means more bookkeeping, and one is usually enough.

**The parser's stack is the call stack** — the theory demands unbounded stack for
nesting, and recursion supplies it. Deep enough nesting produces
`StackOverflowError`, which is the theory made visible.

**expect(END)** — insisting the whole input was consumed. Without it, trailing
garbage is silently ignored.

**Left recursion** — a rule beginning with itself compiles to infinite recursion.
Rewrite as iteration with `*`, which is the standard transformation.
