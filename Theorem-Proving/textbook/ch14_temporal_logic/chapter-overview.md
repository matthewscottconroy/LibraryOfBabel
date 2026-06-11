# Chapter 14 Overview: Temporal Logic

---

## Central Question

How do we specify and verify properties that involve *time*: "the system will eventually respond," "the request is always eventually granted," "the system never enters a deadlock state"? Temporal logic extends modal logic with operators for time, providing the language for formal specification of reactive systems.

---

## Why This Chapter Matters

Temporal logic is the specification language for model checking (Chapter 13). Every major model checker — SPIN (LTL), NuSMV (CTL), PRISM (PCTL) — is built around a temporal logic. The formal specification and verification of concurrent systems, hardware circuits, communication protocols, and safety-critical software all use temporal logic. Understanding the logic is prerequisite for understanding the tools.

---

## Key Definitions

**Linear Temporal Logic (LTL).** LTL formulas are interpreted over infinite *paths* $\pi = s_0, s_1, s_2, \ldots$ (sequences of states):

$$\phi ::= p \mid \neg\phi \mid \phi \land \psi \mid X\phi \mid \phi\ U\ \psi$$

with derived operators:
- $F\phi \equiv \top\ U\ \phi$ ("finally / eventually $\phi$")
- $G\phi \equiv \neg F\neg\phi$ ("globally / always $\phi$")
- $\phi\ W\ \psi \equiv (\phi\ U\ \psi) \lor G\phi$ ("weak until")
- $\phi\ R\ \psi \equiv \neg(\neg\phi\ U\ \neg\psi)$ ("release")

**Semantics at position $i$ on path $\pi$:**
- $\pi, i \vDash X\phi$ iff $\pi, i+1 \vDash \phi$ (next)
- $\pi, i \vDash \phi\ U\ \psi$ iff $\exists j \geq i: \pi, j \vDash \psi$ and $\forall k, i \leq k < j: \pi, k \vDash \phi$ (until)

**Computation Tree Logic (CTL).** CTL formulas are interpreted over computation trees (branching time). Path quantifiers $A$ (all paths) and $E$ (some path) prefix temporal operators:

$$\phi ::= p \mid \neg\phi \mid \phi \land \psi \mid AX\phi \mid EX\phi \mid A[\phi\ U\ \psi] \mid E[\phi\ U\ \psi]$$

with derived: $AF\phi = A[\top\ U\ \phi]$, $EF\phi = E[\top\ U\ \phi]$, $AG\phi = \neg EF\neg\phi$, $EG\phi = \neg AF\neg\phi$.

**CTL\*** CTL* is a superset of both LTL and CTL: path quantifiers ($A, E$) can prefix arbitrary LTL path formulas (including nested $U$ operators). CTL is the fragment where each temporal operator is immediately preceded by a path quantifier.

---

## LTL vs. CTL: Expressibility

LTL and CTL are incomparable in expressive power:

**LTL but not CTL:** $FGp$ ("eventually, $p$ holds from that point on forever") cannot be expressed in CTL because it requires reasoning about a specific path (not branching).

**CTL but not LTL:** $AG(EFp)$ ("from every reachable state, $p$ is eventually achievable") cannot be expressed in LTL, which is interpreted over a single linear path.

**CTL\*** strictly subsumes both, but its model checking is more expensive.

---

## Main Theorems

### LTL Satisfiability is PSPACE-Complete

**Theorem (Sistla & Clarke 1985).** The satisfiability problem for LTL (does a formula have a model?) is PSPACE-complete.

**Proof sketch (membership in PSPACE).** An LTL formula $\phi$ over $n$ variables is satisfiable iff it has a *finite lasso model* — a path of the form $s_0, \ldots, s_k, s_{k+1}, \ldots, s_{k+m}$ where position $k+m$ is followed by a return to position $k+1$ (the "lasso"). The lasso's length is exponential in $|\phi|$, but can be checked using polynomial space by an alternating Turing machine. PSPACE-hardness reduces from QBFL.

### CTL Model Checking is PTIME

**Theorem (Clarke, Emerson, Sistla 1986; Queille & Sifakis 1982).** CTL model checking (given a finite Kripke structure $M$ and CTL formula $\phi$, determine whether $M \vDash \phi$) is solvable in time $O(|M| \cdot |\phi|)$.

**Algorithm (labelling algorithm).** Process $\phi$ bottom-up on its parse tree. For each subformula $\psi$, compute the set of states satisfying $\psi$ and label them. For the critical case $E[\phi\ U\ \psi]$:
1. Mark all states satisfying $\psi$ (these are the target states).
2. Propagate backward: if a state $s$ has $\phi$ and has a successor in the marked set, mark $s$.
3. Repeat until fixpoint. The fixed point is the set of states satisfying $E[\phi\ U\ \psi]$.

Each step is $O(|M|)$; convergence takes at most $|M|$ steps; there are $|\phi|$ subformulas. Total: $O(|M| \cdot |\phi|)$.

### LTL Model Checking via Automata

**Algorithm (Vardi & Wolper 1986).** For a Kripke structure $M$ and LTL formula $\phi$:

1. Construct a *Büchi automaton* $\mathcal{A}_{\neg\phi}$ recognising all paths *not* satisfying $\phi$.
2. Form the product automaton $M \times \mathcal{A}_{\neg\phi}$.
3. Check if the product automaton accepts any path (reachability + cycle detection).
4. If yes: a counterexample exists (the system violates $\phi$). If no: $M \vDash \phi$.

**Complexity:** $O(|M| \cdot 2^{|\phi|})$ — exponential in formula size (due to automaton construction), but linear in model size.

**Büchi automaton.** A Büchi automaton is an $\omega$-automaton (runs on infinite words) that accepts infinite words which visit an accepting state infinitely often.

---

## Safety and Liveness Properties

**Safety property.** A property $P$ is a *safety property* if: whenever $P$ is violated, there is a *finite prefix* of the execution witnessing the violation. Example: "the system never enters a deadlock" — a deadlock has a finite witnessing prefix.

**Liveness property.** A property $P$ is a *liveness property* if every *finite prefix* can be extended to an infinite execution satisfying $P$. Example: "the system eventually terminates" — any finite prefix can be extended (by running longer), but the infinite execution may never terminate.

**Theorem (Alpern & Schneider 1985).** Every temporal property can be decomposed into a safety component and a liveness component.

---

## Fairness

Concurrent systems often require *fairness assumptions* to ensure that components that are continuously enabled are not permanently starved. Two common variants:

**Weak fairness (compassion).** If a process is continuously enabled, it is infinitely often executed.

**Strong fairness (justice).** If a process is infinitely often enabled, it is infinitely often executed.

LTL formulas for fairness:
- Weak: $GF\text{enabled}_i \to GF\text{executed}_i$
- Strong: $GF\text{enabled}_i \to GF\text{executed}_i$ (same formula, but the enabling condition must be persistent under weak fairness and intermittent under strong)

Under fairness assumptions, many properties that are false (a process can be permanently starved in an unfair scheduler) become true.

---

## Historical Context

**Arthur Prior (1957)** introduced tense logic in *Time and Modality*, the first systematic formal study of tense operators. He was motivated by the philosophy of time.

**Amir Pnueli (1977)** introduced LTL for program verification in his seminal paper "The Temporal Logic of Programs." He received the Turing Award in 1996 for this work.

**Edmund Clarke and E. Allen Emerson (1981)** introduced CTL and the first model checking algorithm. Independently, **Joseph Sifakis** and **Jean-Pierre Queille (1982)** proposed similar ideas. All three received the Turing Award in 2007.

**Moshe Vardi and Pierre Wolper (1986)** developed the automata-theoretic approach to LTL model checking, connecting linear temporal logic to Büchi automata theory.

**Temporal logic's practical impact:** The SPIN model checker (Holzmann, 1997, NASA) has been used to verify communication protocols and embedded systems. The NuSMV model checker has verified hardware designs. PRISM handles probabilistic temporal logic (PCTL) for Markov chains.

---

## Connections to Other Chapters

- **Chapter 12** (Modal Logic): LTL and CTL are modal logics with specialised accessibility relations (the next-state relation of a transition system).
- **Chapter 13** (Formal Verification): model checking algorithms check temporal logic properties of hardware and software systems.
- **Chapter 10** (Computability): LTL satisfiability is PSPACE-complete; CTL model checking is P-complete — these results use complexity theory concepts.
