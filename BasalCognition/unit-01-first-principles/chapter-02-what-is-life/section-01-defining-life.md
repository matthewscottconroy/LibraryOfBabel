# Section 2.1: Defining Life — A Surprisingly Hard Problem

---

## Section Introduction

NASA's working definition of life — "a self-sustaining chemical system capable of Darwinian evolution" — is useful for astrobiology, where the question is whether to keep searching after you've found a particular kind of chemistry. It is less useful as a philosophical account of what life actually is. As definitions go, it is more pragmatic than explanatory: it picks out a class of systems that happen to be what we're looking for, without illuminating what makes them the kind of thing they are.

The difficulty of defining life is not merely terminological. It reflects a genuine scientific puzzle. We are made of exactly the same atoms as the non-living world, following exactly the same physical laws. What arrangement, what organization, what process makes the difference?

This section surveys the major approaches to this question and builds toward the organizational account that will serve as the foundation for the chapter's central argument.

---

## 2.1.1 The NASA Definition and Its Discontents

The NASA definition has three components:

**Self-sustaining**: The system maintains its own organization using energy and materials from its environment.

**Chemical system**: The relevant processes are chemical — involving molecules, reactions, and the transformation of matter.

**Darwinian evolution**: The system can evolve by natural selection — it must reproduce, with heritable variation and differential fitness.

Each component captures something important. The self-sustaining requirement distinguishes living systems from passive crystals, which are organized but do not actively maintain their organization. The chemical requirement places life in the physical world and distinguishes it from purely formal systems (like algorithms) that could be said to "evolve" in some sense. The Darwinian requirement ensures that the system can generate adaptive complexity over time — that it is not merely organized, but that its organization reflects the history of selection.

But the definition has well-known problems.

**Viruses are not self-sustaining**, depending on host cell machinery to replicate, and yet they replicate, mutate, and evolve by natural selection. Most biologists would say viruses are on the boundary of life, not clearly outside it. The NASA definition places them outside — which may be the right answer, but it requires a more careful argument than the definition itself provides.

**Fire is self-sustaining and evolves in some sense**: a fire propagating through a forest undergoes selection pressure — some sparks land on combustible material and grow; others don't. Flames have heritable variation (different chemical compositions of fuel generate different flame temperatures, which affect propagation). No one thinks fire is alive. But the NASA definition, applied naively, might not clearly exclude it.

**Mules do not reproduce** and thus cannot undergo Darwinian evolution, despite being paradigm cases of living organisms. The NASA definition applies to lineages, not individual organisms — but then it is a definition of *life* in the abstract, not of *being alive*.

These are not fatal objections — they can be addressed by refinements of the definition. But they illustrate the gap between a practical operational criterion and a genuine theoretical account.

---

## 2.1.2 Life as Thermodynamic Dissipation: Schrödinger's Negative Entropy

Erwin Schrödinger's 1944 lectures, published as *What Is Life?*, gave a different and more theoretical approach. Schrödinger asked: given that the second law of thermodynamics says entropy must increase in any closed system, how do living organisms maintain their organized, low-entropy structure over time? His answer: by feeding on negative entropy — *negentropy* — from their environment (Schrödinger, 1944).

The organism takes in highly ordered, energy-rich material (food), processes it to extract the energy needed for its own organization, and expels disordered, energy-poor material (waste). By coupling itself thermodynamically to the environment in this way, it can maintain its own order even as it contributes to the overall increase in entropy of the system (organism plus environment plus food source).

Schrödinger's insight was profound. It placed life firmly within physics and chemistry, explained the apparent paradox of biological order, and pointed toward the importance of metabolism as the core biological process. It also foreshadowed the modern thermodynamic account of life associated with researchers like Jeremy England (2013), who has argued that the emergence of replicating structures is a thermodynamic near-inevitability given appropriate conditions — that life is not a cosmic accident but a statistical attractor.

But Schrödinger's account, while necessary, is not sufficient as a definition of life. A refrigerator maintains internal order by pumping heat to the environment — it is, in Schrödinger's sense, a negentropy-feeding device. We do not think refrigerators are alive. The additional element that distinguishes living systems from refrigerators is that living systems maintain their own organization — they produce the components that constitute them, not merely keep existing components cold.

---

## 2.1.3 Life as Autocatalytic Chemistry: Kauffman's Origin Models

Stuart Kauffman's work on the origin of life introduces a powerful additional concept: **autocatalytic sets** (Kauffman, 1993). An autocatalytic set is a collection of molecules in which each molecule's formation is catalyzed by other molecules in the set. The set as a whole catalyzes its own production — it is collectively self-reproducing.

Kauffman showed mathematically that as the number of different molecular species in a system increases, the probability that the system will contain an autocatalytic set increases rapidly, eventually becoming near-certain above a threshold of complexity. This suggests that the origin of life — the emergence of self-reproducing chemical systems — may have been a phase transition, a threshold crossing, rather than an improbable accident.

The autocatalytic set concept is important for our purposes because it captures a key feature of life that mere thermodynamic dissipation does not: **self-reproduction via the catalysis of one's own components.** A fire burns and dissipates energy but does not produce the molecules that constitute its own burning reactions. An autocatalytic set, by contrast, generates its own catalysts.

However, autocatalytic sets face the challenge of Darwinian evolution: for selection to operate, there must be heritable variation, and it is not clear how autocatalytic sets (as opposed to template-replicating systems like RNA or DNA) can maintain heritable variation across generations.

The resolution of this tension — between the collective self-reproduction of autocatalytic chemistry and the linear heritability of template-based replication — remains an active research area (Pross, 2012; Gánti, 2003).

---

## 2.1.4 Life as Organizational Closure: The Key Move

The most theoretically satisfying account of life — and the one that most directly illuminates the relationship between life and cognition — is the organizational account developed by Maturana and Varela (discussed at length in Section 2.2) and extended by their successors in systems biology and theoretical biology.

The core concept is **organizational closure** (or "operational closure" in Maturana and Varela's terminology). A system is organizationally closed if its components are produced by the processes constituting the system itself — if the system, as a whole, is the cause of its own constitution.

This is what distinguishes a living cell from a crystal, from a fire, and from a refrigerator:

- A **crystal** maintains its organized structure, but its components are not produced by the crystal's own processes. Calcium carbonate ions from the seawater attach to the crystal's surface; the crystal does not catalyze the production of calcium carbonate.

- A **fire** produces heat and light but does not produce the fuel that feeds it or the oxygen that sustains it.

- A **refrigerator** maintains an organized internal state but does not produce its own compressor, evaporator coils, or refrigerant.

- A **cell** produces its own lipids (to maintain its membrane), its own enzymes (to catalyze its reactions), its own DNA (to store and transmit its organization), and its own ribosomes (to translate that DNA into proteins). The cell's processes produce the very components that constitute those processes. It is, in a rigorous technical sense, self-producing.

Maturana and Varela called this self-producing organization **autopoiesis** — from the Greek *autos* (self) and *poiesis* (production). We examine this concept in detail in Section 2.2.

---

## 2.1.5 Edge Cases: Where Life Blurs

A definition of life is tested by its hard cases. Three are particularly instructive:

**Viruses** lack their own metabolic machinery — they cannot produce their own components without a host cell. But they carry and replicate genetic information, and they evolve. They are, in the language of autopoiesis, not autopoietic in themselves but only in combination with a host cell. This suggests that the host-virus complex is the relevant unit of analysis — which raises interesting questions about the boundaries of biological individuality that Chapter 22 (on symbiosis) addresses.

**Prions** are misfolded proteins that catalyze the misfolding of other proteins — a form of self-reproduction without nucleic acids. They evolve by natural selection (Masel & Jansen, 2000). But they cannot produce their own components independently; they depend on existing proteins in a host cell. They are perhaps the most minimal self-reproducing "entity" we know of — and their existence challenges any definition of life that requires nucleic acids.

**Origin-of-life intermediates** — pre-cellular chemical systems at various stages of the transition from chemistry to biology — exist in a grey zone. At what point does an autocatalytic chemical system become alive? This question probably does not have a sharp answer. Life likely emerged gradually, through a series of transitions, rather than appearing suddenly at a definite threshold. This means that the question "is this thing alive?" may not always have a definite answer — not because we lack information, but because life admits of degrees, particularly at its origin.

This last point has direct implications for basal cognition: if life admits of degrees, and cognition is constitutive of life, then cognition may also admit of degrees — which is exactly the position this book takes.

---

## Section Summary

No single definition of life is fully satisfactory, but several important features emerge from the survey:

| Approach | Core Insight | Limitation |
|----------|-------------|------------|
| NASA (self-sustaining + Darwinian) | Captures the practical-biological cluster | Excludes viruses; fails to explain *why* |
| Thermodynamic (Schrödinger) | Places life in physics; explains the negentropy puzzle | Refrigerators also feed on negentropy |
| Autocatalytic (Kauffman) | Captures self-reproduction; suggests life is near-inevitable | Does not explain heritable variation |
| Organizational (Maturana & Varela) | Explains what distinguishes living from non-living organization | Technical and abstract; resists operational definition |

The organizational account is the most theoretically powerful and will serve as the foundation for Section 2.2 and the chapter's central argument. Its key claim — that living systems produce their own components through their own processes — is what makes the connection between life and cognition logically tight rather than merely empirically correlated.

---

## References for Section 2.1

England, J.L. (2013). Statistical physics of self-replication. *Journal of Chemical Physics*, 139(12), 121923.

Gánti, T. (2003). *The Principles of Life* (E. Szathmáry & J. Griesemer, Eds.). Oxford University Press.

Kauffman, S.A. (1993). *The Origins of Order: Self-Organization and Selection in Evolution*. Oxford University Press.

Masel, J., & Jansen, V.A.A. (2000). Designing drugs to stop the propagation of prions. *Journal of Theoretical Biology*, 203(1), 1–11.

Pross, A. (2012). *What Is Life? How Chemistry Becomes Biology*. Oxford University Press.

Schrödinger, E. (1944). *What Is Life? The Physical Aspect of the Living Cell*. Cambridge University Press.
