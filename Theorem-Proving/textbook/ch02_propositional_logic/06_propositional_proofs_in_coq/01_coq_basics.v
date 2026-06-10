(* Propositional Logic in Coq
   Chapter 2, Section 6

   Coq uses the Calculus of Inductive Constructions.
   Propositions live in the sort Prop.
   A proof of P is a term of type P.
*)

Section PropositionalLogic.

Variables p q r : Prop.

(* ── Basic proofs ─────────────────────────────────────────── *)

(* Identity *)
Theorem id_proof : p -> p.
Proof. intro H. exact H. Qed.

(* Modus Ponens *)
Theorem modus_ponens : p -> (p -> q) -> q.
Proof. intros Hp Hpq. apply Hpq. exact Hp. Qed.

(* Modus Tollens *)
Theorem modus_tollens : (p -> q) -> ~q -> ~p.
Proof.
  intros Hpq Hnq Hp.
  apply Hnq. apply Hpq. exact Hp.
Qed.

(* ── Conjunction ──────────────────────────────────────────── *)

Theorem and_intro : p -> q -> p /\ q.
Proof. intros Hp Hq. split; assumption. Qed.

Theorem and_elim_l : p /\ q -> p.
Proof. intros [Hp _]. exact Hp. Qed.

(* ── Disjunction ──────────────────────────────────────────── *)

Theorem or_intro_l : p -> p \/ q.
Proof. intro Hp. left. exact Hp. Qed.

Theorem or_elim : p \/ q -> (p -> r) -> (q -> r) -> r.
Proof.
  intros [Hp | Hq] Hpr Hqr.
  - apply Hpr. exact Hp.
  - apply Hqr. exact Hq.
Qed.

(* ── De Morgan (using classical logic) ───────────────────── *)

Require Import Classical.

Theorem de_morgan_and : ~(p /\ q) <-> ~p \/ ~q.
Proof.
  split.
  - intro H.
    destruct (classic p) as [Hp | Hnp].
    + right. intro Hq. apply H. split; assumption.
    + left. exact Hnp.
  - intros [Hnp | Hnq] [Hp Hq].
    + apply Hnp. exact Hp.
    + apply Hnq. exact Hq.
Qed.

(* ── Exercise stubs ───────────────────────────────────────── *)

(* Exercise 1: contrapositive *)
Theorem contrapositive_ex : (p -> q) -> (~q -> ~p).
Proof. (* fill in *) Admitted.

(* Exercise 2: p /\ (p -> q) -> q *)
Theorem and_mp_ex : p /\ (p -> q) -> q.
Proof. (* fill in *) Admitted.

End PropositionalLogic.
