# Organelles

One of the most radical innovations in the history of life was the invention of the internal membrane. Bacteria accomplish an extraordinary amount of biochemistry in a single undivided compartment — but eukaryotes discovered something that bacteria mostly did not: that separating incompatible chemistries into distinct membrane-enclosed spaces allows each chemistry to be optimized independently. The nucleus keeps genomic DNA away from cytoplasmic proteases. The lysosome concentrates digestive enzymes at pH 4.7 without acidifying the whole cell. Mitochondria maintain a proton gradient of ~180 mV across their inner membrane — a voltage sufficient to drive ATP synthesis — without dissipating that gradient into the surrounding cytoplasm. The eukaryotic cell is not just a bigger bacterium. It is a fundamentally different architectural strategy.

Eukaryotic cells are divided into functionally specialized compartments — organelles — each with a distinct membrane composition, internal chemistry, and protein inventory. Understanding the function and properties of each organelle is essential for understanding eukaryotic gene regulation, protein trafficking, metabolic compartmentalization, and the design of synthetic biology interventions that must function within these compartments.

## The Nucleus

The nucleus houses the genome and is the site of transcription, pre-mRNA processing, and ribosome biogenesis. Key features:

- **Double membrane (nuclear envelope):** Outer membrane is continuous with the rough ER; inner membrane is lined by the nuclear lamina (type III intermediate filaments that provide structural support)
- **Nuclear pore complexes (NPCs):** ~120 MDa, 30 nm aqueous channel. Proteins enter if they have a **nuclear localization signal (NLS)** recognized by importins. mRNA and ribosomes exit via nuclear export signals (NES). Small molecules ($<$ 40 kDa) diffuse freely; large proteins require active transport.
- **Nucleolus:** Non-membrane-bounded sub-compartment within the nucleus; site of rRNA transcription (by RNA Pol I) and ribosome assembly. Multiple nucleoli may be present; their size reflects ribosome biosynthesis rate.

**Chromatin organization:** DNA in the nucleus is not uniformly distributed. Heterochromatin (condensed, gene-poor) is concentrated near the nuclear periphery (associated with lamins and the nuclear envelope). Euchromatin (open, gene-rich) is more central. Topologically Associating Domains (TADs) — ~1 Mb regions of preferential self-interaction — organize chromosomes into functional domains.

## Mitochondria

Mitochondria are the ATP-producing organelles of eukaryotic cells. Their endosymbiotic origin from an ancient $\alpha$-proteobacterium is reflected in:

- **Double membrane:** Outer membrane is permeable to small molecules (via porins); inner membrane is highly impermeable — essential for maintaining the proton gradient that drives ATP synthesis
- **Cristae:** Invaginations of the inner membrane that dramatically increase its surface area; packed with electron transport chain complexes (I–IV) and ATP synthase (Complex V)
- **Mitochondrial genome:** 16.5 kb in humans; encodes 13 proteins (all ETC/ATP synthase subunits), 22 tRNAs, and 2 rRNAs; 70S-like ribosomes for translating these transcripts
- **Dynamics:** Mitochondria undergo constant fusion and fission — a quality control mechanism. Dysfunctional mitochondria (depolarized) undergo selective autophagy (**mitophagy**, regulated by PINK1/Parkin).

**ATP synthesis:** The proton motive force ($\Delta\psi \approx -180$ mV, $\Delta$pH $\approx 0.5$ across the inner mitochondrial membrane) drives ATP synthase: $\sim 2.5$ ATP per NADH (Complex I entry), $\sim 1.5$ ATP per FADH$_2$ (Complex II entry). Total $\approx 30$ ATP per glucose in oxidative phosphorylation. The mitochondrial genome's persistence — despite billions of years of endosymbiosis — reflects the cost of transporting hydrophobic ETC subunits across the mitochondrial inner membrane: it turns out that it is easier to encode and translate them locally than to import them from the cytoplasm.

## Endoplasmic Reticulum (ER)

The ER is a continuous network of tubules and sheets (continuous with the outer nuclear membrane) occupying up to 10% of cell volume.

**Rough ER (rER):** Studded with ribosomes on the cytoplasmic face. Functions:
- **Protein import:** Secretory and membrane proteins are co-translationally translocated into the rER lumen through the Sec61 translocon
- **Protein folding and quality control:** Molecular chaperones (BiP/GRP78, calnexin, calreticulin) assist folding; misfolded proteins are retained and retrotranslocated for ERAD (ER-associated degradation)
- **N-linked glycosylation:** En bloc transfer of the Glc$_3$Man$_9$GlcNAc$_2$ precursor to Asn in the N-X-S/T sequon

**Smooth ER (sER):** Lacks ribosomes. Functions:
- **Lipid synthesis:** Phospholipids and cholesterol biosynthesis
- **Ca$^{2+}$ storage:** Lumenal Ca$^{2+}$ is ~500 µM (vs. ~100 nM cytoplasmic); released by IP3Rs (IP3 receptor channels) in response to signaling
- **Drug detoxification:** Cytochrome P450 enzymes in liver sER oxidize xenobiotics

**The Unfolded Protein Response (UPR):** When unfolded protein accumulates in the ER lumen, three sensor proteins (IRE1, PERK, ATF6) are activated, triggering transcriptional programs that expand ER capacity and reduce protein synthesis. The UPR is a canonical example of a feedback control system at the cellular level: the ER monitors its own load, detects overload, and upregulates capacity accordingly.

## Golgi Apparatus

The Golgi is the trafficking hub — it receives newly synthesized secretory/membrane proteins from the ER, processes and sorts them, and dispatches them to their correct destinations (plasma membrane, lysosomes, secretory vesicles, back to ER).

Structure: cis-Golgi network (receives ER cargo) → medial Golgi → trans-Golgi → trans-Golgi network (TGN, sorting hub). The stacked architecture creates a processing gradient.

Functions:
- **Glycan processing:** Trimming and extension of N-glycans; O-glycosylation; addition of sialyl groups
- **Proteolytic cleavage:** Activation of proproteins (e.g., prohormone processing)
- **Sulfation and phosphorylation:** Glycosaminoglycan sulfation; mannose-6-phosphate tag for lysosomal targeting
- **Sorting:** TGN receives multiple sorting signals and routes cargo to appropriate destinations

## Lysosomes and Vacuoles

**Lysosomes** are acidic (pH ~4.7) organelles containing ~60 hydrolytic enzymes. They are the cell's degradation center:
- **Autophagy:** Double-membrane autophagosomes deliver cytoplasmic contents (including dysfunctional organelles) to lysosomes for degradation and recycling
- **Endolysosomal pathway:** Endocytosed cargo delivered via early → late endosomes → lysosomes
- **Signaling hub:** Lysosomes sense amino acid availability and activate mTORC1 kinase — a master growth regulator

**Plant vacuoles** can occupy 90% of cell volume; maintain turgor pressure; also serve storage and degradative functions.

## Ribosomes

Ribosomes are the molecular machines of translation:
- **Prokaryotic 70S:** 30S (small, decodes mRNA) + 50S (large, catalyzes peptide bond formation). ~4500 ribosomes per *E. coli* cell.
- **Eukaryotic 80S:** 40S + 60S. A mammalian cell has $\sim 10^7$ ribosomes.
- **Catalytic mechanism:** The peptidyl transferase center (PTC) is composed entirely of rRNA — ribosomes are ribozymes.

## Why This Matters for Computational Biology

In synthetic biology, the target compartment of a transgene product must be specified. A protein intended for the secretory pathway needs a signal sequence for ER entry; a nuclear protein needs an NLS. In metabolic engineering, pathway enzymes must be in the correct compartment to access their substrates — pyruvate dehydrogenase is in the mitochondrial matrix; fatty acid synthase is cytoplasmic. In ODE models of signaling, compartment volumes matter: a signaling molecule present at 1000 copies in a nucleus of 500 fL has a concentration of $\sim 3$ µM — but if released into the cytoplasm (5000 fL), the concentration drops to 0.3 µM. Tracking compartment concentrations correctly requires knowing organelle volumes and the number of each organelle per cell. These numbers are not afterthoughts — they enter directly into any quantitative model of a pathway that crosses a compartment boundary.
