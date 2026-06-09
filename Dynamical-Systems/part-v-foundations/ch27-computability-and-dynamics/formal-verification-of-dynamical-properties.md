# 27.5 Formal Verification of Dynamical Properties

We've seen that many natural questions about dynamical systems are undecidable. But undecidability is not a single, monolithic phenomenon — it comes in degrees. The arithmetic hierarchy ($\Sigma_1^0$, $\Pi_1^0$, $\Sigma_2^0$, $\Pi_2^0$, ...) classifies how hard undecidable problems are relative to each other. It turns out that the dynamical properties we care about most — transitivity, minimality, recurrence — land at specific, identifiable levels of this hierarchy.

Understanding where a property sits tells you what kind of computational certificate you'd need to verify it. A $\Sigma_1^0$ property has a finite witness (you can certify it by finding the orbit entering a set). A $\Pi_2^0$ property requires checking infinitely many eventually-true statements. And so on.

**Definition 27.5.1.** A property $P$ of a dynamical system is *$\Pi_1^0$* (or *co-c.e.*) if the set $\{(f, x) : (f,x) \text{ has } P\}$ is a countable intersection of computable open sets.

**Examples of $\Pi_1^0$ properties:**
- "The orbit of $x$ never enters the open set $U$" (requires checking all steps)
- "The system $f$ has no periodic orbit in $U$"
- "The system $f$ is nonexpansive on $U$"

These are "safety" properties in the language of formal verification: something bad never happens. Checking them requires an infinite vigilance — you must verify at every step — and so they can be verified in the limit by a co-c.e. process, but not by a finite computation.

**$\Sigma_1^0$ (c.e.) properties:**
- "The orbit of $x$ eventually enters $U$"
- "The system $f$ has a periodic orbit in $U$"

These are "reachability" properties: something good eventually happens. They have finite certificates — once the orbit enters $U$, you know.

With this language, we can place the standard dynamical properties in their correct complexity slots.

**Theorem 27.5.2 (Hierarchy of Dynamical Properties).** Let $f$ be a computable dynamical system on $[0,1]$:
- Transitivity is $\Pi_3^0$-complete: $h_{\text{top}}(f) > 0$ is $\Pi_1^0$-hard.
- Existence of a dense orbit is $\Sigma_2^0$ (c.e. in the limit).
- Minimality ($=$ all orbits dense) is $\Pi_2^0$.
- The set of recurrent points is $\Pi_2^0$.

Each of these placements is sharp — meaning the property is not only in that class but complete for it. Transitivity requires not just verifying one orbit is dense, but checking a quantified statement ("there exists a dense orbit"), which requires alternating quantifiers. Minimality requires *all* orbits to be dense, adding another quantifier alternation. Recurrence requires a point to return to a neighborhood infinitely often — a $\forall\exists$ statement, hence $\Pi_2^0$.

This hierarchy has practical meaning. If you're trying to formally verify that a specific control system is transitive, you're working on a $\Pi_3^0$-complete problem — provably harder than verifying it has positive entropy, and much harder than verifying it has a periodic orbit. The arithmetic hierarchy is not just a theoretical curiosity; it's a map of the verification landscape.

We'll pick up these threads again in Chapter 32, when we encounter the Borel hierarchy and the descriptive set-theoretic perspective on classification. The arithmetic hierarchy of Section 27.5 and the Borel hierarchy of Chapter 32 are two faces of the same structure — one from below (computability), one from above (definability in a Polish space) — and when they meet, the most important results of modern ergodic theory come into view.
