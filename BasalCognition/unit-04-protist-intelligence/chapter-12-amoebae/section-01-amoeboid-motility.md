# Section 1: Amoeboid Motility and Gradient Sensing

## Introduction

Movement is cognition's oldest accomplishment. Before there was anything to learn or remember, before there were signals to integrate or goals to represent, there had to be directed movement — the capacity to go somewhere rather than anywhere. And the fundamental cellular mechanism for directed movement, the pseudopod, is ancient enough that it predates the divergence of the amoeba lineage from our own by perhaps a billion years.

Understanding how an amoeba moves toward food is not simply an exercise in cell biology. It is an exploration of how physical chemistry can give rise to directed behavior — how gradients in molecular concentration can be translated into spatial asymmetry in a cell body, and how that asymmetry can be amplified and maintained long enough to constitute purposeful action. The mechanisms we find here — signal amplification, symmetry-breaking, noise filtering, local excitation and global inhibition — are computational strategies. They solve problems. The fact that they are implemented in lipid bilayers and cytoskeletal filaments rather than in neural circuits does not diminish their functional sophistication.

---

## 1.1 The Architecture of Motility

An amoeba moves by a process that looks, from the outside, almost like a decision. The cell extends a protrusion — the pseudopod — in the direction of travel, fills it with cytoplasm, then retracts the trailing edge. The cell body flows forward. Extend, fill, retract: a cycle that repeats in various elaborations across all motile cells.

The pseudopod itself is constructed from a meshwork of actin filaments — the same protein that constitutes the thin filaments of your own muscle. Actin is remarkable. In its globular (G-actin) form, it is a small soluble protein. But under the right conditions, G-actin polymerizes into long filaments (F-actin) that are mechanically stiff and capable of exerting pushing forces against a membrane simply by growing longer. The assembly of these filaments at the leading edge of a pseudopod is what physically drives the membrane forward (Pollard & Borisy, 2003).

But filament polymerization does not happen everywhere at once. If it did, the cell would simply inflate uniformly and go nowhere. The key to directed movement is spatial regulation — a mechanism that restricts actin polymerization to one region of the cell while simultaneously inhibiting it everywhere else. This spatial regulation is the crux of what we might call the cell's motility computation.

The regulatory machinery is organized around a family of proteins called Rho GTPases, particularly Rac and Cdc42, which act as molecular switches (Ridley et al., 2003). When active (GTP-bound), they stimulate actin polymerization through downstream effectors including the Arp2/3 complex, which nucleates branched actin networks. When inactive (GDP-bound), they do not. The spatial pattern of Rho GTPase activity thus determines the spatial pattern of actin assembly, and hence the direction of protrusion.

---

## 1.2 Gradient Sensing: The Problem of Detection

Chemotaxis — directed movement along a chemical gradient — requires that a cell know not merely that a chemical is present but where it is more concentrated. For a macroscopic animal with a nose, this is straightforward: sniff here, sniff there, compare. But a cell cannot easily do this sequentially; it must, in some sense, compare concentrations simultaneously at different points on its surface.

The first conceptual puzzle is one of signal-to-noise. A gradient of chemoattractant might produce only a small fractional difference in receptor occupancy between the front and back of a cell. If the cell is 10 micrometers long and the gradient is shallow — as gradients typically are at biologically relevant distances — the difference in concentration from front to back might be as small as 1–2% (Devreotes & Zigmond, 1988). The thermal noise in receptor binding at this scale is substantial. How does the cell extract directional information from such a noisy signal?

The answer lies in what has come to be called the LEGI model — Local Excitation, Global Inhibition (Parent & Devreotes, 1999). The basic logic is as follows: at the front of the cell, where receptor occupancy is slightly higher, there is a local excitatory response that drives actin polymerization. Simultaneously, a global inhibitory signal — one that spreads rapidly throughout the cell — suppresses the excitatory response everywhere the local signal is not dominant. The result is a sharp spatial contrast: strong excitation at the front, inhibition everywhere else.

This is, in computational terms, a spatial filtering algorithm. It extracts a weak directional signal from noisy receptor occupancy data by combining local amplification with global suppression. The same logic appears in various forms throughout neural computation — lateral inhibition in the retina, for example, performs a closely analogous function in extracting spatial contrast from visual input.

---

## 1.3 Symmetry Breaking and the Initial Choice

Here is a philosophically interesting problem: before the cell has started moving in any direction, it is (approximately) spherically symmetric. Receptor occupancy is roughly uniform all around. How does the first pseudopod form?

The answer involves a phenomenon called spontaneous symmetry breaking — a process familiar from physics but with important biological instances. Even in the absence of an external gradient, the signaling networks inside an amoeba are capable of generating random, spontaneous pulses of activity that produce pseudopods (Weiner et al., 2007). These pulses are not uniform: they occur at particular locations and then influence each other competitively.

Think of it this way: when a small random fluctuation triggers a pulse of Rac activity at one location on the cell membrane, the resulting local excitation also activates the global inhibitor. This inhibition suppresses Rac activity at all other locations. If that initial fluctuation is strong enough, it wins — it establishes itself as the dominant site of protrusion and suppresses competitors, defining a front and therefore a direction.

In a homogeneous environment, where the initial fluctuation is truly random, the choice of direction is stochastic. The cell moves, but in a direction that reflects noise rather than information. In a gradient, the probability of that initial fluctuation occurring at the up-gradient side of the cell is biased by the small asymmetry in receptor occupancy. The external gradient doesn't specify direction top-down; it biases a stochastic symmetry-breaking event.

This has an important conceptual implication: even in a "directed" chemotaxis, what the cell is actually doing is generating pseudopods stochastically and using the gradient to bias which ones win the competition. The cell is not computing a vector and moving toward it. It is exploring probabilistically and allowing the gradient to select (Arrieumerlou & Meyer, 2005). This is fundamentally different from how we initially might have imagined chemotaxis to work, and it tells us something about the nature of cellular "decision-making" more generally.

---

## 1.4 Mechanosensing: Feeling the Physical World

Chemical gradients are not the only information available to a moving amoeba. The physical properties of its environment — the stiffness of the substrate, the topology of surfaces, the presence of physical obstacles — also influence its behavior. Sensing these physical properties is called mechanosensing, and it is far more sophisticated in amoebae than was appreciated until recently.

Amoebae can sense substrate stiffness and preferentially migrate toward stiffer regions — a behavior called durotaxis. The mechanism involves mechanosensitive proteins in the cell-substrate adhesion complex and ion channels in the membrane that open in response to membrane deformation (Plotnikov et al., 2012). When the cell extends a pseudopod onto a stiff substrate, the resistance it encounters feeds back through the adhesion complex to affect actin dynamics at the leading edge.

Perhaps more remarkable is that amoebae can detect and respond to very small mechanical forces. Studies of Dictyostelium have shown that the cell can sense pressure differences as small as a few piconewtons — forces generated by neighboring cells during aggregation (Moeendarbary et al., 2013). This mechanosensitivity is not merely an interesting biophysical curiosity; it means that in crowded multicellular contexts, the mechanical state of the local environment carries information that the cell can read and respond to.

---

## 1.5 The Cytoskeleton as Distributed Computation

We are now in a position to step back and appreciate a synthesis: the cell's cytoskeleton — the dynamic network of actin filaments, microtubules, and their associated proteins — functions as a distributed information-processing system.

It is not a processor in any conventional sense. There is no central unit that integrates inputs and produces outputs. Instead, information is encoded spatially in the pattern of protein activities across the cell surface, and it is processed through physical interactions: mutual excitation, mutual inhibition, mechanical coupling, molecular competition. The "output" — the direction and speed of movement — emerges from this distributed processing rather than being specified anywhere.

Several features of this system are particularly worth noting:

**Adaptability**: The system can change its sensitivity in response to sustained stimulation — a process analogous to sensory adaptation in neural systems. When exposed to a uniform concentration of chemoattractant, an amoeba initially responds vigorously but then adapts, allowing it to detect new gradients against a high background (Devreotes & Zigmond, 1988). The molecular mechanism involves covalent modifications to receptor-associated proteins that reduce their signaling efficacy.

**Robustness**: The gradient-sensing system remains functional across a remarkable range of absolute concentrations — spanning roughly four orders of magnitude. This robustness is achieved partly through the adaptation mechanism and partly through the mathematical properties of the ratio-detecting circuits in the LEGI model.

**Integration**: The cell integrates chemical signals from multiple sources simultaneously and can modulate its response based on their combination. Chemical attractants and repellents, substrate stiffness, cell-cell contact signals — all of these impinge on the same cytoskeletal machinery and are integrated in the spatial pattern of its activity.

None of this requires a brain. But neither should it prompt us to say "there is nothing cognitive going on here." The cell solves a navigation problem using a physical instantiation of signal amplification, noise filtering, and competitive dynamics. Whether we choose to call these processes "cognitive" depends on what we decide that word should mean — a decision we should make carefully and explicitly rather than by default.

---

## References

Arrieumerlou, C., & Meyer, T. (2005). A local coupling model and compass parameter for eukaryotic chemotaxis. *Developmental Cell*, 8(2), 215–227.

Devreotes, P. N., & Zigmond, S. H. (1988). Chemotaxis in eukaryotic cells: A focus on leukocytes and Dictyostelium. *Annual Review of Cell Biology*, 4, 649–686.

Moeendarbary, E., Valon, L., Fritzsche, M., Harris, A. R., Moulding, D. A., Thrasher, A. J., ... & Charras, G. T. (2013). The cytoplasm of living cells behaves as a poroelastic material. *Nature Materials*, 12(3), 253–261.

Parent, C. A., & Devreotes, P. N. (1999). A cell's sense of direction. *Science*, 284(5415), 765–770.

Plotnikov, S. V., Pasapera, A. M., Bhatt, B., & Waterman, C. M. (2012). Force fluctuations within focal adhesions mediate ECM-rigidity sensing to guide directed cell migration. *Cell*, 151(7), 1513–1527.

Pollard, T. D., & Borisy, G. G. (2003). Cellular motility driven by assembly and disassembly of actin filaments. *Cell*, 112(4), 453–465.

Ridley, A. J., Schwartz, M. A., Burridge, K., Firtel, R. A., Ginsberg, M. H., Borisy, G., ... & Horwitz, A. R. (2003). Cell migration: integrating signals from front to back. *Science*, 302(5651), 1704–1709.

Weiner, O. D., Marganski, W. A., Wu, L. F., Altschuler, S. J., & Kirschner, M. W. (2007). An actin-based wave generator organizes cell motility. *PLOS Biology*, 5(9), e221.
