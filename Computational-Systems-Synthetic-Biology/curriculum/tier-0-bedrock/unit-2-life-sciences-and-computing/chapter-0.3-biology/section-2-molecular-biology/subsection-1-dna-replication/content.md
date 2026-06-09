# DNA Replication

Consider what must happen every time a cell divides: 3 billion base pairs of DNA — a sequence that if printed would fill thousands of books — must be copied in its entirety, with an error rate of roughly one mistake per billion nucleotides, in under eight hours. This is not a process that engineers could readily design from scratch. It is the product of three billion years of selection for both speed and accuracy, and the result is a molecular machine of staggering precision. The quantitative parameters of replication — fidelity, processivity, and rate — are essential background for understanding mutation rates, evolutionary modeling, and the design of DNA-based synthetic systems.

DNA replication is the process by which a cell duplicates its genome before division. Because every cell division propagates the complete genetic information of the organism, the machinery of replication must achieve extraordinary accuracy while operating at high speed.

## Semiconservative Replication and Origin Architecture

Replication is **semiconservative**: each daughter duplex contains one parental strand and one newly synthesized strand. This was demonstrated definitively by the Meselson-Stahl experiment (1958) using density-gradient centrifugation of $^{15}$N/$^{14}$N-labeled DNA.

Replication initiates at defined **origins of replication (ori)**. In *E. coli*, there is a single origin (*oriC*, ~250 bp) recognized by the initiator protein **DnaA**. The human genome, at ~3 Gb, uses ~50,000 origins; replication from a single origin at the *E. coli* rate of ~1000 nt/s would take over 27 hours to replicate 3 Gb. Multiple origins firing concurrently bring S-phase duration to ~8 hours.

## The Replication Fork: Core Machinery

At each origin, replication proceeds bidirectionally, creating two **replication forks**. The ensemble of proteins at each fork is the **replisome**:

| Component | Function | Key Numbers |
|---|---|---|
| **Helicase** (DnaB in *E. coli*) | Unwinds dsDNA using ATP hydrolysis | ~500 bp/s, hexameric ring |
| **Primase** (DnaG) | Synthesizes short RNA primers | 10–12 nt primers |
| **DNA Pol III core** | Processive 5'→3' synthesis, 3'→5' proofreading | ~1000 nt/s, $10^7$ nt processivity |
| **Sliding clamp** (β-clamp) | Tethers polymerase to DNA | Dramatically increases processivity |
| **Clamp loader** (γ complex) | Loads β-clamp using ATP | |
| **Topoisomerase I/II** | Relieves positive supercoiling ahead of fork | |
| **SSB** (single-strand binding protein) | Stabilizes unwound ssDNA | |
| **DNA Pol I** | Removes RNA primers, fills gaps | 10–20 nt/s |
| **DNA Ligase** | Seals nicks between Okazaki fragments | |

## Leading and Lagging Strand Synthesis

Because DNA polymerases can only extend from a 3'-OH terminus, and the two strands of the duplex are antiparallel, one strand is synthesized continuously while the other requires discontinuous synthesis:

- **Leading strand**: synthesized continuously 5'→3' in the same direction the fork advances. A single primer is needed at the start.
- **Lagging strand**: synthesized in short segments (**Okazaki fragments**) running antiparallel to fork movement. Each fragment (1000–2000 nt in bacteria; 100–200 nt in eukaryotes) requires a new primer. After extension, RNA primers are removed, gaps filled by DNA Pol I (bacteria) or Pol δ (eukaryotes), and nicks sealed by ligase.

The two polymerases at a fork are held together by the **clamp loader/clamp** architecture, forming a **trombone model** in which the lagging-strand template loops out to allow both polymerases to move in the same physical direction. This elegant solution to the antiparallel problem is a wonderful example of how molecular machines can solve geometric constraints that would otherwise seem intractable.

## Fidelity: Three Lines of Defense

The raw error rate of DNA Pol III is approximately $10^{-5}$ to $10^{-6}$ per base incorporated. After all proofreading and repair, the final replication error rate in *E. coli* is approximately $10^{-9}$ per base per replication — an astonishing 1000-fold improvement from the polymerase's intrinsic selectivity.

**Layer 1 — Base selectivity:** Watson-Crick geometry discriminates correct from incorrect base pairs at the nucleotide insertion step. A mismatched base pair distorts the active site, slowing catalysis.

**Layer 2 — 3'→5' proofreading:** Pol III has an intrinsic 3'→5' exonuclease. When a mismatch is incorporated, the terminus is a poor substrate for further extension. The polymerase switches to exonuclease mode, excises the mispaired nucleotide (~2–3 nt), and resumes synthesis. This reduces the error rate from $\sim 10^{-5}$ to $\sim 10^{-7}$.

**Layer 3 — Mismatch repair (MMR):** Post-replication, the MutS/MutL/MutH system (bacteria) scans newly synthesized DNA and corrects any remaining mismatches. This achieves the final $\sim 10^{-9}$ rate. (Defects in human MMR genes — *MSH2*, *MLH1* — cause Lynch syndrome, a hereditary colorectal cancer predisposition.)

The overall error rate can be summarized:

$$\epsilon_{\text{final}} = \epsilon_{\text{pol}} \times f_{\text{proof}} \times f_{\text{MMR}} \approx 10^{-5} \times 10^{-2} \times 10^{-2} = 10^{-9}$$

Each layer contributes roughly two orders of magnitude of improvement. The key insight is that no single mechanism achieves the required fidelity — it requires layered defenses, each operating on the residual errors left by the previous one.

## The End-Replication Problem and Telomeres

Linear eukaryotic chromosomes face the **end-replication problem**: after removal of the terminal RNA primer on the lagging strand, the 5' end cannot be filled in (no upstream 3'-OH to extend from). Each cell division would shorten chromosomes by ~50–200 bp.

**Telomeres** — repetitive TTAGGG sequences (mammals) extending 5–15 kb — provide a buffer. **Telomerase**, a reverse transcriptase carrying its own RNA template, extends the 3' overhang. Telomerase is active in germline cells and most cancers (~85%), and absent (or very low) in most somatic cells. Somatic cells accumulate telomere shortening, which contributes to cellular senescence.

## Worked Example: Calculating Replication Time

*E. coli* genome: $4.6 \times 10^6$ bp. Single bidirectional origin. Fork rate: 1000 nt/s per strand. Two forks advance simultaneously.

$$t = \frac{L/2}{v} = \frac{2.3 \times 10^6 \text{ nt}}{1000 \text{ nt/s}} = 2300 \text{ s} \approx 38 \text{ min}$$

*E. coli* growing with 20-min doubling time solves this by overlapping replication cycles: a new round of replication begins before the previous one finishes, such that cells contain multiple partially-replicated chromosomes simultaneously.

## Why This Matters for Computational Biology

Replication fidelity numbers directly parameterize evolutionary models. The per-base per-replication mutation rate $\mu \approx 10^{-9}$ to $10^{-10}$ in bacteria and $\sim 10^{-8}$ in humans determines the rate at which sequence space is explored. Sequencing error rates ($\sim 10^{-3}$ to $10^{-5}$ for short-read platforms) must be understood relative to true biological mutation rates when interpreting variant calls — the error rate of your sequencer may dwarf the biological mutation rate, so variant calling always involves distinguishing real mutations from technical artifacts. In synthetic biology, the high processivity of Pol III (processivity $\sim 10^7$ nt without dissociating) inspires the design of DNA synthesis enzymes, and understanding Okazaki fragment length informs models of lagging-strand synthesis in in vitro replication systems.
