# Section 1: Physarum Biology — An Organism Like No Other

## Introduction

To understand what Physarum computes, you first need to understand what Physarum is — and it takes some effort, because it does not fit any of the categories that biological intuition ordinarily operates with.

Physarum polycephalum is not a plant. It is not a fungus, despite spending part of its life producing spores and living in decaying wood and leaf litter. It is not an animal. It is not quite a single cell — not in the sense that a bacterium or a Paramecium is a single cell. It is not quite a multicellular organism either. It is a plasmodium: a single, continuous cytoplasmic mass enclosed in one membrane, containing hundreds of thousands or millions of nuclei that share a common cytoplasm, all coordinating in the absence of any cell boundaries between them.

There is no precedent in everyday experience for this. The closest analogy is a single enormous cell — a cell that might span several centimeters, cover a rock face or a petri dish, and contain more cytoplasm and more genetic material than most multicellular organisms many times its size. But even this analogy obscures more than it reveals, because a normal cell's nuclei are fixed in their positions, while Physarum's cytoplasm — nuclei and all — flows continuously through the tubular network that constitutes the organism's body.

---

## 1.1 The Life Cycle

Physarum's life cycle includes several dramatically different phases, and understanding the transitions between them is important for contextualizing the cognitive experiments we discuss in later sections.

**Spore stage**: Under harsh environmental conditions — desiccation, extreme temperature, food deprivation — Physarum forms spores. These are resistant, dormant cells with thick walls, capable of surviving for years. Each spore contains a single haploid nucleus.

**Myxamoeba and swarm cell stage**: When conditions improve, spores germinate into small, free-living haploid cells called myxamoebae (which move by pseudopodia, like the amoebae of the previous chapter) or swarm cells (which have flagella and swim). These haploid cells feed on bacteria and yeast by phagocytosis — a solitary lifestyle resembling that of free-living amoebae.

**Plasmodium formation**: Two myxamoebae of compatible mating types fuse to form a diploid cell, which then repeatedly undergoes nuclear division without cell division, producing the multinucleate plasmodium. Once formed, the plasmodium can grow indefinitely — as long as food is available and conditions are suitable, it will continue feeding, expanding, and merging with any other compatible plasmodia it encounters.

**Sclerotium**: Under conditions of desiccation but not extreme cold or starvation, the plasmodium can reversibly differentiate into a sclerotium — a hard, dry resting stage that can remain dormant and then rehydrate and resume activity when water returns.

**Fruiting body stage**: When starved, the plasmodium migrates toward light (a striking reversal of its usual light-avoidance behavior) and forms elaborate fruiting bodies — stalked structures bearing spore capsules — completing the cycle.

For most of the experiments discussed in this chapter, we are concerned with the plasmodial stage, which is the feeding, growing, network-forming stage that exhibits the spatial problem-solving behaviors that have captured researchers' attention.

---

## 1.2 Cytoplasmic Streaming: The Engine of the Network

The most striking observable feature of the Physarum plasmodium, visible even to the naked eye in a translucent specimen, is the rhythmic, bidirectional flow of cytoplasm through the tubular network. This flow — called cytoplasmic streaming — reverses direction approximately every 60–90 seconds, and it is the physical mechanism by which nutrients, signaling molecules, and genetic material are distributed throughout the organism.

The streaming is driven by contractions of actomyosin — the same actin-myosin interaction that drives muscle contraction in animals. The tube walls of the Physarum network contain a layer of actomyosin that contracts rhythmically, driving cytoplasm forward in peristaltic waves. The rhythm of these contractions is not imposed from outside; it is an intrinsic oscillation generated within the tube wall tissue itself, coordinated across the network by mechanical coupling through the flowing cytoplasm (Alim et al., 2013).

The coordination of these rhythmic contractions across a network spanning centimeters is itself a striking phenomenon. There is no pacemaker, no command center, no cell whose job is to synchronize the rest. Instead, the contractions of different tubes influence each other mechanically — through the pressure changes caused by flowing cytoplasm — and the result is a coordinated, wave-like oscillation that travels across the network. This is another instance of the pattern we see throughout basal cognition: global coordination emerging from local interactions, without central control.

The rhythmicity of cytoplasmic streaming has important consequences for information processing. The frequency of oscillation varies with conditions: nutrients increase it, light and cold decrease it (Alim et al., 2013). Different parts of the network can oscillate at slightly different frequencies, and the phase relationships between oscillating regions carry information about the spatial distribution of nutrients and other conditions. This distributed encoding of environmental information in the rhythm of cytoplasmic streaming is, as we will see, central to how Physarum solves spatial problems.

---

## 1.3 Tube Formation and Network Remodeling

The Physarum plasmodium does not simply fill available space uniformly. It organizes itself into a network of interconnected tubes — thick "veins" that carry most of the flow, connected by finer tributaries — and this network architecture is not fixed. It is continuously remodeled in response to the distribution of food sources, the geometry of the substrate, and the organism's own internal dynamics.

How do tubes form? Tubes arise from regions of cytoplasm where the oscillatory flow has become sufficiently regular and strong to generate positive feedback. The key feedback is between tube diameter and flow: thicker tubes offer less resistance to flow, and therefore carry more flow, and therefore receive more actomyosin material delivered by that flow, and therefore maintain and expand their walls (Tero et al., 2007). Thinner tubes, carrying less flow, lose material and eventually thin to the point of rupture.

This is a positive-feedback mechanism operating at the level of individual tubes, and its dynamics are exactly what is needed to explain network optimization. In a network connecting multiple food sources, the tubes that lie along shorter, more efficient paths will carry more flow — because flows from multiple branches converge on them — and will therefore be reinforced, while tubes along longer, inefficient paths will be disfavored and will regress. The network self-organizes toward efficiency without any global computation of what efficiency requires.

The tube network also responds to the spatial distribution of food. When a food source is encountered at some point in the network, the local oscillation frequency increases, and this perturbation propagates through the network as a mechanical wave. The result is a redistribution of flow that preferentially routes cytoplasm toward the food source and stimulates the formation of thicker tubes connecting the organism to the food (Alim et al., 2017).

We will return to these mechanisms in the next section, when we see how they produce the maze-solving and network optimization behaviors that made Physarum famous.

---

## 1.4 What Kind of Thing Is Physarum?

It is worth pausing to reflect on the conceptual puzzle that Physarum poses before we examine its problem-solving capabilities.

Physarum is a single organism — it has one continuous membrane, shares cytoplasm across its entire body, and behaves in a unified way as it explores its environment. In that sense, it is an individual. But it is also, in some sense, a distributed system: there is no privileged location, no command center, no cell or nucleus more important than any other. Information about conditions at one part of the network is transmitted to other parts through the physical dynamics of cytoplasmic flow and mechanical coupling — not through any dedicated signaling channel.

The closest functional analogy within familiar biology might be the nervous system viewed at a low level of abstraction: a network of connected elements, each locally responsive, whose collective dynamics produce integrated behavior. But the slime mold's "nervous system" is, in a sense, also its body and its circulatory system and its musculature — all rolled into one continuous, flowing, self-remodeling structure.

This is why the computational experiments with Physarum are so philosophically interesting. The question is not just "can this organism solve problems?" The question is "what does it mean to solve a problem when the solving happens in a distributed physical system with no separated information-processing layer?" The slime mold forces us to think carefully about what computation is, where it happens, and what the relationship is between physical dynamics and cognitive function.

---

## References

Alim, K., Andrew, N., Pringle, A., & Brenner, M. P. (2017). Mechanism of signal propagation in Physarum polycephalum. *Proceedings of the National Academy of Sciences*, 114(20), 5136–5141.

Alim, K., Amselem, G., Peaudecerf, F., Brenner, M. P., & Pringle, A. (2013). Random network peristalsis in Physarum polycephalum organizes fluid flows across an individual. *Proceedings of the National Academy of Sciences*, 110(33), 13306–13311.

Tero, A., Kobayashi, R., & Nakagaki, T. (2007). A mathematical model for adaptive transport network in path finding by true slime mold. *Journal of Theoretical Biology*, 244(4), 553–564.
