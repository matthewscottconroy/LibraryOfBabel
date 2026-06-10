# 5.1 Writing Complete Proofs: Standards and Pitfalls

## What Is a Complete Proof?

A proof is complete when a careful reader can verify every step without needing to fill in any gaps themselves. This doesn't mean a proof must be maximally formal — routine algebraic steps don't need line-by-line justification. But every *non-routine* step should be explicit.

A useful test: could you implement this proof in a proof assistant? If yes, it's complete enough. If there are gaps that "seem obvious" but you're not sure how to fill in formally, the proof is incomplete.

**A complete proof includes:**
1. A clear statement of what is being proved
2. Identification of the proof strategy
3. All hypotheses stated when first used
4. All definitions invoked when first applied
5. A justification for every logical step that isn't immediate
6. A clear conclusion (the statement that was to be proved, confirmed)

## Common Pitfalls

### Pitfall 1: Missing Justifications

*Flawed:* "Since $f$ is differentiable, it is continuous, so it achieves its maximum on $[0,1]$."

*Issue:* The step "achieves its maximum on $[0,1]$" requires the Extreme Value Theorem — a non-trivial theorem that should be cited.

*Fixed:* "Since $f$ is differentiable, it is continuous. By the Extreme Value Theorem, a continuous function on a closed bounded interval achieves its maximum."

### Pitfall 2: Confusing $\Rightarrow$ with $\Leftrightarrow$

*Flawed:* "We want to show $A = B$. We have $A \subseteq B$, so $A = B$."

*Issue:* $A \subseteq B$ alone doesn't give $A = B$; you also need $B \subseteq A$.

*Fixed:* "We show $A \subseteq B$ and $B \subseteq A$. [Proofs of both inclusions.] Therefore $A = B$."

### Pitfall 3: Circular Reasoning

*Flawed:* "To show $P$, assume $P$. Then clearly $P$ holds."

Circular reasoning occurs when the conclusion appears as an assumption (directly or indirectly). It's invalid even when the final conclusion happens to be true.

Subtle form: proving $A \to B$ by assuming $B$ and concluding $A$ would establish the original if the converse were true — but you haven't proved the converse, you've just assumed it.

### Pitfall 4: Incorrect Use of Induction

**Forgetting the base case:**

*Claim:* All horses are the same color.

*"Proof":* By induction on the number of horses $n$.

*Base case ($n = 1$):* A single horse has the same color as itself. ✓

*Inductive step:* Assume any $n$ horses have the same color. Given $n + 1$ horses $h_1, \ldots, h_{n+1}$:
- The first $n$ horses $h_1, \ldots, h_n$ are all the same color (by IH).
- The last $n$ horses $h_2, \ldots, h_{n+1}$ are all the same color (by IH).
- Both groups share $h_2, \ldots, h_n$, so all $n + 1$ horses are the same color. ✓

The flaw: for $n = 1 \to n = 2$, the "both groups share $h_2, \ldots, h_n$" is empty (no overlap between $\{h_1\}$ and $\{h_2\}$). The base case $n = 2$ is not proved — and it's false.

**Assuming what you want to prove in the IH:**

Some students write the inductive step as "assume $P(n+1)$, prove $P(n+1)$" — which is trivially true but proves nothing.

### Pitfall 5: Quantifier Errors

*Flawed:* "For all $x$, if $P(x)$ then $Q(x)$. We know $P(x)$. Therefore $Q(x)$."

*Issue:* "$P(x)$" with a free variable $x$ is not a statement — it depends on which $x$ we're talking about. You must either say "for the specific $x = t$, we know $P(t)$" or keep the quantifier.

*Fixed:* "For all $x$, if $P(x)$ then $Q(x)$. Let $a$ be a specific element with $P(a)$. By the universal statement, $Q(a)$."

### Pitfall 6: Division by Zero, Undefined Operations

*Flawed proof that $1 = 2$:*
Let $a = b = 1$.  
$a^2 = ab$  
$a^2 - b^2 = ab - b^2$  
$(a+b)(a-b) = b(a-b)$  
$a+b = b$  
$2 = 1$

*Flaw:* Dividing both sides by $a - b = 0$. Division by zero is not a valid operation.

### Pitfall 7: Implicit Existence Assumptions

*Flawed:* "Let $x$ be the maximum of $f$. Then..."

*Issue:* Does the maximum exist? It might not if the domain is unbounded or the function is unbounded.

*Fixed:* Either prove the maximum exists first, or cite a theorem (like the Extreme Value Theorem) that guarantees existence, or add an assumption that the maximum exists.

## Writing Style for Proofs

**Be direct.** State what you're doing before you do it. "We prove by induction" before the induction begins. "We consider two cases" before the cases. "We apply the first isomorphism theorem" before the calculation.

**Use complete sentences.** Mathematical symbols are shorthand for words. $\forall n, P(n)$ should be read aloud as "for all $n$, $P(n)$ holds." A proof full of symbol strings without prose is hard to read.

**Signal logical structure.** Words like "since," "therefore," "thus," "because," "it follows that," "hence," "we conclude" are not mere filler — they signal the direction of reasoning (forward vs. backward, assumption vs. conclusion).

**Label your steps.** In a complex proof, label key equations or inequalities: "By equation $(*)$..." — so a reader can trace references.

**Be precise about quantification.** "For any $\epsilon > 0$..." means the following statement holds for all positive $\epsilon$. Don't write "for some $\epsilon > 0$..." when you mean "for all."

## The Standard Proof Template

For a typical theorem of the form "If [hypotheses], then [conclusion]":

---
*Proof.* [State the strategy: "We proceed by direct proof / induction / contradiction / contrapositive."]

[If direct:] Assume [hypotheses]. 

[Work through the argument, one step at a time, with brief justifications for non-obvious steps.]

Therefore, [conclusion]. $\square$

---

For an "if and only if" statement, do both directions separately:

---
*Proof.* 

($\Rightarrow$) Assume [left side]. [Derive right side.]

($\Leftarrow$) Assume [right side]. [Derive left side.]

$\square$

---

## The Connection to Formal Proofs

The standards described here are the informal version of what proof assistants demand formally. In Lean 4 (Chapter 21), every proof step must be an explicit application of a rule or tactic. There are no "obvious" steps.

The discipline of writing rigorous informal proofs is the best preparation for formal proof assistants. If you can articulate why every step in your informal proof is valid, you can translate it to Lean. If you cannot, there are hidden gaps to resolve.

Conversely, failing at informal rigor often looks like: "I know the theorem is true, but I can't figure out how to formalize it." This usually means the informal proof has a gap that the formalization exposed.

Practice: take one of your proofs and ask, for every step, "Why does this follow?" If you can answer with a definition, a lemma, or an axiom, the step is justified. If the answer is "it's just obvious," dig deeper.
