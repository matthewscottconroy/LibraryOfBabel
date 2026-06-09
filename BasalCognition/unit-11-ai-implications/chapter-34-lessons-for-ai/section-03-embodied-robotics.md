# Section 3: Embodied Robotics

## The Insect That Outperforms the Robot

In 1991, Rodney Brooks published a paper that should have been more disruptive than it was. "Intelligence Without Representation" (Brooks, 1991) argued not just for a different approach to robotics but for a different theory of what intelligence is. The paper's central claim was deceptively simple: the world is its own best model. If a robot needs to know where an obstacle is, the most reliable and computationally cheapest approach is not to build an internal map of obstacles and update it continuously — it is to look at the obstacle when you need to avoid it. Internal models are expensive, slow to update, and always somewhat wrong. The world, on the other hand, is always right, is always current, and is available for free.

This was not just a practical engineering heuristic. It was a philosophical position about the nature of intelligence: that cognition is not, at its core, about constructing and maintaining accurate internal representations, but about maintaining a productive loop of action and perception with an environment. This view, which Brooks had already demonstrated in hardware with his subsumption architecture robots, resonated deeply with the enactivist philosophy of Varela, Thompson, and Rosch (1991) and, as we now recognize, with what we know about basal cognition in non-neural organisms.

## Subsumption Architecture

Brooks's subsumption architecture is a layered system of behavior-producing modules, each of which reads sensory input and produces motor output directly, without passing through a central world model. Crucially, higher-level behaviors do not command lower-level ones — they *subsume* them, selectively suppressing or modifying their outputs. The lowest layers handle the most primitive behaviors — collision avoidance, for instance — and are always running, always influencing the robot's behavior. Higher layers implement more complex behaviors but work on top of the primitive layer's guarantees.

The result is a system that cannot be paralyzed by uncertainty. Traditional planning-based robots could be brought to a halt by incomplete information — if the map is wrong, the plan fails. Brooks's robots kept moving, kept sensing, kept adapting, because their behavior was always driven by current sensory data rather than by a potentially stale internal model. Early demonstrations — the robot Allen navigating a cluttered office, the six-legged robot Attila walking across rough terrain — showed that surprisingly sophisticated behavior could emerge from this architecture with no more than a handful of behavior layers (Brooks, 1991).

The architecture was explicitly biological in inspiration. Brooks noted that the insect nervous system — which implements something functionally similar, with reflex arcs and behavioral hierarchies — produces robust navigation in complex environments without anything like the symbolic world models of AI planning systems. The cockroach navigates at speeds and over terrain that would challenge any robot of its time. The thesis was that the cockroach had solved the right problem, while classical AI had been solving a different problem — one that was cleaner mathematically but less relevant to intelligent behavior in the physical world.

## Soft Robotics: The Body as Computation

A second strand of biologically inspired robotics has developed from a different direction: the recognition that rigid, articulated robot bodies are unusual in the biological world. Most of the organisms discussed in this book — and most animals — do not have rigid link-and-joint skeletons. They are soft, compliant, and continuous. The tentacle of an octopus, the trunk of an elephant, the body of a caterpillar — these are not jointed manipulators with clearly defined degrees of freedom. They are continuous deformable structures whose mechanics are themselves part of the control system.

Soft robotics takes this seriously (Rus & Tolley, 2015). If a robot arm is made of soft, elastic material, then the arm itself can perform much of the mechanical computation that a rigid robot would need to do with sensors and actuators. A soft gripper does not need to precisely compute the orientation and stiffness of an object it is grasping — the soft material conforms to the object, distributing contact forces in a way that automatically handles geometric uncertainty. The compliance of the body is doing cognitive work.

This principle — morphological computation, the offloading of information processing into the physical properties of the body — is fundamental to biological locomotion (Pfeifer, Lungarella, & Iida, 2007). The passive dynamics of a human leg during walking — the pendulum-like swing of the lower leg, the elastic storage and return of energy in the Achilles tendon — mean that the nervous system does not need to explicitly compute every detail of the gait cycle. The physics does much of the work. Similarly, the passive dynamics of a slime mold's cytoplasmic flow implement an analog computation of path optimization that the organism does not need to represent or plan explicitly.

Soft robotics is still young as a field, but it has already produced systems that outperform rigid robots in certain manipulation tasks, that can navigate through confined spaces that would be impenetrable by rigid systems, and that are inherently safer in contact with humans. The principles that make them work are biological: compliance, distributed sensing, and the exploitation of body dynamics as a computational resource.

## Evolutionary Robotics

A third tradition within embodied robotics takes the biological parallel most literally: it uses evolutionary algorithms to design robot bodies and control systems together, rather than separating them. In evolutionary robotics, a population of candidate robots (or simulated robots) is evaluated on a task, the best performers reproduce with variation, and the process iterates. The result is robot designs that were not explicitly engineered — they emerged from an optimization process that explored the space of possible body-controller combinations (Sims, 1994).

The most important finding from evolutionary robotics is that body and brain co-evolve in ways that are mutually enabling. A body shape that is easy to control turns out to be very different from a body shape optimized for strength or speed independently of control requirements. Evolutionary robotics consistently finds solutions in which the body's passive dynamics and the controller's active dynamics are tightly coupled — solutions that a human engineer designing body and controller separately would be unlikely to discover (Pfeifer & Bongard, 2007).

This co-evolution is precisely what we observe in biological organisms. The proprioceptive feedback that the cerebellum uses to coordinate limb movement, for instance, exploits regularities in limb mechanics that evolution has tuned over millions of years. The plant root's capacity to navigate around obstacles by growing through soil exploits the mechanical properties of the root tip, which is structured precisely for deformable penetration of granular media (Bengough et al., 2011). In each case, the body and the control system have been shaped together, by the same selection pressure, into a coherent whole.

## Levin's Target Morphology Approach

Michael Levin's work on bioelectricity, discussed extensively in earlier chapters of this book, has a specific relevance to robotics that is worth drawing out explicitly. Levin and colleagues have argued that the regeneration and development of biological organisms — from planarian flatworms to developing embryos — is best understood not as a set of local cell-by-cell instructions but as a goal-directed process in which the organism is "trying" to achieve a target morphology (Levin, 2021).

The target morphology concept is striking because it inverts the usual direction of biological explanation. Instead of asking "what local signals cause this cell to do what it does?" Levin's framework asks "what is the whole system converging toward, and how does local cellular behavior implement that convergence?" The answer involves bioelectric gradients — patterns of ion flow and membrane voltage across tissue — that encode positional information about the organism's current state relative to its target state, and that guide cell behavior (including cell division, migration, and differentiation) toward closing that gap.

For robotics, this suggests a different way of specifying robot behavior. Rather than writing a control program that specifies what the robot should do in each situation, one might specify a target state — a desired morphology or configuration — and design the robot's dynamics so that it converges to that state from a range of initial conditions. This is how biological morphogenesis works, and it confers the remarkable robustness of biological development: a planarian flatworm can be cut into many pieces, each of which will regrow the missing parts, because each piece contains the information needed to converge back to the target morphology (Levin, 2021).

The engineering translation of this idea — morphogenetic robotics — is in its early stages, but it is one of the more intellectually ambitious directions in the field. It would produce robots that can self-repair, that can reconfigure their bodies in response to damage, and that are specified by goals rather than programs. That is a description that applies, in a biological sense, to almost every multicellular organism on Earth.

## The Common Thread

What unites subsumption architecture, soft robotics, evolutionary robotics, and the target morphology approach is a rejection of the classical AI assumption that intelligence is a brain-in-a-box problem — a problem of getting the central processing right, with the body serving as a passive input-output interface. Each of these approaches recognizes, in a different way, that the body is part of the cognitive system, and that designing the body is as much a cognitive engineering problem as designing the algorithm.

This is not a romantic or anti-scientific position. It is an engineering position, grounded in the observation that biological systems — which have been optimizing for robust, efficient performance in complex physical environments for hundreds of millions of years — consistently exploit the cognitive resources of their bodies in ways that our engineering intuitions have been slow to appreciate.

The slime mold does not carry a map of the maze; it explores the maze with its body. The bacterium does not store a model of the chemical gradient; it computes the gradient in real time with its receptor adaptation machinery. The plant root does not plan a route through the soil; it grows adaptively in response to local mechanical and chemical cues. These are not inferior versions of cognition; they are existence proofs that intelligent behavior in complex environments can be achieved without the computational overhead we have assumed to be necessary.

---

## References

Bengough, A. G., McKenzie, B. M., Hallett, P. D., & Valentine, T. A. (2011). Root elongation, water stress, and mechanical impedance: A review of limiting stresses and beneficial root tip traits. *Journal of Experimental Botany*, 62(1), 59–68.

Brooks, R. A. (1991). Intelligence without representation. *Artificial Intelligence*, 47(1–3), 139–159.

Levin, M. (2021). Bioelectric signaling: Reprogrammable circuits underlying embryogenesis, regeneration, and cancer. *Cell*, 184(8), 1971–1989.

Pfeifer, R., & Bongard, J. (2007). *How the Body Shapes the Way We Think: A New View of Intelligence*. MIT Press.

Pfeifer, R., Lungarella, M., & Iida, F. (2007). Self-organization, embodiment, and biologically inspired robotics. *Science*, 318(5853), 1088–1093.

Rus, D., & Tolley, M. T. (2015). Design, fabrication and control of soft robots. *Nature*, 521(7553), 467–475.

Sims, K. (1994). Evolving virtual creatures. In *Proceedings of the 21st Annual Conference on Computer Graphics and Interactive Techniques (SIGGRAPH '94)* (pp. 15–22). ACM.

Varela, F. J., Thompson, E., & Rosch, E. (1991). *The Embodied Mind: Cognitive Science and Human Experience*. MIT Press.
