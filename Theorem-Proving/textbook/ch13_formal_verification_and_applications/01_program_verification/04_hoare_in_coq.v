(* Hoare Logic in Coq *)
(* We implement a small imperative language and verify programs using Hoare triples *)

Require Import Coq.Arith.Arith.
Require Import Coq.Bool.Bool.
Require Import Coq.omega.Omega.
Require Import Coq.Lists.List.
Require Import FunctionalExtensionality.

(* =====================================================
   The Language: IMP
   ===================================================== *)

(* Variables are natural number identifiers *)
Definition Var := nat.

(* State: maps variables to values *)
Definition State := Var -> nat.

(* Update a state *)
Definition update (st : State) (x : Var) (n : nat) : State :=
  fun y => if Nat.eqb y x then n else st y.

(* Arithmetic expressions *)
Inductive AExp : Type :=
  | ANum  : nat -> AExp
  | AVar  : Var -> AExp
  | APlus : AExp -> AExp -> AExp
  | AMul  : AExp -> AExp -> AExp.

Fixpoint aeval (st : State) (a : AExp) : nat :=
  match a with
  | ANum n => n
  | AVar x => st x
  | APlus a1 a2 => aeval st a1 + aeval st a2
  | AMul a1 a2 => aeval st a1 * aeval st a2
  end.

(* Boolean expressions *)
Inductive BExp : Type :=
  | BTrue  : BExp
  | BFalse : BExp
  | BEq    : AExp -> AExp -> BExp
  | BLe    : AExp -> AExp -> BExp
  | BNot   : BExp -> BExp
  | BAnd   : BExp -> BExp -> BExp.

Fixpoint beval (st : State) (b : BExp) : bool :=
  match b with
  | BTrue => true
  | BFalse => false
  | BEq a1 a2 => Nat.eqb (aeval st a1) (aeval st a2)
  | BLe a1 a2 => Nat.leb (aeval st a1) (aeval st a2)
  | BNot b' => negb (beval st b')
  | BAnd b1 b2 => andb (beval st b1) (beval st b2)
  end.

(* Commands *)
Inductive Com : Type :=
  | CSkip  : Com
  | CSeq   : Com -> Com -> Com
  | CAsgn  : Var -> AExp -> Com
  | CIf    : BExp -> Com -> Com -> Com
  | CWhile : BExp -> Com -> Com.

(* =====================================================
   Big-Step Operational Semantics
   ===================================================== *)

Reserved Notation "st '=[' c ']=>' st'" (at level 40).

Inductive ceval : Com -> State -> State -> Prop :=
  | E_Skip : forall st, st =[ CSkip ]=> st
  | E_Seq  : forall c1 c2 st st' st'',
      st =[ c1 ]=> st' -> st' =[ c2 ]=> st'' -> st =[ CSeq c1 c2 ]=> st''
  | E_Asgn : forall x a st,
      st =[ CAsgn x a ]=> update st x (aeval st a)
  | E_IfTrue : forall b c1 c2 st st',
      beval st b = true -> st =[ c1 ]=> st' -> st =[ CIf b c1 c2 ]=> st'
  | E_IfFalse : forall b c1 c2 st st',
      beval st b = false -> st =[ c2 ]=> st' -> st =[ CIf b c1 c2 ]=> st'
  | E_WhileFalse : forall b c st,
      beval st b = false -> st =[ CWhile b c ]=> st
  | E_WhileTrue : forall b c st st' st'',
      beval st b = true ->
      st =[ c ]=> st' -> st' =[ CWhile b c ]=> st'' ->
      st =[ CWhile b c ]=> st''

where "st '=[' c ']=>' st'" := (ceval c st st').

(* =====================================================
   Hoare Logic
   ===================================================== *)

(* Assertions are predicates on states *)
Definition Assertion := State -> Prop.

(* Hoare triple: {P} c {Q} *)
Definition hoare_triple (P : Assertion) (c : Com) (Q : Assertion) : Prop :=
  forall st st', st =[ c ]=> st' -> P st -> Q st'.

Notation "{{ P }} c {{ Q }}" := (hoare_triple P c Q) (at level 90).

(* Assignment rule: {Q[x := a]} x := a {Q} *)
Theorem hoare_asgn : forall Q x a,
  {{ fun st => Q (update st x (aeval st a)) }} CAsgn x a {{ Q }}.
Proof.
  intros Q x a st st' Heval HQ.
  inversion Heval; subst.
  exact HQ.
Qed.

(* Consequence rule *)
Theorem hoare_consequence_pre : forall P P' Q c,
  {{ P' }} c {{ Q }} ->
  (forall st, P st -> P' st) ->
  {{ P }} c {{ Q }}.
Proof.
  intros P P' Q c Hhoare Himp st st' Heval HP.
  apply Hhoare with (st := st).
  - exact Heval.
  - apply Himp. exact HP.
Qed.

(* Sequence rule *)
Theorem hoare_seq : forall P Q R c1 c2,
  {{ P }} c1 {{ Q }} -> {{ Q }} c2 {{ R }} -> {{ P }} CSeq c1 c2 {{ R }}.
Proof.
  intros P Q R c1 c2 H1 H2 st st'' Hseq HP.
  inversion Hseq; subst.
  apply H2 with (st := st').
  - exact H6.
  - apply H1 with (st := st); assumption.
Qed.
