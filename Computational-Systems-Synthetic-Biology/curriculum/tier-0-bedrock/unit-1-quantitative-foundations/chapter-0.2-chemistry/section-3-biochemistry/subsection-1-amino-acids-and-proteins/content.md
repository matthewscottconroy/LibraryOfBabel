# Amino Acids and Proteins

In 1969, Christian Anfinsen won the Nobel Prize for demonstrating that a protein's three-dimensional structure is completely determined by its amino acid sequence. He showed that ribonuclease A, denatured into a random coil by urea and a reducing agent, would spontaneously refold into its native, catalytically active structure when those denaturing conditions were removed. There was no template, no external scaffold, no cellular machinery involved — just the primary sequence and the laws of thermodynamics.

This observation is both liberating and daunting. Liberating, because it means that if you can design a sequence, you can in principle design a structure and a function. Daunting, because it means that the twenty amino acids, combined in chains of hundreds to thousands of residues, encode a three-dimensional shape of almost incomprehensible complexity through their pairwise and higher-order interactions. The relationship between sequence and structure — deciphered computationally by AlphaFold2 fifty years after Anfinsen — remains one of the most stunning achievements in the history of computational biology.

Proteins are the molecular machines of life — they catalyze reactions, transmit signals, provide structural support, and carry out virtually every cellular function. The structure and properties of proteins are entirely determined by the sequence of their 20 standard amino acids. Understanding amino acid chemistry and protein structure hierarchy is foundational for computational protein analysis, molecular modeling, structural bioinformatics, and synthetic biology.

## The Twenty Standard Amino Acids

All 20 standard amino acids share the same core structure: an $\alpha$-carbon bonded to an amino group (–NH$_3^+$ at neutral pH), a carboxylate group (–COO$^-$ at neutral pH), a hydrogen, and a variable **side chain** (R group) that defines the amino acid's identity and chemical properties.

**Nonpolar aliphatic:** Glycine (Gly, G), Alanine (Ala, A), Valine (Val, V), Leucine (Leu, L), Isoleucine (Ile, I), Proline (Pro, P), Methionine (Met, M). These residues prefer the hydrophobic core of proteins. Glycine is the smallest (R = H) — highly flexible. Proline is rigid and breaks $\alpha$-helices.

**Aromatic:** Phenylalanine (Phe, F), Tryptophan (Trp, W), Tyrosine (Tyr, Y). Hydrophobic, but Tyr has an –OH group. Phe and Trp absorb UV at 280 nm — the basis of A$_{280}$ protein quantification. Trp fluoresces, making it useful for conformational studies.

**Polar uncharged:** Serine (Ser, S), Threonine (Thr, T), Cysteine (Cys, C), Asparagine (Asn, N), Glutamine (Gln, Q). These form hydrogen bonds. Ser and Thr are phosphorylation targets. Cys forms disulfide bonds.

**Charged (positive at pH 7):** Lysine (Lys, K, pKa ~10.5), Arginine (Arg, R, pKa ~12.5), Histidine (His, H, pKa ~6.0). Lys and Arg are permanently positive; His is the only residue that can switch between charged and uncharged near physiological pH — making it the key acid-base catalyst in many enzymes.

**Charged (negative at pH 7):** Aspartate (Asp, D, pKa ~3.9), Glutamate (Glu, E, pKa ~4.1). Both are negatively charged carboxylates at physiological pH. They participate in salt bridges, coordinate metal ions, and act as catalytic residues.

## Protein Structure Hierarchy

**Primary structure:** The linear sequence of amino acids connected by peptide bonds. The primary structure encodes everything — the 3D fold, function, stability, interactions. It is determined by the gene sequence.

**Secondary structure:** Regular, repeating local structures stabilized by hydrogen bonds within the polypeptide backbone.

- **$\alpha$-helix:** Right-handed helix with 3.6 residues per turn. The C=O of residue $i$ forms an H-bond with the N-H of residue $i+4$. The helix is ~5.4 Å per turn. Side chains project outward. Amphipathic helices have hydrophobic and hydrophilic faces — common in membrane proteins and coiled coils.

- **$\beta$-sheet:** Adjacent strands hydrogen bond between backbone C=O and N-H groups. Can be parallel (strands running in same direction) or antiparallel (opposite directions — antiparallel is more stable due to H-bond geometry). $\beta$-sheets are the basis of the immunoglobulin fold, the TIM barrel, and amyloid fibrils.

- **Loops and turns:** Non-repetitive structures connecting helices and sheets. Often found at protein surfaces; frequently mediate protein-protein interactions.

**Tertiary structure:** The complete 3D fold of a single polypeptide chain. Stabilized by:
- **Hydrophobic effect:** Non-polar side chains pack into the interior away from water — the dominant driving force for folding (~$\Delta G_{\text{fold}} \approx -50$ kJ/mol for a typical 100-residue protein)
- **Hydrogen bonds:** Between backbone and side chain groups
- **Van der Waals interactions:** Packing of atoms in the hydrophobic core
- **Electrostatic interactions:** Salt bridges between charged residues
- **Disulfide bonds:** Covalent, in oxidizing environments (ER, extracellular)

**Quaternary structure:** Assembly of multiple polypeptide chains (subunits). Examples: hemoglobin ($\alpha_2\beta_2$ tetramer), DNA polymerase holoenzyme (multi-subunit), ribosome (two major subunits).

## Protein Stability and Folding

**Thermodynamics of folding:** The native fold is the minimum free energy state. $\Delta G_{\text{fold}} = G_{\text{unfolded}} - G_{\text{native}} > 0$ means the native state is more stable — typical values are only $+20$ to $+60$ kJ/mol. Proteins are only marginally stable, poised to unfold under mild denaturing conditions.

The marginal stability of proteins is not a flaw — it is a feature. A protein that is too stable cannot change shape, and conformational change is essential for function. Allosteric enzymes must flex between T-state and R-state configurations; transcription factors must bind and release DNA; molecular motors must cycle through multiple conformations to do mechanical work. The price of this functional flexibility is that proteins are always only a few $k_BT$ away from unfolding.

**Chaperones:** Prevent misfolding and aggregation during synthesis and stress.
- **Hsp70 (DnaK in bacteria):** Binds exposed hydrophobic segments of unfolded proteins; releases upon ATP hydrolysis
- **GroEL/GroES (Hsp60/Hsp10):** "Anfinsen cage" — provides an isolated hydrophobic chamber for proteins to fold without aggregation
- **Hsp90:** Stabilizes metastable proteins, particularly kinases and steroid hormone receptors; a major hub in eukaryotic proteostasis

## Intrinsically Disordered Proteins

Many proteins or protein regions have no fixed structure in isolation — they are **intrinsically disordered proteins (IDPs)**. They fold upon binding to their partners (coupled folding-binding). IDPs are enriched in transcription factors, signaling proteins, and hub proteins in interaction networks. They often participate in phase separation to form membrane-less organelles (P-bodies, stress granules, the nucleolus).

The existence of IDPs was a conceptual shock to the structural biology community, trained to think that structure equals function. It turns out that disorder is often the function: IDPs can bind many different partners with moderate affinity (rather than one partner with high affinity), enabling the promiscuous binding that makes protein interaction hubs possible. Phase separation — the liquid-liquid demixing of IDP-rich regions into condensates — is now recognized as a major organizational principle in cell biology.

## Why This Matters for Computational Biology

Protein sequence determines structure, structure determines function, function determines biology. The entire enterprise of structural bioinformatics — homology modeling, structure prediction (AlphaFold), protein-protein interaction prediction, binding site identification, protein design — is grounded in understanding the relationship between amino acid sequence and 3D structure. In systems biology, knowing which residues mediate protein-protein interactions determines which edges in a network can be disrupted. In synthetic biology, protein engineering requires understanding stability (to identify positions that can tolerate mutations), binding (to engineer new specificities), and catalysis (to redirect enzymatic function). Every amino acid property enumerated here has a direct computational application.

```python
# Amino acid properties lookup table
amino_acids = {
    'Ala': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': 1.8,  'size': 'small'},
    'Arg': {'charge': +1, 'pKa_sc': 12.5, 'hydrophobicity': -4.5, 'size': 'large'},
    'Asn': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': -3.5, 'size': 'medium'},
    'Asp': {'charge': -1, 'pKa_sc': 3.9,  'hydrophobicity': -3.5, 'size': 'medium'},
    'Cys': {'charge': 0,  'pKa_sc': 8.3,  'hydrophobicity': 2.5,  'size': 'small'},
    'Glu': {'charge': -1, 'pKa_sc': 4.1,  'hydrophobicity': -3.5, 'size': 'medium'},
    'Gln': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': -3.5, 'size': 'medium'},
    'Gly': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': -0.4, 'size': 'tiny'},
    'His': {'charge': 0,  'pKa_sc': 6.0,  'hydrophobicity': -3.2, 'size': 'medium'},
    'Ile': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': 4.5,  'size': 'large'},
    'Leu': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': 3.8,  'size': 'large'},
    'Lys': {'charge': +1, 'pKa_sc': 10.5, 'hydrophobicity': -3.9, 'size': 'large'},
    'Met': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': 1.9,  'size': 'large'},
    'Phe': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': 2.8,  'size': 'large'},
    'Pro': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': -1.6, 'size': 'medium'},
    'Ser': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': -0.8, 'size': 'small'},
    'Thr': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': -0.7, 'size': 'medium'},
    'Trp': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': -0.9, 'size': 'large'},
    'Tyr': {'charge': 0,  'pKa_sc': 10.1, 'hydrophobicity': -1.3, 'size': 'large'},
    'Val': {'charge': 0,  'pKa_sc': None, 'hydrophobicity': 4.2,  'size': 'medium'},
}

# Simple protein hydrophobicity profile (Kyte-Doolittle)
def hydrophobicity_profile(sequence, window=9):
    """Compute sliding-window hydrophobicity profile."""
    kd = {aa: amino_acids[aa3]['hydrophobicity']
          for aa3 in amino_acids
          for aa in [aa3[:1]]}  # simplified
    # Use three-letter mapping
    aa1_to_kd = {'A':1.8,'R':-4.5,'N':-3.5,'D':-3.5,'C':2.5,'E':-3.5,'Q':-3.5,
                 'G':-0.4,'H':-3.2,'I':4.5,'L':3.8,'K':-3.9,'M':1.9,'F':2.8,
                 'P':-1.6,'S':-0.8,'T':-0.7,'W':-0.9,'Y':-1.3,'V':4.2}
    half = window // 2
    profile = []
    for i in range(half, len(sequence) - half):
        window_seq = sequence[i-half:i+half+1]
        avg = sum(aa1_to_kd.get(aa, 0) for aa in window_seq) / window
        profile.append(avg)
    return profile

# Example: bacteriorhodopsin transmembrane region fragment (highly hydrophobic)
tm_helix = 'TWLNLFSMLALVGFAFYVPFSNKTGVVD'
profile = hydrophobicity_profile(tm_helix)
import numpy as np
print(f"Mean hydrophobicity (TM helix): {np.mean(profile):.2f}")
print(f"Predicted transmembrane (>1.6): {sum(v > 1.6 for v in profile)} / {len(profile)} windows")
```
