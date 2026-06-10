(* First-order logic in Coq *)
(* Coq's core logic IS a form of first-order/higher-order logic *)

Require Import Classical.

(* =====================================================
   Universal and Existential Quantifiers
   ===================================================== *)

(* ∀ in Coq is the forall keyword -- it's the Π-type *)
(* ∃ in Coq is written as `exists` *)

(* Instantiating a universal *)
Example forall_elim : (forall n : nat, n + 0 = n) -> 5 + 0 = 5.
Proof.
  intro H.
  apply H.
Qed.

(* Proving a universal by introducing a generic variable *)
Example forall_intro : forall n : nat, n + 0 = n.
Proof.
  intro n.
  rewrite Nat.add_comm.
  reflexivity.
Qed.

(* Proving an existential by exhibiting a witness *)
Example exists_intro : exists n : nat, n > 100.
Proof.
  exists 101.
  lia.
Qed.

(* Eliminating an existential: extract the witness and hypothesis *)
Example exists_elim : (exists n : nat, n > 100) -> True.
Proof.
  intros [n Hn].  (* destruct: bind witness and proof *)
  trivial.
Qed.

(* =====================================================
   Negation and Classical Logic
   ===================================================== *)

(* In constructive Coq, ¬P = P -> False *)
(* Classical adds LEM: P ∨ ¬P *)

Example lem_usage (P : Prop) : P \/ ~P.
Proof.
  apply classic.
Qed.

(* De Morgan's laws -- constructively valid *)
Example de_morgan_1 : forall P Q : Prop, ~(P \/ Q) <-> ~P /\ ~Q.
Proof.
  intros P Q. split.
  - intro h. split; intro hp; apply h.
    + left; exact hp.
    + right; exact hp.
  - intros [hnp hnq] [hp | hq].
    + exact (hnp hp).
    + exact (hnq hq).
Qed.

(* ¬∀x P(x) ↔ ∃x ¬P(x) -- requires classical *)
Example not_forall_exists : forall (A : Type) (P : A -> Prop),
  (~ forall x, P x) <-> exists x, ~ P x.
Proof.
  intros A P.
  split.
  - intro H.
    apply Classical_Pred_Type.not_all_ex_not.
    exact H.
  - intros [x Hx] Hall.
    exact (Hx (Hall x)).
Qed.

(* =====================================================
   First-Order Structures
   ===================================================== *)

(* A structure for a simple language: one unary predicate *)
Section Structure.
  Variable Domain : Type.
  Variable Even : Domain -> Prop.
  Variable zero : Domain.
  Variable succ : Domain -> Domain.

  (* Sentence: every element has a successor *)
  Hypothesis H_succ : forall x : Domain, True.

  (* Axiom: zero is even *)
  Hypothesis H_zero : Even zero.

  (* Axiom: succ (succ x) is even iff x is even *)
  Hypothesis H_succ2 : forall x, Even (succ (succ x)) <-> Even x.

  (* Theorem: succ^4 of anything has the same parity as the original *)
  Theorem succ4_parity : forall x, Even (succ (succ (succ (succ x)))) <-> Even x.
  Proof.
    intro x.
    rewrite H_succ2.
    rewrite H_succ2.
    reflexivity.
  Qed.

End Structure.

(* =====================================================
   Peano Arithmetic in Coq
   ===================================================== *)

(* Coq's nat satisfies all Peano axioms by construction *)

(* PA1: 0 is a natural number *)
Check (0 : nat).

(* PA2: Every nat has a successor *)
Check Nat.succ.

(* PA3: 0 is not a successor *)
Check Nat.succ_ne_zero.

(* PA4: succ is injective *)
Check Nat.succ_inj.

(* PA5: Induction principle *)
Check nat_ind.
