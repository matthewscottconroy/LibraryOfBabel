# Prime Editing: Mechanism

You now have the components: a nickase fused to a reverse transcriptase, and an extended guide RNA with a primer-binding site and a reverse transcription template. But knowing the parts doesn't yet tell you how they fit together into a functional molecular machine — or where that machine can fail. The mechanism of prime editing is a seven-step process, and at each step there is a specific way the system can revert to the unedited state. Understanding those failure modes is not just intellectually satisfying; it is the direct explanation for why PE3 improves on PE2, why PE5 improves on PE3, and how you should approach troubleshooting when efficiency is lower than expected.

Prime editing achieves precise genome modifications through a defined sequence of molecular events. Understanding each step explains both why the system works and where each potential failure mode lies — knowledge that directly informs experimental troubleshooting and pegRNA design.

## The Seven-Step Mechanism

### Step 1: Target Binding

The PE2 protein (nCas9-RT fusion) loaded with the pegRNA binds to the genomic target site by the same PAM-recognition and R-loop formation mechanism as standard Cas9. The sgRNA spacer base-pairs with the target strand; the PAM (NGG) is recognized on the non-target strand.

The R-loop forms in the 5′-to-3′ direction on the target strand, displacing the non-target strand as a single-stranded flap. The 3′ extension of the pegRNA (containing the RT template and PBS) remains single-stranded and points away from the DNA.

### Step 2: Nicking the PAM-Containing Strand

Unlike base editors (which use nCas9 D10A that cuts the target/spacer-complementary strand), PE2 uses **nCas9 H840A**, which cuts the **non-target strand** (the PAM-containing strand).

The H840A mutation inactivates the HNH domain, leaving RuvC active. RuvC cuts the non-target strand 3 bp upstream of the PAM, generating:
- A nicked non-target strand with a free **3′ hydroxyl** (3′ flap)
- An intact target strand

### Step 3: 3′ Flap Hybridization to the PBS

The 3′ end of the nicked non-target strand must hybridize to the **PBS (Primer Binding Site)** at the 3′ end of the pegRNA. This hybridization is driven by the complementarity between the PBS and the nicked strand's 3′ terminus.

The free energy of PBS hybridization must be sufficient to compete with the re-annealing of the nicked strand to the target strand (which drives the system back to the unedited state). This is why PBS length (Tm 30–40°C) is critical: too weak and the 3′ flap rarely engages the pegRNA; too strong and the geometry may be unfavorable.

### Step 4: Reverse Transcription Using the RT Template

With the 3′ flap hybridized to the PBS, the **M-MLV reverse transcriptase** domain (tethered to the Cas9 N-terminus) synthesizes a new DNA strand using the pegRNA's RT template as the template:

$$\text{PBS:3′ flap (DNA primer)} \xrightarrow{\text{RT}} \text{new DNA flap containing desired edit}$$

Reverse transcription proceeds 3′→5′ on the RNA template, creating a new DNA 3′ flap that contains:
1. The sequence complementary to the PBS (redundant with genomic sequence at this end)
2. The desired edit, exactly as encoded in the RT template
3. Additional sequence matching the surrounding genomic sequence (the "homology arm" for flap resolution)

The RT terminates at the 5′ end of the RT template (when it reaches the sgRNA scaffold junction). This sets the length of the newly synthesized DNA flap.

### Step 5: 3′ Flap Resolution — Displaced vs. New Flap

After reverse transcription, there are now **two competing 3′ flaps**:
- The **unedited 3′ flap**: the original nicked non-target strand end (unextended, containing the original sequence)
- The **edited 3′ flap**: the newly reverse-transcribed DNA containing the desired edit

Cellular **5′ flap endonucleases** (primarily FEN1) remove 5′ flaps. The unedited flap is structurally a 5′ flap relative to the newly synthesized strand. FEN1 removes it, leaving only the edited 3′ flap.

Alternatively, the equilibrium between flap states resolves stochastically: if the edited flap anneals to the target strand (which it can, since it carries sequence complementary to the target strand), it becomes a substrate for ligation.

### Step 6: Ligation

DNA ligase seals the nick between the edited 3′ flap and the 5′ end of the non-target strand, creating a **heteroduplex** with:
- One strand: the original target strand (still containing the old sequence)
- Other strand: the newly synthesized non-target strand (containing the desired edit)

This heteroduplex is a **mismatch** at the site of the edit.

### Step 7: Mismatch Resolution

The heteroduplex is resolved by one of two outcomes:
- **Desired outcome**: mismatch repair (MMR) uses the edited non-target strand as template to fix the complementary target strand → both strands contain the edit → permanent edit
- **Undesired outcome**: MMR uses the original target strand as template → reverts the non-target strand to the original sequence → no edit

This stochastic resolution is a major efficiency-limiting step. The probability of desired resolution is approximately 50% if MMR treats both strands equally — explaining why PE2 efficiencies are often 10–30% (editing is occurring in more cells than the final edited percentage suggests, but many revert in this step).

## The Role of the Nicking sgRNA (PE3)

PE3 adds a second sgRNA that directs nCas9 to nick the **target (unedited) strand** after the edit has been installed on the non-target strand. This nick forces the cell's repair machinery to use the intact non-target strand (now containing the edit) as the template for nick repair — converting the mismatch repair equilibrium from 50/50 to strongly favor the edit.

The nicking sgRNA must target a position:
- 40–90 bp from the prime editing nick site
- On the opposite strand (PAM on the non-target strand for the nick sgRNA)
- Should not create an off-target site that competes with the prime editing nick

PE3 typically increases efficiency 2–4-fold over PE2, at the cost of slightly increased indel frequency (because two nicks in the same region can occasionally be converted to a DSB by cellular nucleases).

## Quantitative Efficiency Expectations

For a well-designed pegRNA with optimal PBS length and RT template length in HEK293T cells:

| System | Efficiency Range | Indel Frequency |
|--------|----------------|----------------|
| PE2 | 5–25% | 1–3% |
| PE3 | 15–50% | 4–8% |
| PE5 (PE3 + MLH1dn) | 25–75% | 5–10% |
| PEmax (optimized PE3) | 30–60% | 4–7% |

Efficiency varies significantly by: edit type (transitions > transversions > insertions > deletions in most contexts), cell type (dividing cells > non-dividing), and pegRNA design quality.

## Why This Matters

The mechanism of prime editing contains multiple steps, each with its own efficiency and fidelity considerations. Each step where the system can revert to the unedited state — failed PBS hybridization, FEN1 not removing the unedited flap, MMR choosing the wrong strand — reduces overall efficiency. This mechanistic understanding directly motivates the PE3 and PE5 improvements: each addresses a specific bottleneck (MMR resolution and mismatch repair suppression, respectively). Future prime editing improvements will likely target other steps, such as increasing RT processivity for longer insertions or engineering the flap equilibrium to favor the edited outcome. For practitioners, understanding the mechanism enables rational troubleshooting: low efficiency with PE2 → add nicking sgRNA (PE3); PE3 still low → add MLH1dn (PE5); indels too high → optimize nick sgRNA placement.
