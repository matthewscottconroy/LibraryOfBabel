# Proof: Safety and Recurrence in a Two-State System

## The System

Consider the Kripke structure $M$ with states $S = \{s_0, s_1\}$, transition
relation $R = \{(s_0, s_0), (s_0, s_1), (s_1, s_0)\}$ (every edge except the
self-loop at $s_1$), and labeling $L(s_1) = \{p\}$, $L(s_0) = \emptyset$.

A **trace** is an infinite sequence $\pi = \pi(0), \pi(1), \pi(2), \ldots$ of
states; $\pi$ is **valid** iff $(\pi(i), \pi(i+1)) \in R$ for every $i$.

## Theorem

For every valid trace $\pi$ of $M$:

1. **Safety:** $\pi \vDash G\,\neg(p \land Xp)$ — the system never spends two
   consecutive steps in $s_1$.
2. **Recurrence:** $\pi \vDash G F \neg p$ — the system returns to $s_0$
   infinitely often.

## Proof

**Safety.** Fix a position $i$ and suppose for contradiction that
$\pi, i \vDash p \land Xp$. By the labeling, $p$ holds only at $s_1$, so
$\pi(i) = s_1$ and $\pi(i+1) = s_1$. Validity of $\pi$ gives
$(s_1, s_1) \in R$, contradicting the definition of $R$. Hence
$\pi, i \vDash \neg(p \land Xp)$ for every $i$, i.e.
$\pi \vDash G\,\neg(p \land Xp)$. $\square$

**Recurrence.** Fix a position $i$; we must find $j \geq i$ with
$\pi, j \vDash \neg p$, i.e. $\pi(j) = s_0$. Two cases on $\pi(i)$:

- *Case $\pi(i) = s_0$:* take $j = i$. ✓
- *Case $\pi(i) = s_1$:* since $(\pi(i), \pi(i+1)) \in R$ and the only edge
  out of $s_1$ is $(s_1, s_0)$, we get $\pi(i+1) = s_0$; take $j = i + 1$. ✓

In either case $F\neg p$ holds at $i$; since $i$ was arbitrary,
$\pi \vDash GF\neg p$. $\square$

## Remarks

- The safety property has the characteristic form: any violation would be
  witnessed by a *finite prefix* (two consecutive $s_1$'s), so refuting it
  needs only finite evidence. The recurrence property $GF\neg p$ is a
  *liveness*-style property: no finite prefix can refute it.
- The recurrence proof actually establishes something stronger: a **bounded
  response** — $s_0$ recurs within at most one step ($G(\neg p \lor X\neg p)$),
  which implies $GF\neg p$. Bounded liveness properties are safety properties
  in disguise; genuine liveness arises when no uniform bound exists.
- The Lean formalization (`lean_proof.lean`) models traces as functions
  $\mathbb{N} \to \mathrm{Fin}\ 2$ and proves both properties for *all* valid
  traces, quantifying over the (uncountable) set of traces — something no
  finite simulation could check. This is model checking by theorem proving
  rather than by state-space search (compare Chapter 14, §5).
