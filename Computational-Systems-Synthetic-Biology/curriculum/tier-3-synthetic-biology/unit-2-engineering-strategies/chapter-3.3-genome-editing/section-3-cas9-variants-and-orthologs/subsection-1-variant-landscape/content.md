# The Cas9 Variant and Ortholog Landscape

A child with Leber's congenital amaurosis — a form of inherited blindness — has a mutation in a gene expressed in photoreceptors. You want to correct it with CRISPR. But the mutation sits in a region of the genome with no NGG PAM nearby. Your delivery route is adeno-associated virus, which has a packaging limit of 4.7 kb. SpCas9 alone is 4.2 kb. The numbers don't work. This is not a corner case — PAM restrictions and size constraints are practical barriers that arise constantly in real editing projects. The solution is not to abandon CRISPR but to reach for a different Cas nuclease. The expanding toolkit of Cas9 variants and orthologs was built precisely to solve these problems, and choosing among them requires understanding what each one does differently and why.

SpCas9 is the workhorse of genome editing, but it is not always the best tool. PAM restrictions, protein size, specificity requirements, and the need for RNA editing have driven the development of dozens of Cas9 variants and related effectors. This section surveys the landscape of Cas nucleases and explains when each is appropriate.

## Why Alternatives to SpCas9 Are Needed

SpCas9 has three practical limitations:

1. **PAM restriction**: NGG must be present in the genomic DNA immediately adjacent to the target site. Approximately 1 in 8 bp offers a valid NGG PAM, but some critical positions (e.g., a specific pathogenic SNP) may not have a nearby NGG.

2. **Size**: SpCas9 is 4.2 kb, near the packaging limit for adeno-associated virus (AAV, ~4.7 kb capacity). After adding promoter, sgRNA cassette, and poly-A signal, a SpCas9 AAV approaches or exceeds capacity, limiting in vivo delivery.

3. **Specificity**: wild-type SpCas9 has meaningful off-target activity in some guide-sequence contexts, requiring additional strategies.

## Engineered SpCas9 PAM Variants

Rather than switching to an entirely different protein, PAM requirements can be altered by mutating the PAM-interacting domain of SpCas9.

### SpCas9-VQR (D1135V/R1335Q/T1337R)

Recognizes **NGA** PAM with activity comparable to wild-type SpCas9 on NGG. Approximately doubles the number of targetable sites by providing access to NGA positions.

### xCas9 (Evolved SpCas9)

Engineered by phage-assisted continuous evolution (PACE). Recognizes NG, GAA, and GAT PAMs with reduced efficiency. Useful for regions with no NGG nearby.

### SpRY (Near-PAMless)

The most permissive engineered PAM variant: recognizes **NRN** (where R = purine) and **NYN** (where Y = pyrimidine) PAMs. Effective across nearly all possible 3-base PAM sequences.

**Tradeoff**: SpRY has reduced specificity because PAM recognition is weakened. Must be used with high-fidelity protein engineering or RNP delivery to manage increased off-target activity.

## Smaller Cas9 Orthologs for AAV Delivery

For in vivo delivery via AAV, protein size is a critical constraint.

### SaCas9 (*Staphylococcus aureus* Cas9)

- Size: **3.2 kb** (1,053 aa), fits comfortably in AAV with promoter and sgRNA cassette
- PAM: **NNGRRT** (5-nucleotide PAM; more restrictive than NGG)
- Activity: comparable to SpCas9 in most contexts
- Use case: in vivo therapeutic editing (liver, eye, muscle via AAV delivery)
- Published applications: targeting PCSK9 for cholesterol reduction (Ran et al. 2015)

### CjCas9 (*Campylobacter jejuni* Cas9)

- Size: **2.95 kb** (984 aa): the smallest known Cas9
- PAM: NNNNRYAC (8-nucleotide, highly restrictive)
- Activity: lower than SaCas9 in many contexts
- Use case: situations where size is the limiting factor

## Cas12 Family (formerly Cpf1)

**Cas12a (Cpf1)** from *Acidaminococcus* and *Lachnospiraceae* is mechanistically distinct from Cas9:

- **5′ PAM**: TTTN on the non-target strand, rather than 3′ NGG
- **Staggered cut**: creates a 5-nt 5′ overhang rather than blunt ends — relevant for some HDR templates
- **Self-processing**: Cas12a processes its own crRNA arrays from a single transcript; no tracrRNA needed; multiple guides can be expressed from one cassette
- **Single RuvC domain**: no HNH; Cas12a cuts both strands with a single catalytic domain
- **Trans-cleavage**: upon target binding and cis-cleavage, Cas12a becomes a non-specific ssDNA nuclease (collateral cleavage). This is the basis for SHERLOCK diagnostics.

PAM: TTTV (where V = A, C, or G); AT-rich PAM means Cas12a targets AT-rich sequences that SpCas9 cannot.

**Use cases for Cas12a**:
- Multiplexed gene editing with crRNA arrays
- AT-rich genomes (Plasmodium, some plant genomes)
- SHERLOCK and DETECTR diagnostic applications

## Cas13 Family: RNA Targeting

**Cas13** proteins target RNA rather than DNA, making them fundamentally different in scope:

- No PAM requirement (RNA substrates)
- Creates no DSB in the genome
- Upon target RNA binding: Cas13 activates non-specific ssRNA cleavage (collateral activity)
- Orthogonal subtypes: Cas13a (LwaCas13a), Cas13b, Cas13d (CasRx)

**Applications**:
- RNA knockdown (programmable alternative to RNAi)
- RNA editing when combined with ADAR deaminase (without any genome cuts)
- Diagnostics: SHERLOCK (Specific High-sensitivity Enzymatic Reporter UnLOCKing) uses Cas13 collateral cleavage to amplify a detectable signal upon target RNA recognition
- Antiviral applications: targeting viral RNA genomes

## dCas9: The Programmable DNA Binding Protein

**dCas9** (catalytically dead Cas9) harbors D10A and H840A mutations, abolishing all nuclease activity. It retains sgRNA loading and DNA binding. dCas9 is the basis for:

- **CRISPRi**: dCas9-KRAB fusion represses nearby genes when targeted to promoters
- **CRISPRa**: dCas9-VP64 or dCas9-VPR fusion activates genes
- **Epigenome editing**: dCas9-DNMT3A (methylation), dCas9-TET1 (demethylation), dCas9-p300 (histone acetylation)
- **Imaging**: dCas9-GFP labels specific genomic loci for live-cell microscopy

## Comparison Table

| Nuclease | PAM | Size (kb) | Cut Type | Key Applications |
|----------|-----|---------|---------|----------------|
| SpCas9 | NGG | 4.2 | Blunt DSB | General editing |
| SpCas9-HF1/eSpCas9 | NGG | 4.2 | Blunt DSB | High-specificity editing |
| SpRY | NRN/NYN | 4.2 | Blunt DSB | PAM-flexible editing |
| SaCas9 | NNGRRT | 3.2 | Blunt DSB | AAV delivery in vivo |
| CjCas9 | NNNNRYAC | 2.95 | Blunt DSB | Ultra-compact AAV |
| Cas12a | TTTV | 3.9 | 5′ overhang | Multiplexing, diagnostics |
| Cas13a | None (RNA) | 2.8 | RNA collateral | RNA knockdown, diagnostics |
| dCas9 | NGG | 4.2 | None | CRISPRi/a, epigenome |
| nCas9 (D10A) | NGG | 4.2 | Nick (target strand) | Base editing, prime editing |

## Why This Matters

The expansion of the Cas nuclease toolkit from a single protein to dozens of variants and orthologs represents the maturation of CRISPR from a proof-of-concept into a fully engineerable platform. Selecting the right nuclease for a given application — considering PAM accessibility, size constraints, specificity requirements, and whether DNA or RNA is the target — is now a standard step in experimental design. The field's rate of discovering and engineering new Cas variants shows no signs of slowing: structure-guided engineering and directed evolution continue to expand what is targetable, deliverable, and correctable.
