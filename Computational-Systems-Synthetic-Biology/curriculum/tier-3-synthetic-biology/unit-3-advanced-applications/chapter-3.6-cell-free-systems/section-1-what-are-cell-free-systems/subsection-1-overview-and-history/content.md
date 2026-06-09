# Cell-Free Systems: Overview and History

Imagine cracking open a bacterium, scooping out its contents, and setting them to work in a test tube — transcribing, translating, and metabolizing, all without a cell wall in sight. That is, in essence, what cell-free systems do. They harness the molecular machinery of life — ribosomes, RNA polymerases, chaperones, metabolic enzymes — outside the living cell. By lysing cells and collecting the soluble extract (or reconstituting the minimal set of required components from purified proteins), researchers can perform transcription, translation, and metabolism in a test tube. This access to biological machinery without the constraints of a living cell has driven both fundamental discoveries in molecular biology and practical applications from protein synthesis to diagnostic biosensors.

## The Fundamental Concept

A living cell is an extraordinarily complex system, but its core biochemical functions — transcription of DNA to RNA, translation of RNA to protein, and small-molecule metabolism — are performed by discrete, biochemically characterized molecular machines. If these machines can be isolated and provided with appropriate substrates (DNA templates, NTPs, amino acids, ATP), they will function outside the cell.

Cell-free systems capitalize on this separability:

**Inputs**: DNA template (containing gene of interest), amino acids, energy sources (ATP, GTP), salts, redox buffers

**Processing**: transcription by RNA polymerase, translation by ribosomes, post-translational modification by cellular enzymes (where present in the extract)

**Outputs**: protein product + depleted substrates + byproducts

The critical distinction from cellular expression: **no growth, no division, no membrane constraints**. The cell-free reaction is an open, controllable chemical system.

## Historical Development

### Foundational Era (1950s–1960s): Decoding Molecular Biology

Cell-free systems were essential tools in the molecular biology revolution:

**1954**: Zamecnik and Keller demonstrated in vitro incorporation of radiolabeled amino acids into protein using rat liver microsomes and soluble fractions. This was the first in vitro protein synthesis, proving that ribosomes were the site of translation.

**1961**: Nirenberg and Matthaei used cell-free *E. coli* extracts to decode the first codon: poly-U RNA directed synthesis of polyphenylalanine — proving UUU encodes phenylalanine. This cell-free decoding strategy subsequently cracked the entire genetic code. Cell-free systems were the instrument through which one of biology's deepest secrets was revealed.

**1965–1967**: Systematic cell-free decoding of all 64 codons by Nirenberg, Khorana, and colleagues completed the genetic code. Cell-free translation was the enabling technology.

### Application Era (1980s–1990s): Protein Production

As molecular cloning became routine, cell-free systems were applied to protein synthesis for research use:

**1980s**: commercialization of wheat germ extract and rabbit reticulocyte lysate as cell-free translation systems for quick production of small amounts of protein from mRNA templates. Used for: in vitro transcription/translation (IVTT), functional studies without cloning into expression vectors, protein interaction studies.

**1990**: Spirin et al. demonstrated continuous-exchange cell-free protein synthesis (CECF): continuously supply nutrients through a dialysis membrane while removing inhibitory byproducts. Protein yields increased from µg/mL to mg/mL.

### Reconstitution Era (2001): PURE System

**2001**: Shimizu et al. (Ueda lab, Tokyo) published the PURE (Protein synthesis Using Recombinant Elements) system: the complete translation apparatus reconstituted from 36 individually purified components (ribosomes, all translation factors, aminoacyl-tRNA synthetases, ATP regeneration system). This was a landmark achievement — the first completely defined, reconstituted protein synthesis system, eliminating all unknown components from the extract.

The PURE system demonstrated that translation could be reconstituted from scratch with only the components whose functions were known — providing a minimal parts list for protein synthesis and enabling experimental investigation of each component's individual role.

### Synthetic Biology Era (2010s–present): Design and Engineering

Cell-free systems evolved from tools for studying translation into platforms for synthetic biology:

**2012**: Pardee, Noireaux, Collins, and colleagues demonstrated lyophilized cell-free reactions as field-deployable biosensors. Paper-based diagnostics containing dried cell-free extract + toehold-switch DNA could detect Zika virus RNA in patient samples at room temperature.

**2014**: Sun et al. (Noireaux lab) demonstrated 384-well plate characterization of promoter variants in cell-free extracts — demonstrating cell-free as a high-throughput platform for genetic circuit prototyping.

**2016**: SHERLOCK (Specific High-sensitivity Enzymatic Reporter UnLOCKing) system for pathogen diagnostics: combines isothermal amplification with Cas13 collateral cleavage in a cell-free (or cell-free-like) format. Applied to diagnostics for Zika, Dengue, SARS-CoV-2.

**2016–present**: industrial-scale cell-free protein synthesis. Sutro Biopharma and others scale cell-free reactions to 100 L bioreactors for pharmaceutical protein production, including non-natural amino acid incorporation impossible in standard fermentation.

## Two Paradigms: Extract-Based vs. Reconstituted

The field has consolidated around two complementary paradigms:

**Extract-based systems** (TX-TL, S30, S12 extract): crude lysates containing all the machinery that was in the cell. Simple to prepare, high yield, tolerant of non-natural inputs. The unknown components (proteases, mRNases, metabolic enzymes not needed for translation) are both an advantage (natural efficiency) and a limitation (unpredictable interactions with new circuit designs).

**Reconstituted systems** (PURE system): all components individually purified and combined in defined proportions. Complete knowledge of what is in the reaction. Enables experiments impossible with extracts (remove or modify any component; add orthogonal components). Lower yield per cost; more expensive.

The choice between them depends on the application: rapid prototyping of new genetic circuits → extract-based; fundamental investigation of translation mechanism or non-natural amino acid incorporation → PURE.

## Why Cell-Free Systems Are a Distinct Paradigm

The key advantages of cell-free over cellular systems are not merely incremental:

**Speed**: cell-free reactions produce measurable protein in 1–2 hours. Genetic circuit testing takes 4–8 hours. Compare to cell-based: transformation → selection → culture → induction → measurement = 2–5 days minimum.

**Accessibility**: reactions can be designed, started, and interpreted on the same day. No overnight incubation waiting for cells to grow.

**Flexibility**: anything can be added to the reaction — non-natural substrates, non-natural amino acids, synthetic DNA templates, foreign proteins that would be toxic to cells. The open reaction format enables experiments that cells would not survive.

**Portability**: cell-free reactions can be lyophilized and reconstituted with water at the point of use — enabling diagnostic applications in resource-limited settings without refrigeration.

## Why This Matters

Cell-free systems have twice been at the center of paradigm shifts in biology: first when they enabled decoding of the genetic code in the 1960s, and again when synthetic biology adopted them as rapid prototyping platforms in the 2010s. They are not merely convenience tools — they are a distinct experimental paradigm that makes possible measurements and experiments that cellular systems cannot support. As synthetic biology increasingly uses cell-free systems for both circuit design (before committing to cell-based implementation) and as end products in themselves (diagnostics, biomanufacturing), understanding their principles, capabilities, and limitations is essential for modern practitioners of synthetic biology.
