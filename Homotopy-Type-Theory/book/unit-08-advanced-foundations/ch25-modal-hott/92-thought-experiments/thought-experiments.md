# Thought Experiments — Chapter 25: Modal HoTT and Cohesive Geometry

## Thought Experiment 1: What Is a Space?

Before there was differential geometry, there were spaces. Before there were manifolds, there was the intuition that physical space is something you can move around in — it has points, paths, and directions. Euclid formalized flat space. Riemann generalized it to curved space. Einstein gave curvature a physical meaning.

But what *is* a space? A topologist says: a set with a topology. A differential geometer says: a set with a smooth structure. A physicist says: a space is whatever you can write an equation of motion on. A type theorist says: a space is a type.

Now consider: in cohesive HoTT, the same type $\mathbb{R}$ carries three different structures simultaneously:
- As a cohesive type: the real line with smooth structure, the line on which calculus happens
- As its flat shadow $\flat \mathbb{R}$: the set of real numbers, isolated points, no paths
- As its shape $\int \mathbb{R} \simeq \mathbf{1}$: the contractible homotopy type, a single point

*Which of these is the "real" real line?* Is $\mathbb{R}$ the smooth line, the discrete set, or the contractible space? Or is it all three simultaneously, and the different modalities are simply different *aspects* of a single object?

*How does this compare with Leibniz's account of space (as relational, not absolute) and with Newton's account (as an absolute container)? Is the cohesive account closer to one of these?*

## Thought Experiment 2: Gauge Fields as Types

In classical physics, a gauge field is a connection on a principal bundle — a function assigning to each point of spacetime a Lie algebra element, subject to certain transformation rules. Two gauge fields are "the same" if they are related by a gauge transformation.

In cohesive HoTT, a gauge field is an element of the type $\mathsf{Conn}(M, G)$. Two gauge fields are "the same" if there is a path between them — a gauge transformation. Gauge equivalence is literally HoTT equality.

*What are the philosophical implications?* In classical physics, the "physical state" is the gauge-equivalence class of a connection — not the connection itself. The connection is a representative. In cohesive HoTT, there is no separate notion of "equivalence class" — the moduli stack $\mathsf{Conn}(M, G)/\mathcal{G}$ is literally the type $\mathsf{Conn}(M, G)$ because paths in this type ARE gauge transformations.

Does this mean that in cohesive HoTT, the "gauge ambiguity" is absorbed into the identity structure of the type? Is this a feature or a problem?

*Compare with the HoTT philosophical position on identity*: in HoTT, there is no "raw equality" separate from proofs of equality. Identity is always witnessed. In the same way, gauge equivalence in cohesive HoTT is always witnessed by a gauge transformation — not just a formal equivalence class.

## Thought Experiment 3: The Shape Modality and the Flow of Time

The shape modality $\int A$ sends a space to its homotopy type, forgetting all smooth structure. For the real line $\mathbb{R}$: $\int \mathbb{R} \simeq \mathbf{1}$ — the real line is contractible.

Here is the disturbing consequence: the *timeline* $\mathbb{R}$ (the real line as the parameter of physical time) has trivial shape. Its homotopy type is a point. There are no topological "features" of the timeline.

But of course — the real line is contractible. You can continuously retract it to a point. Nothing topologically interesting happens along the timeline.

Now consider *time with a periodic identification*: the circle $S^1 = \mathbb{R}/\mathbb{Z}$. The shape of $S^1$ is the HoTT circle — $\int S^1 \simeq S^1$. The fundamental group $\pi_1(S^1) = \mathbb{Z}$ captures the "winding number" around the circle. Periodic time has topological features that linear time lacks.

*What does it mean for time to have a non-trivial shape?* In physics, periodic time arises in thermodynamics (thermal quantum field theory, where the inverse temperature is the period of imaginary time). Does the non-trivial shape of periodic time $S^1$ capture something physically meaningful that the trivial shape of linear time $\mathbb{R}$ misses?

*More concretely*: the partition function of a quantum field theory at finite temperature is the path integral over periodic time. In cohesive HoTT, this would be the integral $\int_{S^1}$ of some functional. The non-trivial shape of $S^1$ (specifically, the winding number) should be related to the statistical mechanics content of the theory.

## Thought Experiment 4: What Is a Connection?

A connection on a bundle allows you to "compare" fibers at different points — to ask whether an element of the fiber at $p$ is "the same" as an element of the fiber at $q$, when transported along a path.

In ordinary HoTT, this comparison is given by *transport*: for a family $P : A \to \mathsf{Type}$, transport along a path $\gamma : a =_A b$ gives a function $\mathsf{transport}(\gamma) : P(a) \to P(b)$.

In cohesive HoTT, there are two kinds of "sameness":
1. **HoTT transport**: via the identity type, using the path structure of the type
2. **Connection**: via the flat modality, using the cohesive structure

*What is the relationship between these two notions?* A flat connection (zero curvature) should correspond to HoTT transport — the cohesive transport equals the homotopy-type transport. A curved connection should give a transport that *differs* from the HoTT transport by the curvature.

*Can you make this precise?* Given a principal $G$-bundle $P : M \to BG$ and a connection $\nabla : M \to \flat BG$, how does the $\nabla$-transport along a path $\gamma : m_1 =_M m_2$ relate to the HoTT transport in the family $P$?

## Thought Experiment 5: The Discreteness of Constants

The flat modality $\flat A$ gives the "discrete shadow" of a type — the elements of $A$ with no continuous relationships between them. In physics, constants of nature are discrete in this sense: the speed of light $c$, Planck's constant $\hbar$, the electron charge $e$ are specific real numbers, but they don't vary continuously. They are constants.

In cohesive HoTT, a physical constant would be an element of $\flat \mathbb{R}$ — a "crisp" real number. A physical quantity that *can* vary — like the value of a scalar field at a point — is an element of $\mathbb{R}$ (the cohesive real line).

*Does this distinction between constants (crisp) and variables (cohesive) correspond to something physically meaningful?* In quantum field theory, the distinction between coupling constants and field values is physically important. Coupling constants can (in principle) be changed by changing the theory; field values are dynamical and vary from point to point.

*Can the crisp/cohesive distinction formalize the coupling constant/field value distinction?* Or is there a better cohesive account of this difference?

## Thought Experiment 6: Geometry from Logic

The modalities $\int, \flat, \sharp$ are logical operations — they transform propositions and types using universal properties. Yet they capture geometric concepts: shape (homotopy type), discreteness, and codiscreteness.

This raises a deep question: is geometry a consequence of logic (or type theory), or is it an additional ingredient?

In classical mathematics: geometry requires axioms about space (Euclid's postulates, or their modern equivalents). Logic alone does not determine geometry.

In cohesive HoTT: the geometry arises from adding three modalities with specified adjoint relationships. These are *logical* additions (modal operators), not *geometric* postulates (axioms about specific spaces).

*Does this mean geometry is "reducible" to logic (or type theory)?* Or are the modalities themselves geometric in nature, smuggling geometry into the logical framework under cover of adjoint functors?

*Compare with Kant's account of space*: Kant argued that space is not empirically given but is a form of intuition — a precondition for experience, not a feature of the world. Does the cohesive account support or undermine the Kantian view? (Hint: if space arises from modalities, and modalities are part of the logical structure of type theory, then space is in some sense *built into the logic*. Is this Kantian?)
