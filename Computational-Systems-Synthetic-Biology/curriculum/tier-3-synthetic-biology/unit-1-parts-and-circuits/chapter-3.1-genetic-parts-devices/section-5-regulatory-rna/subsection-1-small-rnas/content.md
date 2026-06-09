# Small RNAs: Post-Transcriptional Regulation in Bacteria

For decades after the discovery of the double helix, RNA was thought of primarily as an intermediary — a messenger ferrying genetic information from DNA to the ribosome. That view began to crack when researchers noticed bacterial cells producing abundant, short RNA molecules that were not coding for anything. By the 1980s it was clear that some of these molecules were regulating gene expression in ways that had nothing to do with protein synthesis. By the 2000s, the field had exploded: hundreds of these **small regulatory RNAs** had been catalogued in *E. coli* alone, and their mechanisms were being worked out in molecular detail. For synthetic biology, the appeal is immediate: an RNA regulator requires no protein, responds in minutes rather than hours, and can in principle be programmed simply by changing its sequence. The toolkit of RNA-based control is now mature enough to be used routinely alongside traditional transcription-factor-based regulation.

**Small RNAs (sRNAs)** are non-coding RNA molecules, typically 50–250 nucleotides in length, that regulate gene expression post-transcriptionally in bacteria by base-pairing with target mRNAs. They provide a fast, reversible, and multiplexable layer of gene regulation that is distinct from—and complementary to—transcription factor-based control.

## Classes of Bacterial sRNAs

### Hfq-Dependent sRNAs (trans-encoded)
The largest class of regulatory sRNAs in *E. coli* (>80 known) are **trans-encoded**: they are transcribed from genomic locations separate from their target mRNAs and have limited (imperfect) complementarity to their targets. Key features:

- **Hfq chaperone**: an RNA-binding protein that stabilizes sRNA:mRNA duplex formation by facilitating their interaction. Hfq binds sRNAs at their 3' end (typically at a Rho-independent terminator stem-loop) and the mRNA near the target site, bringing them into proximity.
- **Target recognition**: the sRNA's seed region (typically 6–12 nt) base-pairs with a sequence near the 5' UTR of the target mRNA, often overlapping the RBS or start codon.
- **Mechanism**: the sRNA:mRNA duplex can (a) sterically block ribosome access to the RBS, inhibiting translation; (b) recruit RNase E to the mRNA, promoting degradation; or (c) in some cases, unfold inhibitory mRNA structure and activate translation.
- **Stoichiometry**: sRNA and mRNA are often co-degraded, making the relationship stoichiometric rather than catalytic (unlike protein-based regulators).

### Cis-encoded sRNAs (antisense)
These are encoded on the opposite strand from their target gene, giving perfect complementarity. They typically regulate plasmid copy number, phage gene expression, or transposon movement. The RNA I/RNA II regulatory system for ColE1 plasmid copy number is the prototypical example.

## Design Principles for Synthetic sRNAs

The modular structure of Hfq-dependent sRNAs makes them tractable for engineering:

```
5'---[scaffold region]---[seed region]---3'[terminator]
         (Hfq binding)  (target binding)
```

To create a synthetic sRNA targeting a new mRNA:
1. Choose the target region: best results when targeting within −30 to +10 relative to the AUG (RBS and start codon region)
2. Design the seed region: 12–20 nt complementary to the target, with GC content 40–60%
3. Attach to a validated scaffold: MicC, MicF, or other characterized Hfq-binding scaffolds
4. Verify: no predicted off-targets with > 8 nt complementarity in the transcriptome

**Synthetic sRNA repression levels**: well-designed synthetic sRNAs achieve 2–10-fold repression of target mRNA. For stronger repression, multiple sRNAs targeting different positions can be combined.

## The MicC and DsrA Scaffolds

Two commonly used scaffolds for synthetic sRNA design:

**MicC scaffold** (Sharma et al.): a natural *E. coli* sRNA that represses *ompC*. The target-binding region can be replaced with a designed seed sequence while retaining Hfq-binding capability. Enables tunable repression when fused to designed target sequences.

**DsrA scaffold**: naturally activates mRNA translation by unfolding inhibitory secondary structure. Used for gain-of-function sRNA designs.

## Advantages Over Transcription Factor-Based Regulation

| Feature | sRNA regulation | TF-based regulation |
|---|---|---|
| Speed | 5–15 min response | 30–60 min (requires new protein) |
| Size | 50–250 nt RNA | 200–500 aa protein |
| Multiplexibility | Same Hfq pool; many sRNAs | Requires unique TF:operator pairs |
| Resource cost | Low (no translation needed) | High (protein synthesis required) |
| Reversibility | Fast (sRNA:mRNA codegradation) | Slower (protein dilution/degradation) |

## Worked Example: sRNA Repression of a Competing Metabolic Enzyme

In a 3-hydroxypropionic acid (3-HP) production strain, the malonyl-CoA consuming fatty acid synthesis pathway competes with the 3-HP biosynthetic route. Direct knockout of *fabB* (β-ketoacyl-ACP synthase) eliminates fatty acid synthesis and kills the cell.

**sRNA approach**: design a synthetic sRNA targeting the *fabB* RBS sequence. The sRNA provides:
- Partial (~4-fold) repression rather than complete elimination
- Rapid (15 min) response if expressed from an inducible promoter
- Easy tunability: use a library of sRNA variants with different seed region lengths to achieve the optimal partial repression

Result: cells with optimal sRNA-mediated *fabB* partial repression showed 2.3-fold improvement in 3-HP titer compared to wild-type, while maintaining sufficient fatty acid synthesis for membrane integrity.

## RNA-Based Logic with sRNAs

sRNAs enable RNA-level Boolean computation:
- **NOT gate**: sRNA suppresses target mRNA → output OFF when sRNA is ON
- **AND gate**: require two sRNAs simultaneously absent (double de-repression)
- **Multi-input integration**: multiple sRNAs targeting the same mRNA at different sites integrate additively

The computational power of sRNAs in the context of cell-free systems has been particularly explored, where Toehold switches (section 5.4) extend RNA logic to programmable two-state devices.

## Why This Matters

sRNAs bridge the gap between transcriptional control (slow but stable) and post-translational control (fast but requiring complex protein machinery). In metabolic engineering, they offer a way to fine-tune enzyme expression levels dynamically—turning down a competing pathway branch without knockout, or up-regulating a bottleneck enzyme in response to a sensed metabolite. Their small size (no protein required), rapid response, and design-by-sequence principles make them attractive tools for the next generation of adaptive cellular control systems.
