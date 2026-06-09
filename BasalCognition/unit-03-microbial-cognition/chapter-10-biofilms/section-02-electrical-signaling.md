# Section 2: Electrical Signaling in Biofilms

One of the most surprising discoveries in biofilm biology of the past decade has been the demonstration that bacterial biofilms can propagate long-range electrical signals — waves of membrane potential change that travel across centimeters of biofilm in a matter of minutes, coordinating the metabolic state of bacteria throughout the community. This discovery, primarily from the laboratories of Gürol Süel and Jintao Liu at UCSD and Joel Prindle at Northwestern, has fundamentally changed our view of biofilms from passive, chemically coordinated communities to dynamic, electrically active ones.

---

## The Potassium Wave Discovery

The key paper, published by Prindle et al. in *Nature* in 2015, is one of the most important in the recent history of microbiology. The team grew biofilms of *Bacillus subtilis* on agar surfaces and used genetically encoded fluorescent sensors — particularly a membrane voltage-sensitive indicator — to image membrane potential across the biofilm in real time.

What they observed was striking: periodic oscillations in membrane potential that originated in the interior of the biofilm and propagated outward as traveling waves, reaching the periphery of the biofilm over the course of 10-20 minutes. These waves were not diffusion waves of a chemical signal — they traveled too fast and too far for simple diffusion of a small molecule. They were electrical waves, propagated by ion channel-mediated ion fluxes through the biofilm cell community.

The propagation mechanism involves potassium ions and potassium channels. When cells in the biofilm interior experience metabolic stress (particularly, glutamate depletion), they depolarize — opening potassium channels and releasing potassium into the extracellular space. This released potassium depolarizes neighboring cells (by raising the extracellular potassium concentration, which depolarizes the Nernst potential for potassium), which in turn release potassium and depolarize their neighbors. The result is a self-propagating wave of depolarization — a potassium wave — that travels from the stressed interior to the metabolically active periphery of the biofilm (Prindle et al., 2015).

---

## Metabolic Coordination via Electrical Signals

What does this potassium wave do? Prindle et al. showed that the wave serves as a metabolic coordination signal. The stressed interior cells that initiate the wave are competing for glutamate (an essential metabolite) with the rapidly growing peripheral cells. By triggering the potassium wave, the interior cells cause a transient depolarization of peripheral cells, which transiently inhibits the peripheral cells' metabolism (through the effect of depolarization on the electrochemical gradient driving nutrient import). This metabolic inhibition briefly suppresses the peripheral cells' growth, allowing glutamate to redistribute to the interior cells.

This is, in effect, a redistribution of metabolic resources from periphery to interior mediated by an electrical signal — a form of homeostatic resource allocation at the community level, implemented through biofilm-wide electrical communication. The interior cells do not produce a chemical that physically delivers glutamate to them; instead, they broadcast an electrical signal that induces the periphery to temporarily reduce its own glutamate consumption, making more available for diffusion to the interior.

The analogy to neural coordination is obvious and intentional. The potassium channel-mediated wave in biofilms uses the same ionic currency (potassium) and the same basic mechanism (depolarization-triggered channel opening propagating a wave) as the action potential of neurons, though the propagation mechanism (elevated extracellular potassium rather than voltage-gated sodium channels) and timescale (minutes rather than milliseconds) differ. Whether this represents deep evolutionary homology, convergent evolution, or mere coincidence of mechanism is an open question.

---

## Calcium Waves and Multi-Signal Integration

Subsequent work from the Süel laboratory revealed that biofilms can propagate not just potassium waves but calcium waves — waves of intracellular calcium concentration that travel across the biofilm with different dynamics and trigger different cellular responses. Calcium waves in biofilms appear to be involved in sporulation coordination: calcium signals promote sporulation initiation, and the spatial pattern of calcium waves within the biofilm helps coordinate when and where sporulation begins.

The coexistence of potassium waves (coordinating metabolism) and calcium waves (coordinating sporulation) in a single biofilm is a striking demonstration of multi-channel electrical communication in a non-neural system. The biofilm is using different ionic signals to carry different types of information across the same tissue — a multiplexed electrical communication system implemented with bacterial ion channels.

---

## Biofilm Electrical Signals and Prey Behavior

A more recent and remarkable finding extends the biofilm's electrical communication beyond its own boundaries. Research from the Bhattacharya laboratory has shown that the potassium waves emitted by *B. subtilis* biofilms can be detected by nearby *Bacillus subtilis* cells in the planktonic phase (outside the biofilm), which respond by migrating toward the biofilm. The wave thus functions not just as internal coordination signal but as an external attractant signal — the biofilm is broadcasting its electrical state to potential recruits.

This is quorum sensing of a different kind — not chemical counting of population density but electrical broadcasting of biofilm metabolic state. Planktonic cells detecting the wave can infer that a large, metabolically active biofilm is nearby (because only a large, active biofilm produces detectable potassium waves), and may use this information to decide whether to join the biofilm or avoid it.

This cross-community electrical signaling extends the cognitive light cone of the biofilm: it is not just coordinating its internal state but broadcasting information that influences the behavior of entities outside itself.

---

## References

Prindle, A., Liu, J., Asally, M., Ly, S., Garcia-Ojalvo, J., & Süel, G. M. (2015). Ion channels enable electrical communication in bacterial communities. *Nature*, *527*(7576), 59–63.

Süel, G. M. (2021). Collective electrical oscillations in biofilms. In *Bacterial Biofilms* (pp. 1–20). Springer.
