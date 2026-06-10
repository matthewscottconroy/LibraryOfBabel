# 3.1 Detours, Normal Forms, and the Normalization Theorem

## The Problem with Raw Proofs

Natural deduction lets us construct proofs freely. But this freedom means we can write *redundant* proofs — proofs that do unnecessary work.

The quintessential redundancy is a *detour*: an introduction immediately followed by the corresponding elimination. For example:

**A conjunction detour:**
$$\frac{\Gamma \vdash A \quad \Gamma \vdash B}{\Gamma \vdash A \wedge B} \wedge\text{I} \quad \vdash A \quad \wedge\text{E}_1$$

We proved $A \wedge B$ from proofs of $A$ and $B$, then immediately extracted $A$. But we already had a proof of $A$! The pair-formation and first-projection cancel each other.

**An implication detour:**
$$\frac{\dfrac{[\![A]\!] \quad \cdots \quad B}{A \to B}\to\text{I} \quad A}{B}\to\text{E}$$

We proved $A \to B$ (by assuming $A$ and deriving $B$), then applied it to a proof of $A$ to get $B$. But we could have derived $B$ directly by substituting the proof of $A$ for the assumption.

These detours are *roundtrips* that introduce then immediately eliminate a connective.

## Beta Reductions: Removing Detours

Each connective has a corresponding **β-reduction** (also called a *local reduction* or *detour elimination*):

**Conjunction (β-reduction for $\wedge$):**
$$\frac{\Gamma \vdash A \quad \Gamma \vdash B}{\Gamma \vdash A \wedge B} \wedge\text{E}_1 \;\Rightarrow_\beta\; \Gamma \vdash A$$

The detour reduces to just using the original proof of $A$.

**Implication (β-reduction for $\to$):**
$$\frac{[\![A]\!]^u \;\cdots\; B}{A \to B}\to\text{E on } A \;\Rightarrow_\beta\; \text{substitute proof of }A\text{ for }[A]^u\text{ in derivation of }B$$

This is *substitution*: the proof of $A \to B$ (which assumed $A$ and derived $B$) is "unrolled" by substituting the actual proof of $A$ for every use of the assumption.

**Disjunction (β-reduction for $\vee$):**
$$\frac{\text{left injection of } A}{\text{case analysis on } A \vee B: \text{case }A\text{ yields }C, \text{ case }B\text{ yields }C}\;\Rightarrow_\beta\; \text{case }A\text{ applied to the proof of }A$$

Since we know which disjunct holds, we don't need to handle both cases — just use the appropriate branch.

**Universal quantifier (β-reduction for $\forall$):**
$$\frac{[\forall\text{I over }a]}{\forall\text{E at }t}\;\Rightarrow_\beta\; \text{substitute }t\text{ for }a\text{ in the derivation}$$

**The Curry-Howard correspondence.** Under the correspondence:
- Proofs are programs.
- β-reduction is computation (reducing a redex, applying a function to an argument).
- Normalization is computation to a value.

The detour-elimination process corresponds exactly to the evaluation of programs in the λ-calculus!

## Eta Reductions: Removing Expansions

There's a dual kind of redundancy: an elimination immediately followed by the corresponding introduction. These are called **η-reductions**:

**Conjunction (η-reduction for $\wedge$):**
$$\text{If }\Gamma \vdash A \wedge B\text{, then } \frac{\Gamma \vdash A \quad \Gamma \vdash B}{\Gamma \vdash A \wedge B}$$
where the proofs of $A$ and $B$ are obtained by $\wedge$E$_1$ and $\wedge$E$_2$ from the original. This is a no-op: you're taking apart a pair just to reassemble it.

**Implication (η-reduction for $\to$):**
$$\text{If }\Gamma \vdash A \to B\text{, then } \lambda a. (f\ a) \;\Rightarrow_\eta\; f$$

where $f : A \to B$ and $\lambda a. (f\ a)$ is "the function that takes $a$ and applies $f$ to it" — which is just $f$.

η-reductions are less commonly discussed in logic but central in type theory: they enforce *extensionality* (two functions that produce the same output for every input are equal).

## Normal Forms

**Definition.** A derivation is in *normal form* if it contains no β-redexes (no detours).

**Definition.** A derivation is in *β-normal form* if no β-reduction can be applied.

**Theorem (Normalization, Prawitz 1965).** Every derivation in natural deduction for intuitionistic propositional logic can be reduced to a normal form by a finite sequence of β-reductions.

More precisely:
- **Weak normalization:** Every derivation has at least one reduction sequence that terminates in a normal form.
- **Strong normalization:** Every reduction sequence from any derivation terminates in a normal form (there are no infinite reduction sequences).

The strong normalization theorem is the more powerful result. It says the proof-simplification process always terminates, no matter how you choose to simplify.

## The Subformula Property

The normalization theorem has a beautiful structural consequence.

**Theorem (Subformula Property).** In a normal form derivation, every formula appearing in the derivation tree is a subformula of the conclusion or of one of the undischarged hypotheses.

A *subformula* of $\varphi$ is $\varphi$ itself or any formula appearing as a component (e.g., $A$ and $B$ are subformulas of $A \wedge B$).

**Why this is remarkable.** A proof of $A \to B$ from no hypotheses, in normal form, can only contain formulas built from $A$ and $B$. No "foreign" formulas appear. Proofs are *analytic*: they analyze the conclusion into its components, not by invoking external machinery.

**Corollary (Consistency).** The system is consistent: $\bot$ is not provable from no hypotheses.

*Proof.* Suppose $\vdash \bot$. Normalize the proof. By the subformula property, every formula in the proof is a subformula of $\bot$. But $\bot$ has no proper subformulas, and $\bot$ itself has no introduction rule. So the normal form proof must have a leaf — but leaves are identity axioms, which require a hypothesis. With no hypotheses and no axioms to derive $\bot$ from, the proof can't exist. $\square$

**Corollary (Decidability of propositional logic).** Whether $\Gamma \vdash \varphi$ is decidable for propositional logic, by searching among proofs of bounded complexity (bounded by the size of $\varphi$ and $\Gamma$, using the subformula property).

## Normal Form Proofs Look Like "Backward Reasoning"

Here's an intuitive picture of what normal form proofs look like.

A normal form proof has two phases:
1. **The elimination phase (downward from hypotheses):** Apply elimination rules to the hypotheses, extracting information from compound formulas.
2. **The introduction phase (upward to the conclusion):** Use the extracted information to build up the conclusion using introduction rules.

In a normal form proof, you never go "up and then down" — you don't introduce a connective that you'll later eliminate. The proof is directed: decompose what you have, then build what you need.

This gives proofs a "diamond shape": wide in the middle (where you're working with atomic formulas and small subformulas), narrow at the top (hypotheses) and bottom (conclusion).

## Connection to Computation

The normalization theorem is the first major bridge between logic and computation.

Under the Curry-Howard correspondence:
- A proof of $A$ is a program of type $A$.
- A β-redex in a proof is a *reducible expression* (redex) in a program.
- Normalizing a proof is *running a program* to completion (computing its value).
- The strong normalization theorem says all programs terminate (in the simply typed λ-calculus, which corresponds to intuitionistic propositional logic).

**Strong normalization = termination of typed λ-calculus.**

This is not true for untyped λ-calculus: the term $(\lambda x. x x)(\lambda x. x x)$ reduces to itself (it's the prototypical non-terminating program, $\Omega$). Adding types prevents this self-application and guarantees termination.

But termination comes at a cost: not all computable functions can be expressed in the simply-typed λ-calculus. The Ackermann function, for example, is computable but not definable in STLC. To get more expressive type systems, we need dependent types — at the cost of losing decidability of type checking in some settings.

## The Normalization Hierarchy

Different logical systems have different normalization properties:

| System | Logic | Normalization |
|--------|-------|---------------|
| Simply typed λ-calculus | Intuitionistic propositional | Strong normalization |
| System F (polymorphism) | Second-order propositional | Strong normalization |
| Gödel's System T | Arithmetic | Strong normalization |
| Martin-Löf Type Theory | Dependent types | Strong normalization (with restrictions) |
| Coq's Calculus of Constructions | Higher-order dep. types | Strong normalization |

Strong normalization is the "gold standard" for a well-behaved type theory: it means every computation terminates, every program has a value, and there are no infinite loops.

When we add classical axioms (like double negation elimination), normalization becomes more subtle — classical logic corresponds to programs with control operators (call/cc), and reduction can diverge in some settings.

## Eta-Expansion and Extensionality

We mentioned η-reductions above. In the other direction, we have η-expansions: adding introductions after eliminations.

**Eta-expansion for $\to$:** If $\Gamma \vdash f : A \to B$, replace $f$ by $\lambda a. f(a)$.

η-expansions produce η-long normal forms (also called "fully η-expanded" terms). In η-long normal form, every function-typed term is explicitly a λ-abstraction, and every product-typed term is explicitly a pair.

η-long normal forms are important in proof theory because they make the structure of a proof maximally explicit. They're also important in higher-order unification and in the theory of categorical models of type theory.

In HoTT, the distinction between η-short and η-long forms becomes particularly interesting because of the presence of function extensionality (which says two functions that agree on all inputs are equal, i.e., η-expansion is an equality, not just a definitional reduction).

## Normalization and Totality

The strong normalization theorem for type theory says: every well-typed term reduces to a value. This is the type-theoretic formulation of *totality*: all programs terminate, and every proof is valid.

This is why proof assistants like Lean, Coq, and Agda have strict termination checking: they verify that all recursive functions are structurally decreasing (or otherwise provably terminating), ensuring strong normalization holds for the whole system.

If a proof assistant allowed non-terminating programs, you could "prove" anything: a non-terminating proof of $\bot$ would be indistinguishable from a legitimate proof, and the system would be inconsistent.

We'll return to this when we study the Calculus of Constructions and MLTT in Chapters 10–11.
