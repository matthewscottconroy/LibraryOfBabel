# Section 3: Network Optimization in the Mycelium

## The Engineering Problem That Fungi Solve

Engineers who design communication and transportation networks face a fundamental trade-off. A network that connects all nodes to all other nodes is maximally robust and maximally efficient — but it is prohibitively expensive to build and maintain. A network that is too sparse saves on material but creates bottlenecks, single points of failure, and long travel times between distant nodes. The ideal network is somewhere in between: efficient enough to move things quickly, robust enough to tolerate failures, and not so expensive that it drains more resources than it delivers.

This is not an easy optimization problem. Solving it exactly requires knowing the full network layout, the traffic demands between all pairs of nodes, and the cost of each possible link. Real engineering solutions — the Tokyo rail network, the internet's backbone topology, the vascular network of a large organism — are the product of decades of planning, mathematical optimization, and iterative refinement.

The mycelium solves a version of this problem in real time, without any central planner, using only local rules operating at the level of individual hyphal tips. The result, as we shall see, is a network that approaches the efficiency and robustness of engineered networks — not perfectly, but impressively well.

## How Mycelium Allocates Resources

The basic mechanism of mycelial network optimization is a combination of reinforcement and pruning, operating through the feedback loops we described in the previous section.

The reinforcement side: when a hyphal pathway is carrying high cytoplasmic flow — because it connects a nutrient-rich region to the main colony — that flow is self-reinforcing. High flow exerts hydraulic pressure that slightly expands the hyphal tube, reducing its resistance, which further increases flow. More flow means more delivery of growth-promoting factors to the downstream tips, which grow faster and branch more, creating more connections that feed yet more flow through the productive pathway. This positive feedback loop amplifies successful connections and makes them progressively more committed and more efficient.

The pruning side: in hyphae carrying low or zero cytoplasmic flow, the opposite process occurs. Without the mechanical stress of flow, the hyphal tube does not expand. Without the delivery of growth factors, the tip slows and eventually stops growing. The stationary hyphal cytoplasm is then subject to autolysis — enzymatic digestion, beginning with the retraction of the Spitzenkörper and the reorganization of the cytoskeleton. The tube is recycled; its contents flow back toward more productive regions. This is metabolic efficiency: resources invested in unproductive hyphae are recovered and reinvested.

The combined result of reinforcement and pruning is network remodeling. Over time, the mycelium converges on a network architecture that reflects the productive and unproductive regions of its environment. The thick, well-connected cords that remain after pruning are not arbitrary; they are the topology that emerged from the interplay of exploration, reinforcement, and recycling.

## Experimental Demonstrations of Optimization

The clearest experimental evidence for mycelial network optimization comes from studies by Lynne Boddy, Mark Fricker, and their collaborators, who have used laboratory microcosms to track network development in *Phanerochaete velutina* and related species under controlled conditions.

In a representative experiment, a fungal colony is established on a uniform medium and then offered two food sources of different quality at known locations. Over days, the colony extends toward both food sources, connects them, and then progressively remodels its network. Fricker and colleagues have quantified this remodeling using image analysis tools that track changes in hyphal diameter, network connectivity, and resource flow over time (Fricker et al., 2017).

The results show that mycelial networks are not static once food sources are connected. The network continues to change, with high-value connections growing thicker and low-value connections thinning or disappearing. The final network architecture is measurably more efficient — in terms of the ratio of material cost to transport performance — than the initial exploratory network, and it approaches the efficiency of networks designed by human engineers for similar connectivity problems.

More striking still are experiments in which the network is interrupted — a cord is severed, a food source is removed, or a new food source is added. The mycelium responds by rerouting: flow redirects through alternative pathways, new growth extends toward the new resource, and old connections to depleted resources are pruned. The network adapts. It is not locked into its initial architecture; it continues to optimize as conditions change.

## Comparison with Physarum Polycephalum

The mycelium's network-optimization abilities cannot be discussed without reference to *Physarum polycephalum*, the yellow slime mold that has become famous in the scientific literature for its capacity to solve maze-like spatial problems. Physarum is not a fungus — it is a member of the amoebozoa, an entirely different branch of life — but it shares with fungi the property of existing as an extended multinucleate cytoplasmic network that forages for food.

The key experiment, published by Toshiyuki Nakagaki and colleagues in 2000, showed that Physarum placed in a maze and given food sources at the start and end could find the shortest path through the maze by trial and error followed by reinforcement of productive routes (Nakagaki et al., 2000). This result received enormous attention because it demonstrated, in a simple biological system, a computational process — shortest-path finding — that was thought to require sophisticated neural computation.

A follow-up study extended this finding dramatically: when food sources were placed at the locations of cities in the Tokyo metropolitan area, Physarum grew a transport network that closely resembled the actual Tokyo rail network, matching it for efficiency and robustness (Tero et al., 2010). This result was published in *Science* and became one of the most-cited demonstrations of biological network optimization.

Fungal mycelium has not been subjected to precisely the same experimental paradigms, but the comparison is instructive. Both Physarum and mycelium are extended cytoplasmic networks. Both use reinforcement and pruning to optimize their transport geometry. Both show adaptive responses to changes in the environment. The mechanisms, however, differ in important ways: Physarum achieves its optimization through oscillatory cytoplasmic flows that create a kind of active contraction-expansion wave, while fungal mycelium uses tip growth and anastomosis combined with passive flow reinforcement. These are different solutions to the same engineering problem, which suggests that network optimization is an adaptive challenge that has been solved multiple times by similar general principles.

## Mathematical Models of Mycelial Growth

The behavior of the mycelial network is amenable to mathematical modeling, and this modeling has been a productive area of research at the intersection of mycology, network science, and applied mathematics.

Several classes of model have been developed. The simplest are agent-based models in which individual hyphal tips are treated as agents following simple local rules — grow in the direction of highest nutrient concentration, branch when nutrients exceed a threshold, anastomose when a compatible tip is within range. These models, despite their simplicity, reproduce many of the gross features of mycelial network development: the exploratory front, the thickening of productive cords, the pruning of unproductive branches (Meškauskas et al., 2004).

More sophisticated models incorporate fluid dynamics, treating cytoplasmic streaming as flow through a network of tubes with variable resistance. In these models, tubes whose flow exceeds a threshold are reinforced (their resistance decreases); tubes whose flow falls below a threshold are pruned (their resistance increases to the point of closure). The equations governing this process are a biological analog of the electrical circuit laws — Kirchhoff's laws applied to cytoplasmic flow. Models in this class have been shown to converge on near-optimal network topologies for a variety of spatial configurations of food sources (Fricker et al., 2017).

The mathematical analysis reveals something important: the mycelium's optimization strategy is not hill-climbing in the conventional sense (finding the nearest local optimum and stopping). Because the network topology is continuously remodeled as the environment changes, the optimization process can recover from bad early decisions and find better solutions when new information arrives. This is a property that simple hill-climbing algorithms do not share, and it reflects the inherent advantage of a physical network that learns by physically restructuring itself.

## The Cost-Benefit Trade-off in Network Architecture

Fricker and colleagues have quantified the mycelial network's trade-offs using metrics borrowed from transportation network analysis (Fricker et al., 2017). The key metrics are:

- **Transport efficiency**: the average path length between all pairs of nodes, weighted by the frequency of traffic between them
- **Robustness**: the fraction of node-pairs that remain connected after random removal of a fraction of links
- **Material cost**: the total length of the network, weighted by tube diameter

Plotting these three metrics against each other for real mycelial networks and for mathematically optimal networks of similar size reveals that mycelial networks occupy a region of the trade-off space that is Pareto-efficient: they achieve high transport efficiency and high robustness at relatively low material cost. They do not achieve the absolute optimum in any single metric, but they achieve a good balance across all three simultaneously — which is exactly what engineering optimization theory predicts a well-designed network should do.

This quantitative comparison is important because it moves the discussion of fungal network "intelligence" from the realm of vague analogy to the realm of measurable performance. The mycelium does not just "look like" an intelligent network; it performs like one, achieving near-optimal solutions to a trade-off problem that is mathematically well-defined.

## What the Optimization Reveals

The network optimization behavior of mycelium tells us something important about the relationship between local rules and global outcomes — a theme that will recur throughout this unit and the next.

The mycelium does not optimize globally. No part of it has access to information about the whole network. Every decision — every hyphal extension, every branching event, every anastomosis, every pruning — is made locally, by a small patch of cytoplasm responding to its immediate chemical and physical environment. The global optimization is an emergent consequence of these local decisions, mediated by the network's physical structure.

This is the same general principle that underlies the network-optimization behavior of ant colonies, the collective decision-making of bee swarms, and — as some theorists argue — the information processing of the brain. In each case, global intelligence is an emergent property of local interactions among simpler agents. The mycelium makes this principle visible in unusually concrete form, because the "agents" (hyphal tips) and the "medium" (cytoplasmic flow) are both accessible to direct measurement.

The lesson for our broader inquiry: cognition may not require a central processor. It may not require a "place" where decisions are made. It may emerge from the interaction of many simpler processes, each responding only to its local environment, none of which individually performs anything that looks like reasoning or decision-making. The mycelium does not have a brain. But the mycelium solves problems that a brain would recognize as problems. That is worth thinking about carefully.

---

## References

Fricker, M. D., Heaton, L. L. M., Jones, N. S., & Boddy, L. (2017). The mycelium as a network. *Microbiology Spectrum*, 5(3), FUNK-0033-2016.

Meškauskas, A., Fricker, M. D., & Moore, D. (2004). Simulating colonial growth of fungi with the neighbour-sensing model of hyphal growth. *Mycological Research*, 108(11), 1241–1256.

Nakagaki, T., Yamada, H., & Tóth, Á. (2000). Maze-solving by an amoeboid organism. *Nature*, 407(6803), 470.

Tero, A., Takagi, S., Saigusa, T., Ito, K., Bebber, D. P., Fricker, M. D., Yumiki, K., Kobayashi, R., & Nakagaki, T. (2010). Rules for biologically inspired adaptive network design. *Science*, 327(5964), 439–442.
