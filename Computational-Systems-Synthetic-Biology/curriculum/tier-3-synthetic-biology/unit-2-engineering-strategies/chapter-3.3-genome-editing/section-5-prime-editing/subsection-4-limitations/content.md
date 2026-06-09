# Prime Editing Limitations

Every tool has a range of applicability, and the more you understand about a technology, the more precisely you can identify where it fits and where it doesn't. Prime editing is genuinely remarkable in its versatility — the ability to write any small sequence change without a double-strand break is a real advance. But it is not a universal replacement for CRISPR knockouts, base editors, or HDR. The limitations described here are not failures of the technology; they are the honest boundaries of a complex molecular machine, and knowing them is what allows you to choose the right tool for your specific biological question. They are also, to a first approximation, a research agenda: each limitation corresponds to an active engineering effort to overcome it.

Prime editing is the most versatile precision genome editing tool available, but it has real limitations that constrain when and where it can be applied. Understanding these limitations is essential for choosing the right editing strategy and for knowing when to use HDR, base editing, or other approaches instead.

## Limitation 1: Lower Efficiency Than Base Editors for Simple Transitions

For the subset of edits that both base editors and prime editors can make (C→T and A→G transitions), **base editors consistently outperform prime editors**:

| Edit | Base Editor | Prime Editor (PE5) |
|------|------------|-------------------|
| C→T transition | 40–80% | 25–65% |
| A→G transition | 50–85% | 20–50% |

When the target edit is a simple transition mutation and the base to be edited falls within the editing window, choose a base editor over prime editing. The higher efficiency and simpler design (no RT template needed) make base editors preferable in these cases.

## Limitation 2: RT Template Length Constrains Insertion Size

The reverse transcriptase domain of PE2 has limited processivity — it can copy approximately 30–44 nt of RT template reliably. Longer RT templates can be used, but efficiency falls steeply:

$$\text{Efficiency} \propto \frac{1}{\text{RT template length}} \text{ (approximately)}$$

Practical consequences:
- **Point mutations and small edits (≤10 bp)**: prime editing works well
- **Insertions of 10–44 bp**: possible but efficiency drops to 5–15% typically
- **Insertions > 44 bp**: largely inaccessible by current prime editing systems

Large insertions (entire genes, promoters, reporters) require HDR or recombination-based strategies. The insert size limit is a fundamental constraint of the reverse transcription mechanism, not something that can be easily overcome by protein optimization alone.

## Limitation 3: pegRNA Is Larger and More Complex to Design Than sgRNA

A standard sgRNA is a ~100-nt RNA with a fixed scaffold and a 20-nt spacer. A pegRNA is 130–200 nt and requires careful design of three interdependent elements — spacer, PBS, and RT template — whose sequences interact and must satisfy multiple constraints simultaneously:

**PBS design constraints**:
- 8–15 nt length
- Tm of 28–40°C
- Must not form secondary structure with the RT template or scaffold

**RT template design constraints**:
- Must encode the exact desired edit
- Must end with sequence that anneals to the unedited genomic strand (flap resolution)
- Optimal length: 10–25 nt for point mutations; up to 40 nt for insertions
- Must avoid internal hairpin formation

**Combined constraints**: the full 3′ extension (PBS + RTT) must not fold back on itself or on the sgRNA scaffold.

**Computational tools**: PRIME-BE, PegFinder, PE Designer (from David Liu lab) automate pegRNA design. Even so, multiple pegRNA designs should be tested because computational prediction of RT template efficiency is imperfect. A typical experiment tests 2–4 pegRNA designs, increasing the effort relative to sgRNA design.

## Limitation 4: Reduced Efficiency in Non-Dividing and Primary Cells

Prime editing efficiency in primary cells (neurons, HSCs, T cells, hepatocytes) is generally 3–10-fold lower than in HEK293T (a rapidly dividing transformed line):

- **HEK293T**: 20–60% with PE5
- **Primary T cells**: 5–20%
- **Primary neurons**: 1–5%
- **Post-mitotic cardiomyocytes**: < 1% in most reports

The efficiency gap in non-dividing cells reflects both reduced cellular repair activity and delivery challenges. The MMR-biasing approach (PE3/PE5) is less effective when the cell divides slowly, because nick repair may occur before cell division copies the edited strand.

## Limitation 5: Pegfilters and Delivery Challenges

PE2 is a ~2039 aa (7.5 kb coding sequence) protein. This creates significant delivery challenges:

**AAV delivery**: single-AAV packaging limit ~4.7 kb. PE2 CDS alone (7.5 kb) far exceeds this. Dual-AAV split-intein approaches are required — adding complexity and reducing efficiency.

**mRNA delivery**: PE2 mRNA is ~7.5 kb. mRNA of this length has lower translation efficiency and greater immunogenicity than shorter mRNAs. LNP delivery of PE2 mRNA + pegRNA has been achieved but requires optimized formulations.

**RNP delivery**: PE2 protein is large (~232 kDa). Electroporation of PE2 RNP is achievable but requires higher protein concentrations than Cas9 RNP.

## Limitation 6: Indel Byproducts

PE3 and PE5 increase efficiency but also increase indel byproducts (unintended insertions/deletions at the edit site), typically from 2% (PE2) to 5–10% (PE3/PE5). The mechanism: two nicks in close proximity occasionally converge to a DSB, which is repaired by NHEJ with random indels.

Indel frequency matters in therapeutic contexts: if 5–10% of corrected alleles also carry indels, the therapeutic product is a mixture of corrected and disrupted alleles. For dominantly-acting genes, this may be tolerable; for recessive diseases requiring precise correction, it may not be.

**Mitigation**: PE3b (edit-specific nick sgRNA) reduces indels by separating the two nicking events temporally; optimize nick sgRNA placement.

## When to Use Each Technology

```
Target edit type:
  ├── C→T or G→A transition → Cytosine base editor (CBE)
  ├── A→G or T→C transition → Adenine base editor (ABE)
  ├── Any transversion, any insertion ≤44 bp, any deletion ≤80 bp → Prime editing
  ├── Insertion > 44 bp or complex rearrangement → HDR or recombination
  └── Loss of function (any indel acceptable) → NHEJ (standard SpCas9)

Cell type consideration:
  ├── Rapidly dividing cells → PE3 or PE5 routinely achievable
  └── Primary/non-dividing → PE2 + epegRNA (PE3 nicking sgRNA may not help much)

Delivery constraint:
  ├── In vivo AAV → Consider SaCas9-based base editor (smaller) or dual-AAV PE
  └── Ex vivo primary cells → RNP electroporation (PE2 protein + pegRNA)
```

## Why This Matters

A technology's limitations define its appropriate scope. Prime editing does not replace HDR for large insertions, nor does it replace base editors for simple transitions requiring high efficiency. The knowledge that prime editing efficiency is lower in post-mitotic cells explains why therapeutic prime editing efforts initially focus on liver (accessible by LNP) and hematopoietic cells (accessible by ex vivo RNP electroporation) before expanding to harder targets. Understanding these limitations also drives the next generation of improvements: researchers are actively engineering more processive reverse transcriptases to extend the insertion size limit, developing better delivery vehicles for large proteins, and exploring circular pegRNA designs to improve stability. Each limitation is a defined engineering target.
