(* Modus Ponens in Coq *)

Theorem modus_ponens : forall P Q : Prop, P -> (P -> Q) -> Q.
Proof.
  intros P Q HP HPQ.
  apply HPQ.
  exact HP.
Qed.

(* Direct term-mode proof *)
Definition mp : forall P Q : Prop, P -> (P -> Q) -> Q :=
  fun P Q hp hpq => hpq hp.

(* Using `tauto` *)
Theorem mp_tauto : forall P Q : Prop, P -> (P -> Q) -> Q.
Proof. tauto. Qed.
