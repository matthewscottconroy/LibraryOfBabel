# Section 4: Physarum as a Computing Substrate

## Introduction

Andrew Adamatzky is a professor at the University of the West of England in Bristol, and he may be the most enthusiastic advocate of Physarum polycephalum as a computing device currently working in any laboratory anywhere in the world. Over the past two decades, his group has explored whether the self-organizing dynamics of slime mold networks can be used to perform computational operations — not just the optimization tasks described in the previous section, but genuine logical and arithmetic operations.

The results are fascinating, the limitations are real, and the implications for our understanding of what computation is are more subtle than either enthusiastic popular accounts or dismissive critics suggest.

---

## 4.1 Logical Operations in Physarum Networks

Adamatzky's basic experimental strategy is to represent logical variables as the presence or absence of food sources at specific positions in a Physarum network. The organism's network response — whether it connects or fails to connect specific positions, which paths it reinforces, how it routes cytoplasmic flow — constitutes the output of a computation. By appropriate placement of inputs and outputs, and appropriate design of the spatial geometry, Adamatzky has claimed to demonstrate Physarum implementing several logical gate operations.

**Boolean gates**: By arranging food sources at positions designated as inputs A and B, and monitoring whether a protoplasmic connection forms at a designated output position, the geometry can be designed so that connections form only when both inputs are present (AND gate), when either input is present (OR gate), or when an input is absent (NOT gate). The formal publication of this work (Adamatzky, 2010) provided photographic evidence of these configurations in living Physarum cultures.

**Path selection**: A related set of experiments showed that Physarum, navigating between a source and a destination, preferentially selects paths that avoid repellents (light, salt) and include attractants (food). By placing repellents and attractants strategically, the path the organism takes can be used to encode a computational result.

These demonstrations are genuine — there are published photographs and videos of Physarum doing exactly what is described. The question is what they mean.

---

## 4.2 What Is Actually Being Computed?

Here we need to be careful. There is a temptation, which popular science writing occasionally succumbs to, of treating the demonstration that Physarum can implement a logical gate as evidence that the slime mold is performing digital computation in any deep sense. This overclaims.

Consider the AND gate example. The geometry is arranged so that protoplasmic connections form at the output position only when both food sources are present. This works — but it works because of the physics of network formation: when two food sources are both present, the flow dynamics establish connections to both, and the resulting network topology includes a path through the output region. When only one food source is present, the network forms efficiently to that source, but not through the output region.

The Physarum is not, in any meaningful sense, computing the conjunction of two propositions. It is forming a network according to flow dynamics, and the experimenter has arranged the geometry so that the network that forms under these conditions happens to pass through a region designated as "output true" if and only if both designated inputs are present. The logical operation is an emergent consequence of the geometry and the physics — it is not implemented in any representational or symbolic machinery inside the organism.

This distinction matters for two reasons. First, it limits generalizability: the "AND gate" works because the experimenter designed the spatial arrangement to produce this outcome. A different logical operation requires a different spatial arrangement, designed by a human, for the same organism. The organism is not programmable in the way a digital computer is. Second, it clarifies the nature of the computation: what Physarum computes is determined by its physics and the boundary conditions imposed by the experimental setup, not by any internal program.

Both of these points have been acknowledged by Adamatzky himself, who is a sophisticated thinker about these issues. His position, stated clearly in his books on unconventional computing (Adamatzky, 2010), is not that Physarum replaces silicon but that it demonstrates a class of computation — "collision-based computing," "reaction-diffusion computing," "network computing" — that is physically implemented and therefore potentially useful for specific problem domains where biological-scale, massively parallel, self-organizing computation would be advantageous.

---

## 4.3 Real Capabilities and Real Limitations

To summarize what the Physarum computing literature has established with reasonable rigor:

**Genuinely demonstrated capabilities**:
- Path optimization between two or more nodes under cost constraints
- Network formation that approximates minimum spanning trees and Steiner trees
- Avoidance of obstacles and repellents in maze-like geometries
- Periodic behavior that can be entrained to external stimuli

**Capabilities that have been demonstrated experimentally but require careful interpretation**:
- Logical gate operations (real behavior, but not "true" symbolic logic)
- Robot control using Physarum-derived outputs (interesting proof-of-concept, limited scalability)

**Capabilities not demonstrated**:
- Programmable computation (the ability to execute arbitrary algorithms)
- Reliable, reproducible output over many trials with the same setup (there is biological variability)
- Scaling to complex problems (the maze experiments work at centimeter scales; it is not clear they scale to larger networks without loss of performance)

The biological variability point deserves emphasis. Physarum is a living organism, and its behavior is subject to the biological noise intrinsic to living systems — variation in genetic background, physiological state, history of previous encounters with stimuli. The demonstrations in published papers represent successful cases, but the failure rate and the between-experiment variability are not always clearly reported. This is a real limitation for any engineering application of Physarum computing.

---

## 4.4 The Deeper Significance

Despite these limitations, the Physarum computing program has made a genuine intellectual contribution, and it is worth identifying what that contribution is.

The most important insight is that computation is substrate-independent in a more radical sense than is commonly appreciated. We are accustomed to thinking of computation as something that happens in silicon, in carefully designed logical circuits with precise, reproducible characteristics. The Physarum experiments demonstrate that something functionally resembling computation — something that solves spatial optimization problems, finds efficient paths, implements logical relationships — can emerge in biological tissue whose primary "design" is for nutrient transport, not computation.

This raises the possibility, taken seriously by some theorists, that computation is a very general property of certain kinds of physical dynamics, not a special achievement of electronic engineering. If the flow dynamics of a slime mold can approximate optimal networks, what other physical systems might implement computation in this implicit, emergent sense? Reaction-diffusion chemical systems, active matter systems, even geological or meteorological dynamics might, in principle, implement computational processes — if computation is defined broadly enough.

The philosophical implication for the study of cognition is uncomfortable but important: if computation does not require silicon, and perhaps does not even require neurons, then "computational processes underlie intelligence" tells us much less than we thought it did. The claim becomes nearly trivially true — any system that does anything interesting at all involves information processing in some sense — rather than a substantive constraint on what kinds of systems can be intelligent.

The slime mold, in the end, challenges us not just to extend our definition of intelligence but to interrogate what definition we were using in the first place.

---

## References

Adamatzky, A. (2010). *Physarum Machines: Computers from Slime Mould*. World Scientific.

Adamatzky, A., & Müller, H. (2013). Slime mould tactile sensor. *Sensors and Actuators B: Chemical*, 188, 38–44.

Nakagaki, T., Yamada, H., & Tóth, Á. (2000). Maze-solving by an amoeboid organism. *Nature*, 407, 470.

Tero, A., Takagi, S., Saigusa, T., Ito, K., Bebber, D. P., Fricker, M. D., ... & Nakagaki, T. (2010). Rules for biologically inspired adaptive network design. *Science*, 327(5964), 439–442.
