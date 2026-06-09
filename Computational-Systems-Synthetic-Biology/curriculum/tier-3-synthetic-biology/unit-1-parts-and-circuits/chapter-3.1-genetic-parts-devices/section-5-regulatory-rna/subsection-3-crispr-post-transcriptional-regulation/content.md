# CRISPR-Based Post-Transcriptional Regulation: CRISPRi, CRISPRa, and Cas13

Most people encounter CRISPR as a genome-editing tool — a molecular scissors that cuts DNA at a programmed location. But the most powerful applications in synthetic biology may be the ones that deliberately do not cut. By disabling the nuclease activity of Cas9 while keeping its programmable DNA-binding intact, researchers created a tool that can silence any gene in the genome with a 20-nucleotide guide sequence, no DNA damage, and no permanent alteration. By fusing that same dead Cas9 to activator domains, they built a tool that can turn on any gene on demand. By switching to the RNA-targeting Cas13 family, they opened up a new layer of programmable regulation entirely at the post-transcriptional level. Together, these tools — CRISPRi, CRISPRa, and Cas13 — represent a fundamental expansion of what it means to program a cell.

The CRISPR toolkit extends well beyond genome editing. By using catalytically modified Cas proteins, it is possible to regulate gene expression without permanently altering DNA—either silencing genes (CRISPRi) or activating them (CRISPRa) at the transcriptional level, or degrading specific RNAs at the post-transcriptional level (Cas13).

## CRISPRi: Transcriptional Repression with Catalytically Dead Cas9

**dCas9** (dead Cas9) is a Cas9 variant bearing two catalytic mutations—D10A in the RuvC domain and H840A in the HNH domain—that abolish nuclease activity while preserving DNA binding and sgRNA-dependent targeting. When directed to a gene promoter or coding sequence by an sgRNA, dCas9 acts as a programmable roadblock to transcription.

### Mechanism in Bacteria

In bacteria, CRISPRi repression occurs through two distinct mechanisms:
1. **Promoter targeting**: dCas9 bound at or near the −10/−35 elements physically prevents RNAP from binding or initiating transcription.
2. **Coding strand targeting**: dCas9 bound within the transcribed region physically blocks elongating RNAP. As described in section 4.2, this triggers Rho-dependent termination, causing transcript cleavage and degradation.

**Repression levels in *E. coli***:
- Promoter targeting: 10–100-fold repression
- Coding strand targeting (early coding region): 100–1000-fold repression
- Spacer position matters critically: even 20 bp differences in targeting position can change repression 10-fold

**Design rules**:
- Target non-template (coding) strand of the promoter for maximum blockage of RNAP binding
- Avoid targeting within 200 bp of the TSS on the template strand (less effective)
- In coding region: target the non-template strand within the first 100–200 nt for maximum Rho coupling
- Guide RNA GC content: 40–60% for efficient binding
- Avoid extensive secondary structure in the guide-matching region

### Mechanism in Mammalian Cells

In eukaryotes, dCas9 alone provides modest repression (3–5-fold). Adding a repressor domain dramatically increases efficacy:

- **dCas9-KRAB**: Krüppel-associated box domain recruits histone deacetylases and methyltransferases → chromatin condensation → 100–1000-fold repression in mammalian cells; can spread to nearby genes (bystander silencing)
- **dCas9-SID4X**: transcriptional silencer domain from the mSin3 complex
- **dCas9-LSD1**: histone demethylase; removes H3K4me3 active mark; particularly effective for silencing enhancers

The CRISPRi repression can be **heritable** through cell division if chromatin marks are established, giving epigenetic memory of the silencing event. This can be exploited for differentiation engineering but is a complication in other applications.

## CRISPRa: Transcriptional Activation

CRISPRa (activation) uses dCas9 fused to transcriptional activator domains, targeted to sequences upstream of a gene's promoter (typically −50 to −400 bp from TSS):

**First-generation**: dCas9-VP64 (four tandem copies of the HSV VP16 activation domain). Modest activation (2–10-fold) in most contexts.

**SAM (synergistic activation mediator)**: dCas9-VP64 + sgRNA containing MS2 hairpin loops that recruit MS2-p65-HSF1 fusion proteins. The sgRNA itself recruits additional activators. Achieves 10–100-fold activation.

**VPR (VP64-p65-Rta)**: three activation domains in tandem fused directly to dCas9. 10–1000-fold activation depending on target.

**Target site rules for activation**:
- Must target the non-template strand within 400 bp upstream of TSS; the activator needs to contact transcription machinery
- Multiple guide RNAs targeting different sites around a promoter act synergistically
- Chromatin accessibility is critical: targeting nucleosome-occluded sites is ineffective

## Orthogonal dCas9 Proteins for Multiplexed Control

Multiple dCas9 orthologs can target different genes simultaneously:
- **Sp-dCas9** (from *S. pyogenes*): NGG PAM
- **Sa-dCas9** (from *S. aureus*): NNGRRT PAM; smaller (1053 aa vs. 1368 aa)
- **As-dCas12a** (from *Acidaminococcus*): TTTN PAM; targets opposite strand; single crRNA without tracrRNA
- **Lb-Cas13a**: RNA-targeting (see below)

Each ortholog recognizes only its own sgRNA scaffold. By combining Sp-dCas9 for repression with Sa-dCas9 for activation (or vice versa), two independent layers of gene regulation can coexist in the same cell with independent guide RNAs.

## Cas13: RNA-Level Repression and Diagnostics

**Cas13** (HEPN nuclease family) binds and cleaves RNA rather than DNA:
- Requires only a **crRNA** (no tracrRNA), targeting a 22–30 nt protospacer on the RNA
- No PAM requirement on the RNA target
- Upon activation by target binding, Cas13 enters a **collateral cleavage mode**: it nonspecifically degrades any nearby single-stranded RNA (the "bystander cleavage" effect)

**Applications**:
1. **Programmable RNA knockdown**: target any mRNA for degradation without genome editing; reversible (the RNA and Cas13 can be removed)
2. **SHERLOCK (Specific High-sensitivity Enzymatic Reporter UnLOCKing)**: exploit collateral cleavage for diagnostics—target RNA triggers Cas13, which then cleaves a reporter RNA linked to fluorescence or colorimetric output
3. **Transcriptome editing**: base-editing versions of Cas13 (REPAIR, RESCUE) can deaminate A→I or C→U in target RNAs, correcting point mutations at the RNA level without altering DNA

**Worked example—SHERLOCK for COVID-19**:
1. Patient sample RNA is amplified by SHERPA (SHERLOCK+RPA, isothermal amplification)
2. Amplified product activates LwaCas13a loaded with a crRNA targeting SARS-CoV-2 nucleocapsid RNA
3. Cas13 collateral cleavage cleaves a fluorescent RNA reporter
4. Fluorescence output detectable with a simple LED reader or by eye (with lateral flow strip)
5. Detection limit: ~1 aM (single-molecule level) with 30 min amplification

## Why This Matters

CRISPRi/a represents a fundamental shift from permanent genetic modification to programmable, reversible gene regulation. For metabolic engineering, CRISPRi genome-wide screens can identify every gene whose repression improves product yield—providing a systems-level view impossible with traditional genetic approaches. For circuit design, the ability to repress or activate any gene by simply designing a 20-nt guide sequence means that new circuit topologies can be prototyped in days rather than weeks. Cas13-based diagnostics are transforming point-of-care disease detection by enabling RNA-specific sensing at attomolar sensitivity without cold chain requirements.
