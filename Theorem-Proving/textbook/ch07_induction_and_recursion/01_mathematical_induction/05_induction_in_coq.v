(* Mathematical induction in Coq *)
(* Coq's nat type has built-in induction via the `induction` tactic *)

Require Import Lia.
Require Import Arith.

(* The nat type is defined inductively:
   Inductive nat : Set :=
     | O : nat
     | S : nat -> nat

   The induction principle:
     forall P, P 0 -> (forall n, P n -> P (S n)) -> forall n, P n *)

(* Classic: sum of first n natural numbers *)
Fixpoint sum_to (n : nat) : nat :=
  match n with
  | O => 0
  | S m => S m + sum_to m
  end.

Theorem sum_formula : forall n : nat, 2 * sum_to n = n * (n + 1).
Proof.
  induction n as [| n IH].
  - (* Base case: n = 0 *)
    simpl. reflexivity.
  - (* Inductive step: assume 2 * sum_to n = n * (n+1), prove for S n *)
    simpl sum_to.
    lia.  (* linear arithmetic finishes it *)
Qed.

(* Sum of squares: sum_{i=0}^{n} i^2 = n(n+1)(2n+1)/6 *)
Fixpoint sum_sq (n : nat) : nat :=
  match n with
  | O => 0
  | S m => (S m) * (S m) + sum_sq m
  end.

Theorem sum_sq_formula : forall n : nat, 6 * sum_sq n = n * (n + 1) * (2 * n + 1).
Proof.
  induction n as [| n IH].
  - simpl. reflexivity.
  - simpl sum_sq. lia.
Qed.

(* Strong induction: P(n) assuming P(k) for all k < n *)
Theorem strong_induction : forall P : nat -> Prop,
  (forall n, (forall k, k < n -> P k) -> P n) ->
  forall n, P n.
Proof.
  intros P IH n.
  assert (H : forall m k, k < m -> P k).
  { induction m as [| m IHm].
    - intros k Hk. inversion Hk.
    - intros k Hk. apply IH. intros j Hj.
      apply IHm. lia. }
  apply IH. intros k Hk. exact (H (S n) k (Nat.lt_succ_of_lt Hk (Nat.lt_succ_diag_r n))).
Qed.

(* Well-foundedness: induction on < for nat *)
Require Import Coq.Arith.Wf_nat.

(* lt_wf : well_founded lt *)
Check lt_wf.

(* Induction on a measure *)
Theorem euclid_terminates_wf : well_founded (fun a b : nat => a < b).
Proof.
  exact lt_wf.
Qed.

(* Structural induction on a list *)
Require Import Coq.Lists.List.
Import ListNotations.

Theorem length_append : forall {A : Type} (l1 l2 : list A),
  length (l1 ++ l2) = length l1 + length l2.
Proof.
  intros A l1 l2.
  induction l1 as [| h t IH].
  - simpl. reflexivity.
  - simpl. rewrite IH. reflexivity.
Qed.

Theorem rev_length : forall {A : Type} (l : list A),
  length (rev l) = length l.
Proof.
  intro A.
  induction l as [| h t IH].
  - simpl. reflexivity.
  - simpl. rewrite length_append. simpl. lia.
Qed.
