# Hoare Logic

> "I conclude that the purpose of the axiomatic approach to programming is primarily as a tool of program design — to formulate and prove the correctness of designs before they are coded."
> — C.A.R. Hoare, "An Axiomatic Basis for Computer Programming", 1969

## The Dream of Provably Correct Programs

Software errors cost the world hundreds of billions of dollars annually. The Ariane 5 rocket's first flight ended in explosion — a floating-point conversion error. The Therac-25 radiation machine killed patients — a race condition in the control software. The Mars Climate Orbiter was lost — a unit conversion mistake.

These are not random failures. They are the consequence of programs doing exactly what they were instructed to do — just not what their programmers *intended*. The gap between intention and instruction is where bugs live.

**Hoare logic** is one approach to eliminating this gap. Developed by C.A.R. Hoare (following Floyd's earlier work) in 1969, it provides a formal system for *proving* that programs do what we intend — before they are deployed, before they can fail.

The dream: write programs and their correctness proofs simultaneously, with a machine checking both. This is now reality in systems like CompCert, seL4, and CakeML.

## The Hoare Triple

The central concept is the **Hoare triple**:
$$\{P\}\; C\; \{Q\}$$

Read: "If precondition $P$ holds before executing command $C$, then postcondition $Q$ holds when $C$ terminates."

- $P$ is the **precondition**: what we assume before $C$ runs
- $C$ is the **command**: the program fragment we are verifying
- $Q$ is the **postcondition**: what we guarantee after $C$ runs (if it runs at all)

**Partial correctness**: The triple $\{P\} C \{Q\}$ asserts *conditional* correctness — if $C$ terminates and $P$ was true at the start, then $Q$ is true at the end. It says nothing about whether $C$ terminates.

**Total correctness**: The triple $[P]\; C\; [Q]$ (different notation in some presentations) additionally requires that $C$ terminates whenever $P$ holds.

**Examples**:
- $\{x = 5\}\; x := x + 1\; \{x = 6\}$: valid. If $x$ starts as 5, after incrementing, $x$ is 6.
- $\{x \geq 0\}\; \text{while } x \neq 0 \text{ do } x := x - 1\; \{x = 0\}$: partial correctness holds. If the loop terminates (which it will for non-negative $x$), then $x = 0$.
- $\{true\}\; \text{while } true \text{ do skip}\; \{false\}$: trivially valid as partial correctness (the premise "the loop terminates" is never satisfied), but total correctness fails.

## The Proof Rules

Hoare logic provides rules for each command construct, allowing us to build proofs bottom-up from basic commands to complex programs.

### Assignment Rule

$$\{P[E/x]\}\; x := E\; \{P\}$$

The assignment rule runs *backwards*: if you want $P$ to hold after the assignment $x := E$, you need $P$ with $E$ substituted for $x$ to hold beforehand.

**Example**: What precondition ensures $\{x + y = 10\}$ holds after $x := x + 1$?

Substitute: $P[x+1/x] = (x+1) + y = 10 = x + y = 9$.

So: $\{x + y = 9\}\; x := x + 1\; \{x + y = 10\}$ ✓

This backwards thinking is initially counterintuitive but elegant — it computes the *weakest* condition under which the postcondition is achievable.

### Sequence Rule

$$\frac{\{P\}\; C_1\; \{R\} \quad \{R\}\; C_2\; \{Q\}}{\{P\}\; C_1; C_2\; \{Q\}}$$

The sequence rule chains Hoare triples: find an intermediate condition $R$ that holds after $C_1$ and before $C_2$.

### Conditional Rule

$$\frac{\{P \wedge B\}\; C_1\; \{Q\} \quad \{P \wedge \neg B\}\; C_2\; \{Q\}}{\{P\}\; \text{if } B \text{ then } C_1 \text{ else } C_2\; \{Q\}}$$

In the "then" branch, we know $P$ and the condition $B$ both hold. In the "else" branch, $P$ holds and $\neg B$ holds. Both branches must establish $Q$.

### While Loop Rule (The Hardest Part)

$$\frac{\{I \wedge B\}\; C\; \{I\}}{\{I\}\; \text{while } B \text{ do } C\; \{I \wedge \neg B\}}$$

The key insight: the proof of a loop requires a **loop invariant** $I$ — a condition that:
1. Holds before the loop begins
2. Is preserved by each loop iteration: if $I \wedge B$ holds before the body $C$, then $I$ holds after
3. Combined with loop termination ($\neg B$), gives the desired postcondition

Finding the loop invariant is the *creative* part of program verification. It cannot be automated in general (decidability fails). It is the analogue of the induction hypothesis in mathematical proofs — a statement that encapsulates the invariant progress of the loop.

### Consequence Rule

$$\frac{P \vDash P' \quad \{P'\}\; C\; \{Q'\} \quad Q' \vDash Q}{\{P\}\; C\; \{Q\}}$$

We can strengthen the precondition or weaken the postcondition: if we know $P$ implies $P'$ (so $P$ is "more" than we need), and $Q'$ implies $Q$ (the result "more" than we wanted), then $\{P\} C \{Q\}$ follows.

## A Complete Worked Example: Integer Division

We prove the integer division algorithm correct.

**Program**:
```
q := 0;
r := x;
while r ≥ d do
    r := r - d;
    q := q + 1
```

**Specification**: $\{x \geq 0 \wedge d > 0\}$ *above program* $\{x = q \cdot d + r \wedge 0 \leq r < d\}$

**Loop invariant** (the key discovery):
$$I : \quad x = q \cdot d + r \wedge r \geq 0$$

**Step 1**: Show $I$ holds before the loop.
Before the loop: $q = 0$, $r = x$. Then $q \cdot d + r = 0 \cdot d + x = x$ ✓ and $r = x \geq 0$ ✓.

**Step 2**: Show the body preserves $I$ assuming $I \wedge r \geq d$.

Before iteration: $x = qd + r$ and $r \geq 0$ (IH) and $r \geq d$ (loop condition).
After $r := r - d$: new $r' = r - d \geq 0$. After $q := q + 1$: new $q' = q + 1$.
Check: $q' \cdot d + r' = (q+1)d + (r-d) = qd + d + r - d = qd + r = x$ ✓.
And $r' = r - d \geq 0$ since $r \geq d$. ✓

**Step 3**: The loop terminates and the postcondition follows.
When the loop exits: $I$ holds ($x = qd + r \wedge r \geq 0$) and $\neg(r \geq d)$ holds ($r < d$).
Together: $x = qd + r \wedge 0 \leq r < d$. ✓

**Termination** (for total correctness): $r$ is a natural number that strictly decreases by $d > 0$ at each step. By well-foundedness of $\mathbb{N}$, the loop terminates. ✓

## Weakest Preconditions: Computing Backwards

The **weakest precondition** $\text{wp}(C, Q)$ is the least restrictive condition under which $C$ guarantees $Q$. The Hoare triple $\{P\} C \{Q\}$ holds iff $P \Rightarrow \text{wp}(C, Q)$.

Computing $\text{wp}$ backwards through a program gives a systematic, automatable verification algorithm:

$$\text{wp}(x := E, Q) = Q[E/x]$$
$$\text{wp}(C_1; C_2, Q) = \text{wp}(C_1, \text{wp}(C_2, Q))$$
$$\text{wp}(\text{if } B \text{ then } C_1 \text{ else } C_2, Q) = (B \wedge \text{wp}(C_1, Q)) \vee (\neg B \wedge \text{wp}(C_2, Q))$$
$$\text{wp}(\text{while } B \text{ do } C, Q) = \text{lfp}(\lambda X.\; (\neg B \wedge Q) \vee (B \wedge \text{wp}(C, X)))$$

The while case requires a least fixed point — which in practice is found by finding a loop invariant, as above.

## Real-World Formal Verification

**CompCert**: A C compiler formally verified in Coq (Xavier Leroy, INRIA). CompCert is proved to produce machine code that computes exactly what the C source code specifies — no bugs introduced by the compiler. Used in safety-critical avionics software.

**seL4**: A formally verified operating system microkernel (NICTA, now CSIRO). Proved correct in Isabelle/HOL — the 8,700-line C implementation satisfies a 200-page formal specification. Used in high-security military and aviation systems.

**CakeML**: A formally verified Standard ML compiler. The compiler itself is verified in HOL4 — you can prove properties of programs compiled with CakeML and trust that those properties survive compilation.

**Amazon AWS**: Uses TLA+ (Temporal Logic of Actions) for formal specification of distributed protocols (Dynamo, S3). Found and prevented bugs that would have been very difficult to find through testing.

## Lean 4 Example

```lean
-- Simple Hoare-style verification in Lean 4
-- We prove a specification about a pure function

def divide (x d : ℕ) (hd : d > 0) : ℕ × ℕ :=
  (x / d, x % d)

theorem divide_correct (x d : ℕ) (hd : d > 0) :
    let (q, r) := divide x d hd
    x = q * d + r ∧ r < d := by
  simp [divide]
  constructor
  · exact (Nat.div_add_mod x d).symm
  · exact Nat.mod_lt x hd

-- For imperative programs, we would use separation logic or
-- effect-based verification frameworks in Lean 4
```

## Exercises
See [problems/ch13_applications/01_hoare_logic_problems.md](../../../problems/ch13_applications/01_hoare_logic_problems.md)
