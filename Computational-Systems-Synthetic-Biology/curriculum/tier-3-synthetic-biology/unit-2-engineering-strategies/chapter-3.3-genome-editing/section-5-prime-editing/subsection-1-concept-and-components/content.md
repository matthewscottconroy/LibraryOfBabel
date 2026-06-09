# Prime Editing: Concept and Components

By 2019, the genome editing toolkit had real gaps. Base editors were powerful for transition mutations, but they couldn't install transversions. They couldn't make small insertions. They couldn't make deletions. For those edits, you still needed HDR — with its cell-cycle restriction, its need for a separately delivered repair template, and its notoriously low efficiency in the primary cells where therapeutic editing matters most. Then Andrew Anzalone, in David Liu's lab, described an approach that seemed almost too elegant to be true: instead of relying on the cell's own repair machinery, what if you could directly write new sequence into the genome using a reverse transcriptase, with the template for the new sequence encoded in the guide RNA itself? The 2019 *Nature* paper on prime editing described exactly that — and it changed the upper boundary of what precise genome editing could achieve.

Prime editing, introduced by Anzalone et al. in *Nature* in 2019, represents the most versatile precision genome editing tool developed to date. Unlike base editors, which are limited to transition mutations, prime editing can introduce all 12 types of point mutations, small insertions, and small deletions at any genomic position — all without double-strand breaks and without a separate HDR repair template.

## The Core Concept: Writing New Sequence

The conceptual innovation of prime editing is using **reverse transcriptase** to write new sequence information directly into the genome. Instead of cutting both strands and relying on homologous recombination, prime editing:

1. Nicks one strand of the target DNA
2. Uses the nicked strand as a primer
3. Extends a new sequence from an RNA template embedded in the guide RNA itself
4. Incorporates the new sequence into the genome

This approach — writing from RNA to DNA — is borrowed from retroviruses and retrotransposons, but directed to a specific genomic location by Cas9-mediated target recognition.

## Why Prime Editing Is Needed

The genome editing landscape before prime editing:

| Tool | Substitutions | Insertions | Deletions | DSB-free | HDR-template-free |
|------|-------------|-----------|----------|---------|-------------------|
| NHEJ | No (stochastic) | Stochastic | Stochastic | No | Yes |
| HDR | Yes | Yes | Yes | No | No |
| CBE | C→T only | No | No | Yes | Yes |
| ABE | A→G only | No | No | Yes | Yes |
| Prime editing | All 12 types | Yes (≤44 bp) | Yes (≤80 bp) | Yes | Yes |

Prime editing fills the gap between base editors (precise but limited) and HDR (versatile but inefficient and DSB-dependent). Approximately 89% of known pathogenic point mutations could in principle be corrected by prime editing.

## PE2 Components

The PE2 system consists of two components: the **PE2 fusion protein** and the **pegRNA**.

### The PE2 Fusion Protein

PE2 = **nCas9 (H840A nickase)** + flexible linker + **M-MLV reverse transcriptase (RT)**

**nCas9 (H840A)**: unlike the D10A nickase used in base editors (which cuts the target strand), PE2 uses the H840A nickase, which cuts the **PAM-containing strand** (non-target strand). This generates a free 3′ OH on the non-target strand, which serves as the primer for reverse transcription.

**M-MLV reverse transcriptase**: the RT domain from Moloney Murine Leukemia Virus, engineered with five mutations (D200N/L603W/T306K/W313F/T330P) to increase processivity, reduce template-switching, and reduce RNase H activity. These mutations allow the RT to copy the pegRNA template faithfully without degrading it.

The total protein is approximately 1,368 aa (nCas9) + 671 aa (RT) = ~2,039 aa, or ~7.5 kb of coding sequence.

### The pegRNA (Prime Editing Guide RNA)

The pegRNA is the most novel component of prime editing. It is an extended sgRNA containing three functional domains:

```
5′-[sgRNA scaffold]-[20-nt spacer]---[RT template]---[PBS]-3′
                                   ←──────────────────────→
                                        3′ extension
```

**Spacer (20 nt)**: identical in function to a standard sgRNA spacer; directs nCas9 to the target genomic site.

**sgRNA scaffold**: the tracrRNA sequence that binds nCas9. Same as in standard sgRNAs.

**RT template (RTT)**: 10–30 nt of RNA encoding the desired edit plus flanking sequence. This is the sequence that will be reverse-transcribed into the new DNA flap. It must be carefully designed (section 5.2 discusses this in detail).

**PBS (Primer Binding Site)**: 8–15 nt complementary to the 3′ end of the nicked non-target strand. After nicking, the 3′ flap of the non-target strand must hybridize to the PBS to initiate reverse transcription.

## pegRNA Design Logic

The PBS is complementary to the 3′ terminus of the nicked non-target strand. Because the nick occurs at the H840A site (3 bp upstream of the PAM on the non-target strand), the 3′ terminus of the nicked strand is the sequence immediately 5′ of the nick.

**PBS length rule**: PBS should be 8–15 nt, with a Tm of ~30–40°C (tested empirically). Too short: poor primer binding, low efficiency. Too long: reduced RT processivity.

**RT template length rule**: the RT template must encode the full edit plus at least 6–10 nt of flanking sequence (for the flap to recombine correctly with the unedited strand). Longer templates (>30 nt) reduce efficiency; insertions are limited to ~44 bp maximum.

## Why This Design Works

The elegance of prime editing is that all the information for the desired edit is encoded in the pegRNA — no separate DNA template needs to be delivered. The pegRNA serves simultaneously as:
- A guide RNA for Cas9-mediated target recognition
- A primer for the 3′ flap
- The template for reverse transcription of the new sequence

This dramatically simplifies delivery compared to HDR (which requires both Cas9/sgRNA and a donor template).

## The Prime Editing Editor Protein Series

- **PE1**: nCas9(H840A) + wild-type M-MLV RT; low efficiency (~0.5–5%)
- **PE2**: nCas9(H840A) + engineered M-MLV RT (5 mutations); ~2–25% efficiency
- **PE3**: PE2 + additional "nicking sgRNA" targeting the non-edited strand; ~3–50% efficiency (section 5.3)
- **PE5**: PE3 + MLH1dn (dominant-negative MLH1, inhibits mismatch repair); highest efficiency for some edit types

## Why This Matters

The conceptual framework of prime editing — using programmable reverse transcription to write new sequence at a specific genomic location — is a genuine advance in the logic of genome editing. It decouples editing precision from the cell's repair machinery by using a template-copying mechanism that is Cas9-guided but otherwise cell-autonomous. The practical impact is that researchers can now design any small sequence change and deliver it with a single RNA molecule plus a single protein, without DSBs, without donor template, and without cell cycle restriction. The main remaining limitations (efficiency, pegRNA size, insertion size) are active areas of engineering, and several second-generation prime editing variants (PEmax, epegRNA) have already substantially improved on the original PE2 performance.
