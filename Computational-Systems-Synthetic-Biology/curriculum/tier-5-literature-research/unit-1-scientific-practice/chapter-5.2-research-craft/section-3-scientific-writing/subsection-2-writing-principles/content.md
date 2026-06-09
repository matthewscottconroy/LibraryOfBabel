# Writing Principles for Scientific Communication

Here are two ways to report the same experiment. First: "We observed an increase in gene expression." Second: "GFP fluorescence increased 3.2-fold ± 0.4 (mean ± SD; n = 4 biological replicates; p = 0.003, Welch's two-tailed t-test) after induction with 1 mM IPTG for 4 hours." Both sentences describe the same result. One is useless; the other is science. The difference is not elegance or style — it is whether the sentence contains enough information for a reader to evaluate the claim. Scientific writing is not a stylistic exercise — it is a communication problem. The goal is to convey precisely what you did, what you found, and what it means to a reader who was not present in your lab. Every writing principle described in this section serves that communication goal. Ambiguity, imprecision, passive construction, and bloated sentences are not stylistic choices to tolerate; they are failures to communicate that undermine the scientific content of your paper.

## Precision over Ambiguity

The most important principle of scientific writing is **precision**: stating exactly what was measured, what was found, and under what conditions. Vague language hides the actual result and prevents readers from evaluating the evidence.

**Imprecise:** "We observed an increase in gene expression."

**Precise:** "GFP fluorescence increased 3.2-fold ± 0.4 (mean ± SD; n = 4 biological replicates; p = 0.003, Welch's two-tailed t-test) after induction with 1 mM IPTG for 4 hours."

The imprecise version tells the reader almost nothing: How large was the increase? How variable? How many experiments? Was it statistically tested? The precise version enables the reader to judge whether the result is convincing and whether the magnitude is biologically meaningful.

**Make measurement explicit:**
- "Cells were fluorescent" → "Mean GFP fluorescence was 850 ± 120 AU (mean ± SD) compared to 42 ± 8 AU in uninduced cells, measured by flow cytometry"
- "The model fit the data well" → "The model explained 94% of the variance in the experimental data (R² = 0.94)"
- "Growth was affected" → "Growth rate decreased from 0.85 ± 0.06 h⁻¹ to 0.31 ± 0.04 h⁻¹ (p < 0.001)"

## Active Voice and Past Tense

Scientific writing convention for the methods and results sections is **past tense** (you are reporting what was done and what was found) and **active voice** (the subject performs the action).

**Passive voice (avoid):** "Measurements were taken at 30-minute intervals."
**Active voice (preferred):** "We measured fluorescence at 30-minute intervals."

**Passive voice (avoid):** "The model was used to predict growth rates."
**Active voice (preferred):** "The FBA model predicted growth rates across 12 carbon sources."

Active voice is clearer (you know who did what), more concise (fewer words), and more engaging. The common belief that scientific writing requires passive voice is wrong — it is a convention from a previous era that most journals and style guides have abandoned.

**Tense conventions by section:**
- **Introduction:** Present tense for established facts ("Negative autoregulation reduces noise"); past tense for specific prior results ("Becskei & Serrano (2000) demonstrated that...")
- **Methods:** Past tense ("We transformed cells...", "Cells were grown...")
- **Results:** Past tense ("Expression increased 3-fold...", "The model predicted...")
- **Discussion:** Mixed — present tense for interpretation and established facts; past tense for your results

## One Idea Per Paragraph

Every paragraph should advance one argument or present one result. This principle is violated constantly in scientific writing, producing paragraphs that sprawl across multiple unrelated topics and leave the reader unable to track the argument.

**Paragraph structure:**
1. **Topic sentence:** State the main point of the paragraph. The reader should be able to understand the paper's argument by reading only the topic sentences.
2. **Body:** Evidence, data, reasoning, and context that support the topic sentence.
3. **Closing sentence:** Either confirms the main point or transitions to the next paragraph.

**Test your paragraphs:** After writing, identify the one claim that each paragraph is making. If you cannot state it in one sentence, the paragraph is doing too much.

## Define Every Acronym and Technical Term on First Use

Write out every acronym on first use: "flux balance analysis (FBA)." Every technical term that a reader in an adjacent field might not know should be defined in context on first use. Do not assume that because your concept is obvious to you, it is obvious to your reader.

**Common omissions in systems biology writing:**
- GEM (genome-scale metabolic model) — not universally known
- EMU (elementary metabolite unit) — known only to specialists in ¹³C MFA
- FFL (feedforward loop) — needs introduction even in molecular biology contexts
- pLDDT (predicted local distance difference test) — needs definition when discussing AlphaFold

## Reporting Statistics Completely

Every statistical comparison in a results section should be reported with:

1. The **test statistic** and type (F, t, χ², Z)
2. **Degrees of freedom**
3. The exact **p-value** (not "< 0.05" but "= 0.023")
4. The **effect size** (fold change, Cohen's d, eta-squared, odds ratio)
5. The **error type** (SD, SEM, 95% CI) and n (biological replicates)

**Example:** "The addition of 1 mM IPTG increased sfGFP fluorescence 4.1-fold (from 210 ± 35 AU to 860 ± 120 AU; mean ± SD; n = 5 biological replicates; t(8) = 11.2, p < 0.001, Welch's two-tailed t-test)."

**Error bars:** Always state what error bars represent. If the figure caption says "Error bars indicate SEM," the reader knows the bars show uncertainty in the mean estimate; if it says "Error bars indicate SD," the reader knows they show the spread of the data. Never show error bars without labeling them.

**Confidence intervals** are more informative than p-values alone. A 95% CI for a fold change of [3.2, 5.1] tells the reader both the estimate and its precision.

## Conciseness: Removing Deadwood

Scientific prose should be as long as necessary and no longer. Every sentence should contribute. Common sources of deadwood:

**Redundant phrases:**
- "In order to" → "To"
- "Due to the fact that" → "Because"
- "At the present time" → "Currently" or "Now"
- "It is interesting to note that" → Remove entirely; if it's interesting, the reader will find it interesting
- "It is well known that" → Remove and state the fact directly

**Hedging that adds nothing:**
- "It appears that" → State the finding directly if you have evidence; express uncertainty with specific error estimates
- "We believe that" → Use only when expressing genuine uncertainty; otherwise, state the claim directly

**Throat-clearing introductions:**
- "In this study, we examined..." → Delete; the paper is the study; just make the finding the topic sentence.

## Citing the Literature

**Cite primary sources, not reviews:** If you are stating that "Cas9 cleaves DNA at a specific position determined by the guide RNA," cite Jinek et al. (2012), not a 2021 review that describes this fact. Reviews are for pointing readers to literature they should read, not as the authoritative source for factual claims.

**Cite the most directly relevant paper:** If five papers all demonstrated a particular finding, cite the original (earliest) paper and/or the paper that established the point most definitively, not a random subset.

**Do not over-cite:** Introductions with 6–8 citations per sentence are often covering the author's lack of selectivity. Choose the 2–3 most important references for each claim.

## Revision as the Core of Writing

First drafts are not papers — they are the raw material from which papers are made. Professional scientists revise their papers 5–15 times between first draft and submission, often more between submission and publication. The willingness to revise, and the skill to know what needs revising, distinguishes scientists who publish cleanly from those who struggle at every submission.

**Revision protocol:**

1. **First draft:** Write without editing. Get the ideas on paper. Do not stop to polish sentences.
2. **Second pass (structure):** Does each section do its job? Does the Introduction end with a clear statement of the approach? Does each Results subsection state its finding in the first sentence?
3. **Third pass (paragraph level):** Is each paragraph focused on one idea? Are the topic sentences clear?
4. **Fourth pass (sentence level):** Is every sentence precise? Is there deadwood? Are acronyms defined?
5. **Fifth pass (statistics):** Is every comparison reported with complete statistical information?
6. **External review:** Give the paper to a colleague who is not a co-author. Ask them to identify every sentence they had to re-read. Revise those sentences.

**"Writing is thinking":** The process of writing a clear Methods section forces you to confront what you actually did. The process of writing a clear Discussion forces you to articulate what your results actually mean. Writing is not separate from science — it is part of the scientific process.

## Takeaway

Scientific writing principles — precision, active voice, one-idea paragraphs, complete statistical reporting, conciseness — all serve the same goal: accurate communication of what was done, found, and concluded. These principles are not stylistic preferences but communicative necessities. Learning to write well requires practice, feedback, and revision: the process of writing precisely is also the process of thinking precisely about your science.
