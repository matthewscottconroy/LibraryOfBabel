# iGEM and Open Standards in Synthetic Biology

In January 2003, a handful of MIT students proposed something that, in retrospect, sounds almost quaint: what if biological parts could be standardized, like resistors and capacitors in electronics? What if a promoter from one lab's toolkit could be reliably combined with a ribosome binding site from another, and a coding sequence from a third, and the assembled system would behave predictably? The students went to work on biological oscillators in E. coli. Their project was modest; the idea behind it was not. Two decades later, the International Genetically Engineered Machine Foundation has become the largest synthetic biology education and community-building initiative in the world — and the conceptual framework those MIT students proposed, of interchangeable biological parts assembled into circuits, has become the organizing metaphor for an entire discipline.

The **International Genetically Engineered Machine (iGEM) Foundation** is both a student competition and a community of practice that has had an outsized influence on how synthetic biology is practiced, standardized, and governed at the global level. What began as a January term independent study project at MIT in 2003 has become the largest synthetic biology education and research initiative in the world, involving thousands of teams from dozens of countries annually and maintaining a repository of thousands of standardized biological parts.

## Origins and History

**2003**: Five MIT students (including Tom Knight and Randy Rettberg) designed the first iGEM project: biological oscillators in E. coli, with the goal of demonstrating that biology could be engineered from standard parts like electronic components.

**2004**: The first iGEM competition involved five teams, all from MIT, working over the summer.

**2007**: iGEM expanded internationally; teams from Europe and Asia participated for the first time.

**2010s**: iGEM grew to hundreds of teams per year from universities on six continents. High school division added. Community labs and non-traditional teams integrated.

**2020s**: iGEM Foundation became fully independent of MIT. Annual Jamboree moved to different global locations. Approximately 350–450 teams participate per year, with >50,000 alumni.

## The BioBrick Standard and Registry of Standard Biological Parts

The most lasting technical legacy of iGEM is the **BioBrick standard** (RFC 10 and subsequent standards) and the **Registry of Standard Biological Parts** (parts.igem.org).

**BioBrick standard (RFC 10)**: defined a prefix/suffix scheme using specific restriction sites (EcoRI-SpeI prefix; XbaI-PstI suffix) that allow standard assembly of biological parts:
- Any part flanked by standard prefix and suffix is a **Basic Part**
- Two basic parts can be assembled into a **Composite Part** using standard restriction-ligation
- The assembled composite also has prefix and suffix, allowing iterative assembly

The BioBrick standard made parts interchangeable: a promoter characterized by one team could, in principle, be combined with an RBS from another team and a reporter gene from a third team, with predictable results.

**Registry of Standard Biological Parts**: a public database of BioBrick parts. As of 2024, contains >20,000 parts including:
- Promoters (constitutive, inducible, regulated)
- Ribosome binding sites (RBS)
- Coding sequences (reporters, enzymes, regulatory proteins)
- Terminators
- Composite parts (circuits, modules)

Each part has a datasheet with:
- Sequence (in standard vector)
- Part type and function
- Measurement data (where available)
- Characterization notes from submitting teams and subsequent users

**Limitations of BioBrick RFC 10**: the restriction sites used for assembly (EcoRI, SpeI, XbaI, PstI) prevent those restriction sites from appearing within parts — limiting sequence freedom. Modern alternatives (Golden Gate, Gibson Assembly, BASIC assembly) do not have this constraint and have largely replaced RFC 10 in research settings, though RFC 10 remains in use in iGEM for its simplicity.

**Successor standards**:
- **MoClo (Modular Cloning)**: uses BsaI Golden Gate assembly with defined overhangs for position-specific parts (promoter, 5'UTR, CDS, terminator)
- **SEVA (Standard European Vector Architecture)**: modular plasmid system for Gram-negative bacteria
- **BASIC assembly**: DNA-pair ligation with unique linkers; computationally designed assemblies

## Measurement Standards

iGEM has been a major driver of **measurement standardization** in synthetic biology:

**Measurement Kit**: iGEM provides teams with a standard GFP construct (BBa_J364000 — a constitutively expressed GFP) and fluorescence calibration beads. All teams measure their GFP constructs under the same conditions and report results in **MEFL (Molecules of Equivalent Fluorescein)** units — a calibrated absolute fluorescence unit.

This allows comparison of GFP expression levels across teams, instruments, and years — something that was previously impossible because arbitrary fluorescence units differ between plate readers.

**InterLab Study**: iGEM's annual InterLab measurement study has run since 2014. Teams receive the same 2–8 standardized genetic constructs and measure GFP expression following a defined protocol. Results from hundreds of teams are compared to determine measurement reproducibility. Key finding: proper calibration (using particle standards and fluorescein) reduces inter-lab coefficient of variation from >5× to <1.5× — demonstrating that standardized protocols dramatically improve reproducibility.

## iGEM Biosafety Requirements

Every iGEM team must complete a **Safety Form** as part of their project submission, addressing:
- Risk groups of organisms used
- Whether their chassis and parts are on the IGSC concerns list
- Biosafety measures in place at their institution
- Environmental release intentions (prohibited in iGEM unless approved)

**iGEM Safety Committee** reviews all Safety Forms and flags projects with potential biosafety concerns for additional review. Teams may be required to modify their projects before presenting at the Jamboree.

**Prohibited in iGEM**:
- Organisms in Risk Group 3 or 4
- Sequences from Select Agents (unless specifically approved by iGEM Safety Committee with institutional registration)
- Intentional environmental release of engineered organisms
- Experiments designed to enhance pathogen virulence or transmissibility

iGEM's biosafety requirements have created a culture of safety awareness among thousands of student researchers — many of whom go on to careers in synthetic biology, biotechnology, and policy.

## Open Science Model

iGEM operates on an explicitly open science model:
- All Registry parts are freely available for academic and non-commercial use
- Teams are required to contribute new parts to the Registry as a condition of competition
- All team wikis (documenting project design, methods, and results) are permanently archived and publicly accessible
- iGEM has resisted commercialization of Registry parts, maintaining open access

This open model has been influential in shaping the broader culture of synthetic biology toward sharing — contrasting with the proprietary approach of many biotech companies and with the patenting of research tools in other fields (e.g., the foundational CRISPR patent disputes).

## Why This Matters

iGEM has shaped synthetic biology's culture, practice, and norms at a global scale. The BioBrick standard — even where it has been superseded technically — established the conceptual framework of interchangeable, characterized biological parts that remains central to synthetic biology thinking. The InterLab study has advanced measurement standardization more effectively than any top-down regulatory effort. The Safety Form process has introduced biosafety thinking to thousands of student researchers before they move into research careers. And the Registry — even with its limitations in data quality and part characterization completeness — represents the most ambitious attempt to build a freely accessible library of biological parts, demonstrating that open sharing of biological knowledge is possible and valuable. For researchers entering the field, iGEM's contributions are not historical artifacts — they are infrastructure actively used in research laboratories worldwide.
