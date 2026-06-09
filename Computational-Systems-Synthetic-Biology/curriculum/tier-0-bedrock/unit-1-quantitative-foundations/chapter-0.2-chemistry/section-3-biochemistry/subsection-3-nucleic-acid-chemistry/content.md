# Nucleic Acid Chemistry

In 1953, Watson and Crick described the double helix and ended their famous paper with a sentence of breathtaking understatement: "It has not escaped our notice that the specific pairing we have postulated immediately suggests a possible copying mechanism for the genetic material." The pairing they described — A with T, G with C, held together by hydrogen bonds across the major groove — is the physical mechanism of heredity. Every DNA replication event, every transcription, every CRISPR edit is grounded in the thermodynamics of these base pairs.

DNA and RNA are the information molecules of life. Their chemistry — the structure of their bases, the rules of base pairing, the thermodynamics of duplex formation — underpins every molecular biology technique and bioinformatics analysis. For computational biologists, nucleic acid chemistry is essential for understanding PCR, sequencing, CRISPR guide RNA design, aptamer engineering, and RNA secondary structure prediction. The quantitative treatment of nucleic acid hybridization thermodynamics is not merely academic: it is the algorithm inside every RNA secondary structure prediction tool and every PCR primer design program you will use.

## DNA Structure

**Nucleotides** consist of a nitrogenous base, a pentose sugar (deoxyribose in DNA, ribose in RNA), and one to three phosphate groups.

**Bases:**
- Purines (two-ring): Adenine (A), Guanine (G)
- Pyrimidines (one-ring): Cytosine (C), Thymine (T, DNA only), Uracil (U, RNA only)

Thymine differs from uracil by a 5-methyl group — this methyl group is added to uracil to produce thymine, providing a mechanism for recognizing deaminated cytosine (which produces uracil) as damaged DNA rather than a normal base.

**Watson-Crick base pairing:**
- A–T (DNA): 2 hydrogen bonds
- A–U (RNA): 2 hydrogen bonds  
- G–C: 3 hydrogen bonds

G-C pairs are stronger because of the additional hydrogen bond — this is why high GC-content DNA has a higher melting temperature.

**B-form DNA** (the canonical form in aqueous solution):
- Right-handed double helix
- ~10.5 bp per turn (34 Å per turn, 3.4 Å per base pair)
- Major groove (~12 Å wide) and minor groove (~6 Å wide)
- Major groove is wider and deeper — most transcription factors read sequence information through major groove contacts

**A-form DNA:** Occurs in RNA-DNA hybrids and double-stranded RNA. Shorter, wider, right-handed, 11 bp/turn. The 2'-OH groups of RNA prevent B-form geometry.

**Z-form DNA:** Left-handed helix. Occurs in GC-rich sequences under high salt or superhelical stress. Biological role under investigation; may be a signal for Z-DNA binding proteins.

**Supercoiling:** The double helix can be further coiled — **positive supercoiling** (overwound) or **negative supercoiling** (underwound). In bacteria, DNA is ~6% negatively supercoiled, which facilitates strand separation during transcription and replication. **Linking number** $Lk = Tw + Wr$ where $Tw$ is twist and $Wr$ is writhe. **Topoisomerases** change Lk:
- Topo I (Type I): cuts one strand, changes Lk by ±1
- Topo II (Type II, Gyrase in bacteria): cuts both strands, changes Lk by ±2; gyrase introduces negative supercoiling using ATP

## RNA Structure

RNA is single-stranded but folds back on itself through intramolecular base pairing:

**Secondary structure elements:**
- **Hairpin loop:** A stem formed by complementary sequences with a single-stranded loop at the top
- **Internal loop/bulge:** Mismatches within a helix
- **Pseudoknot:** A more complex fold where a loop base-pairs with a complementary sequence outside the hairpin — creates knot-like topology; important in ribozymes and riboswitches

**Tertiary structure:** Long-range interactions between secondary structure elements. The most complex RNA structures include:
- **Ribosomes:** rRNA forms the catalytic core of the peptidyl transferase center
- **Ribozymes:** RNA enzymes (hammerhead, HDV, Group I/II introns, RNase P)
- **Riboswitches:** mRNA elements that bind small molecules and control gene expression — act as RNA-based sensors

**Chemical differences from DNA:** The 2'-hydroxyl group of ribose makes RNA:
- More susceptible to alkaline hydrolysis (a hydroxyl ion deprotonates the 2'-OH, which attacks the adjacent phosphodiester bond — half-life of RNA at pH 7 is months, but hours at pH 11)
- A better catalyst (the 2'-OH can participate in catalysis as a nucleophile)
- Unable to adopt the B-form helix

## Thermodynamics of Base Pairing

The stability of a nucleic acid duplex is characterized by:
- **Melting temperature $T_m$:** The temperature at which 50% of the duplex is dissociated. Measured by UV absorbance at 260 nm (hyperchromic effect: single-stranded DNA absorbs ~40% more light than duplexed DNA due to loss of base stacking).

**Nearest-neighbor model:** The $\Delta G$ of duplex formation depends on the identity of each adjacent base pair (dinucleotide step):

$$\Delta G = \sum_i \Delta G_{i,i+1} + \Delta G_{\text{init}}$$

where $\Delta G_{i,i+1}$ is the free energy contribution of each nearest-neighbor pair. There are 10 unique dinucleotide steps for DNA, each with experimentally determined $\Delta H$ and $\Delta S$ values (SantaLucia 1998 unified nearest-neighbor parameters).

**Simple $T_m$ estimates:**
- For oligos $< 14$ nt: $T_m \approx 2°C \times (A+T) + 4°C \times (G+C)$ (Wallace rule)
- For longer sequences: $T_m \approx 81.5 + 16.6 \log[Na^+] + 0.41 \times \%GC - 675/L$ (Primer Blast formula)

**Why this matters for PCR:** Primers must anneal efficiently at a single temperature. Too high $T_m$: primers won't anneal at the cycling temperature, no amplification. Too low $T_m$: primers anneal non-specifically, multiple products. Rules of thumb: design primers with $T_m \approx 60°C$, balanced GC content (40-60%), and no self-complementarity.

## Guide RNA Design for CRISPR-Cas9

The CRISPR-Cas9 system requires a **guide RNA (gRNA)** that is complementary to the target DNA. Thermodynamic considerations for effective guide design:
1. **Avoid self-folding:** The guide spacer should not form internal hairpins ($\Delta G_{\text{fold}} > -2$ kcal/mol)
2. **GC content:** 40-80% preferred. Too low = weak binding; too high = off-target risk
3. **Secondary structure avoidance:** The 12 nt "seed region" at the 3' end of the spacer (adjacent to PAM) must be single-stranded for Cas9 binding
4. **Minimize off-targets:** Sequences with >3 mismatches in the seed region are usually not cleaved

## Why This Matters for Computational Biology

Nucleic acid thermodynamics is the physical basis of every hybridization-based technology: PCR, microarrays, FISH, Northern/Southern blots, molecular beacons. Free energy minimization is the algorithm underlying RNA secondary structure prediction (mfold, RNAfold) — these tools compute the structure that minimizes the sum of nearest-neighbor free energies. CRISPR guide RNA design, ASO (antisense oligonucleotide) design, and siRNA design all depend on thermodynamic models of nucleic acid hybridization. Aptamer design and selection requires understanding what makes RNA structures stable. Understanding the chemistry is understanding why these tools work and when they fail.

```python
# Nearest-neighbor Tm calculation (SantaLucia 1998 parameters)
# Simplified for illustration

def tm_simple(sequence, Na_mM=50, oligo_nM=250):
    """
    Estimate Tm using simplified nearest-neighbor (SantaLucia 1998).
    """
    # Thermodynamic parameters for DNA/DNA duplexes (kcal/mol)
    # Format: (dH, dS) where dS is in cal/mol/K
    nn_params = {
        'AA': (-7.9, -22.2), 'AT': (-7.2, -20.4), 'TA': (-7.2, -21.3), 'CA': (-8.5, -22.7),
        'GT': (-8.4, -22.4), 'CT': (-7.8, -21.0), 'GA': (-8.2, -22.2), 'CG': (-10.6, -27.2),
        'GC': (-9.8, -24.4), 'GG': (-8.0, -19.9),
    }
    # Initiation
    dH = 0.1   # kcal/mol
    dS = -2.8  # cal/mol/K (initiation for all sequences)
    
    seq = sequence.upper().replace('U', 'T')
    for i in range(len(seq)-1):
        pair = seq[i:i+2]
        comp_pair = seq[i:i+2].translate(str.maketrans('ATGC', 'TACG'))[::-1]
        for p in [pair, comp_pair]:
            if p in nn_params:
                dH += nn_params[p][0]
                dS += nn_params[p][1]
                break
    
    # Salt correction
    dS += 0.368 * (len(seq) - 1) * np.log(Na_mM * 1e-3)
    
    # R = 1.987 cal/mol/K, CT = oligo concentration (total strand conc)
    R = 1.987
    CT = oligo_nM * 1e-9
    dS_total = dS * 1e-3  # convert cal/mol/K to kcal/mol/K
    Tm = dH / (dS_total + R * 1e-3 * np.log(CT/4)) - 273.15
    return Tm

import numpy as np

primers = {
    'Low GC (40%)': 'ATTTCGGATAACGGTATCCA',  # 40% GC
    'Medium GC (55%)': 'ATGCGTACGATCGGCTAATG',  # 55% GC  
    'High GC (70%)': 'GCGCATCGGCTATCGGCATG',   # 70% GC
}

print("Primer Tm estimates:")
for name, seq in primers.items():
    gc = sum(b in 'GC' for b in seq) / len(seq) * 100
    tm = tm_simple(seq)
    print(f"  {name}: GC={gc:.0f}%, Tm≈{tm:.1f}°C")
```
