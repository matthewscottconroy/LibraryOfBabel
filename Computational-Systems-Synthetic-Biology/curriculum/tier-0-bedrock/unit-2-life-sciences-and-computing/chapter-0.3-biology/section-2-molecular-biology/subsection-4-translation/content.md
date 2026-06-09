# Translation

The ribosome is arguably the most important molecular machine in the history of life. Before there were cells, before there were nuclei, before there were organelles, there was the ribosome — or something like it. The translation machinery that reads mRNA and produces protein is so ancient, so universally conserved, and so central to everything a cell does that it consumes roughly 80% of cellular energy in fast-growing bacteria. When *E. coli* races to double in 20 minutes, the vast majority of that metabolic budget is going into making ribosomes and running them. Translation is the process by which the information in DNA is finally cashed out as function — and its quantitative parameters directly determine protein production rates and are central to any model of gene expression.

Translation is the decoding of mRNA into protein by the ribosome. It is one of the most conserved and energy-intensive processes in the cell — in rapidly growing *E. coli*, ~80% of cellular energy is allocated to ribosome synthesis and translation. The quantitative parameters of translation (initiation rate, elongation rate, codon usage, ribosome density) directly determine protein production rates and are central to any model of gene expression.

## The Genetic Code

The genetic code maps codons (triplets of mRNA nucleotides read 5'→3') to amino acids. Key features:

- **64 codons** encode 20 amino acids plus 3 stop codons (UAA, UAG, UGA)
- The code is **degenerate** (redundant): most amino acids are encoded by 2–6 synonymous codons
- The code is **nearly universal**: with minor exceptions (mitochondria, some ciliates), the same code operates across all life — strong evidence for a single origin of life
- **Codon usage bias**: synonymous codons are not used equally; organisms are biased toward codons decoded by abundant tRNAs, affecting translation speed and accuracy

**Start codon**: AUG (Met/fMet); rare alternative starts (GUG, UUG in bacteria) occur
**Stop codons**: UAA ("ochre"), UAG ("amber"), UGA ("opal") — decoded by release factors, not tRNAs

## Transfer RNA Structure and Aminoacylation

**tRNA** molecules are ~73–93 nt adaptor molecules that link codons to amino acids. Their conserved cloverleaf secondary structure folds into an L-shaped tertiary structure:
- **Anticodon loop**: 3-nt anticodon pairs with the mRNA codon (antiparallel, 3'→5')
- **Acceptor stem**: 3'-CCA terminus is the site of amino acid attachment (aminoacyl-tRNA)

**Aminoacyl-tRNA synthetases (aaRS)**: 20 enzymes, one per amino acid. Each aaRS must recognize:
1. Its cognate amino acid
2. Its cognate tRNA (the "second genetic code")

The reaction uses ATP: amino acid + tRNA + ATP → aminoacyl-tRNA + AMP + PPi

This is the step at which amino acids are irreversibly committed to their tRNA identity — after this point, the ribosome cannot distinguish amino acid identity and relies entirely on codon-anticodon base pairing.

**Wobble base pairing**: The third codon position (3' position) pairs with the first anticodon position (5' position) with relaxed base-pairing rules. Wobble allows one tRNA to decode multiple synonymous codons. Inosine (hypoxanthine) at the wobble position can pair with U, C, or A.

## Ribosome Structure and the Translation Cycle

The **ribosome** is a 2-subunit ribonucleoprotein complex:

| Feature | Prokaryote | Eukaryote |
|---|---|---|
| Full complex | 70S (2.5 MDa) | 80S (4.3 MDa) |
| Small subunit | 30S (16S rRNA + 21 proteins) | 40S (18S rRNA + 33 proteins) |
| Large subunit | 50S (23S + 5S rRNA + 34 proteins) | 60S (28S + 5.8S + 5S rRNA + 49 proteins) |

The ribosome has three tRNA-binding sites:
- **A site (aminoacyl)**: incoming aminoacyl-tRNA
- **P site (peptidyl)**: tRNA bearing the growing peptide chain
- **E site (exit)**: deacylated tRNA exiting the ribosome

**The elongation cycle** (repeated for each amino acid):
1. **Decoding**: Aminoacyl-tRNA•EF-Tu•GTP ternary complex enters the A site; cognate tRNA is selected by codon-anticodon matching; GTP hydrolysis triggers accommodation
2. **Peptidyl transfer**: The 23S rRNA (in bacteria) catalyzes transfer of the peptide from the P-site tRNA to the A-site amino acid; the peptide bond forms in ~50 ms. The ribosome is a ribozyme — the peptidyl transferase center is RNA.
3. **Translocation**: EF-G•GTP promotes movement of the mRNA by one codon; A-site becomes empty; former A-site tRNA moves to P site; former P-site tRNA moves to E site

Elongation rate: **~15–20 amino acids/s** in *E. coli*; ~5–10 aa/s in mammalian cells. A 300 aa protein requires ~15–20 s of elongation.

## Initiation: Bacterial vs. Eukaryotic

**Bacterial initiation** is simpler and more robust:
1. The small (30S) subunit binds the **Shine-Dalgarno (SD) sequence** in the 5' UTR. The SD sequence (`AGGAGG`) is complementary to the 3' end of 16S rRNA; the spacing between SD and the start codon (optimally 5–10 nt) determines initiation efficiency
2. The initiator tRNA (**fMet-tRNA^fMet**) binds the P site
3. The 50S subunit joins, forming the 70S initiation complex
4. The SD sequence is the primary determinant of translation initiation rate in bacteria and is the key engineering target for controlling protein levels

**Eukaryotic initiation** (cap-dependent):
1. The **43S pre-initiation complex** (40S + Met-tRNA + eIF1/1A/3) binds the 5' cap via **eIF4F** (eIF4E + eIF4G + eIF4A)
2. The complex **scans** 5'→3' from the cap until it encounters an AUG in a favorable **Kozak context** (`GCCGCC`**ACC**`AUGG` — the bold nucleotides flanking AUG are most critical)
3. Start codon recognition triggers eIF5-mediated GTP hydrolysis and 60S joining

## Ribosome Density and the Polysome

Multiple ribosomes translate a single mRNA simultaneously — a **polysome** (polyribosome). In *E. coli*, one ribosome initiates roughly every **2–5 s** on a well-translated mRNA. Given an elongation rate of 15 aa/s and a 300 aa ORF, translation takes ~20 s, so a polysome might carry ~4–10 ribosomes.

Ribosome density can be measured by **ribosome profiling (Ribo-seq)**, which uses nuclease footprinting of ribosomes followed by sequencing of the protected ~30-nt fragments.

## Termination and Recycling

When a stop codon enters the A site, no aminoacyl-tRNA decodes it. Instead:
- **RF1** (UAA, UAG) or **RF2** (UAA, UGA) in bacteria enter the A site and catalyze hydrolysis of the peptide-tRNA bond, releasing the polypeptide
- **RF3•GTP** promotes RF1/RF2 dissociation
- **RRF (ribosome recycling factor)** and EF-G disassemble the post-termination complex, regenerating subunits for the next round

## Why This Matters for Computational Biology

Translation rate is the primary determinant of protein abundance (along with protein degradation rate). In synthetic biology, controlling protein levels requires tuning both transcription rate and translation initiation rate — typically via promoter engineering and RBS (ribosome binding site) optimization. The **RBS Calculator** (Salis lab) uses thermodynamic models of 30S-mRNA folding to predict translation initiation rates from sequence. Codon optimization affects elongation speed and co-translational folding. In genome-scale models, ribosome allocation is a critical constraint — bacteria optimize codon usage to match tRNA abundances, and this is captured in models like the ME-model for *E. coli*. Ribo-seq data enable measuring translation efficiency genome-wide and are inputs for models of translational regulation. The key insight for engineering is that mRNA and protein levels are not simply proportional: the same mRNA can be translated at vastly different rates depending on its 5' UTR structure, codon composition, and the availability of specific charged tRNAs under the growth conditions you care about.
