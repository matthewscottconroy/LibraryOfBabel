(* Coq Proof Workflow
   Chapter 4, Section 5

   The Coq proof cycle:
   1. State theorem with Theorem / Lemma
   2. Enter proof mode with Proof.
   3. Apply tactics to transform goals
   4. Close with Qed. (or Admitted. for stubs)
*)

(* ── Basic workflow ──────────────────────────────────────────── *)

Theorem example1 : forall P Q : Prop, P -> (P -> Q) -> Q.
Proof.
  intros P Q HP HPQ.
  apply HPQ.
  exact HP.
Qed.

(* ── Structured proofs with bullets ─────────────────────────── *)

Theorem example2 : forall P Q R : Prop,
  P /\ Q -> (Q -> R) -> P /\ R.
Proof.
  intros P Q R [HP HQ] HQR.
  split.
  - exact HP.
  - apply HQR. exact HQ.
Qed.

(* ── Searching for lemmas ────────────────────────────────────── *)

(* Search nat.         -- list all nat lemmas *)
(* Search (_ + _ = _). -- search by pattern  *)

(* ── Admitted stubs for exercises ───────────────────────────── *)

Theorem exercise1 : forall P Q : Prop, P /\ Q -> Q /\ P.
Proof. Admitted.

Theorem exercise2 : forall P Q : Prop, P \/ Q -> Q \/ P.
Proof. Admitted.

Theorem exercise3 : forall P Q R : Prop,
  (P -> Q) -> (Q -> R) -> P -> R.
Proof. Admitted.
