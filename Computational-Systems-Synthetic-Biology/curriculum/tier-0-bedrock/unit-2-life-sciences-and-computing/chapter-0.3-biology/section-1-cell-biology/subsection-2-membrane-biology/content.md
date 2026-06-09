# Membrane Biology

Every cell is, at its most fundamental level, a chemistry separated from other chemistry by a membrane. That sentence sounds almost trivial — but it is not. Without the plasma membrane, there is no inside and outside, no way to maintain concentration gradients, no substrate for sensing environmental signals. The first cells were not defined by their DNA. They were defined by their membranes. And today, every signal transduction model you will ever build begins at the membrane — the place where the world meets the cell.

The plasma membrane is a cell's interface with the world. It is not merely a passive barrier but an active, dynamic platform for sensing environmental signals, controlling the import and export of molecules, generating electrochemical gradients, and initiating signaling cascades. Understanding membrane biology is essential for computational models of signaling, transport, and synthetic biosensor design.

## The Plasma Membrane as a Selective Barrier

The plasma membrane is a lipid bilayer (see chemistry section for composition) that is:
- Permeable to: small nonpolar molecules (O$_2$, CO$_2$, N$_2$, steroid hormones), small uncharged polar molecules (water via aquaporins, urea, glycerol)
- Impermeable to: ions (Na$^+$, K$^+$, Ca$^{2+}$, Cl$^-$), charged metabolites (glucose-6-phosphate, ATP, amino acids), macromolecules (proteins, nucleic acids)

This selective permeability creates concentration and electrical gradients that store potential energy. The plasma membrane potential ($\sim -70$ mV in neurons, $-50$ mV in many other cells) represents $\sim k_BT \times 2$ per elementary charge — a significant free energy per ion that drives secondary active transport. It turns out that this small voltage — barely enough to light an LED — is the currency that powers secondary active transport throughout the cell, enabling nutrient uptake, pH regulation, and much of the signaling machinery that interests us in systems biology.

## Channel Proteins: Passive Ion Transport

**Ion channels** are transmembrane proteins that form aqueous pores allowing specific ions to cross the membrane rapidly (up to $10^8$ ions/s per channel) down their electrochemical gradient. Key features:

- **Selectivity:** K$^+$ channels are 100-fold more selective for K$^+$ over Na$^+$ despite the smaller size of Na$^+$. The selectivity filter uses carbonyl oxygens to mimic the first coordination shell of water, selecting for the correct ionic radius.
- **Gating:** Channels open and close in response to stimuli:
  - **Voltage-gated:** Na$^+$, K$^+$, Ca$^{2+}$ channels in neurons — activated by membrane depolarization; form the basis of action potentials
  - **Ligand-gated:** Acetylcholine receptor (nicotinic), GABA receptor, AMPA receptor — activated by neurotransmitter binding
  - **Mechanosensitive:** MscL in bacteria — opens under membrane tension; sensory mechanotransduction in eukaryotes
  - **Temperature-gated:** TRP channels (TRPV1 activated by heat and capsaicin)

**Quantitative model:** The **Hodgkin-Huxley equations** describe action potential generation as a system of ODEs for membrane voltage and channel gate states:

$$C_m \frac{dV}{dt} = -g_{Na} m^3 h (V - E_{Na}) - g_K n^4 (V - E_K) - g_L (V - E_L) + I_{\text{ext}}$$

where $m$, $h$, $n$ are gating variables (each with their own first-order ODEs) and $E_{\text{ion}}$ are Nernst potentials. This is one of the most successful quantitative models in all of biology. Hodgkin and Huxley wrote these equations in 1952 — before anyone knew what an ion channel looked like at the molecular level — based entirely on voltage-clamp measurements of squid giant axon currents. The fact that this model correctly predicted action potential shape, conduction velocity, and refractory period from nothing more than measured conductances is a stunning demonstration of what quantitative mechanistic modeling can do when the right variables are identified.

## Transporters: Facilitated Diffusion and Active Transport

**Transporters** (carriers) differ from channels: they bind their substrate specifically, undergo a conformational change, and release it on the other side. They are slower ($10^2 – 10^4$ molecules/s) but more selective.

**Uniporters:** Move one solute type down its gradient (GLUT1 glucose transporter in red blood cells and brain).

**Symporters:** Move two solutes in the same direction; one drives the other against its gradient. Na$^+$/glucose symporter (intestinal absorption): glucose uptake against its gradient is driven by Na$^+$ flowing down its gradient.

**Antiporters:** Move two solutes in opposite directions. Na$^+$/H$^+$ exchanger (NHE): extrudes H$^+$ (acid) using Na$^+$ gradient — important for pH regulation.

**Primary active transporters (ATPases):**
- **Na$^+$/K$^+$-ATPase:** Pumps 3 Na$^+$ out and 2 K$^+$ in per ATP hydrolyzed, maintaining the Na$^+$ and K$^+$ gradients essential for neuronal signaling. Consumes ~30% of cellular ATP in neurons.
- **Ca$^{2+}$-ATPase (SERCA):** Pumps Ca$^{2+}$ into the ER; maintains cytoplasmic Ca$^{2+}$ at ~100 nM (vs. ER lumen at ~500 µM and extracellular at 1–2 mM)
- **H$^+$-ATPase (V-ATPase):** Acidifies lysosomes, endosomes, and the Golgi lumen
- **ABC transporters (ATP-Binding Cassette):** Large family; multidrug resistance proteins; export hydrophobic molecules including chemotherapy drugs from cancer cells

## Signaling Receptors

Three major receptor classes initiate intracellular signaling:

**Receptor tyrosine kinases (RTKs):** Ligand binding (e.g., EGF, VEGF, insulin) induces receptor dimerization and autophosphorylation of cytoplasmic tyrosine residues. Phosphorylated tyrosines recruit adaptor proteins (Grb2, SHC), initiating the Ras-MAPK cascade, PI3K-Akt pathway, and STAT signaling.

**G protein-coupled receptors (GPCRs):** The largest family of cell surface receptors (~800 genes in humans). Seven transmembrane helices; couple to heterotrimeric G proteins ($\alpha\beta\gamma$). Ligand binding causes GDP→GTP exchange on G$\alpha$, dissociation of G$\beta\gamma$, and activation of downstream effectors (adenylyl cyclase for cAMP production; phospholipase C for IP3/DAG production; ion channels).

**Nuclear receptors:** Ligand-activated transcription factors. Ligand (steroid hormone, thyroid hormone, vitamin D, retinoic acid) diffuses into the cell and binds the receptor in the cytoplasm, causing translocation to the nucleus and activation of target genes.

## Endocytosis

Cells internalize membrane proteins, lipids, and extracellular molecules by **endocytosis**:
- **Clathrin-mediated:** Most cargo-specific; adaptor proteins recruit clathrin coat; 100–150 nm coated vesicles; delivers ligand-receptor complexes to lysosomes for degradation (receptor downregulation) or recycling
- **Macropinocytosis:** Large membrane ruffles engulf extracellular fluid; bulk uptake; important in immune cells and cancer
- **Phagocytosis:** Professional phagocytes (macrophages, neutrophils) engulf pathogens and dead cells; actin-driven

## Why This Matters for Computational Biology

The cell membrane is where the environment meets intracellular signaling networks. Every signal transduction model begins at the membrane. The quantitative parameters (receptor copy number, $K_d$ for ligand, dimerization rates, internalization rates) determine the dynamic range and response time of a signaling pathway. The Hodgkin-Huxley model is the prototype of a detailed biophysical model — and its success demonstrates the power of quantitative mechanistic modeling in biology. In synthetic biology, designing membrane-based biosensors (e.g., a cell that detects glucose concentration by expressing GLUT1 reporter fusions) requires understanding transporter kinetics (Km, Vmax) as functions of membrane concentration. Equally important: receptor internalization and recycling mean that the effective number of receptors at the surface is not fixed but depends on signaling state — a feedback that must be accounted for in any quantitative model of RTK or GPCR-mediated signaling. The membrane is not just where signals enter the cell; it is where some of the most interesting regulatory dynamics happen.
