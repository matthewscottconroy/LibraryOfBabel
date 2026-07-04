# CTL*: The Full Branching-Time Logic

LTL evaluates formulas along a single execution and quantifies over executions only implicitly, once, at the top. CTL welds every temporal operator to a path quantifier. CTL* removes both restrictions: the path quantifiers $A$ and $E$ and the temporal operators $X, U, F, G$ combine freely. The result is the logic in which LTL and CTL both live as syntactic fragments — and the setting in which their expressive powers can finally be compared on equal terms.

## State Formulas and Path Formulas

The central syntactic device of CTL* is a mutual recursion between two sorts of formulas. **State formulas** are true or false at a *state*; **path formulas** are true or false of an *infinite path*.

**Definition (CTL* syntax).** Over a set $AP$ of atomic propositions:

$$
\begin{aligned}
\text{state formulas}\quad \Phi &::= p \mid \neg \Phi \mid \Phi_1 \land \Phi_2 \mid E\,\varphi \mid A\,\varphi \\
\text{path formulas}\quad \varphi &::= \Phi \mid \neg \varphi \mid \varphi_1 \land \varphi_2 \mid X\,\varphi \mid \varphi_1\, U\, \varphi_2
\end{aligned}
$$

with $p \in AP$, and the usual abbreviations $F\varphi \equiv \top\, U\, \varphi$, $G\varphi \equiv \neg F \neg \varphi$, $A\varphi \equiv \neg E \neg \varphi$.

Two features do all the work. First, every state formula counts as a path formula (evaluated at the path's first state), so quantifiers and temporal operators can nest arbitrarily. Second, path formulas may stack temporal operators *without* an intervening quantifier — $FG\,p$, $GF\,p \to GF\,q$ — which is exactly what CTL forbids.

**Definition (CTL* semantics).** Let $M = (S, \to, L)$ be a Kripke structure with a total transition relation, $\pi = s_0 s_1 s_2 \ldots$ an infinite path, and $\pi^i$ the suffix starting at $s_i$. Then:

- $M, s \vDash p$ iff $p \in L(s)$; Boolean clauses are standard;
- $M, s \vDash E\,\varphi$ iff $M, \pi \vDash \varphi$ for **some** path $\pi$ starting at $s$;
- $M, s \vDash A\,\varphi$ iff $M, \pi \vDash \varphi$ for **every** path $\pi$ starting at $s$;
- $M, \pi \vDash \Phi$ (a state formula) iff $M, s_0 \vDash \Phi$;
- $M, \pi \vDash X\,\varphi$ iff $M, \pi^1 \vDash \varphi$;
- $M, \pi \vDash \varphi\, U\, \psi$ iff $\exists j \geq 0.\ M, \pi^j \vDash \psi$ and $M, \pi^k \vDash \varphi$ for all $k < j$.

A shallow embedding in Lean 4 makes the two-sorted design vivid — the quantifiers are precisely the coercions between the two sorts:

```lean
variable {S : Type} (R : S → S → Prop)

def Path (S : Type) := ℕ → S
def isRun (π : Path S) : Prop := ∀ i, R (π i) (π (i + 1))

-- State formulas are predicates on states; path formulas on paths
def StateFormula (S : Type) := S → Prop
def PathFormula (S : Type) := Path S → Prop

-- The path quantifiers convert one sort into the other
def EQuant (φ : PathFormula S) : StateFormula S :=
  fun s => ∃ π, π 0 = s ∧ isRun R π ∧ φ π

def AQuant (φ : PathFormula S) : StateFormula S :=
  fun s => ∀ π, π 0 = s → isRun R π → φ π
```

## LTL and CTL as Syntactic Fragments

**Definition (LTL fragment).** An LTL formula is a path formula containing no $A$ or $E$. An LTL specification $\varphi$ corresponds to the CTL* state formula $A\,\varphi$: the property must hold on all executions.

**Definition (CTL fragment).** A CTL formula is a state formula in which every temporal operator is immediately preceded by a path quantifier: the only permitted blocks are $AX, EX, A U, E U$ (hence the derived $AF, EF, AG, EG$).

Examples: $A\,FG\,p$ ("on every path, eventually $p$ holds forever") is CTL* but not CTL, since the block $FG$ contains no inner quantifier. $AG\,EF\,p$ ("from every reachable state, $p$ remains reachable") is CTL. $A(GF\,\mathit{run} \to GF\,\mathit{done})$, a fairness implication, is the $A$-closure of an LTL formula.

## Expressiveness: The Strict Hierarchy

The two fragments are *incomparable*, and both are strictly weaker than CTL*.

**Theorem (Emerson-Halpern, 1986).** No CTL formula is equivalent to the LTL formula $FG\,p$ (that is, to the CTL* formula $A\,FG\,p$).

*Proof sketch.* First see why the natural candidate fails. The closest CTL formula is $AF\,AG\,p$, and it is strictly stronger. Consider the structure $M$:

$$
s_0 \{p\} \circlearrowleft, \qquad s_0 \to s_1 \{\ \}, \qquad s_1 \to s_2 \{p\} \circlearrowleft
$$

($s_0$ and $s_2$ carry $p$ and have self-loops; $s_1$ does not carry $p$.) Every path from $s_0$ either stays at $s_0$ forever or eventually reaches $s_2$ and stays; either way $FG\,p$ holds, so $M, s_0 \vDash A\,FG\,p$. But $AG\,p$ holds only at $s_2$ (from $s_0$ the state $s_1$ is reachable), and the path $s_0 s_0 s_0 \cdots$ never reaches $s_2$; hence $M, s_0 \nvDash AF\,AG\,p$. So the candidate translation is wrong. The full theorem rules out *every* CTL formula: given a purported CTL equivalent $\Phi$, one chains copies of this escape gadget into a family of structures and shows by induction on $\Phi$ that, for chains deeper than $|\Phi|$, $\Phi$ takes the same value on a pair of structures that disagree on $A\,FG\,p$. $\square$

**Theorem (Clarke-Draghicescu, 1988).** No LTL formula is equivalent to the CTL formula $AG\,EF\,p$.

*Proof (the two-structure trace argument).* LTL satisfaction is determined by the set of traces: $M \vDash A\varphi$ iff every trace of $M$ satisfies $\varphi$. Now take $M_1$ with states $s \{\ \}$ and $t \{p\}$ and transitions $s \to s$, $s \to t$, $t \to t$; and $M_2$ with the single state $s \{\ \}$ and $s \to s$. Then $M_1, s \vDash AG\,EF\,p$ (from every reachable state, $t$ is reachable), while $M_2, s \nvDash AG\,EF\,p$. But $\mathrm{Traces}(M_2) = \{\emptyset^\omega\} \subseteq \mathrm{Traces}(M_1)$. If some LTL formula $\varphi$ were equivalent to $AG\,EF\,p$, then from $M_1 \vDash A\varphi$ every trace of $M_1$ satisfies $\varphi$; hence so does $M_2$'s only trace; hence $M_2 \vDash A\varphi$, i.e. $M_2 \vDash AG\,EF\,p$ — contradiction. $\square$

The moral: LTL cannot distinguish trace-equivalent structures, while CTL can; CTL cannot follow one path through nested modalities, while LTL can. (Clarke and Draghicescu also give the sharp criterion: a CTL formula is LTL-expressible iff it is equivalent to the LTL formula obtained by deleting its path quantifiers.)

**Corollary (Strict hierarchy).** $\mathrm{LTL} \subsetneq \mathrm{CTL}^*$ and $\mathrm{CTL} \subsetneq \mathrm{CTL}^*$, and LTL, CTL are incomparable. Both inclusions are witnessed inside CTL*: $A\,FG\,p \in \mathrm{CTL}^* \setminus \mathrm{CTL}$ and $AG\,EF\,p \in \mathrm{CTL}^* \setminus \mathrm{LTL}$; the intersection is nonempty ($AG\,p = A\,G\,p$). Above CTL* sits the modal $\mu$-calculus, into which CTL* translates (Dam, 1994).

## Model-Checking Complexity

Expressiveness is bought with complexity — but the price is paid only in the formula, never in the model.

| Logic | Model checking | Algorithm | Satisfiability |
|-------|----------------|-----------|----------------|
| CTL   | PTIME | $O(|M| \cdot |\phi|)$ (Clarke-Emerson-Sistla, 1986) | EXPTIME-complete |
| LTL   | PSPACE-complete (Sistla-Clarke, 1985) | $O(|M| \cdot 2^{|\phi|})$ via Büchi automata (Vardi-Wolper, 1986) | PSPACE-complete |
| CTL*  | PSPACE-complete (Emerson-Lei, 1987) | $O(|M| \cdot 2^{|\phi|})$ | 2EXPTIME-complete |

Two readings of this table matter in practice. First, CTL* model checking costs *no more than LTL's*: the Emerson-Lei algorithm recursively evaluates maximal state subformulas with the CTL labelling algorithm and hands the remaining path formulas to an LTL checker. Adding branching quantifiers to LTL is free. Second, in all three logics the dependence on $|M|$ is linear, and specifications are short while models are astronomically large — so the exponent $2^{|\phi|}$ is usually harmless, and the real fight is against $|M|$ (the state-explosion problem of the next section).

## Choosing a Logic in Practice

- **Choose LTL** when properties concern executions: protocols, software, anything where a counterexample should be a single readable trace. Fairness assumptions ($GF\,e \to GF\,x$) can be written *inside* the formula — impossible in CTL — and trace semantics supports compositional assume-guarantee reasoning. SPIN and TLA+ are linear-time; the industrial property languages PSL and SVA have linear-time cores.
- **Choose CTL** when you need possibility properties — $AG\,EF\,\mathit{reset}$, "a recovery state is always reachable" — or when the $O(|M|\cdot|\phi|)$ algorithm and its smooth BDD-based symbolic implementation matter, as in hardware verification with NuSMV/nuXmv.
- **CTL***, finally, is less an implementation target than the *lingua franca*: the logic in which the LTL-versus-CTL comparison can even be stated, and the natural home of specifications mixing fairness (linear) with reachability (branching), e.g. $AG\,EF\,\mathit{reset} \land A(GF\,\mathit{sched} \to GF\,\mathit{progress})$.

A useful rule of thumb: engineers think in traces, so linear-time logics dominate software and protocol work; branching-time logics survive where symbolic model checkers rule, in hardware.

## Exercises
See [problems/ch14_temporal_logic/](../../../problems/ch14_temporal_logic/)
