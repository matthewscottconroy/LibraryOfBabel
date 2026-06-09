# Genetic Manipulation Tools

The ability to read a genome is powerful. The ability to write one — to introduce, delete, and modify genetic information at will — is transformative. The three classical mechanisms by which DNA enters bacterial cells are not mere laboratory curiosities; they are the tools that built modern molecular biology, that enabled the construction of every synthetic circuit discussed in this curriculum, and that continue to shape how we think about engineering living systems. Understanding them mechanistically is necessary both for experimental design and for modeling how microbial populations acquire new genetic information.

The ability to introduce, delete, and modify genetic information in microorganisms is the enabling technology of molecular biology and synthetic biology. Three classical mechanisms — **transformation**, **transduction**, and **conjugation** — allow DNA to enter bacterial cells, each with different efficiency, cargo capacity, and host range.

## Transformation: Uptake of Naked DNA

**Transformation** is the uptake of free DNA from the environment by a bacterial cell. It occurs naturally in bacteria that are **competent** — able to bind and internalize DNA.

### Natural Competence

Some bacteria develop natural competence at specific growth phases or under specific environmental signals:
- **Streptococcus pneumoniae**: competent during early exponential phase; regulated by the ComCDE quorum sensing system; ~10% of cells in a population are simultaneously competent
- **Bacillus subtilis**: competent in late exponential/early stationary phase; ~10–20% of cells at any time; regulated by a sigma factor cascade (ComK)
- **Haemophilus influenzae**: requires specific DNA uptake sequences (DUS: `AAGTGCGGT`); discriminates self from non-self DNA

Mechanism of natural transformation in *B. subtilis*:
1. ComB/C binds dsDNA, ComC cleaves one strand
2. The remaining ssDNA strand is threaded through a membrane pore (ComE/EC complex)
3. ssDNA is protected by SsbB inside the cell
4. RecA-mediated homologous recombination integrates the incoming strand if a homologous region exists

Transformation frequency: typically $10^{-4}$ to $10^{-6}$ transformants per recipient cell for chromosomal integration.

### Artificial Transformation

*E. coli* is not naturally competent; artificial methods are used:

**Chemical transformation (heat shock)**: Cells are incubated with ice-cold CaCl₂ → competent cells are mixed with plasmid DNA → briefly heated to 42°C (heat shock) → recovered in SOC medium for ~1 hour → plated.
- Efficiency: $10^6$–$10^9$ colonies per µg of supercoiled plasmid DNA (high-quality competent cells)
- Mechanism poorly understood; Ca²⁺ may destabilize the outer membrane and neutralize charge repulsion between DNA and the membrane

**Electroporation**: High-voltage electrical pulse (~2500 V/cm, 5 ms for *E. coli*) creates transient pores in the cell membrane through which DNA enters.
- Efficiency: $10^9$–$10^{10}$ colonies per µg plasmid (highest of any method)
- Works for many bacteria that resist chemical transformation
- Critical parameters: voltage, pulse duration, cell density, DNA quality, cuvette gap

Transformation efficiency matters: for library transformations (generating diverse mutant libraries for directed evolution or CRISPR screens), you need sufficient colonies to sample the entire library. A $10^6$-member library requires at least $5 \times 10^6$ transformants (5-fold coverage) to ensure most variants are represented.

## Transduction: Phage-Mediated DNA Transfer

**Transduction** uses bacteriophage as a vehicle to transfer DNA between cells. Because phage DNA packaging is not perfectly specific, host DNA can be packaged instead of phage DNA.

### Generalized Transduction

During **generalized transduction**, phage accidentally packages a fragment of host chromosomal DNA (essentially any ~40–50 kb fragment in the case of P1 phage). The pseudo-virion injects this DNA into a new host, where it can integrate by homologous recombination.

Key system: **P1 phage** transduction in *E. coli*:
- P1 infects donor strain; produces a lysate containing ~99.9% true P1 phage + ~0.1% transducing particles carrying host DNA
- Transducing particles infect recipient strain
- The donor DNA fragment (if homologous to the recipient chromosome) integrates by RecA-mediated HR
- Frequency: ~$10^{-6}$ to $10^{-8}$ per recipient per phage particle; transducing particles are ~1 in 10⁶ phage

P1 transduction is used routinely for **allele exchange** in *E. coli*: a mutation or marker constructed in one strain can be moved to any other *E. coli* strain by P1 transduction without repeated genetic construction.

### Specialized Transduction

**Specialized transduction** occurs only with temperate phage (like λ). Upon induction, λ may excise imprecisely, taking adjacent bacterial genes (*att* site flanking genes) with it. The resulting phage can transduce these specific genes to new hosts at high frequency. This was historically important for phage genetics experiments.

## Conjugation: Plasmid Transfer via Mating Bridge

**Conjugation** is the most efficient mechanism for transfer of large DNA molecules between bacteria. It requires cell-to-cell contact mediated by the **F pilus** and a dedicated **type IV secretion system (T4SS)**.

### The F Plasmid System

The *E. coli* **F (fertility) plasmid** (94.5 kb) is the paradigm:
- F+ cells (carry F plasmid) form pili that retract to bring cells into contact with F– cells
- A nick at the **oriT (origin of transfer)** generates a rolling-circle intermediate
- ssDNA is transferred 5'→3' into the recipient; the donor retains a copy (ssDNA is replicated in both cells)
- Conjugation is efficient: under optimal conditions (exponential phase, good contact), nearly every F+ cell can transfer F to an F– cell; the recipient becomes F+ within 30 min
- Transfer rate: $10^{-1}$ to $10^{-3}$ transconjugants per donor per hour (much higher than transformation or transduction)

Conjugation has a much higher capacity for DNA transfer than transformation (entire 100 kb plasmids readily transferred) and can cross species boundaries more readily than transformation.

## Lambda Red Recombinase System

The **λ Red system** enables **precise chromosomal editing** without selection markers using homologous recombination:

The λ Red proteins (Exo, Beta, Gam) are expressed from a helper plasmid:
- **Gam**: inhibits host RecBCD exonuclease, protecting linear DNA ends
- **Exo**: 5'→3' exonuclease creates 3'-ssDNA overhangs from linear PCR product
- **Beta**: single-strand annealing protein; promotes recombination between the ssDNA overhangs and the chromosome

Protocol (Datsenko-Wanner):
1. PCR-amplify a resistance cassette with 50-nt homology arms matching the genomic target
2. Electroporate into cells expressing λ Red proteins
3. Recombinants replace the target locus with the cassette
4. Cassette can be removed by FLP recombinase (FRT sites flank the cassette)

Efficiency: ~$10^{-6}$ to $10^{-4}$ per electroporated cell; the Keio collection of ~4000 *E. coli* single-gene knockouts was constructed this way. The Keio collection is a beautiful example of how a single experimental platform, applied systematically, can transform a field: knowing the phenotype of each single-gene deletion in *E. coli* has been essential for understanding gene function, essential gene networks, and the relationship between genotype and growth phenotype.

## Why This Matters for Computational Biology

Transformation efficiency directly determines the scale of synthetic biology experiments: library size is limited by transformation efficiency. Transduction frequency determines how rapidly antibiotic resistance genes spread through an *E. coli* population — models of HGT dynamics use these empirical transfer rates. Conjugation is a key parameter in models of horizontal gene transfer in the gut microbiome, in wastewater treatment plants, and in agricultural soils. Understanding conjugation mechanics is necessary to design **genetic kill switches** that prevent engineered bacteria from transferring synthetic DNA to environmental organisms. λ Red recombineering is the basis for MAGE (multiplex automated genome engineering) — an approach for large-scale chromosomal editing used in genome-scale engineering projects such as producing an all-reassigned codon *E. coli* genome.
