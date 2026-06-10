(* Structural induction in Coq *)
(* Coq is built for structural induction -- it's the foundation of all proofs *)

Require Import List.
Import ListNotations.
Require Import Lia.

(* Inductive trees *)
Inductive tree (A : Type) : Type :=
  | Leaf : tree A
  | Node : tree A -> A -> tree A -> tree A.

Arguments Leaf {A}.
Arguments Node {A}.

(* Size of a tree *)
Fixpoint size {A : Type} (t : tree A) : nat :=
  match t with
  | Leaf => 0
  | Node l _ r => 1 + size l + size r
  end.

(* Height of a tree *)
Fixpoint height {A : Type} (t : tree A) : nat :=
  match t with
  | Leaf => 0
  | Node l _ r => 1 + max (height l) (height r)
  end.

(* Theorem: size <= 2^height - 1 *)
Theorem size_le_exp_height : forall {A : Type} (t : tree A),
  size t < 2 ^ (height t).
Proof.
  induction t as [| l IHl a r IHr].
  - simpl. lia.
  - simpl.
    pose proof (Nat.pow_gt_zero 2 (height l) (by lia)).
    pose proof (Nat.pow_gt_zero 2 (height r) (by lia)).
    lia.
Qed.

(* Mirror/reflection of a tree *)
Fixpoint mirror {A : Type} (t : tree A) : tree A :=
  match t with
  | Leaf => Leaf
  | Node l a r => Node (mirror r) a (mirror l)
  end.

(* Mirror preserves size *)
Theorem mirror_size : forall {A : Type} (t : tree A),
  size (mirror t) = size t.
Proof.
  induction t as [| l IHl a r IHr].
  - reflexivity.
  - simpl. lia.
Qed.

(* Mirror is an involution: mirror (mirror t) = t *)
Theorem mirror_involution : forall {A : Type} (t : tree A),
  mirror (mirror t) = t.
Proof.
  induction t as [| l IHl a r IHr].
  - reflexivity.
  - simpl. rewrite IHl, IHr. reflexivity.
Qed.

(* Inorder traversal *)
Fixpoint inorder {A : Type} (t : tree A) : list A :=
  match t with
  | Leaf => []
  | Node l a r => inorder l ++ [a] ++ inorder r
  end.

(* Inorder of mirror = reverse of inorder *)
Theorem inorder_mirror : forall {A : Type} (t : tree A),
  inorder (mirror t) = rev (inorder t).
Proof.
  induction t as [| l IHl a r IHr].
  - reflexivity.
  - simpl.
    rewrite IHl, IHr.
    rewrite rev_app_distr.
    simpl.
    rewrite rev_app_distr.
    rewrite app_assoc.
    reflexivity.
Qed.

(* Structural induction on rose trees (arbitrary branching) *)
Inductive rose (A : Type) : Type :=
  | RLeaf : A -> rose A
  | RNode : list (rose A) -> rose A.

Fixpoint rose_size {A : Type} (t : rose A) : nat :=
  match t with
  | RLeaf _ => 1
  | RNode children => 1 + fold_left (fun acc c => acc + rose_size c) children 0
  end.
