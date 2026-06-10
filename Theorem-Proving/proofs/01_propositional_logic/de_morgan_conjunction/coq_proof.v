(* De Morgan's Law (Conjunction) in Coq *)

Require Import Classical.

Theorem de_morgan_and : forall P Q : Prop,
  ~(P /\ Q) <-> (~P \/ ~Q).
Proof.
  intros P Q.
  split.
  - intro H.
    destruct (classic P) as [HP | HNP].
    + right. intro HQ. apply H. split; assumption.
    + left. exact HNP.
  - intros [HNP | HNQ] [HP HQ].
    + apply HNP. exact HP.
    + apply HNQ. exact HQ.
Qed.
