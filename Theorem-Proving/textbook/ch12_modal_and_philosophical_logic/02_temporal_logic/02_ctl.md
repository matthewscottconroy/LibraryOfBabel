# Computation Tree Logic (CTL)

## Branching vs. Linear Time

**LTL** (Linear Temporal Logic) reasons about a single linear future — it quantifies over *all possible futures* implicitly. **CTL** makes the branching structure of time explicit: it can say things like "there is *some* possible future in which $\varphi$ holds" vs. "in *every* possible future, $\varphi$ holds."

CTL uses **path quantifiers**:
- **A** (All paths): the property holds on all computation paths
- **E** (there Exists a path): the property holds on some computation path

Combined with temporal operators: **X** (next), **F** (finally/eventually), **G** (globally/always), **U** (until).

**CTL requires the A/E and X/F/G/U to be paired together**:

| Formula | Reading |
|---------|---------|
| $AG\, p$ | On all paths, $p$ is always true (safety: $p$ never fails) |
| $EF\, p$ | On some path, $p$ eventually becomes true |
| $AF\, p$ | On all paths, $p$ eventually becomes true (liveness) |
| $EG\, p$ | On some path, $p$ is always true (there is an infinite $p$-run) |
| $AG(p \to AF\, q)$ | Whenever $p$, $q$ will eventually hold (response property) |
| $EF\, EG\, p$ | It is possible to reach a state from which $p$ holds forever |

## LTL vs. CTL

CTL and LTL are **incomparable** — neither subsumes the other:

- **"$p$ holds infinitely often on all paths"** = $AG(AF\, p)$ in CTL; $GF\, p$ in LTL. Both express this, but in different ways.
- **"On some path, $p$ always holds"** = $EG\, p$ in CTL; not expressible in LTL (LTL ranges over all paths, can't say "some path").
- **"$p$ holds until $q$ on all paths"** = $A(p\, U\, q)$ in CTL. "$p\, U\, q$" in LTL. But LTL's $p\, U\, q$ is not directly a CTL formula.

## CTL Model Checking

CTL model checking runs in polynomial time $O(|M| \cdot |\varphi|)$ — very efficient. This is why CTL is preferred for automated hardware and software verification (see ch13/02).

The algorithm labels each state with the subformulas that hold there, working bottom-up through the formula structure.

## Applications

**Hardware verification**: "After every memory write, the cache is eventually invalidated" — $AG(\text{write} \to AF\, \text{invalidated})$.

**Protocol verification**: "From any state, it is always possible to reach the initial state" — $AG(EF\, \text{initial})$ (reachability of reset).

**Mutual exclusion**: "Two processes are never simultaneously in the critical section" — $AG\, \neg(\text{cs}_1 \wedge \text{cs}_2)$.

## Exercises
See [problems/ch12_modal_logic/02_temporal_exercises.md](../../../problems/ch12_modal_logic/02_temporal_exercises.md)
