# PE3 and PE5: Improving Prime Editing Efficiency

The difference between a technology that works in a research lab and one that works as a medicine is often a factor of three to ten in efficiency. PE2 at 5–25% might be enough to demonstrate a proof of concept in HEK293T cells. But treating sickle cell disease means editing hematopoietic stem cells — and those cells are less forgiving, less proliferative, and less tolerant of repeated procedures. Getting from PE2-level efficiency to therapeutic-level efficiency required understanding exactly where prime editing was losing edits, and then engineering around each of those losses. The result was PE3, then PE5, and a set of rational improvements that together raised prime editing efficiency several-fold without fundamentally changing the technology. The logic is a model for how mechanistic understanding drives engineering.

The original PE2 system achieves 5–25% editing efficiency in most cell types — sufficient for research applications but insufficient for therapeutic use, where efficiencies of 30–80% or higher are desired. PE3 and PE5 address the primary mechanistic bottlenecks in prime editing by manipulating the cellular repair machinery that resolves the edited heteroduplex.

## PE3: Biasing Mismatch Repair with a Second Nick

### The Problem PE3 Solves

After PE2 installs the edited sequence on the non-target strand, the cell contains a heteroduplex: one strand with the edit, one with the original sequence. Cellular mismatch repair (MMR) resolves this mismatch, but with approximately equal probability of using either strand as template. This 50/50 resolution means approximately half of all prime editing events are reverted to the original sequence before permanent installation, limiting PE2 efficiency.

### The PE3 Solution

PE3 adds a **second sgRNA** (the "nicking sgRNA") that directs nCas9 to nick the **unedited target strand**, creating a nick ~40–90 bp from the prime editing nick. This second nick:
1. Marks the unedited strand as the "damaged" strand in the MMR context
2. Directs the cell's repair polymerase to use the intact (edited) non-target strand as template
3. Biases the resolution equilibrium: instead of 50/50, repair now favors using the edited strand as template

**Effect**: PE3 typically increases prime editing efficiency **2–4-fold** compared to PE2 for the same pegRNA.

### Nicking sgRNA Design Rules

The nicking sgRNA must satisfy several constraints:

**Position**: the nick should be placed 40–90 bp from the PE2 nick site, on the opposite strand (PAM facing away from the PE2 nick). Shorter spacings risk creating a DSB if both nicks are simultaneously present.

**Orientation**: for a PE2 nick on the non-target strand, the nicking sgRNA should nick the target strand. This means its PAM (NGG) must be on the non-target strand.

**Sequence**: the nicking sgRNA should have minimal off-target predictions (use CRISPOR with standard filtering). Unlike the pegRNA, the nicking sgRNA can be standard 20-nt guide RNA without extension.

**Avoid creating DSBs**: the goal is to create a nick on the target strand, not a DSB. Only one nick on each strand at a time. If the two nick sites are too close, the nicks can converge into a DSB (which is repaired by NHEJ with indel formation, losing the edit).

### PE3b: A Variant for Reduced Indels

PE3 increases indel frequency compared to PE2 (from ~2% to ~5–8%) because the two nicks can occasionally both be present simultaneously. **PE3b** addresses this by designing the nicking sgRNA to target a sequence that is present only in the **edited** allele (not the unedited allele).

Because the nicking sgRNA cannot bind until after the edit is installed (the target sequence doesn't exist in the original genome), the two nicks are temporally separated: first the PE2 nick installs the edit, then the nick sgRNA can bind and nick the now-edited allele. This sequential nicking dramatically reduces DSB formation and indel frequency, while maintaining the MMR-biasing effect.

**Condition for PE3b applicability**: the desired edit must alter the sequence recognized by the nicking sgRNA. This typically works when the edit is close to a PAM sequence that is created or destroyed by the edit.

## PE5: Inhibiting Mismatch Repair

### The Problem PE5 Addresses

Even with PE3's biasing of the nick-repair machinery, the MMR pathway can still revert edits by recognizing the heteroduplex and incorrectly using the unedited strand. This is particularly problematic for:
- Large insertions (>10 bp): MMR recognizes large loops with high efficiency
- Cell types with high MMR activity: rapidly dividing cells expressing high levels of MLH1

### MLH1 Dominant-Negative: Suppressing MMR

The MMR pathway requires the MutLα heterodimer (MLH1 + PMS2) for mismatch resolution. A **dominant-negative MLH1 (MLH1dn)** variant (E34A/H125D mutations) competes with wild-type MLH1 for PMS2 binding but cannot form a functional complex — effectively inhibiting MMR in cells that express it.

Chen et al. (2021) demonstrated that co-expressing MLH1dn with PE3 (= **PE5**) increases prime editing efficiency by:
- 3–7-fold for point mutations compared to PE2
- Even larger improvements for insertions (where MMR recognition is strongest)

### PE5 Efficiency Data

| Edit Type | PE2 | PE3 | PE5 |
|-----------|-----|-----|-----|
| Transition SNV (C→T) | 20% | 45% | 65% |
| Transversion SNV (C→A) | 8% | 20% | 42% |
| 3-bp insertion | 5% | 12% | 35% |
| 10-bp insertion | 2% | 5% | 20% |
| 20-bp deletion | 3% | 8% | 25% |

(Approximate values from HEK293T cells; values vary significantly by locus and pegRNA design)

### Safety Considerations for MLH1dn

Suppressing MMR raises potential concerns: MLH1 is a tumor suppressor (Lynch syndrome is caused by inherited MLH1 loss-of-function). Transient MLH1dn expression during the editing event followed by restoration of normal MMR is the goal. Strategies:
- Transient transfection (plasmid or mRNA): MLH1dn expressed for 24–72 hours, then cleared
- RNP delivery of PE2 protein + pegRNA + mRNA for MLH1dn: all components cleared rapidly
- Do not integrate MLH1dn stably

## PEmax: Codon-Optimized and Architecturally Optimized

Concurrent with PE5, **PEmax** optimizes the prime editor protein itself:
- Codon optimization for human expression
- Addition of bipartite NLS (nuclear localization signals) at both N and C termini
- Additional RT mutations for processivity

PEmax combined with epegRNAs (see next section) represents the current highest-efficiency prime editing system for most applications.

## epegRNAs: Engineered Structural Protection

A separate improvement addresses pegRNA degradation. Standard pegRNAs are prone to 3′ degradation by cellular exonucleases, reducing the effective concentration of functional pegRNAs. **epegRNAs (engineered pegRNAs)** incorporate structured RNA motifs (tevopreQ1 or mpknot) at the 3′ terminus, protecting the 3′ extension from degradation without interfering with PBS or RT template function.

epegRNAs increase prime editing efficiency by 3–4-fold in most contexts, purely by improving pegRNA stability. They should now be used as default — there is essentially no cost to incorporating the protective structure.

## Decision Tree: Which PE Version to Use

```
Design a prime edit:
  ↓
Start with PE3 + epegRNA
  ↓
If efficiency < 10%:
  → Check pegRNA design (PBS Tm, RTT length, spacer efficiency)
  → Try PE3b if edit allows it (reduces indels, may improve efficiency)
  → Move to PE5 + epegRNA (add MLH1dn transiently)
  ↓
If efficiency still < 5%:
  → Try alternative spacer (different PAM side)
  → Extend RTT length
  → Consider ABE/CBE if edit is a transition mutation (higher efficiency)
```

## Why This Matters

PE3 and PE5 are not simply incremental improvements — they reflect a systematic analysis of each mechanistic step in prime editing and targeted engineering to improve the limiting steps. The trajectory from PE1 (1–5% efficiency) to PE5 + epegRNAs (20–75% efficiency) in less than four years mirrors the improvement arc of base editors from BE3 to ABE8e. The lesson is that mechanistic understanding enables rational, step-by-step efficiency improvements, and that no first-generation genome editing system represents the ceiling of what the technology can achieve.
