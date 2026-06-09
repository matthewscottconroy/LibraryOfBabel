# Biosafety Levels

Consider what happened in 1978 at the University of Birmingham. Janet Parker, a medical photographer working one floor above a smallpox research laboratory, contracted smallpox through the building's ventilation system and died. The laboratory's director, Henry Bedson, had been conducting research he believed was adequately contained. It was not. The tragedy led directly to the global eradication program's final push and to a fundamental rethinking of how we match physical containment to the hazard of what we study. That rethinking became codified in the biosafety level system — a framework that, properly followed, has made laboratory-acquired infections remarkably rare.

**Biosafety levels (BSLs)** are a classification system for laboratory containment requirements, established by the CDC and NIH and codified in the *Biosafety in Microbiological and Biomedical Laboratories* (BMBL) manual. The system uses four levels of ascending containment and practice requirements, matched to the potential hazard of the agent being studied. The BSL system applies both to natural pathogens and to recombinant organisms created through synthetic biology. Understanding which level applies to which experiments is one of the first practical biosafety competencies a researcher must acquire.

## BSL-1: Minimal Risk

**Agents**: well-characterized microorganisms not known to cause disease in healthy adults. Examples: *Bacillus subtilis*, *E. coli* K-12, non-pathogenic *Saccharomyces cerevisiae*, bacteriophage lambda.

**Risk group**: Risk Group 1 (RG1) — poses no or negligible risk to healthy adults.

**Practice requirements**:
- Standard microbiological practices: wash hands after working with organisms; decontaminate work surfaces daily; no eating, drinking, or applying cosmetics in lab
- No specific protective equipment beyond lab coat and gloves
- Work can be performed on open benches

**Physical containment**:
- Standard laboratory design with doors that separate the lab from general building circulation
- Sink for hand washing
- No special ventilation requirements

**Typical research context**: introductory microbiology courses, routine molecular biology using E. coli K-12, yeast genetics, bacteriophage work.

**Important nuance for synthetic biology**: E. coli K-12 strains (DH5α, BL21, MG1655) are BSL-1. Pathogenic E. coli strains (O157:H7, K1) are BSL-2. Creating recombinant constructs in K-12 does not automatically elevate the BSL level, but inserting genes encoding toxins or virulence factors can change the classification.

## BSL-2: Moderate Risk

**Agents**: microorganisms that cause human disease but for which effective treatments or preventive measures exist. Examples: *Staphylococcus aureus*, *Salmonella* (non-typhoidal), human influenza virus, HIV, hepatitis B virus (HBV).

**Risk group**: Risk Group 2 (RG2) — moderate individual risk; low community risk.

**Practice requirements**:
- All BSL-1 practices plus:
- Access restricted during work (door closed, "Biohazard" sign posted)
- Sharps (needles, syringes) handled with extreme caution; use blunt-tip alternatives when possible
- All procedures that may generate aerosols performed in a **Biological Safety Cabinet (BSC)** Class II, Type A2
- Decontamination of all infectious waste before disposal (autoclave at 121°C, 15 psi, ≥20 min)
- Annual medical surveillance for personnel working with bloodborne pathogens (OSHA Bloodborne Pathogen Standard applies to HIV, HBV work)

**Physical containment**:
- Self-closing doors
- Autoclave accessible (may be in building, not necessarily in lab)
- Eye wash station
- BSC (biosafety cabinet) for aerosol-generating procedures

**Typical research context**: most clinical microbiology, HIV research, influenza studies, work with primary human cell lines, mouse experiments using BSL-2 agents.

**The BSC**: a key piece of BSL-2 infrastructure. Class II, Type A2 BSCs circulate HEPA-filtered air across the work surface, capturing aerosol particles (including bacteria, viruses) before they reach the user. The cabinet is not airtight — room air enters from the front sash — but the airflow is directional (toward the HEPA filter) so that material on the work surface cannot exit into the room in significant quantities.

## BSL-3: High Risk, Serious or Lethal Disease

**Agents**: agents that can cause serious or potentially lethal disease for which treatments may or may not exist. Examples: *Mycobacterium tuberculosis*, West Nile virus, SARS-CoV-1, SARS-CoV-2 (initially BSL-3; reclassified based on risk assessment at many institutions), Venezuelan equine encephalitis virus (VEEV), *Coxiella burnetii* (Q fever).

**Risk group**: Risk Group 3 (RG3) — high individual risk; low to moderate community risk (limited spread).

**Practice requirements**:
- All BSL-2 practices plus:
- **Two pairs of gloves** required; change outer gloves before leaving lab
- Solid-front or wrap-around gowns, not reused outside the BSL-3 suite
- All manipulations performed in BSC Class II or Class III
- Respirator protection (N95 or PAPR) required for working with aerosolized agents
- Personnel receive annual medical surveillance and baseline serum sample storage (for potential post-exposure testing)

**Physical containment** (specialized facility design):
- **Negative-pressure rooms**: the BSL-3 suite is maintained at lower air pressure than the surrounding corridors. If a door opens accidentally, air flows inward — preventing contaminated lab air from escaping into the building
- Double-door entry with an anteroom (airlock): one door must be closed before the other opens
- HEPA-filtered exhaust air: all exhaust air from BSL-3 labs passes through HEPA filters before discharge
- Sealed walls, floors, and ceiling: no gaps; surfaces wipeable with disinfectants
- Dedicated HVAC system: not shared with other building areas

**Typical research context**: tuberculosis research, rabies (at some institutions), SARS-CoV-2 (during pandemic, many institutions performed live virus work at BSL-3), emerging infectious disease research.

## BSL-4: Extreme Risk, No Vaccine or Treatment

**Agents**: agents that pose a high risk of life-threatening disease for which no vaccine or therapy is available. Examples: Ebola virus, Marburg virus, Nipah virus (some strains), Lassa fever virus, Crimean-Congo hemorrhagic fever virus, variola (smallpox) — restricted to CDC and VECTOR (Russia).

**Risk group**: Risk Group 4 (RG4) — high individual risk; high community risk.

**Practice requirements** (most stringent):
- All BSL-3 practices plus:
- **Positive-pressure suit** ("space suit"): the researcher wears a full-body, pressurized suit supplied with HEPA-filtered, breathing-quality air through a chemical air supply system. The suit is at higher pressure than the room, so any suit breach allows air to exit rather than enter — protecting the researcher
- Shower-out before leaving suite: personnel shower in the pressurized suit, then degrease/decontaminate and remove the suit in the decontamination shower
- Or Class III BSC (glove box): sealed, negative-pressure glove box with HEPA-filtered supply and exhaust; all manipulations through rubber gloves attached to the box

**Physical containment**:
- **Maximum containment**: purpose-built BSL-4 facility with separate HVAC, decontamination chambers, double-HEPA-filtered exhaust
- All materials leaving the suite pass through a double-door autoclave or pass-through dunk tank (formaldehyde or disinfectant solution)
- Building within a building design

**BSL-4 facilities worldwide** (approximate): CDC (Atlanta), USAMRIID (Fort Detrick, MD), NIH (Bethesda), National Bio and Agro-Defense Facility (NBAF, Manhattan, KS), Galveston National Laboratory (Texas), Institut Pasteur (Paris), Robert Koch Institut (Berlin), Public Health Agency of Canada (Winnipeg), Australian Centre for Disease Preparedness (Geelong). Total: ~60 worldwide.

## Comparison Summary

| Feature | BSL-1 | BSL-2 | BSL-3 | BSL-4 |
|---|---|---|---|---|
| Pressure | Normal | Normal | Negative | Negative |
| HEPA exhaust | No | No | Yes | Yes (double) |
| BSC required | No | For aerosols | Always | Class III or suit |
| Gowning | Lab coat | Lab coat | Solid-front gown | Full positive-pressure suit |
| Gloves | Yes | Yes | Two pairs | Built into BSC or suit |
| Shower-out | No | No | No | Yes |
| Example agent | E. coli K-12 | S. aureus | M. tuberculosis | Ebola virus |

## Biosafety Level Determination for Recombinant Organisms

For engineered organisms created in synthetic biology, BSL assignment follows the **highest-risk component** rule:
- E. coli K-12 (BSL-1) expressing a gene encoding a non-toxic protein from a pathogen: remains BSL-1 (the protein product determines risk, not the gene source)
- E. coli K-12 expressing a functional toxin gene (e.g., diphtheria toxin A chain): elevated to BSL-2 or higher (risk from the expressed toxin)
- Chimeric virus incorporating surface proteins from an RG-3 virus: requires BSL-3
- Any work that could reconstitute a BSL-4 agent from its components: requires BSL-4

The IBC reviews these determinations on a per-experiment basis.

## Why This Matters

Biosafety levels are the operational foundation of laboratory safety in biological research. They are not bureaucratic formalities — they are evidence-based containment standards developed through decades of epidemiological study of laboratory-acquired infections (LAIs). The CDC estimates that approximately 10,000 LAIs occurred in the United States during the 20th century, with hundreds of deaths. Adherence to BSL requirements has dramatically reduced this rate. For synthetic biology specifically, where new organisms with unpredictable properties are routinely created, knowing the appropriate BSL for each experiment — and more importantly, knowing when an experiment's BSL designation should be escalated because of uncertain risks — is a foundational competency.
