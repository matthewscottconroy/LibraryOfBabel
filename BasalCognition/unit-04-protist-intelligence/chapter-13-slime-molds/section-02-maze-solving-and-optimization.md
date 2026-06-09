# Section 2: Maze-Solving and Network Optimization

## Introduction

The Nakagaki et al. (2000) paper in *Nature* was brief — barely more than a page — and its elegance lay in its simplicity. The experiment was straightforward: place Physarum in a maze with food at the entrance and exit. Watch what happens. The result was striking enough that it needed no elaborate interpretation: the organism solved the maze, finding the shortest path with no false steps once the network had settled.

But the paper's importance went beyond the maze. It articulated, for the first time clearly and publicly, a principle that would become one of the central insights of unconventional computing: that adaptive physical networks can implement optimization algorithms without any centralized computation. The slime mold was not solving the maze the way a computer would solve it — by searching, backtracking, evaluating paths — but by physics. And the physics was enough.

---

## 2.1 The Maze Experiment

The experimental setup was clean and replicable. Physarum plasmodia were placed in a maze consisting of a flat chamber with barriers forming a series of branching pathways. Food was placed at two specific points: the entrance and the exit of the maze. Several possible paths connected entrance to exit, including at least one short direct path and several longer indirect paths.

Initially, the Physarum spread throughout the accessible space of the maze, occupying all available paths with protoplasmic tubes. There was no obvious preference for any particular route at this stage. But over the course of 4–8 hours, the network began to reorganize. Tubes in dead-end paths retracted. Tubes in longer indirect routes thinned. Tubes in the shortest path connecting the two food sources thickened and consolidated. By the end of the experiment, the Physarum had reorganized itself into a structure that approximated, and often exactly matched, the shortest path through the maze (Nakagaki, Yamada, & Tóth, 2000).

The mechanism is the one described in Section 1: flow reinforcement. Both food sources generated oscillatory perturbations in cytoplasmic flow. Where two oscillatory signals traveling from opposite directions converged on the same tube — as they did preferentially on the shortest path — the constructive interference between them produced stronger, more regular flow in that tube. Stronger flow reinforced the tube, making it thicker and more conductive. The positive feedback loop ran to completion: the preferred tube captured more and more of the total flow, while competing tubes that carried less flow were drained and collapsed.

The maze-solving behavior is not, in any interesting sense, "search." The organism does not evaluate paths, hold them in memory, compare them, and select the best one. It spreads everywhere at once, and the physics of its flow network selects the efficient path through a process of competitive reinforcement. The "solution" emerges from the self-organizing dynamics of the network under the constraints imposed by the geometry of the maze.

This is an important distinction that is sometimes glossed over in popular accounts. Physarum does not "think" its way to the shortest path. It occupies all paths and then lets flow dynamics prune the inefficient ones. The computational work is not done by any representational or symbolic process; it is done by the physical relaxation of the network to a low-energy state.

---

## 2.2 The Tokyo Rail Network

The maze paper attracted enormous attention, and Nakagaki's group followed it with a series of increasingly sophisticated experiments. The most famous of these, published in *Science* in 2010, compared the network that Physarum builds in a constrained environment to the Tokyo area rail network — one of the most efficient urban transportation systems in the world (Tero et al., 2010).

The experimental design was elegant: the researchers prepared a flat substrate (agar, to mimic the terrain) on which they placed oat flakes at positions corresponding to the 36 major cities and towns in the greater Tokyo area. They placed Physarum at the position of Tokyo itself. Obstacles (salt, which Physarum avoids) were placed to represent geographic features — mountains, large water bodies — that constrain where rail lines can run.

The Physarum grew, explored, and over approximately 24 hours reorganized its network to connect the food sources efficiently. The final network — photographed and digitized — was then compared quantitatively to the actual Tokyo rail network using measures of efficiency, fault tolerance, and cost (defined as total tube length).

The results were striking. The Physarum network closely resembled the actual rail network on all three metrics. It was nearly as efficient (defined as average shortest-path length between all pairs of nodes), nearly as fault-tolerant (defined as the fraction of node pairs that remain connected after a single node failure), and of comparable cost. In some respects, the Physarum network was actually slightly better than the human-engineered system on efficiency and fault tolerance while being comparably economical in total length (Tero et al., 2010).

The authors extracted from the Physarum dynamics a mathematical model of adaptive network formation and showed that this model could reproduce not just the Tokyo result but also qualitatively similar networks when initialized with other city distributions. The model had three key elements: reinforcement of tubes carrying high flow, decay of tubes carrying low flow, and a noise term that prevents the network from becoming trapped in local optima. With these three elements, the model robustly found networks with a good balance of efficiency, robustness, and cost.

---

## 2.3 The Mechanism: Flow Reinforcement and Competitive Dynamics

It is worth examining the mechanism in slightly more mathematical detail, because doing so illuminates both the power and the limits of Physarum as a computing substrate.

Consider a simple network with two paths connecting two food sources. Path A is shorter; path B is longer. Both paths contain tubes of initial diameter D. Cytoplasm flows through both paths in response to pressure differences generated by the oscillatory contractions of the tube walls.

The flow rate through a tube depends on the pressure gradient and on the tube's conductivity, which scales approximately with the fourth power of the tube radius (Hagen-Poiseuille law for viscous flow through a cylinder). This strong dependence on radius means that small differences in tube diameter lead to large differences in flow rate.

The key feedback is the rule for diameter change:

dD/dt = f(|Q|) - D

where Q is the volumetric flow rate through the tube and f is an increasing function. This rule says that tube diameter increases when flow is high (above some threshold) and decreases when flow is low. The flow itself depends on the network conductivity, which depends on the diameters. This creates a self-reinforcing dynamics: high-flow tubes grow, increasing their conductivity, capturing more flow, growing further. Low-flow tubes shrink, decreasing their conductivity, losing more flow, shrinking further (Tero et al., 2007).

In a two-path network, this dynamics reliably converges to the shorter path, because the shorter path offers lower resistance for a given diameter — which means flow through the shorter path is slightly higher from the beginning — which means the shorter path's tubes are reinforced slightly faster — which means the shorter path's advantage compounds until it has captured all the flow and the longer path has regressed to nothing.

This is differential reinforcement learning in a physical network. The fact that it converges on efficient solutions is not magic; it follows from the mathematical properties of the flow equations and the reinforcement dynamics. But the biological implementation — in a living organism with no nervous system — is extraordinary.

---

## 2.4 Generalizations and Limitations

Subsequent work extended the Physarum network optimization paradigm in several directions. Researchers have used Physarum to explore networks for other real-world transportation systems — including the road networks of the United Kingdom, Canada, and portions of the Roman Empire — with broadly similar results: Physarum networks tend to be qualitatively similar to good human engineering solutions, particularly when obstacle-avoidance constraints are imposed to mimic terrain (Adamatzky & Jones, 2010).

It is important to be clear about what Physarum is and is not doing in these experiments.

**What Physarum does well**: It finds networks with good balance between efficiency (path length), robustness (fault tolerance), and cost (total material). It does this through a biologically implemented optimization dynamic that is well-suited to exactly these criteria. The biology has been shaped by evolution to find efficient networks connecting food sources while using minimal material — which are the same criteria that good transportation network design optimizes.

**What Physarum does not do**: It does not solve arbitrary optimization problems. The problems it solves are those that can be recast as network flow optimization under the specific reinforcement rule that its biology implements. Problems that require different computational structures — Boolean logic operations, sequential computation, sorting — are not naturally solved by Physarum flow dynamics without additional engineering.

**What the comparison reveals**: The similarity between Physarum networks and human-engineered transportation networks is not just evidence for Physarum's capabilities. It is also evidence that evolution has shaped biological network formation to optimize the same criteria that human engineers care about — efficiency, robustness, economy — because organisms with better-connected networks for resource transport outcompeted those with worse ones. The slime mold is a biological record of an optimization pressure that shaped the physics of life itself.

The slime mold cannot solve your differential equations. But it can build you a Tokyo subway system. The question of whether this constitutes intelligence depends on what you think intelligence is for.

---

## References

Adamatzky, A., & Jones, J. (2010). Road planning with slime mould: If Physarum built motorways it would route M6/M74 through Newcastle. *International Journal of Bifurcation and Chaos*, 20(10), 3065–3084.

Nakagaki, T., Yamada, H., & Tóth, Á. (2000). Maze-solving by an amoeboid organism. *Nature*, 407, 470.

Tero, A., Kobayashi, R., & Nakagaki, T. (2007). A mathematical model for adaptive transport network in path finding by true slime mold. *Journal of Theoretical Biology*, 244(4), 553–564.

Tero, A., Takagi, S., Saigusa, T., Ito, K., Bebber, D. P., Fricker, M. D., ... & Nakagaki, T. (2010). Rules for biologically inspired adaptive network design. *Science*, 327(5964), 439–442.
