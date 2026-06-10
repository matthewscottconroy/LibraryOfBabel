# Thought Experiments — Chapter 23: Cubical Type Theory

## Thought Experiment 1: The Stuck Machine

Imagine a programming language where you can write any well-typed program, but some programs — when they compile correctly and are given correct inputs — simply freeze. Not because of an infinite loop, but because the evaluator reaches an expression it has no rule to reduce. The program is semantically meaningful, the type is correct, but the machine cannot proceed.

This is the situation with Book HoTT and canonicity. The expression `transport (ua succ-equiv) 3` is well-typed (its type is ℕ, since succ-equiv : ℕ ≃ ℕ). We know semantically that its value should be 4. But there is no reduction rule. The machine stops.

Now imagine two ways of fixing this:

**Fix A**: Add a reduction rule as an axiom: `transport (ua e) a = e a`. This works, but the axiom has to be checked as a special case every time `ua` appears. The machine resumes, but only because we told it "trust me, this reduces to `e a`."

**Fix B**: Change what `ua(e)` *means*. Make it a concrete object — a function of the interval — with a *definition*. Then `transport (ua(e)) a` reduces by definition, not by trust.

CCHM is Fix B. The Glue type is what makes `ua(e)` a definition rather than a postulate. The computation rule is not an extra axiom — it's a consequence of the Glue type's reduction rules.

*What does this tell us about the difference between asserting something and defining it?*

## Thought Experiment 2: The Interval as Time

Here is an operational metaphor for the CCHM interval. Think of dimension variables as *time parameters*. A path $p : a =_A b$ is a process: at time $i = 0$ the process is in state $a$, and at time $i = 1$ it is in state $b$. The path $p(i)$ is the state of the process at time $i$.

Under this metaphor:
- Reflexivity `refl_a` is a stationary process: always in state $a$
- Path reversal `sym p` is the time-reverse: run the process backward
- Path concatenation $p \cdot q$ is sequential composition: first run $p$, then run $q$
- A homotopy $H : p = q$ is a family of processes indexed by a second time parameter $j$: at $j = 0$ the process runs as $p$, at $j = 1$ it runs as $q$, and in between it smoothly deforms

The `hcomp` operation is: given a partial specification of what the process should do (defined on some faces of the time-cube), fill in the missing parts consistently.

Under this metaphor, what would it mean for *transport* to have a computation rule? Transport says: if the *type* of state is changing over time (the process runs in different type-spaces at different times), you can still move a state from time 0 to time 1. The computation rule says: when the type changes by an equivalence $e$ (i.e., via `ua(e)`), moving the state through time applies $e$ to it.

*Is this metaphor faithful? Where does it break down? What would a "partial process" (a partial element) be in operational terms?*

## Thought Experiment 3: What If We Used a Boolean Algebra?

The CCHM interval $\mathbb{I}$ is a De Morgan algebra, not a Boolean algebra. The critical difference: in a Boolean algebra, every element satisfies either $i = 0$ or $i = 1$ (excluded middle). In $\mathbb{I}$, the interior points — those where neither $i = 0$ nor $i = 1$ holds — are essential.

Suppose we tried to define cubical type theory using a *Boolean* algebra on the interval. This would mean: every dimension variable either collapses to 0 or to 1. There are no interior points.

Under this hypothesis:
- The type $a =_A b$ would consist only of terms $p$ where $p(0) = a$ and $p(1) = b$, but the "interior" of $p$ would not exist as a coherent object — every interior point would collapse to an endpoint.
- There would be no room for homotopies between paths (which require a genuine 2-cube, with interior points in *both* dimensions).
- Higher-dimensional coherences would vanish entirely.

*What does this tell us about why the axiom of excluded middle is incompatible with HoTT? Why does genuine homotopy theory require "interior points" in its paths?*

## Thought Experiment 4: Glue as a Patching Construction

The Glue type can be understood as a *patching* construction. Here is an analogy.

You have two overlapping pieces of fabric: a red piece $T$ and a blue piece $B$. On the overlap (specified by $\phi$), there is a continuous bijection $e$ between corresponding points of the red and blue pieces. The Glue type is the *patched fabric* — the single piece of fabric you get by gluing $T$ and $B$ along their overlap via $e$.

Now consider what it means to "transport" across the patched fabric. If you start at a point on the red side and move to the blue side, you cross the boundary where $e$ connects them. The transport rule says: crossing from red to blue applies $e$ to your position.

The path `ua(e)` is the patch: at $i = 0$ you are on the red side (type $A$), at $i = 1$ on the blue side (type $B$), and the Glue type smoothly connects them via $e$.

*How does this analogy extend to the case where $\phi$ specifies only part of a face (not the whole endpoint)? What would a "partial patching" mean geometrically?*

## Thought Experiment 5: The Categorical Identity of Cubical Sets

In classical algebraic topology, a *CW complex* is a space built by attaching cells: you start with a set of 0-cells (points), glue 1-cells (edges) along their endpoints, glue 2-cells (disks) along their boundaries, etc. The Kan condition on a simplicial set says: any sphere (boundary of a simplex with one face removed) can be filled.

The cubical analogue: a type in CCHM is a fibrant cubical set — any open box (cube with one face missing) can be filled. The `hcomp` operation is the box-filler.

Now here is a thought experiment. Suppose you have a type $A$ in CCHM and you want to verify it satisfies the Kan condition at dimension 2. You need to show: for any specification of three faces of a square (left, right, bottom), there exists a top face making a coherent square.

Write down this condition as a type in CCHM itself. What does it look like? (Hint: it is a statement about `hcomp`.)

*Is the Kan condition something you can prove for a type, or is it something that's guaranteed by the type theory's rules? What is the difference between a fibrant type and a non-fibrant type in CCHM?*

## Thought Experiment 6: XTT and the "Sameness" of Boundaries

XTT adds *boundary separation*: terms that agree on all faces are definitionally equal.

Here is a consequence. In CCHM, the left unit law for path concatenation ($\text{refl} \cdot p \sim p$) is a *propositional* equality — a path, not a definitional equality. The two paths $\text{refl} \cdot p$ and $p$ are not the *same term* by computation; they are merely *connected* by a 2-cube.

In XTT, these two terms agree on all faces of the 1-cube (they both evaluate to $a$ at $i = 0$ and to $b$ at $i = 1$). By boundary separation, they are definitionally equal.

*Is definitional equality between $\text{refl} \cdot p$ and $p$ desirable? What are the advantages (simpler proofs, no need to rewrite) and disadvantages (harder type checking, more complex equational theory) of having more definitional equalities?*

*More generally: what should "equality" mean in a type theory? Is definitional equality the right notion of "sameness for computation," or should it be reserved for something stricter?*
