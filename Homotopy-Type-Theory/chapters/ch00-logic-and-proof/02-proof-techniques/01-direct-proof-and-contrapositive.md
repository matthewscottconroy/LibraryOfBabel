# 2.1 Direct Proof and Contrapositive

## The Structure of a Proof

A *proof* is a finite sequence of statements, each justified by one of:
- A hypothesis (an assumed premise)
- An axiom or definition
- A previously proved theorem
- A logical inference rule applied to earlier statements

Every statement must earn its place. "It seems clear that..." is not a justification. "By definition of even numbers..." is. This discipline might feel constraining, but it's what gives proofs their reliability.

Before writing any proof, ask yourself: **what am I trying to prove, and what do I have to work with?** The answer shapes which technique to use.

## Direct Proof

A *direct proof* of a statement $P \to Q$ works like this:
1. Assume $P$ (the hypothesis).
2. Derive $Q$ (the conclusion) from $P$, using definitions, previously known facts, and logical rules.

This is the default proof technique. When you see "prove that if $P$ then $Q$," start by assuming $P$ and working forward.

**Writing convention:** State what you're assuming explicitly at the start ("Assume $P$..."). State what you've concluded at the end ("Therefore $Q$"). The $\square$ symbol (or "QED") marks the end.

**Example 1.** Prove: for all integers $n$, if $n$ is even then $n^2$ is even.

*Proof.* Assume $n$ is an even integer. By definition of "even," there exists an integer $k$ such that $n = 2k$. Then:
$$n^2 = (2k)^2 = 4k^2 = 2(2k^2)$$
Since $2k^2$ is an integer, $n^2 = 2(2k^2)$ is even by definition. $\square$

Let's annotate what happened:
- We invoked the *definition* of "even" to write $n = 2k$.
- We used *algebra* (arithmetic operations preserve integers).
- We invoked the definition again to conclude $n^2$ is even.

Every step is explicit.

**Example 2.** Prove: for all real numbers $a$ and $b$, $(a + b)^2 \geq 4ab$.

*Proof.* The inequality $(a + b)^2 \geq 4ab$ is equivalent to $a^2 + 2ab + b^2 \geq 4ab$, which simplifies to $a^2 - 2ab + b^2 \geq 0$, which is $(a - b)^2 \geq 0$. The last statement is true for all real numbers (any real number squared is non-negative). Since each step was an equivalence (expanding, rearranging), the original inequality holds. $\square$

Note the technique: rewrite the goal into a form that's obviously true.

**Example 3.** Prove: if $p$ and $q$ are both odd integers, then $p + q$ is even.

*Proof.* Assume $p$ and $q$ are odd. By definition of "odd," write $p = 2j + 1$ and $q = 2k + 1$ for integers $j$ and $k$. Then:
$$p + q = (2j + 1) + (2k + 1) = 2j + 2k + 2 = 2(j + k + 1)$$
Since $j + k + 1$ is an integer, $p + q$ is even. $\square$

## Proof by Contrapositive

The *contrapositive* of $P \to Q$ is $\neg Q \to \neg P$. These two statements are logically equivalent:
$$\models (P \to Q) \leftrightarrow (\neg Q \to \neg P)$$
Verify this with a truth table: the two formulas have identical truth tables.

Therefore: to prove $P \to Q$, it suffices to prove $\neg Q \to \neg P$.

**When to use contrapositive:** When $\neg Q$ gives you something useful to work with, and/or when $\neg P$ is what you naturally end up with. Often the contrapositive is cleaner when the hypothesis $P$ is hard to use directly.

**Example 4.** Prove: for all integers $n$, if $n^2$ is odd then $n$ is odd.

*Direct proof attempt:* Assume $n^2$ is odd. We want to show $n$ is odd. From "$n^2$ is odd" it's not immediately obvious how to conclude something about $n$...

*Contrapositive approach:* Prove: if $n$ is even, then $n^2$ is even.

*Proof (by contrapositive).* We prove: if $n$ is even, then $n^2$ is even. Assume $n$ is even; write $n = 2k$. Then $n^2 = 4k^2 = 2(2k^2)$, which is even. $\square$

The contrapositive was Example 1 — we already proved it! The contrapositive approach reduced a new problem to one we'd already solved.

**Example 5.** Prove: for all integers $m, n$, if $mn$ is odd then both $m$ and $n$ are odd.

*Proof (by contrapositive).* We prove: if $m$ is even or $n$ is even, then $mn$ is even.

*Case 1: $m$ is even.* Write $m = 2k$. Then $mn = 2kn = 2(kn)$, which is even.
*Case 2: $n$ is even.* By the same argument (swapping $m$ and $n$), $mn$ is even.

In either case, $mn$ is even. $\square$

(This also illustrates proof by cases, covered more fully in Section 2.3.)

## Knowing When You're Done

A key discipline: recognizing when a proof is complete.

A proof of $P \to Q$ is done when you have derived $Q$ from the assumption $P$ and all intermediate steps are justified. The proof should be *self-contained*: someone who reads only your proof, without knowing the problem, should be able to follow every step.

**Common incompleteness markers:**
- "It is clear that..." (not a justification)
- "Obviously..." (not a justification)
- "One can show that..." (either prove it or cite a theorem)
- "And so we are done" without having stated the conclusion

**Common completeness mistakes:**
- Proving a special case when the general case is needed
- Proving $Q \to P$ when $P \to Q$ was asked
- Using the conclusion in the proof (circular reasoning)

**Example of a flawed "proof":**

*Claim:* For all $n$, $1 + 2 + \cdots + n = \frac{n(n+1)}{2}$.

*"Proof":* This formula clearly holds. Assume $1 + 2 + \cdots + n = \frac{n(n+1)}{2}$. Then the formula holds. $\square$

This is circular: we assumed exactly what we were trying to prove.

## Proof by Direct Calculation

Many proofs in this curriculum consist of a chain of equalities (or inequalities), each step justified by an algebraic rule. The structure is:
$$A = B_1 \quad [\text{by rule 1}]$$
$$= B_2 \quad [\text{by rule 2}]$$
$$= C$$

This *calculational style* is clean and easy to check. Each step should have a brief justification.

**Example 6.** For a group $G$ with elements $a$ and $b$, prove $(ab)^{-1} = b^{-1}a^{-1}$.

*Proof.* We show $b^{-1}a^{-1}$ is the inverse of $ab$ by computing:
$$(ab)(b^{-1}a^{-1}) = a(bb^{-1})a^{-1} \quad [\text{associativity, twice}]$$
$$= a \cdot e \cdot a^{-1} \quad [\text{inverse law: } bb^{-1} = e]$$
$$= aa^{-1} \quad [\text{identity law}]$$
$$= e \quad [\text{inverse law}]$$

Similarly $(b^{-1}a^{-1})(ab) = e$. Since both products equal $e$, and inverses are unique, $(ab)^{-1} = b^{-1}a^{-1}$. $\square$

## The Logical Underpinning

Both direct proof and contrapositive are *proof rules* — specific logical inferences. In natural deduction:

**$\to$-introduction (direct proof):**
$$\frac{\Gamma, P \vdash Q}{\Gamma \vdash P \to Q}$$
"If from the hypotheses $\Gamma$ together with $P$ you can derive $Q$, then from $\Gamma$ alone you can derive $P \to Q$."

**Contrapositive:**
This is not a primitive rule but a derived one, using $\neg$-introduction and $\neg$-elimination. In classical logic, it's straightforward; in intuitionistic logic, one direction is fine ($P \to Q \Rightarrow \neg Q \to \neg P$) but the other ($\neg Q \to \neg P \Rightarrow P \to Q$) requires the double negation law $\neg\neg P \to P$, which is not intuitionistically valid.

This distinction matters in Chapter 5 and in proof assistants: Lean and Coq have constructive logics, and the contrapositive is not always available without extra hypotheses.
