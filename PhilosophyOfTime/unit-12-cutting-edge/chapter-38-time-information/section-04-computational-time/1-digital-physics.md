# 38.4.1 Digital Physics and Computational Models of Time

---

## The Computational Universe

The proposal that the universe is fundamentally a computation has a long history in theoretical physics and computer science. Konrad Zuse, the German engineer who built the first programmable computers in the 1940s, proposed in *Rechnender Raum* (*Calculating Space*, 1969) that the physical world is the output of a discrete cellular automaton running in a two-dimensional space. Edward Fredkin and Tommaso Toffoli developed reversible cellular automata models of physics, attempting to show that conservative physical laws could be implemented by reversible computational rules. Stephen Wolfram's *A New Kind of Science* (2002) argued that simple cellular automata could generate the full complexity of physical phenomena and that the universe might be running on some such computational substrate.

The core idea is deceptively simple: a physical state is a configuration, time is sequential updating, and physical laws are the rules by which configurations update. On this view, time is nothing more or less than the discrete sequence of computational steps — the ticks of the cosmic computer clock. There is no continuous temporal flow beneath the discrete steps; the steps are all there is.

## What Digital Physics Implies for the Nature of Time

If the universe is a computation, several features of time follow immediately.

**Discreteness.** Computational time is intrinsically discrete: there is a first step, a second step, a third step. There is no "between" two steps — no time between time-steps. The apparent continuity of time would be an emergent or approximate feature arising from the fact that the time-step is extraordinarily small (plausibly at the Planck scale, ~5 × 10⁻⁴⁴ seconds). The smooth continuum of classical physics would be an approximation to an underlying discrete computational sequence.

**Direction.** If the update rules of the cosmic computer are irreversible, then computational time has an intrinsic direction: the direction of computation. States are computed from earlier states, not vice versa. This provides a natural ground for the arrow of time that does not rely on the Past Hypothesis or thermodynamic considerations — it is built into the structure of the computation.

**Finiteness.** A finite-state computer (a computer with a finite number of distinct states) will eventually repeat a state — a form of Poincaré recurrence for computation. If the universe is a finite automaton, recurrence is guaranteed in principle. The timescale is so astronomically long as to be physically irrelevant, but the implication — that the universe has a period, a cycle time — is philosophically striking.

**Simulation.** If the universe is a computation, it might in principle be simulated by another computation. This is the "simulation hypothesis": the possibility that our universe is a simulation running on hardware in a "higher" universe. Nick Bostrom (2003) argued that at least one of three propositions must be true: (1) almost all civilizations go extinct before developing the computational power to run universe-scale simulations; (2) almost no civilizations with such power choose to run such simulations; (3) we are almost certainly living in a simulation. The simulation hypothesis raises profound questions about the nature of time: is "our" time the time of the simulation, the time of the hardware, both, or neither?

## Wolfram's New Kind of Science

Stephen Wolfram's most developed version of computational physics proposes that the universe is a specific kind of structure — a graph or network — that evolves by simple rewriting rules. In his 2020 "Wolfram Physics Project," he proposed that space is a hypergraph (a generalization of a graph where edges can connect more than two nodes) and that time consists of the application of rewriting rules that locally update this hypergraph.

On Wolfram's picture, time is genuinely computational: the universe is the accumulation of rule applications, and what we experience as temporal order is the causal order of these rule applications. General relativity and quantum mechanics, Wolfram argues, emerge as approximate large-scale features of the system's behavior.

The philosophical interest of this approach lies in its treatment of the arrow of time. Wolfram argues that the arrow of time is computational: it is the direction in which the rewriting rules are applied. The irreversibility is built in at the fundamental level — not derived from statistical mechanics or the Past Hypothesis — because the rewriting rules generate new structure (they do not erase it). Each application of the rules creates new hypergraph vertices that did not previously exist.

## Critical Assessment

The computational universe proposal faces several serious objections.

**The empirical objection.** Cellular automata and graph-rewriting models produce specific, testable predictions that differ from those of ordinary quantum field theory and general relativity. The Wolfram Physics Project, for example, predicts that Lorentz invariance should be violated at small scales (since the underlying hypergraph has a preferred structure). No such violation has been detected. The empirical track record of computational universe proposals is currently poor.

**The conceptual objection.** "Computation" is a mathematical abstraction that requires an interpreter: a computation computes something *for someone*. A physical world without observers or interpreters is not obviously a "computation" in any meaningful sense. The computational analogy may be misleading. As Earman and Norton (2013) have argued, the "universe as computation" framing inherits all the ambiguities of the computational metaphor applied to physical systems.

**The time problem.** If time is just sequential computational steps, what determines the *rate* at which the computer runs? In a genuine computer, the clock rate is set by external physics — the oscillation of a quartz crystal or cesium atom. The cosmic computer has no external clock; its "rate" is not defined relative to anything. The concept of "time" in digital physics threatens to be vacuous: "time" is just the labeling of successive states by integers, with no physical content beyond the ordering.

**Simulation skepticism.** The simulation hypothesis is not falsifiable in any obvious way. If our universe is a perfect simulation, it would look exactly like a non-simulated universe from the inside. This makes it epistemically idle: no evidence could distinguish our being simulated from our not being simulated.

## What Computational Models Illuminate

Despite these objections, computational approaches to physics have generated genuine insights. The connection between information erasure and thermodynamic entropy (Landauer's principle, Section 38.3) shows that computation has physical consequences and that the thermodynamic arrow of time can be understood as the direction of irreversible computation. Cellular automata models have illuminated how complexity and apparent irreversibility can emerge from simple reversible rules. And the simulation argument, whatever its ultimate merits, has sharpened questions about the ontological status of time and the relationship between mathematical structures and physical reality.

The most defensible lesson of digital physics is modest: the concepts of information, computation, and sequential state-updating provide a powerful and illuminating framework for thinking about physical dynamics, including the arrow of time. This is compatible with the view that the universe is not literally a computation but merely *modeled well* by computational frameworks at certain scales and in certain respects.

---

**References**

Bostrom, Nick. 2003. "Are You Living in a Computer Simulation?" *Philosophical Quarterly* 53 (211): 243–255.

Fredkin, Edward, and Tommaso Toffoli. 1982. "Conservative Logic." *International Journal of Theoretical Physics* 21 (3–4): 219–253.

Wolfram, Stephen. 2002. *A New Kind of Science*. Champaign: Wolfram Media.

Wolfram, Stephen. 2020. *A Project to Find the Fundamental Theory of Physics*. Champaign: Wolfram Media.

Zuse, Konrad. 1969. *Rechnender Raum*. Braunschweig: Vieweg.
