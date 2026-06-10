# Coq Standard Library and Proof Ecosystem

## Coq's Mathematical Heritage

Coq (now also called Rocq) is one of the oldest proof assistants, developed continuously since 1984 at INRIA. Its standard library provides a foundation, and the **Mathematical Components** library (MathComp) built by the Gonthier group extends it with extensive algebraic structures.

## Key Library Components

**Coq Standard Library (`Coq.`):**
- `Coq.Init`: Basic types (`nat`, `bool`, `list`, `option`), tactics
- `Coq.Arith`: Natural number arithmetic
- `Coq.ZArith`: Integer arithmetic
- `Coq.Reals`: Real numbers (axiomatic)
- `Coq.Logic`: Classical logic axioms, Prop axioms
- `Coq.Sets`: Set theory via predicates

**Mathematical Components (`mathcomp.`):**
- Group theory, ring theory, field theory
- Linear algebra over arbitrary fields
- Graph theory
- A large portion of the Feit-Thompson theorem proof (odd order theorem)

## The Odd Order Theorem Formalization

The **Feit-Thompson theorem** (1963): Every finite group of odd order is solvable. The original paper was 255 pages — the most complex theorem proved by humans at the time.

Gonthier, Asperti, et al. formalized this in Coq (2012): 170,000 lines of Coq code, checked by the Coq proof checker. This remains one of the most complex mathematical formalizations ever completed.

## Basic Coq Proof Style

```coq
(* Proving commutativity of addition on natural numbers *)
Theorem add_comm : forall n m : nat, n + m = m + n.
Proof.
  intro n.
  induction n as [| n' IHn'].
  - (* Base case: n = 0 *)
    intro m. simpl. rewrite <- plus_n_O. reflexivity.
  - (* Inductive step: n = S n' *)
    intro m. simpl. rewrite IHn'. rewrite plus_n_Sm. reflexivity.
Qed.

(* Using tactics for a classical proof *)
Theorem classic_example : forall P : Prop, P \/ ~P.
Proof.
  intro P.
  apply classic.  (* From Coq.Logic.Classical *)
Qed.
```

## Comparison with Lean 4

| Feature | Coq | Lean 4 |
|---------|-----|--------|
| Foundation | CIC (pCuIC) | CIC (similar) |
| Default logic | Intuitionistic | Intuitionistic + Classical |
| Proof automation | `omega`, `tauto`, `ring`, `field` | `omega`, `norm_num`, `ring`, `simp` |
| Library | Standard Lib + MathComp | Mathlib4 |
| Extraction | OCaml, Haskell | - |
| Meta-programming | Ltac, Elpi | Lean macros |

Both are excellent tools. Coq has a longer history and some unique libraries; Lean 4 has more modern syntax and the rapidly growing Mathlib.

## Exercises
See [problems/ch13_applications/02_lean_proofs.md](../../../problems/ch13_applications/02_lean_proofs.md)
