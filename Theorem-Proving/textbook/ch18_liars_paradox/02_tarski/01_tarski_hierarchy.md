# Tarski's Hierarchy of Languages

Tarski drew the classical moral from the undefinability theorem: the fault lies with **semantically closed languages** — languages containing names for their own sentences *and* their own truth predicate. His remedy (1933/1956): truth for a language $L$ can be rigorously defined, but only in a richer **metalanguage**. No language defines its own truth.

## Object Language and Metalanguage

- The **object language** $L$ is the language under study (say, first-order arithmetic).
- The **metalanguage** $M$ is the language in which we study it. $M$ must contain (translations of) all sentences of $L$, plus the syntax of $L$ (names $\ulcorner\phi\urcorner$ for $L$-expressions), plus resources $L$ lacks — in particular a predicate $True_L$.

Tarski demanded that any proposed definition of $True_L$ in $M$ meet a precise adequacy condition:

**Convention T (material adequacy).** A definition of $True_L(x)$ in $M$ is *materially adequate* iff for every sentence $\phi$ of $L$, $M$ proves the T-biconditional
$$True_L(\ulcorner\phi\urcorner) \leftrightarrow \phi^*,$$
where $\ulcorner\phi\urcorner$ is a structural-descriptive name of $\phi$ and $\phi^*$ is $\phi$'s translation into $M$.

The canonical instance: "*'Snow is white' is true iff snow is white.*" Convention T is not itself a definition of truth — it is the *test* any definition must pass. By Tarski's theorem, no definition inside $L$ can pass it (on pain of inconsistency); a definition one level up can.

## The Hierarchy

Iterating the object/metalanguage split yields an infinite tower
$$L_0 \subset L_1 \subset L_2 \subset \cdots$$
where $L_0$ is a truth-free base language and $L_{n+1} = L_n + T_n$, a truth predicate **typed** to apply only to sentences of $L_n$. Grammar enforces the typing: $T_n(\ulcorner\phi\urcorner)$ is well-formed only when $\phi \in L_n$.

**How the Liar is blocked.** A Liar for level $n$ would be a sentence $\lambda$ with $\lambda \leftrightarrow \neg T_n(\ulcorner\lambda\urcorner)$. But $\lambda$ contains $T_n$, so $\lambda \in L_{n+1} \setminus L_n$ — and then $T_n(\ulcorner\lambda\urcorner)$ is ill-formed (or simply false, on the liberal reading where $T_n$ holds of nothing outside $L_n$). The diagonal lemma needs the predicate being diagonalized to apply to the diagonal sentence itself; the typing discipline denies exactly this, so **the fixed-point equation has no solution at any level**. "This sentence is false" is not expressible; "this $L_n$-sentence is false" is expressible in $L_{n+1}$, where it is a well-behaved (false) claim about $L_n$. No sentence is the Liar *simpliciter*.

## The Positive Legacy: Truth Defined Compositionally

Tarski did not merely forbid; he showed how to *construct* an adequate truth definition one level up, via **satisfaction** relative to variable assignments. For first-order $L$ interpreted in a structure $\mathfrak{M}$ with assignment $s$ (Chapters 3 and 9):

$$
\begin{array}{ll}
\mathfrak{M} \models R(t_1,\dots,t_k)[s] & \text{iff } (t_1^{\mathfrak{M}}[s], \dots, t_k^{\mathfrak{M}}[s]) \in R^{\mathfrak{M}} \\
\mathfrak{M} \models \neg\phi[s] & \text{iff } \mathfrak{M} \not\models \phi[s] \\
\mathfrak{M} \models \phi \wedge \psi[s] & \text{iff } \mathfrak{M} \models \phi[s] \text{ and } \mathfrak{M} \models \psi[s] \\
\mathfrak{M} \models \exists x\, \phi[s] & \text{iff } \mathfrak{M} \models \phi[s(x \mapsto a)] \text{ for some } a \in |\mathfrak{M}|
\end{array}
$$

A sentence is **true in $\mathfrak{M}$** iff every (equivalently, some) assignment satisfies it. The recursion on formula complexity is legitimate precisely because it is carried out in a metalanguage with enough set theory to handle assignments. This definition — routine to us — is the foundation of all of model theory (Chapter 9); the semantic clauses for first-order logic in Chapter 3 *are* Tarski's construction.

**Worked example.** Let $L_0$ be the language of arithmetic and $M = L_1$ a language with set-theoretic resources. Define $True_0(x)$ := "$x$ codes an $L_0$-sentence satisfied in $(\mathbb{N}, 0, S, +, \times)$ by all assignments," unwinding the clauses above. For each fixed $\phi$, $M$ proves $True_0(\ulcorner\phi\urcorner) \leftrightarrow \phi$: Convention T is met. Within arithmetic itself one can define *partial* truth predicates $True_{\Sigma_n}$ for sentences of bounded quantifier complexity — the hierarchy reappears in miniature inside a single theory, and full truth is exactly the unreachable union.

## The Costs

1. **No universal language.** There is no level at which one can say "every sentence is true or false," or quantify over truth at all levels. Science and philosophy aspire to a universal medium; Tarski's tower forbids one. To say "everything Aristotle said is true" one needs a $T_n$ with $n$ above the level of everything Aristotle said.
2. **English "true" is not typed.** The English predicate *true* appears univocal — we do not detect a subscript when we use it. The hierarchy looks like an artifact of formalization, not a description of natural language.
3. **Kripke's objection: levels cannot be assigned in advance.** Suppose Dean says "everything Nixon says about Watergate is false," and Nixon says "everything Dean says about Watergate is false." On Tarski's account Dean's utterance must carry a level strictly above the level of *all* Nixon's Watergate utterances — one of which is Nixon's quoted remark, whose level must in turn exceed the level of *all* of Dean's, including the first. There is no consistent assignment of fixed levels, yet the pair of utterances is not intuitively meaningless — indeed if Nixon said at least one other false Watergate thing and Dean at least one other true one, both utterances arguably receive determinate values. Level-subscripts, if they exist, must depend on empirical facts — which the typed grammar cannot anticipate.

Quine's related notion of **semantic ascent** — shifting from talk of things to talk of words ("'snow is white' is true" instead of "snow is white") — is benign and ubiquitous; Tarski's point is that each ascent strictly enlarges the language, without end.

## Hierarchies in Proof Assistants

Tarski's stratification is not merely a philosophical dodge; it is standard engineering. In Lean or Coq one *defines* the satisfaction relation $\mathfrak{M} \models \phi$ for a deep-embedded object logic as a recursive function — Tarski's clauses, formalized — but the definition lives in the ambient type theory, one level up from the embedded logic. Lean's metaprogramming framework manipulates object-level syntax (`Expr`) from a metalevel, and *reflection* proves object-level goals by verified computation on their codes: a disciplined, benign use of exactly the object/meta divide. Even the universe hierarchy $\mathsf{Type}\ 0 : \mathsf{Type}\ 1 : \cdots$ (Chapter 11) is a Tarskian tower: collapsing it ($\mathsf{Type} : \mathsf{Type}$) yields Girard's paradox, type theory's Liar. The hierarchy solution "works" — the open question is only whether natural language, which seems to be semantically closed, can be so stratified. Kripke's construction, next, tries to do better.

## Exercises
See [problems/ch18_liars_paradox/](../../../problems/ch18_liars_paradox/)
