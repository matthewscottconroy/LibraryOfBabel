# Gene Synthesis Screening

Until the early 2000s, the Select Agent Program's logic was largely self-contained: if you controlled physical access to dangerous pathogens, you controlled the ability to work with them. That logic depended on one assumption: that you needed the actual pathogen to study it. In 2002, Eckard Wimmer's laboratory at Stony Brook University disproved that assumption. Working from the published poliovirus genome sequence and commercially available reagents, they synthesized the poliovirus genome from scratch. The resulting RNA was infectious. No poliovirus sample had ever been obtained; no biosafety regulations had been violated. The security model for biological agents had assumed that sequence information and physical materials were two different things. Wimmer's experiment proved they were the same thing.

The ability to synthesize arbitrary DNA sequences from chemical precursors — without requiring a biological template — is one of synthetic biology's most powerful capabilities and one of its most significant biosecurity challenges. Gene synthesis screening is the set of technical and policy measures designed to prevent commercial DNA synthesis from being used to produce dangerous pathogen sequences or toxin genes that could be used for bioweapons development.

## Why Gene Synthesis Is a Biosecurity Concern

Before the advent of affordable gene synthesis, obtaining a dangerous pathogen's genetic material required physical access to the pathogen itself — which is controlled through the Select Agent Program and biosafety regulations. Gene synthesis fundamentally changes this: if a pathogen's genome sequence is publicly available (as almost all pathogen genomes are, deposited in GenBank), anyone with access to a gene synthesis company can, in principle, order that sequence as synthetic DNA without ever handling the natural pathogen.

This was demonstrated dramatically in 2002 when Wimmer et al. chemically synthesized the poliovirus genome from scratch using only the published sequence and commercially available reagents. The resulting synthetic RNA was infectious. In 2005, Tumpey et al. reconstructed the 1918 influenza pandemic strain from sequences recovered from archived lung tissue. These experiments were conducted in registered BSL-3 facilities by experienced researchers for legitimate scientific purposes — but they demonstrated the capability.

The concern is not that the original researchers had malicious intent, but that the **capability** to reconstruct dangerous pathogens from sequence data became established, and the technical barrier to doing so continues to decrease.

## Industry Screening Standards

The gene synthesis industry developed voluntary screening standards through the **International Gene Synthesis Consortium (IGSC)**, which publishes a harmonized protocol that its member companies (including Twist Bioscience, Integrated DNA Technologies, Genscript, Evonik/DNA2.0) have committed to implementing.

**IGSC Harmonized Protocol screening pipeline**:

1. **Sequence comparison**: every synthesis order is run against a curated database of **sequences of concern** — primarily Select Agent genomes, but also toxin gene sequences, virulence factors from high-consequence pathogens, and other sequences identified through biosecurity review.

2. **Threshold matching**: matches are flagged if the ordered sequence has ≥80% identity to a sequence of concern over a sliding window (typically 200 bp). The 80% threshold is designed to catch functional sequences while avoiding false positives from evolutionarily distant homologs.

3. **Customer verification**: orders matching flagged sequences require customer identification and, in some cases, verification of institutional affiliation and IBC/Select Agent registration. Anonymous or unverifiable customers cannot receive flagged sequences.

4. **Order refusal**: sequences that unambiguously encode dangerous biological function (e.g., complete botulinum toxin gene, complete anthrax lethal factor) may be refused entirely rather than requiring additional verification.

**Limitations of industry screening**:
- Coverage is voluntary — non-IGSC companies are not bound by this protocol
- International companies (particularly in countries without equivalent frameworks) may not screen orders
- **Evasion by codon reshuffling**: a sequence encoding the same protein as a pathogen virulence factor but with a different nucleotide sequence may escape sequence-based matching while producing the same functional protein. Screening based on protein translation of the ordered sequence (rather than nucleotide comparison alone) partially addresses this but is more complex.
- **Fragmented orders**: ordering the pathogen genome in fragments, each below the threshold match length, could potentially evade detection. IGSC protocols include guidance on detecting suspicious fragmentation patterns.

## U.S. Government Screening Requirements

In October 2023, the Biden Administration issued an **Executive Order on the Safe, Secure, and Trustworthy Development and Use of Artificial Intelligence**, which included provisions specifically addressing biosecurity:

- Directed OSTP and federal agencies to develop minimum standards for screening nucleic acid synthesis orders
- Required federal agencies that fund life sciences research to establish conditions requiring recipients to ensure their research only uses DNA synthesis providers that screen orders

In 2024, the NIH established a policy requiring research institutions receiving NIH funding to only use gene synthesis providers that conduct customer and sequence screening consistent with the IGSC Harmonized Protocol, effective for new grants.

This effectively extended de facto mandatory screening to federally funded research — the first government requirement for screening (as opposed to voluntary industry practice).

## Protein-Based Screening

An important gap in nucleotide sequence screening is that it does not capture:

1. **Evolutionarily diverged sequences**: a functional toxin from a non-Select-Agent source organism with <80% nucleotide identity to known sequences may not be flagged
2. **De novo designed proteins**: computationally designed proteins with toxin-like or virulence-like activity have no reference sequence to match
3. **Codon-shuffled variants**: as noted above, same protein function with different codons

**Protein function prediction screening** is an active research area: using structure prediction (AlphaFold2) and function annotation (BLAST-based, HMM-based, or ML-based) to predict whether an ordered sequence encodes a potentially dangerous protein. This is technically feasible but not yet deployed at scale by synthesis companies.

## Practitioner Responsibilities

For synthetic biologists ordering DNA:

**Before ordering**:
1. Ensure the sequence you are ordering does not encode Select Agent materials without proper institutional registration
2. If ordering sequences derived from pathogen genomes, be prepared to provide institutional affiliation and IBC documentation
3. Be aware that delays may occur if your order triggers screening review

**If your order is flagged**:
- This is not an accusation — it is a standard process when sequence matching occurs
- Provide requested verification documents promptly
- If you believe the flagging is incorrect (false positive), contact the synthesis company's biosecurity team with explanation

**For computational designers**:
- When designing new protein sequences (e.g., for directed evolution of novel enzymes), consider whether the designed protein could have toxic or virulence-like properties
- Functional novelty designs should be reviewed against toxin function databases (ProtTox, ToxinPred) before synthesis ordering

## Gene Synthesis Screening and Open Science

Gene synthesis screening creates a tension with open science principles. Researchers publishing novel gene sequences (e.g., codon-optimized synthetic versions of viral proteins, newly characterized natural toxins) contribute to public databases that then become inputs to screening systems. This is generally appropriate: the screening database expands to cover new risks as they are characterized.

But it also means that legitimate published research can trigger screening delays or verification requirements when others try to reproduce or extend that work. This is an acceptable cost of biosecurity — the alternative (no screening) creates a more serious risk.

## Why This Matters

Gene synthesis screening is the front-line biosecurity measure at the point where dangerous biological information (sequence data) is converted into physical material (DNA). Its effectiveness is the biosecurity community's primary response to the reality that pathogen genome sequences are publicly available. For researchers, understanding how screening works — what triggers a flag, why your order might be delayed, and what documentation is needed — is practical knowledge that will be increasingly relevant as synthesis becomes a more routine part of research workflows. For the broader field, the ongoing development of better screening methods (protein-level, function-level, AI-assisted) is an active research area at the intersection of bioinformatics and biosecurity — one where computational synthetic biologists are particularly well-positioned to contribute.
