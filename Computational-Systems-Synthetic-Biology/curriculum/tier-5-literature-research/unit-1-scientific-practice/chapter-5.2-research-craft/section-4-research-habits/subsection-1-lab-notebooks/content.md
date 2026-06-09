# Lab Notebooks: Documentation as a Research Practice

Scientists have been writing in lab notebooks since at least the seventeenth century. Newton kept one; Faraday filled dozens; Darwin's notebooks are among the most studied documents in the history of science. When Darwin famously wrote "I think" above a sketch of a branching tree, he was doing what every scientist does: recording a developing idea at the moment it existed, in a form that could be returned to, refined, or refuted. That notebook survived. The idea it contained changed biology forever. The lab notebook is the primary record of experimental science. It is where data is recorded before it is analyzed, where interpretations are noted before they are confirmed or refuted, where protocols are developed and refined, and where the intellectual history of a research project lives. A complete, accurate, and well-organized lab notebook makes science reproducible — by you, six months from now; by a successor in your lab; by a collaborator in a different institution; and by anyone attempting to replicate your findings.

The quality of documentation in a lab separates laboratories that produce reproducible, publishable science from those that struggle to reconstruct what was done when the paper is being written. Documentation is not a bureaucratic obligation — it is a scientific practice.

## What Belongs in Every Experiment Entry

Every experiment entry should be complete enough that another person — a competent researcher unfamiliar with your specific project — could understand what was done, why, and what was found. A complete entry includes:

**Date and experiment number:**
A unique identifier (e.g., MJS-045 for experiment 45 in the notebook of M.J. Smith) allows cross-referencing between entries, with data files, and with presentations or papers.

**Rationale and hypothesis:**
One or two sentences explaining why this experiment was done. What question is it answering? What prediction is being tested? "Repeating experiment from last week" is not a rationale. "Testing whether the RBS Calculator accurately predicts translation efficiency for the sfGFP reporter construct in BL21(DE3) at 37°C" is.

**Materials:**
- Exact names and sources of biological materials (strain names, plasmid names, lot numbers where relevant)
- Exact reagent sources, catalog numbers, concentrations
- Protocol reference (if using a standard protocol, cite it by name and version; if modified, describe the modifications)

**Protocol:**
A step-by-step description of what was done, in enough detail that you could reproduce it. Note the actual times and temperatures used, not the nominal protocol values.

**Raw data location:**
Where is the data file? (Exact file path or database accession.) For digital instruments, note the instrument name, serial number, and software version. Raw data should never be stored only in the notebook — it belongs in a backed-up digital repository.

**Observations:**
Real-time notes during the experiment — what did you see? What was unexpected? "The induction plate had a contamination issue in the lower right corner, which I excluded." "The gel staining looked lighter than usual — may need to optimize staining time."

**Interpretation:**
Your initial assessment of the result. Was the hypothesis supported? Was the experiment successful? What do you conclude, and what questions remain? This is written at the time of the experiment, not weeks later when writing a paper. The initial interpretation is often wrong, and that is fine — the notebook should capture the evolution of your understanding.

## Electronic vs. Paper Notebooks

### Paper Notebooks

**Advantages:**
- Simple; always available; no software compatibility issues; legally unambiguous timestamp (ink on paper, signed and dated)
- For IP-sensitive research: traditional paper notebooks with signed witness pages have been the legal standard in patent disputes (though electronic notebooks with verified timestamps are increasingly accepted)

**Disadvantages:**
- Not searchable; difficult to share; easy to lose; cannot link directly to data files; photographs of gels must be printed or pasted in

**Best practices for paper notebooks:**
- Never delete or white-out entries; cross out errors with a single line and date the correction
- Use permanent ink (ballpoint, not pencil)
- Number pages; keep a table of contents
- Tape or paste printouts, photos, or gel images directly into the notebook with the date
- Back up the notebook with regular scanning (phone camera apps like Office Lens or CamScanner)

### Electronic Notebooks (ELNs)

Electronic lab notebooks are increasingly standard, particularly in computational and multi-site research. The major platforms are:

**Benchling** (benchling.com): The leading ELN for biology labs. Integrates with molecular biology tools (sequence editor, plasmid maps, registry of constructs), supports structured data entry, and provides version history. The free academic tier is functional for individual researchers; institutional licenses add team collaboration, audit trails, and API access.

**Notion** (notion.so): Flexible general-purpose tool that can be configured as a lab notebook using templates. Not biology-specific; better for theoretical/computational work where structured biological data entry is not needed.

**OneNote** (Microsoft): Part of Office 365; familiar to many researchers; good for freeform notes and mixed media (images, text, equations). Not biology-specific.

**Jupyter Notebooks** (jupyter.org): The natural lab notebook for computational work. Code, outputs, visualizations, and prose in a single document. Version-controlled in Git; reproducible (re-running the notebook regenerates all outputs from raw data). For computational biology projects, Jupyter notebooks should be the primary record of all analysis.

**Advantages of electronic notebooks:**
- Searchable; linkable to data files; shareable; version history
- Can embed figures, scripts, raw data, and external links
- Automated timestamps

**Disadvantages:**
- Data ownership concerns with cloud providers (read the terms of service carefully)
- Software compatibility may change over time
- Requires consistent internet access for cloud-based systems

**Best practice:** Export your electronic notebook data regularly (monthly) in an open format (PDF, HTML, Markdown) to a local backup. Do not allow your complete experimental record to exist only in a proprietary cloud system.

## Why Documentation Fails and How to Fix It

Several failure modes afflict lab notebook discipline. Understanding them is the first step to overcoming them.

**Failure mode 1: Retroactive writing**
The most common failure: writing up experiments days or weeks after they were done, from memory. Retroactive documentation is unreliable — details are lost, observations are unconsciously shaped by knowledge of the results, and the intellectual evolution of the project is erased. **Fix: Write the rationale and materials section before the experiment; fill in observations and results during and immediately after.**

**Failure mode 2: "I'll fill in the details later"**
Leaving blank spaces to be filled in with reagent lot numbers, exact conditions, or file paths that "I'll look up later." These blanks are never filled in. **Fix: Record every detail at the moment you use it. Have the notebook open during the experiment.**

**Failure mode 3: Undocumented decisions**
Experimental decisions made in the moment (changed protocol step, substituted reagent, excluded a sample) that are not recorded. When the experiment is analyzed later, there is no record of why those decisions were made. **Fix: Treat every deviation from the standard protocol as a note-worthy event. If you deviate, write why.**

**Failure mode 4: Results without interpretation**
Recording data files without recording what they show. "Flow cytometry data saved as: /data/GFP_reporter_2026-05-05.fcs" without noting what was observed in the data. **Fix: After looking at the data, write one paragraph in the notebook describing what you see and what you conclude.**

**Failure mode 5: Good notebook during experiments, nothing during analysis**
Documenting wet lab work meticulously but not documenting computational analysis. Analysis scripts should be treated as protocols, with version numbers and rationale recorded. **Fix: Treat every analysis script as a lab notebook entry. Use Jupyter notebooks for computational work; record analysis decisions in comments.**

## Lab Notebook Conventions for Computational Work

For computational biology, the lab notebook takes a different form:

- **Analysis scripts** (Python, R, MATLAB) should be version-controlled in Git with descriptive commit messages that explain what changed and why
- **Jupyter notebooks** document the analysis alongside the code: rationale, parameter choices, interpretation of outputs, conclusions
- **README files** in each project directory document the project structure, data sources, and how to run the analysis

```
project/
  README.md         # project overview, data location, how to run
  data/
    raw/            # never modified; downloaded or measured data
    processed/      # output of processing scripts; reproducible from raw
  analysis/
    notebooks/      # Jupyter notebooks; one per analysis question
    scripts/        # standalone Python/R scripts
  results/
    figures/        # figure output files
    tables/         # table output files
  NOTEBOOK.md       # lab notebook in Markdown: dates, rationale, decisions
```

## Institutional and Legal Considerations

Lab notebooks are institutional property in most academic and industrial settings. Before leaving a lab (graduation, end of rotation, leaving a company), all notebooks — paper and electronic — should be organized and handed to the PI or designated successor. This is both an ethical obligation and, in IP-sensitive contexts, a legal one.

For research with potential patent applications, electronic notebooks require verifiable timestamps (provided by most ELN platforms' audit trail systems). Consult your institution's technology transfer office for specific requirements.

## Takeaway

Lab notebook discipline is a research practice, not a bureaucratic requirement. A complete notebook entry — rationale, materials, protocol, observations, interpretation — written contemporaneously with the experiment is the foundational record of reproducible science. Electronic notebooks (Benchling, Jupyter) have real advantages in searchability and data linkage. The failure modes of documentation (retroactive writing, undocumented decisions, absent interpretation) can be overcome by making notebook writing a simultaneous, not subsequent, activity to the experiment itself.
