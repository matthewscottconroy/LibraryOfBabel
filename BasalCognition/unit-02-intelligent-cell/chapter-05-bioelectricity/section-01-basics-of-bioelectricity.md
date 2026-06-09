# Section 1: The Basics of Bioelectricity

Every cell in your body maintains an electrical potential difference across its membrane — typically between -20 and -90 millivolts, depending on cell type. This is the resting membrane potential, and it is not simply a consequence of life, a physical side effect of having a membrane. It is an actively maintained state, produced by the continuous work of ion pumps and channels against the tendency of electrochemical gradients to equalize. The cell spends a significant fraction of its ATP budget sustaining this potential. Whatever it is doing, it must be important.

For neurons, the importance is obvious: the action potential — the rapid, propagating reversal of membrane potential — is the fundamental unit of neural signaling. But neurons are only one percent of the cells in the body. What are the other ninety-nine percent doing with their membrane potentials? This is the question that the field of developmental bioelectricity has begun to answer.

---

## Ion Channels: Selective Pores

The lipid bilayer of a cell membrane is effectively impermeable to ions — charged molecules are excluded by the hydrophobic interior of the membrane. Ions cross the membrane only through protein channels — narrow, ion-selective pores that span the membrane and regulate ion flow. The ion channel is one of the oldest molecular machines in biology; homologs of animal potassium channels can be found in bacteria, suggesting that ion channel-based membrane potential regulation dates to the earliest cells.

Ion channels are selective: a potassium channel allows potassium ions through but not sodium or calcium. This selectivity is determined by the physical dimensions and electrostatic character of the channel's selectivity filter — a narrow constriction that strips the ion of its water of hydration shell and interacts with it directly. The energy cost of this stripping, and its compensation by the selectivity filter's partial charges, creates a discrimination mechanism exquisitely tuned to the size and charge of a particular ion.

Channels are also gated: they open and close in response to signals. Voltage-gated channels open when the membrane potential reaches a threshold value. Ligand-gated channels open when a specific small molecule binds. Mechanosensitive channels open in response to membrane tension. The gating mechanism — the conformational change that opens or closes the pore — is the channel's sensory element: it converts an environmental signal into a change in ion conductance, which changes the membrane potential, which influences the behavior of other channels and downstream signaling molecules.

---

## Ion Pumps: Active Maintenance of Gradient

Ion channels allow ions to flow down their electrochemical gradients — in the direction they would move anyway, driven by concentration difference and electric force. They dissipate energy. Ion pumps, by contrast, move ions against their electrochemical gradients, using the energy of ATP hydrolysis (or, in some cases, proton gradients) to pump ions "uphill." It is the pumps that maintain the ion concentration gradients across the membrane; the channels that allow those gradients to drive electrical signaling.

The most important pump in most animal cells is the Na+/K+-ATPase (sodium-potassium pump). For every ATP hydrolyzed, it exports three sodium ions and imports two potassium ions — a net outward movement of positive charge, contributing to the negative interior potential. This pump is not merely a battery charger; its activity is regulated and it contributes to the membrane potential in a way that depends on cellular metabolic state. Cells with impaired ATP production (due to hypoxia, for example) will have depolarized membranes as their pumps slow; this depolarization is itself a signal that triggers downstream responses.

Beyond the Na+/K+-ATPase, cells use a variety of other pumps — H+-ATPases (proton pumps), Ca2+-ATPases, and the Na+/Ca2+ exchanger — to maintain intracellular ion concentrations. Plants and fungi rely predominantly on H+-ATPases to generate their membrane potentials, using the resulting proton gradient to drive diverse transport processes. The molecular identities of the pumps differ, but the fundamental strategy — using energy to maintain electrochemical gradients across membranes — is universal.

---

## The Resting Membrane Potential

The resting membrane potential is an equilibrium (or more precisely, a steady state) at which the net ion flow across the membrane is zero — not because no ions are moving, but because the flows in different channels balance each other out. The actual value of the potential depends on which ion species are most permeable at rest and on their concentration gradients.

For a membrane permeable only to potassium, the equilibrium potential is given by the Nernst equation: E_K = (RT/zF) × ln([K+]_out / [K+]_in). At physiological temperature, this gives about -90 mV for typical neuronal conditions. Real cells are permeable to multiple ions, and their actual resting potential is a weighted average of the Nernst potentials for all permeable species, with the weighting determined by the relative conductances (Goldman-Hodgkin-Katz equation).

The resting membrane potential thus reflects the entire conductance state of the cell membrane — which channels are open, which are closed, which are inactivated. Anything that changes ion channel expression or gating will shift the membrane potential. Conversely, changes in membrane potential will directly affect voltage-gated channels, and indirectly affect ligand-gated channels through downstream signaling. The membrane potential is not a passive epiphenomenon; it is a dynamic variable that encodes the state of the cell's ion channel complement and participates in regulating it through feedback.

---

## Action Potentials Beyond Neurons

The action potential — the brief, stereotyped reversal of membrane potential that propagates along neural and muscle cell membranes — is typically presented as a uniquely neural phenomenon. This is not accurate. Action potentials, or events with similar dynamics, have been described in a wide variety of non-neural cell types, including plants, certain fungi, protozoa, macrophages, and developing embryonic cells.

Plant action potentials, conducted along the phloem, mediate rapid systemic responses to wounding or herbivory. A caterpillar biting a leaf on one side of a plant can trigger an action potential-like electrical signal that travels to the other side of the plant within minutes, preparing distant leaves for defense by activating the production of defensive compounds. The molecular mechanisms are different from animal action potentials (involving chloride channels and H+/Ca2+ currents rather than Na+ and K+), but the functional role — rapid, long-distance electrical communication — is analogous (Mousavi et al., 2013).

In the single-celled alga *Chara* and its relatives, action potentials regulate cytoplasmic streaming — the flow of cytoplasm driven by myosin motors on actin tracks. When the cell is damaged, an action potential propagates rapidly, causing the cessation of streaming (perhaps to prevent leakage of cellular contents through a wound). This is a form of rapid, coordinated cellular response to injury implemented by electrical signaling, entirely independent of nervous systems.

The presence of action potential-like events across such a diverse range of organisms and cell types suggests that the molecular machinery for fast electrical signaling is ancient and widely distributed. The specialization of this machinery for neural signaling in animals is an elaboration of a more general biological capability.

---

## Gap Junctions: Electrical Coupling Between Cells

In multicellular organisms, bioelectricity is not just a property of individual cells but of tissues — communities of cells electrically connected through gap junctions. Gap junctions are intercellular channels formed by hemichannels (connexins in vertebrates, innexins in invertebrates) that align between adjacent cells to form direct cytoplasmic connections. Ions, small molecules, and electrical current can pass directly from cell to cell through these channels, creating tissue-wide electrical coupling.

The consequence is that the membrane potential of any individual cell in a gap-junction-coupled tissue is not solely determined by that cell's own channels and pumps. It reflects the collective electrical state of the coupled tissue, modified by the particular channel complement of the local cell. A perturbation to the membrane potential of one cell (say, by the opening of a mechanosensitive channel) spreads electrotonically to neighboring cells through gap junctions, creating tissue-scale electrical waves.

This electrical coupling has profound implications for developmental biology. If different regions of a developing embryo have different membrane potentials — maintained by locally differential expression of pumps and channels — these bioelectric domains are not isolated patches but interact through gap junctions to create continuous spatial patterns of membrane potential across the tissue. These patterns are dynamically stable, can be altered by experimental manipulation, and — as we will examine in Section 2 — appear to carry positional information that guides cell fate decisions during development.

---

## Evolutionary Origins of Ion Channels

Ion channels are among the most ancient molecular machines we know of. Potassium channels with structural similarity to those in mammalian neurons have been characterized in bacteria (KcsA from *Streptomyces lividans* was among the first ion channel crystal structures, solved by Roderick MacKinnon's group in 1998). Voltage-gated sodium and calcium channels are more derived — they appear to have evolved by gene duplication and specialization from simpler single-domain ancestors — but the core architecture of the ion-selective pore is bacterial in origin.

This evolutionary depth has an important implication: the capacity for bioelectric signaling is not an adaptation of advanced multicellular organisms but a fundamental property of cellular life. Bacteria maintain membrane potentials (typically more positive on the inside than animal cells, reflecting different ion chemistry). Archaea have ion channels. Even simple unicellular eukaryotes like yeast maintain a regulated membrane potential.

Michael Levin has argued that bioelectricity is best understood as the most ancient information medium of life — predating neurotransmitters, predating gap junctions, predating multicellularity (Levin, 2012). From this perspective, the nervous system is not the origin of bioelectric information processing but its latest and most elaborate specialization. The intelligence of neurons is built from materials — ion channels, membrane potentials, electrical gradients — that were solving information-processing problems for billions of years before neurons existed.

---

## References

Levin, M. (2012). Molecular bioelectricity in developmental biology: new tools and recent discoveries. *BioEssays*, *34*(3), 205–217.

MacKinnon, R. (2003). Potassium channels. *FEBS Letters*, *555*(1), 62–65.

Mousavi, S. A. R., Chauvin, A., Pascaud, F., Kellenberger, S., & Farmer, E. E. (2013). Glutamate receptor-like genes mediate leaf-to-leaf wound signalling. *Nature*, *500*(7463), 422–426.
