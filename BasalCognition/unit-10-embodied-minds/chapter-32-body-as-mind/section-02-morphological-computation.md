# Section 32.2: Morphological Computation

## The Body as Computer

The conventional view of embodied cognitive systems is that the brain computes the solution to cognitive problems and the body implements the computed solution. A person reaching for a cup computes, in the brain, the trajectory of the required arm movement and sends motor commands that cause the arm to execute the trajectory. The body is the output device; the brain is the computer.

Rolf Pfeifer and Josh Bongard, in their influential book *How the Body Shapes the Way We Think* (2007), proposed a different picture: the body is itself a computational device. The physical dynamics of the body — its mass distribution, its elasticity, its compliance, the inertial properties of its limbs — solve computational problems that the brain does not need to solve explicitly, because the body's physics solves them automatically. This is **morphological computation**: the offloading of computational work from the neural system onto the body's physical structure.

The implications are significant. If the body computes, then the boundary between "cognitive system" and "physical implementation" is not at the skin but is distributed through the body's morphology. Different bodies will think differently — not only because different bodies send different sensory information to the brain, but because different bodies solve different problems through their physical dynamics. The body's shape is not cognitively neutral; it is part of the cognitive architecture.

## Passive Dynamic Walkers

The most striking demonstrations of morphological computation come from robotics, where researchers have built robots that exhibit complex, natural-looking locomotion without any brain control at all.

**Passive dynamic walkers** are mechanical systems with no actuators or control systems — just a body with carefully designed mass distribution, compliant joints, and rigid limbs. When placed on a slight downhill slope, these systems can walk in a manner that is strikingly human-like: they have appropriate gait kinematics, they can walk indefinitely (as long as the slope provides energy), and they exhibit stable recovery from small perturbations. All of this is achieved through purely passive mechanical dynamics — the system's mass distribution, the geometry of its joints, and the dynamics of leg-ground contact do the "computation" of walking without any control signal from a central processor (McGeer, 1990).

What does passive dynamic walking tell us about biology? It suggests that a significant part of what has traditionally been attributed to the neural control of walking is actually accomplished by the physical dynamics of the body. The brain's contribution to walking is not to compute the entire trajectory of each limb from first principles, but to provide energy and stability corrections to a dynamical system that naturally tends toward walking behavior. The "intelligence" of walking is distributed between the brain (which handles corrections and coordination) and the body (which handles the basic dynamics through morphological computation).

This has been confirmed experimentally: studies of human walking have shown that the basic mechanics of each stride are highly predictable from the body's physical properties, with neural correction systems engaging primarily at the beginning and end of each step and when perturbations occur (Kuo, 2002). The spine is a pendulum, the legs are inverted pendula, and the human gait is, in significant part, the natural dynamics of these pendula being harnessed by a control system rather than replaced by one.

## Tensegrity Structures and Distributed Mechanical Intelligence

The architecture of the biological body is, in many structural respects, a **tensegrity** structure — a design principle first articulated by the engineer R. Buckminster Fuller in which structural integrity is achieved through continuous tension and discontinuous compression, rather than through rigid load-bearing columns.

In a tensegrity structure, a network of tension elements (tendons, fasciae, membranes) keeps compression elements (bones, rigid links) in stable suspension. No single element bears all the load; the entire structure distributes forces through the tension network. This means that forces applied anywhere in the structure propagate throughout the entire structure — a deformation at one end changes the tension throughout the entire tensegrity network.

Donald Ingber at Harvard Medical School has argued that biological cells are tensegrity structures at the microscopic scale: the cytoskeleton (actin filaments, microtubules, intermediate filaments) forms a tension network that keeps the cell's shape stable and propagates mechanical signals throughout the cell (Ingber, 2003). This has significant implications for cellular mechanotransduction — the conversion of mechanical signals into biochemical ones — because mechanical forces applied to the cell surface propagate through the cytoskeletal tensegrity network to reach the nucleus and other organelles, enabling the cell to "feel" its mechanical environment in a distributed, non-local way.

At the whole-organism scale, the musculoskeletal system implements something like tensegrity: muscles and tendons (tension) and bones (compression) form a distributed network that maintains posture and enables movement through coordinated tension modulation. The advantage of this architecture over a purely rigid structure is that it distributes mechanical loads across many elements rather than concentrating them at a few points, providing both flexibility and resilience. But it also has a cognitive advantage: the distribution of mechanical forces through the tension network means that the physical structure itself "computes" how to distribute forces, solving a structural optimization problem through physics rather than through explicit calculation.

## The Octopus Arm: A Peripheral Brain

No example of morphological computation is more striking than the octopus arm. An octopus has a central brain containing approximately 50 million neurons, but this represents only about one-third of its total neural complement: the remaining two-thirds (approximately 350 million neurons in a large *Octopus vulgaris*) are distributed across the eight arms, with each arm containing roughly 40 million neurons in its own autonomous ganglion system (Hochner et al., 2006).

This peripheral neural architecture enables each arm to function as a semi-autonomous computational unit. When an octopus reaches toward a prey item, the arm extends toward the target in a highly stereotyped kinematic pattern that researchers have called "quasi-articulated" motion — it resembles the extension of a jointed arm despite the fact that octopus arms have no joints, only continuously flexible muscular hydrostats (a structure where muscles serve both as the moving elements and as the skeleton, since there are no rigid bones). This quasi-articulated motion is produced by the peripheral neural system of the arm itself, without requiring planning or coordination from the central brain (Sumbre et al., 2001).

The central brain receives sensory information from the arm (which has mechanoreceptors and chemoreceptors distributed across its suckers and surface) and can issue high-level commands ("reach toward that target") but does not compute the detailed muscle activation patterns required for arm movement. Those computations are performed by the arm's peripheral nervous system, working with the physical dynamics of the muscular hydrostat body.

### The Octopus as a Model of Distributed Cognition

The octopus provides a model for understanding how cognitive work can be distributed across a body rather than centralized in a brain. Its architecture is not simply "brain minus" — a deficient version of centralized neural computation. It is a qualitatively different cognitive architecture in which peripheral controllers handle local computations while the central brain manages global goals and integrates cross-arm coordination.

Several features of octopus cognition are best understood in terms of this distributed architecture:

**Color change**: Octopuses are famous for their extraordinarily rapid and precise color and texture changes, used for camouflage, communication, and predator deterrence. The chromatophore system that produces these changes is controlled by a vast network of peripheral neural circuitry that can produce patterns in milliseconds. The central brain sets the "goal" (match this background), but the local implementation — which chromatophores to expand, by how much, in what pattern — is computed by the distributed peripheral system.

**Arm autotomy and regeneration**: An octopus can voluntarily detach one of its arms as a predator-escape strategy, and the detached arm continues to move and behave autonomously for up to an hour after separation, still responding to stimuli. This demonstrates unequivocally that the arm's peripheral nervous system can coordinate sophisticated behavior without any central brain involvement.

**Tactile discrimination**: The suckers of an octopus arm can perform extremely fine tactile discriminations — distinguishing surfaces by texture, identifying objects by shape, and even "reading" spatial patterns of grooves. This tactile cognition appears to be partly mediated by the arm's own peripheral neural system, which performs initial processing before sending signals to the central brain.

### Morphological Computation in Non-Neural Organisms

The octopus is a spectacular neural example, but morphological computation is not limited to neural systems. Several phenomena in non-neural organisms can be understood in these terms:

**The bacterial flagellar motor**: The molecular structure of the bacterial flagellar motor — a reversible rotary motor about 45 nanometers in diameter — implements complex computational operations through its physical dynamics. The motor's "switching" between clockwise and counterclockwise rotation is controlled by the concentration of the signaling molecule CheY-P, but the actual switching dynamics are determined by the mechanical properties of the motor — the number of FliM subunits, their cooperative conformational change, and the resulting stochastic switching kinetics. The motor's physics computes the switching behavior; the signaling network modulates the parameters of that computation (Cluzel et al., 2000).

**Plant tropisms**: The directional growth responses of plants (phototropism, gravitropism, thigmotropism) involve significant morphological computation. The bending of a root or shoot in response to a directional cue involves differential growth on the two sides of the organ, which is controlled by gradients of growth hormones (auxins). But the asymmetric auxin distribution is itself partly generated by the physical asymmetry of the organ — its geometry determines how light is distributed across its cells, how gravity pulls on its starch-filled statoliths, and how mechanical forces are distributed through its tissues. The organ's morphology participates in computing the gradient that drives its response.

**Cytoskeletal dynamics in cell migration**: Migrating cells exhibit directional movement without any neural control, navigating gradients of chemoattractants through a process that involves the physical dynamics of the actin cytoskeleton. The leading edge of the cell extends lamellipodia — flat protrusions of actin gel — and the physical dynamics of actin polymerization, myosin contraction, and adhesion determine which protrusions are stabilized and which retracted, generating directional movement. This morphological computation is sophisticated enough that cells can navigate complex chemical landscapes and follow multiple simultaneous gradient cues.

## What Morphological Computation Implies

The concept of morphological computation has several important implications for our understanding of cognition:

First, it implies that **cognitive architecture is not synonymous with neural architecture**. The cognitive capacities of an organism depend not just on the organization of its nervous system but on the physical properties of its body — its mass distribution, compliance, geometry, and the dynamics of its tissues. Changing the body changes the cognition, even without changing the brain.

Second, it implies that **cognitive evolution includes morphological evolution**. The evolution of more sophisticated cognitive capacities is not achieved solely through changes in neural organization; changes in body morphology that enable morphological computation can also enhance cognitive performance. The evolution of compliant joints, spring-like tendons, and distributed peripheral innervation in the vertebrate limb are partly cognitive evolutionary events.

Third, it implies that **the study of cognition cannot be limited to neuroscience**. Understanding how organisms cognize requires understanding the physical dynamics of their bodies, not just the organization of their neural circuits. Biomechanics, structural biology, and developmental biology are, on this account, part of cognitive science — not ancillary fields that serve neuroscience but fields that study genuine cognitive processes in their own right.

For basal cognition specifically, morphological computation suggests that even organisms without nervous systems may exhibit genuine morphological cognitive processes: the physical dynamics of their cells, membranes, and cytoskeletons participate in information processing and adaptive response in ways that constitute genuine (if minimal) cognition. The bacterium's flagellar motor is a physical computational device; the plant's root tip is a distributed computation system; the slime mold's network dynamics are morphological computation on a macroscopic scale.

---

## References

Cluzel, P., Surette, M., & Leibler, S. (2000). An ultrasensitive bacterial motor revealed by monitoring signaling proteins in single cells. *Science*, 287(5458), 1652–1655.

Hochner, B., Shomrat, T., & Fiorito, G. (2006). The octopus: A model for a comparative analysis of the evolution of learning and memory mechanisms. *Biological Bulletin*, 210(3), 308–317.

Ingber, D.E. (2003). Tensegrity I. Cell structure and hierarchical systems biology. *Journal of Cell Science*, 116(7), 1157–1173.

Kuo, A.D. (2002). Energetics of actively powered locomotion using the simplest walking model. *Journal of Biomechanical Engineering*, 124(1), 113–120.

McGeer, T. (1990). Passive dynamic walking. *International Journal of Robotics Research*, 9(2), 62–82.

Pfeifer, R., & Bongard, J. (2007). *How the Body Shapes the Way We Think: A New View of Intelligence*. MIT Press.

Sumbre, G., Gutfreund, Y., Fiorito, G., Flash, T., & Hochner, B. (2001). Control of octopus arm extension by a peripheral motor program. *Science*, 293(5536), 1845–1848.
