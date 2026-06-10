(* Sets in Coq using the standard library *)
(* Coq represents sets as predicates: Set A = A -> Prop *)

Require Import Coq.Sets.Ensembles.
Require Import Coq.Sets.Powerset.
Require Import Coq.Sets.Finite_sets.

(* The type of sets over a universe U is: Ensemble U = U -> Prop *)

Section SetOperations.
  Variable U : Type.

  (* Membership: x ∈ A means A x *)
  (* In Coq: In U A x *)

  (* Union *)
  Definition my_union (A B : Ensemble U) : Ensemble U :=
    fun x => In U A x \/ In U B x.

  (* Intersection *)
  Definition my_inter (A B : Ensemble U) : Ensemble U :=
    fun x => In U A x /\ In U B x.

  (* Subset *)
  Definition my_subset (A B : Ensemble U) : Prop :=
    forall x, In U A x -> In U B x.

  (* Complement *)
  Definition my_complement (A : Ensemble U) : Ensemble U :=
    fun x => ~ In U A x.

  (* Basic theorem: A ∩ B ⊆ A *)
  Theorem inter_subset_left : forall A B : Ensemble U,
    my_subset (my_inter A B) A.
  Proof.
    intros A B x [hA _].
    exact hA.
  Qed.

  (* De Morgan: ~(A ∪ B) = ~A ∩ ~B *)
  Theorem de_morgan_union : forall A B : Ensemble U,
    forall x, In U (my_complement (my_union A B)) x <->
              In U (my_inter (my_complement A) (my_complement B)) x.
  Proof.
    intros A B x.
    unfold my_complement, my_union, my_inter, In.
    split.
    - intro h. split; intro hc; apply h.
      + left. exact hc.
      + right. exact hc.
    - intros [hA hB] [hAB | hAB].
      + exact (hA hAB).
      + exact (hB hAB).
  Qed.

End SetOperations.

(* Finite sets and cardinality *)
Section FiniteSets.
  Variable U : Type.

  (* The Finite predicate from Coq.Sets.Finite_sets *)
  Check Finite.

  (* Cardinal number of a finite set *)
  Check cardinal.

  (* Key theorem: cardinal is unique *)
  Check cardinal_unique.

End FiniteSets.

(* Characteristic functions: the bijection between sets and Bool-valued functions *)
Definition char_fun {U : Type} (A : Ensemble U) (x : U) : bool :=
  if (Classical_Prop.classic (In U A x)) then true else false.
