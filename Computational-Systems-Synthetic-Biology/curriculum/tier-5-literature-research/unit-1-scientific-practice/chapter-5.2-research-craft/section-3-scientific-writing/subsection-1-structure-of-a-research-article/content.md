# Structure of a Research Article (IMRaD)

Open any top-tier biology journal — Nature, Cell, Molecular Systems Biology — and read the first paragraph of any paper. You will almost never find it begins with a definition. It begins with a story: a biological problem, a puzzle, a phenomenon that demands explanation. By paragraph three, you understand what question the paper addresses and why it matters. By the end of the introduction, you know what the authors found. This deliberate architecture — from broad context to specific question to results to interpretation — is the logic of scientific argument, crystallized over a century of publishing practice into what is now called IMRaD: Introduction, Methods, Results, and Discussion. This structure is not arbitrary — it reflects the logical organization of scientific argument and separates what was done (Methods), what was observed (Results), and what it means (Discussion). Each section has specific conventions that, when followed, make papers readable, reproducible, and interpretable. Understanding these conventions is essential both for writing papers and for reading them critically.

## The Introduction: From General to Specific

The introduction has one job: to make the reader understand why the work was done and why it matters. A well-written introduction moves from the general (what is the biological problem?) to the specific (what is the precise question this paper addresses?) in 3–4 paragraphs.

**Paragraph 1 — The broad context:**
What is the biological system, process, or disease that motivates this work? Why should a non-specialist care? This paragraph establishes relevance and provides the broadest context. It should be written for an intelligent reader in an adjacent field, not for a specialist in the sub-subfield.

*Example:* "The cell cycle — the sequence of events by which a cell replicates its genome and divides — is controlled by a network of regulatory proteins whose activities are coupled by phosphorylation cascades. Dysregulation of cell cycle control underlies most human cancers..."

**Paragraph 2 — The specific problem:**
What is known in the specific area addressed by this paper? What are the key prior results? This paragraph demonstrates that the authors understand the field and situates the work in its context. It should cite the most relevant prior work — not comprehensively, but representatively.

**Paragraph 3 — The gap:**
What is not known? What problem does the existing literature leave unsolved? This is the "gap" that the paper fills. It should be stated precisely — not "we don't know everything about X" (too vague) but "it is unknown whether mechanism Y occurs in context Z" (specific and falsifiable).

**Paragraph 4 — The approach and findings:**
The last paragraph of the introduction states what the authors did and what they found. The last sentence is conventionally: "Here we show/demonstrate/report that..." followed by the main finding. This sentence tells the reader exactly what the paper establishes.

**What the introduction is not:** The introduction is not a comprehensive review of the field. It should cite approximately 15–30 references and cover the essential context, not attempt to cite every paper ever written on the topic. The Introduction does not present data (that is the Results section) or interpret your results in the context of the literature (that is the Discussion).

## The Methods: Reproducibility as the Standard

The methods section has one job: to describe what was done in sufficient detail that an independent, competent researcher could reproduce the experiments. **Reproducibility is the standard, not brevity.** A methods section that omits key details is scientifically deficient regardless of how good the results are.

**What to include:**

**Biological materials:**
- Organisms: species, strain/line, sex, age, source, genetic background
- Cell lines: name, ATCC number, passage number, authentication status
- Plasmids: name, GenBank accession, or sequence in supplementary materials
- Antibodies: target, species, clone, catalog number, dilution, RRID (Research Resource Identifier)
- Primers: sequences, annealing temperature

**Reagents:**
- Chemicals: name, purity, supplier, catalog number
- Kits: name, supplier, catalog number, version

**Equipment:**
- Instruments: manufacturer, model number
- Software: name, version, parameter settings used

**Protocols:**
- Each major experiment type (Western blot, flow cytometry, ChIP-seq, etc.) deserves its own subsection
- If published protocols were used exactly, cite them and state "following the published protocol"; if modified, state the modifications
- Statistical methods: which test, software package, threshold, how multiple comparisons were handled

**Tense and voice:**
- Methods are written in the past tense (you are describing what was done)
- Active voice is preferred: "We measured..." not "Measurements were taken..."
- "We" is appropriate; the passive voice in methods sections is a dated convention that reduces readability

**Data and code availability:**
A statement pointing to where raw data are deposited (GEO, SRA, PRIDE, Zenodo) and where analysis code is available (GitHub with DOI via Zenodo) should appear as the final subsection. This is now required by most journals and funding agencies.

## The Results: Figures First, then Prose

The results section presents the experimental findings in logical order. Each subsection corresponds to one main result. **The figures drive the narrative** — every piece of data that is important enough to appear in the paper should be in a figure or table, and the results text should guide the reader through the figures, not repeat what the figures already show.

**Lead with the finding, not the experiment:**

Wrong: "To determine whether the repressilator oscillates in a growth-limited environment, we grew cells in a microfluidic device and imaged them over 48 hours."

Right: "The repressilator oscillated with a period of 180 ± 25 minutes (mean ± SD; n = 47 cells) under growth-limited conditions (Fig. 1A), compared to a period of 160 ± 30 minutes under standard growth conditions."

The first version makes the reader wait for the result. The second states the finding immediately and provides enough detail to interpret it, with a figure reference.

**Structure of a results paragraph:**
1. Topic sentence: state the finding
2. Describe the experimental approach in one or two sentences
3. Report the quantitative result with error and n
4. Reference the figure: "(Fig. 2A, B)"
5. If necessary: one sentence of interpretive context, linking to the next result

**What the results section is not:** The Results section does not interpret the findings in the context of the literature (that is the Discussion). Do not write "This result is consistent with the model proposed by Smith et al. (2015)" in the Results section — that belongs in the Discussion. The Results section should be a faithful narrative of what was observed, not an argument for your interpretation.

## The Discussion: Interpretation, Context, and Limitations

The Discussion section interprets the results in the context of the broader literature. It is the section where the authors argue for their interpretation, consider alternative explanations, situate the findings in the field, and acknowledge limitations.

**Structure of the Discussion:**

**Paragraph 1 — Summary of main findings:**
One paragraph stating what was shown. This is not the same as the Abstract — it is a prose statement of the key findings in the context of the paper's logic. Begin with "Here we have shown..." or "In this study, we demonstrated..." Keep it concise; the details are in the Results.

**Paragraphs 2–N — Interpretation and context:**
For each major finding, discuss: what does it mean mechanistically? Is it consistent with existing models, or does it require revising them? How does it agree or conflict with prior published results? Are there alternative interpretations? For each alternative, explain why the evidence favors your interpretation over the alternative.

**The limitations paragraph:**
Every Discussion should include an explicit statement of the study's limitations. What did you not test? What assumptions are embedded in your approach? What conditions might make the results not generalize? This is not a sign of weakness — it is a sign of scientific integrity and helps readers understand the appropriate scope of the conclusions.

*Example:* "Several limitations of this study should be noted. First, all experiments were conducted in E. coli MG1655; whether the observed regulatory dynamics generalize to other bacteria or to eukaryotic systems remains to be determined. Second, our model treats protein concentrations as deterministic, which may be an oversimplification for low-copy genes where stochastic effects are important..."

**Future directions:**
The Discussion typically ends with one paragraph on what questions remain open and what experiments or analyses would address them. This provides a roadmap for follow-up work.

**What the Discussion is not:** The Discussion does not re-present the results (that is the Results section's job). "We showed that X, we showed that Y, we showed that Z" without interpretation is not a Discussion. The Discussion is interpretation and argument, not repetition.

## The Abstract: The Paper in 200 Words

Most readers will encounter your paper only through its abstract. An abstract must stand alone — containing the essential context, findings, and implications without requiring access to the full paper.

**Structured abstract** (used in many clinical journals):
- **Background:** 1–2 sentences establishing context and the problem
- **Methods:** 2–3 sentences describing the experimental approach
- **Results:** 3–4 sentences summarizing the main findings with key numbers
- **Conclusions:** 1–2 sentences on implications and significance

**Unstructured abstract** (used in Nature, Science, Cell):
Same content, but in continuous prose. The challenge is to tell the complete story within 150–250 words without sounding like a table of contents.

**Common abstract errors:**
- Stating what the paper "shows" or "investigates" without stating what it actually found ("we investigated X" is not informative)
- Omitting quantitative results in favor of qualitative descriptions ("we observed a significant increase" without reporting the magnitude)
- Including background that is not needed to interpret the findings

## Takeaway

The IMRaD structure is not an arbitrary format — it reflects the logical organization of scientific reasoning. Introduction builds context and defines the question; Methods enables reproducibility; Results presents evidence; Discussion argues for interpretation. Each section has specific conventions about what to include and what to exclude. Writing according to these conventions makes papers more readable, more reproducible, and more credible. Reading with knowledge of these conventions makes you a more effective critical reader of the work of others.
