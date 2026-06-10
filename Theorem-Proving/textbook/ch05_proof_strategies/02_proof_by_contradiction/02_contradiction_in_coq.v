(* Proof by contradiction in Coq *)
(* In Coq's constructive logic, Classical reasoning requires importing Classical *)

Require Import Classical.

(* The law of excluded middle - not provable without Classical *)
Check classic : forall P : Prop, P \/ ~ P.

(* Double negation elimination - classical *)
Check NNPP : forall P : Prop, ~ ~ P -> P.

(* Proof by contradiction: assume ~P, derive False, conclude P *)
Theorem proof_by_contradiction : forall P : Prop, ~ ~ P -> P.
Proof.
  intros P hnnp.
  apply NNPP.
  exact hnnp.
Qed.

(* Example: sqrt(2) is irrational (structural argument) *)
(* We show: if n^2 is even, then n is even *)
Theorem even_square_even : forall n : nat, Nat.even (n * n) = true -> Nat.even n = true.
Proof.
  intro n.
  destruct (Nat.even n) eqn:He.
  - auto.
  - (* n is odd *)
    intro H.
    (* n = 2k+1, so n*n = 4k^2+4k+1 = odd -- contradiction *)
    rewrite Nat.even_mul in H.
    rewrite He in H.
    simpl in H.
    discriminate.
Qed.

(* Contradiction tactic: if we have H : P and H' : ~P, use contradiction *)
Example simple_contradiction (P : Prop) (h : P) (hn : ~ P) : False.
Proof.
  contradiction.
Qed.

(* exfalso: reduce any goal to False *)
Example exfalso_example (P Q : Prop) (h : P) (hn : ~ P) : Q.
Proof.
  exfalso.
  exact (hn h).
Qed.

(* Classical: proof by contradiction for arbitrary goals *)
Example classical_example (P Q : Prop) : ~ ~ (P \/ ~ P).
Proof.
  intro h.
  apply h.
  right.
  intro hp.
  apply h.
  left.
  exact hp.
Qed.
