(* Dependent types in Coq *)

Require Import Coq.Arith.Arith.
Require Import Coq.Lists.List.
Import ListNotations.

(* =====================================================
   Π-types: functions with value-dependent return types
   ===================================================== *)

(* The type of length-indexed vectors *)
Inductive vec (A : Type) : nat -> Type :=
  | vnil  : vec A 0
  | vcons : forall n, A -> vec A n -> vec A (S n).

Arguments vnil  {A}.
Arguments vcons {A n}.

(* Safe head -- only applicable to non-empty vectors *)
Definition vhead {A : Type} {n : nat} (v : vec A (S n)) : A :=
  match v with
  | vcons _ x _ => x
  end.

(* Safe tail *)
Definition vtail {A : Type} {n : nat} (v : vec A (S n)) : vec A n :=
  match v with
  | vcons _ _ xs => xs
  end.

(* Type-safe append: lengths add *)
Fixpoint vappend {A : Type} {m n : nat} (v1 : vec A m) (v2 : vec A n) : vec A (m + n) :=
  match v1 with
  | vnil => v2
  | vcons _ x xs => vcons x (vappend xs v2)
  end.

(* =====================================================
   Σ-types: dependent pairs / subsets
   ===================================================== *)

(* The subset type {x : A | P x} in Coq is written sig (or {x | P x}) *)

(* Example: even natural numbers *)
Definition even_nat := { n : nat | Nat.even n = true }.

Definition zero_even : even_nat := exist _ 0 eq_refl.
Definition two_even  : even_nat := exist _ 2 eq_refl.

(* Sum of two even numbers *)
Lemma even_add : forall m n, Nat.even m = true -> Nat.even n = true ->
    Nat.even (m + n) = true.
Proof.
  intros m n Hm Hn.
  rewrite Nat.even_add.
  rewrite Hm, Hn.
  reflexivity.
Qed.

Definition even_plus (x y : even_nat) : even_nat :=
  match x, y with
  | exist _ m Hm, exist _ n Hn => exist _ (m + n) (even_add m n Hm Hn)
  end.

(* =====================================================
   Propositions as types
   ===================================================== *)

(* In Coq, Prop is a sort of types meant for logical propositions *)
(* A proof of P is a term of type P *)

Theorem modus_ponens : forall P Q : Prop, (P -> Q) -> P -> Q.
Proof.
  intros P Q h hp.
  exact (h hp).
Qed.

(* The proof term: fun P Q h hp => h hp *)
(* This is literally function application -- Curry-Howard in action *)

(* =====================================================
   Inductive families and GADTs
   ===================================================== *)

(* Equality type: the propositional identity type *)
Print eq.
(* Inductive eq (A : Type) (x : A) : A -> Prop :=
     eq_refl : eq A x x *)

(* J-eliminator: the eliminator for eq *)
(* Built into Coq's `match` for Prop types *)

(* Example: transport along equality *)
Theorem transport : forall (A : Type) (P : A -> Prop) (x y : A),
  x = y -> P x -> P y.
Proof.
  intros A P x y Heq Hpx.
  rewrite <- Heq.
  exact Hpx.
Qed.
