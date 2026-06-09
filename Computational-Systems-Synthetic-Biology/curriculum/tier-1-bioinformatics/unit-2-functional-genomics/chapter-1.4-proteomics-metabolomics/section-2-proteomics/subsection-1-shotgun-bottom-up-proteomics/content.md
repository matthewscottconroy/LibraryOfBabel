# Shotgun (Bottom-Up) Proteomics

In 1995, a graduate student at the University of Washington named John Yates III was struggling with a question that seemed almost perverse: what if instead of purifying a protein and sequencing it directly, you simply digested your entire sample — all proteins at once — into peptides and sequenced the peptides? The protein identities would emerge from the peptide sequences by inference. This approach, which Yates eventually called "shotgun proteomics" by analogy with shotgun genome sequencing, seemed chaotic. How could you ever assign peptides back to their source proteins if you had digested everything together? It turns out that the mapping is largely unambiguous, and the throughput gains are so dramatic that the method now underlies virtually all global proteome analysis.

**Bottom-up proteomics**, also called **shotgun proteomics**, is the dominant approach for global protein identification and quantification. Rather than analyzing intact proteins, proteins are first digested into peptides, which are then separated by liquid chromatography and identified by tandem mass spectrometry. The name "bottom-up" refers to the strategy of inferring protein identity from peptide sequences — working upward from small fragments to the full protein.

## Sample Preparation

The quality of proteomics data is determined largely by sample preparation. Standard protocol:

**1. Protein extraction**: Cells or tissues are lysed in a buffer containing a denaturing agent (8 M urea, 4% SDS, or RIPA buffer) to solubilize proteins and inhibit proteases. The protein concentration is measured (BCA or Bradford assay).

**2. Reduction and alkylation of cysteines**: Disulfide bonds are reduced with DTT (dithiothreitol) or TCEP, breaking the bonds that crosslink protein domains. Free cysteines are then irreversibly alkylated with iodoacetamide (IAA), converting them to carbamidomethyl-cysteine (mass addition: +57.021 Da). This prevents re-oxidation and ensures all cysteines are detectable.

**3. Trypsin digestion**: The denatured, reduced, alkylated protein mixture is digested with **trypsin**, a serine protease that cleaves peptide bonds C-terminal to lysine (K) and arginine (R) residues, except when followed by proline. This produces peptides of 7–25 amino acids — the ideal size range for LC-MS/MS analysis. Digestion proceeds overnight at 37°C with a protein:enzyme ratio of 50:1 to 100:1.

**4. Clean-up**: Peptides are desalted using reversed-phase C18 tips (Stage Tips, SepPak cartridges) to remove salts, detergents, and urea that would interfere with LC separation.

The choice of trypsin as the workhorse protease is not arbitrary — it is a pragmatic decision driven by the physics of mass spectrometry. Trypsin cleaves after every K and R, and these basic residues attract protons in ESI, ensuring that every tryptic peptide carries a charge at the C-terminus. This predictably places most tryptic peptides in the m/z 400–1500 range with charge states of 2–3, right in the sweet spot of most mass analyzers. Alternative proteases (LysC, GluC, AspN, chymotrypsin) are sometimes used in parallel to increase sequence coverage, because each protease generates different peptides and thus reveals different regions of the proteome.

## LC-MS/MS Workflow

Peptide mixtures are resolved by **reversed-phase liquid chromatography** (RPLC) — peptides are retained on a C18 stationary phase and eluted with an increasing acetonitrile gradient. Typical LC columns: 25–50 cm long, 75 µm inner diameter (nano-LC), packed with 1.7–2 µm C18 particles; gradient: 2% to 35% acetonitrile over 60–120 minutes. The eluting peptides are immediately ionized by nano-ESI and introduced into the mass spectrometer.

The mass spectrometer runs in **data-dependent acquisition (DDA)** mode: a full MS1 survey scan identifies the most abundant precursor ions, and the top-N (typically N = 10–20) most abundant precursors are then selected for MS2 fragmentation in rapid succession. The cycle repeats continuously as peptides elute.

## Peptide-Spectrum Matching (PSM)

Each MS2 spectrum is assigned to a peptide by database searching (Sequest, Andromeda in MaxQuant, or MSFragger for high-speed searching). The result is a **peptide-spectrum match (PSM)**. After FDR filtering at 1% PSM level, thousands to tens of thousands of PSMs are obtained per run, corresponding to thousands of distinct peptide sequences.

## The Protein Inference Problem

A single peptide can originate from multiple proteins (because tryptic peptides can be shared between protein family members or splice variants). The **protein inference problem** asks: given a set of identified peptides, what is the minimal set of proteins that can explain all observed peptides? The **parsimony principle** (Ockham's razor for proteomics) reports only the minimal protein set: if all peptides from protein B are also found in protein A, protein B is not reported separately — only a protein group containing A (and B) is reported.

MaxQuant applies parsimony and groups proteins into **protein groups**, reporting a master protein (the canonical protein) for each group.

This is one of the deep philosophical tensions in proteomics: the experiment produces evidence at the peptide level, but you want conclusions at the protein level. The protein inference problem is not fully solved, and different software handles it differently. You should always examine the number of unique (non-shared) peptides supporting your proteins of interest — a protein reported on the basis of a single peptide shared with ten other family members is not the same kind of evidence as a protein supported by fifteen unique peptides.

## FDR at PSM and Protein Level

FDR is controlled separately at two levels:
- **PSM-level FDR**: 1% — ensures most individual spectrum-peptide assignments are correct.
- **Protein-level FDR**: 1% — applied to the assembled protein list; requires a minimum number of unique (non-shared) peptides per protein (typically ≥1 or ≥2 unique peptides).

## Pros and Cons vs. Top-Down Proteomics

| Feature | Bottom-Up (Shotgun) | Top-Down |
|---|---|---|
| Protein complexity | Low (peptide level) | High (intact protein) |
| Sequence coverage | Partial (~40–80%) | 100% possible |
| PTM localization | Possible for some PTMs | Comprehensive |
| Isoform resolution | Poor (shared peptides) | Direct |
| Throughput | Very high | Low–moderate |
| Instrument requirements | Standard LC-MS/MS | Ultra-high resolution required |

**Top-down proteomics** analyzes intact proteins without digestion, providing complete sequence coverage and enabling direct characterization of all PTMs and isoforms simultaneously. However, it requires specialized instrumentation (very high-resolution Orbitrap or FT-ICR) and is currently limited in throughput and protein size range (typically <70 kDa).

The comparison reveals a fundamental trade-off that runs through all of systems biology: throughput versus completeness. Top-down proteomics is the more principled approach — you are measuring the actual protein molecule, not a proxy — but its practical limitations mean that the field runs primarily on shotgun data. You will spend your career reading shotgun proteomics papers, and understanding what is irreducibly lost in the digestion step — isoform identity, the combinatorial complexity of multiple co-occurring PTMs, large proteins — is essential for interpreting what those datasets can and cannot tell you.

## Why This Matters

Shotgun proteomics is the workhorse of global protein expression analysis, enabling identification and quantification of thousands of proteins from complex biological samples; it is the foundation for biomarker discovery, understanding drug mechanisms, and characterizing the functional consequences of genetic variation at the protein level. A well-executed shotgun proteomics experiment on a 2022-era Orbitrap instrument can identify 8,000–10,000 proteins from a single cell line in a single day — something that would have required years of classical biochemistry just two decades ago. The throughput enables the systems-level questions: not "what does this one kinase do?" but "how does the entire proteome reorganize in response to this perturbation?"
