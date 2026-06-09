# 10.7 Catastrophe Theory

René Thom's catastrophe theory (1972) is bifurcation theory for gradient systems — systems of the form $\dot{x} = -\nabla_x V(x, \mu)$ where $V$ is a smooth potential function. The question is: what are the possible ways the equilibria of such a system can appear, disappear, or merge as parameters vary?

Thom proved that for up to 4 parameters, the answer is complete: there are exactly seven "elementary catastrophes," and every generic singularity is equivalent to one of them under smooth coordinate changes.

**Definition 10.7.1.** Catastrophe theory (Thom, 1972) classifies the stable singularities of smooth functions $f: {\mathbb R}^n \times {\mathbb R}^k \to {\mathbb R}$ (with $n$ state variables and $k$ parameters) under smooth equivalence.

**Theorem 10.7.2 (Thom's Classification Theorem).** For $k \leq 4$ parameters, every stable singularity is equivalent to one of seven elementary catastrophes:

| Name | Codim | Normal Form |
|------|-------|-------------|
| Fold | 1 | $x^3 + \mu_1 x$ |
| Cusp | 2 | $x^4 + \mu_1 x^2 + \mu_2 x$ |
| Swallowtail | 3 | $x^5 + \mu_1 x^3 + \mu_2 x^2 + \mu_3 x$ |
| Butterfly | 4 | $x^6 + \mu_1 x^4 + \mu_2 x^3 + \mu_3 x^2 + \mu_4 x$ |
| Hyperbolic umbilic | 3 | $x^3 + y^3 + \mu_1 xy + \mu_2 x + \mu_3 y$ |
| Elliptic umbilic | 3 | $x^3 - 3xy^2 + \mu_1(x^2+y^2) + \mu_2 x + \mu_3 y$ |
| Parabolic umbilic | 4 | $x^2 y + y^4 + \ldots$ |

**Application in Dynamics:** Catastrophe theory classifies the bifurcation diagrams of gradient systems $\dot{x} = -\nabla_x V(x, \mu)$. The "catastrophe" is the sudden jump in the equilibrium as $\mu$ varies through a cusp point.

The cusp catastrophe is the most famous and most applied. For a system governed by the cusp potential $V(x, \mu_1, \mu_2) = x^4/4 + \mu_2 x^2/2 + \mu_1 x$, the equilibrium surface is a fold in three-dimensional $(x, \mu_1, \mu_2)$-space. As you move in parameter space, the equilibrium can jump suddenly when you cross the fold curve — a "catastrophic" change in the state despite smooth variation of parameters.

This jump is the phenomenon of *hysteresis*: the equilibrium you're on depends not just on the current parameter value but on the history of how you got there. Catastrophe theory gives a complete classification of such phenomena for gradient systems with few parameters.

Thom's original vision was much broader — he claimed catastrophe theory could model phenomena from biology to linguistics — but the mathematical content is clear and rigorous for gradient systems.

---

## Looking Ahead

Bifurcation theory connects the local (what happens near an equilibrium) to the global (what qualitative structures appear as parameters vary). It is the language in which we describe how simple systems become complex.

Part III of this book develops the information-theoretic side of dynamics. The Shannon-McMillan-Breiman theorem (Chapter 23) connects the KS entropy of Chapter 7 to the typical behavior of long sequences. The renormalization theory of Section 10.6 previews a theme that will reappear: self-similarity and scaling laws as organizing principles for complex behavior.
