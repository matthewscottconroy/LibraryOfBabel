# PURE System Components

Here is a question worth sitting with: how many proteins does it take to make a protein? The answer — worked out painstakingly by Shimizu et al. in 2001 — is at least 36. That is the number of individually purified components that, when mixed together in the right proportions, reconstitute the full machinery of translation from scratch. The result is the PURE (Protein synthesis Using Recombinant Elements) system: the most completely defined cell-free protein synthesis platform in existence. Every component is individually purified and added in controlled amounts, eliminating the "unknown unknowns" of crude cell extracts. Understanding the PURE system's composition explains the minimal requirements for life's core information-processing reaction — translation — and provides the foundation for engineering cell-free systems for new purposes.

## The Complete Component List

Shimizu et al. (2001) reconstructed protein synthesis from 36 individually purified components. These fall into five functional categories.

### Category 1: Ribosomes

**70S ribosome**: the core particle responsible for peptide bond formation. Assembled from:
- **30S subunit** (small): 16S rRNA + 21 ribosomal proteins. Decodes mRNA; discriminates cognate from non-cognate aminoacyl-tRNAs.
- **50S subunit** (large): 23S rRNA + 5S rRNA + 31 ribosomal proteins. Peptidyl transferase center; exit tunnel.

Ribosomes are purified from *E. coli* by ultracentrifugation through sucrose gradients. Yield: ~5–10 mg/L culture. PURE reactions contain ~0.5–1 µM ribosomes (each can synthesize ~1 protein per minute under optimal conditions).

### Category 2: Translation Initiation Factors (IFs)

Three initiation factors are required to assemble the ribosome on the mRNA start codon:

**IF1**: occupies the A site of the 30S subunit; promotes IF3 binding and fMet-tRNAfMet positioning. Stabilizes the open conformation of the 30S subunit.

**IF2 (GTPase)**: binds fMet-tRNAfMet and escorts it to the P site of the 30S ribosome. Hydrolyzes GTP upon 50S joining, releasing IF2 and allowing productive elongation.

**IF3**: dissociates 70S ribosomes into subunits; prevents premature subunit joining; enhances start codon discrimination (ensures AUG at the correct position).

### Category 3: Translation Elongation Factors (EFs)

**EF-Tu (GTPase)**: forms a ternary complex (EF-Tu:GTP:aminoacyl-tRNA) that delivers aminoacyl-tRNA to the A site. Hydrolyzes GTP upon codon-anticodon recognition, triggering accommodation of the aa-tRNA.

**EF-Ts**: GDP/GTP exchange factor for EF-Tu. Converts EF-Tu:GDP (after GTP hydrolysis) back to EF-Tu:GTP for the next round.

**EF-G (GTPase)**: translocase. After peptide bond formation, EF-G:GTP promotes translocation — moving the mRNA by one codon and repositioning the peptidyl-tRNA from A site to P site. GTP hydrolysis is coupled to the mechanical translocation step.

**TUFB (EF-Tu variant)**: some PURE protocols include both EF-Tu (tufA gene product) and TUFB (tufB gene product), which are functionally identical but provide redundancy.

### Category 4: Termination and Recycling Factors

**RF1 (Release Factor 1)**: recognizes UAA and UAG stop codons; catalyzes peptidyl-tRNA hydrolysis, releasing the completed polypeptide from the P site.

**RF2**: recognizes UAA and UGA stop codons. (With RF1, covers all three stop codons.)

**RF3 (GTPase)**: stimulates release of RF1 and RF2 from the ribosome after peptide release; GTP hydrolysis drives this dissociation.

**RRF (Ribosome Recycling Factor)**: works with EF-G to disassemble the post-termination ribosome complex. Together they dissociate the 70S into subunits, release mRNA, and allow tRNA departure — recycling all components for the next initiation event.

### Category 5: Aminoacyl-tRNA Synthetases (aaRS)

All 20 aminoacyl-tRNA synthetases (one per standard amino acid; some organisms have fewer due to transamidation pathways) must be present:

AlaRS, ArgRS, AsnRS, AspRS, CysRS, GlnRS, GluRS, GlyRS, HisRS, IleRS, LeuRS, LysRS, MetRS, PheRS, ProRS, SerRS, ThrRS, TrpRS, TyrRS, ValRS

Each aaRS performs a two-step reaction:
$$\text{Amino acid} + \text{ATP} \rightarrow \text{aa-AMP} + PP_i$$
$$\text{aa-AMP} + \text{tRNA} \rightarrow \text{aa-tRNA} + \text{AMP}$$

In the PURE system, all 20 aaRS are purified from *E. coli* and added in slight excess to ensure complete aminoacylation of all tRNAs.

### Category 6: tRNAs and mRNA

**tRNAs**: total *E. coli* tRNA mixture prepared by phenol extraction. Each amino acid requires one or more cognate tRNAs; the mixture ensures all codons are decodable.

**mRNA**: synthetic mRNA is added, typically transcribed in vitro from a T7 promoter-containing DNA template using T7 RNA polymerase.

**T7 RNA polymerase**: added to the PURE system to enable simultaneous transcription of a DNA template + translation. Allows the DNA template (PCR product or plasmid) to be used directly without prior mRNA preparation.

### Category 7: Energy Regeneration System

Translation consumes large amounts of ATP and GTP (2 GTPs per elongation cycle, plus 1 ATP per aminoacylation). The energy regeneration system replenishes these:

**Standard energy system (PURE)**:
- Creatine kinase (CK) + creatine phosphate (CP): $\text{CP} + \text{ADP} \rightarrow \text{creatine} + \text{ATP}$. Sustains ATP concentration at ~5 mM for 1–4 hours.
- Nucleoside diphosphate kinase (NDK): converts ATP to GTP, CTP, UTP as needed.

**Alternative energy systems** (for longer-duration reactions):
- Pyruvate kinase + phosphoenolpyruvate
- Maltose/maltodextrin system: harnesses the glycolytic pathway in PURE+ systems
- Oxidative phosphorylation (requires membrane vesicles with F1Fo ATP synthase)

### Small Molecule Components

**Magnesium glutamate**: Mg²⁺ is essential for ribosome assembly and polymerase activity. Optimal Mg²⁺ concentration = 5–8 mM in PURE (lower than *E. coli* cytoplasm ~2 mM but compensates for buffering by extract components in crude systems).

**Potassium glutamate**: monovalent cation; ionic strength optimization.

**Spermidine**: polyamine; stabilizes RNA structure and ribosome assembly.

**Putrescine**: another polyamine; optimizes translation rate.

**All 20 amino acids**: provided at 2–4 mM each.

**NTPs** (ATP, GTP, CTP, UTP): for transcription by T7 RNAP; also replenished by CK/CP.

**DTT or β-mercaptoethanol**: reducing agent; prevents disulfide bond formation in proteins that should be non-disulfide-linked.

## PURE System Performance

**Protein yield**: 100–400 µg/mL in batch mode; higher with CECF format (continuous exchange cell-free, where substrates are continuously replenished through a dialysis membrane)

**Reaction duration**: 2–4 hours (batch); 6–24 hours (CECF)

**Cost**: ~10–50× more expensive than crude E. coli extracts per µg of protein produced (due to purified component costs)

**Advantages unique to PURE**:
- Remove any component to study its specific role
- Add alternative or engineered components (e.g., orthogonal aaRS/tRNA for ncAA incorporation)
- No background activity from unknown extract components
- Defined composition enables quantitative modeling of translation kinetics

## Commercial PURE Systems

**PURExpress (NEB, E6800)**: ready-to-use PURE system; add DNA template and start. Produces 15–30 µg/mL of soluble protein for most targets.

**myTXTL (Arbor Biosciences)**: extract-based system (not PURE) but optimized for synthetic biology applications; simpler and less expensive than PURE.

**WEPURE**: a variant of PURE supplemented with wheat germ ribosomes for expressing eukaryotic proteins with disulfide bonds or specific post-translational requirements.

## Why This Matters

The PURE system demonstrates that the core machinery of translation — one of the most complex molecular processes known — can be reduced to a defined list of 36 components that can be individually purified, combined, and made to work. This defines the minimum required parts for protein synthesis from first principles. Every component on the list is essential: removing any one abolishes or dramatically reduces translation. This parts list is both a profound statement about the molecular logic of life and a practical engineering specification. For synthetic biology, the PURE system provides a platform where every component can be replaced with an engineered variant — orthogonal ribosomes, evolved aaRS with new amino acid specificities, or alternative energy regeneration systems — without the confounding background of the 5,000 other proteins present in a crude cell extract.
