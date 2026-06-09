# Institutional Biosafety Committees

Imagine you are a graduate student about to begin a new project. You want to express a gene from a human pathogen in E. coli to purify the protein for biochemical studies. Is this experiment safe? Does it require oversight? Who decides? The answer, at most research institutions in the United States, is: the Institutional Biosafety Committee. Before you pipette a single nucleotide, before you order the synthesis of a single DNA fragment, that committee — a group of scientists, community members, and safety professionals at your institution — has authority over whether and how your experiment may proceed. Understanding how that committee works, what it actually evaluates, and how to work with it effectively is one of the most practically useful things you can learn as a researcher entering the field.

**Institutional Biosafety Committees (IBCs)** are the primary mechanism by which universities, research institutes, and companies implement biosafety oversight of recombinant DNA and synthetic biology research. Every institution receiving NIH funding for recombinant or synthetic nucleic acid research must register an IBC with the NIH Office of Science Policy (OSP) and maintain it in compliance with the NIH Guidelines. Even institutions that do not receive NIH funding typically maintain IBCs as a matter of best practice, liability management, and insurance compliance.

## IBC Composition and Structure

The NIH Guidelines specify minimum IBC composition:
- At least **5 members** with collectively sufficient expertise to assess the safety of proposed research
- At least **two community members** who are not affiliated with the institution and who represent community interests (typically at least one is not a scientist)
- **Biological safety officer (BSO)** as a voting member (required for institutions with BSL-3 or BSL-4 facilities)
- Members with expertise in relevant scientific areas: molecular biology, microbiology, biosafety
- Member knowledgeable in institutional commitments and policies (often a compliance officer or research administrator)

IBCs typically have 8–20 members. They are advisory to institutional administration but have authority to halt research that does not comply with the NIH Guidelines.

## IBC Review Process

**Protocol submission**: researchers submit a **biosafety protocol** (or IBC protocol) before beginning any non-exempt recombinant DNA research. The protocol describes:
- The organisms, genes, and recombinant molecules involved
- The procedures to be performed
- Proposed containment level (BSL)
- Personnel training status
- Waste handling and decontamination procedures

**IBC review**: the IBC evaluates whether the proposed containment level is appropriate, whether personnel training is adequate, and whether waste management plans comply with regulations. The IBC may approve, approve with modifications, or require full committee review.

**Approval duration**: IBC protocols are typically approved for 3 years; renewal requires updated protocol submission.

**Risk assessment by the IBC**: the IBC applies a four-factor risk analysis (probability, severity, reversibility, breadth — covered in Section 2.4) and assigns a final BSL recommendation. The principal investigator's proposed BSL may be elevated by the IBC if the risk analysis warrants it.

## What Research Requires IBC Approval?

Under the NIH Guidelines, the following require IBC approval before initiation (Section III-D):
- Research involving RG2 or higher agents
- Research involving transgenic animals where recombinant DNA is involved
- Human gene transfer research
- Large-scale fermentation (>10 liters) involving recombinant organisms
- Some plant research with recombinant DNA in insect vectors

Research that is **exempt** (Section III-F) does not require IBC approval — including most routine cloning in E. coli K-12, standard cell culture work with non-pathogenic cell lines, and intra-species DNA transfer.

**Important practical note**: institutions often require IBC registration even for exempt research, as a matter of institutional risk management. Check your institution's specific policies.

## The Biological Safety Officer (BSO)

The BSO is the institutional professional responsible for implementing the biosafety program on a day-to-day basis. Responsibilities include:
- Conducting biosafety training for laboratory personnel
- Performing laboratory inspections
- Investigating accidents and near-misses involving biohazardous materials
- Advising the IBC on technical matters
- Maintaining records of registered biosafety protocols
- Ensuring regulatory compliance

At large research universities, the BSO office may have multiple staff members covering different areas (microbiology BSO, biosafety cabinet certification, radiation safety if combined). At smaller institutions, a single BSO may cover all safety programs.

The BSO is the researcher's primary point of contact for biosafety questions — not a regulatory adversary but a resource for navigating the system correctly.

## Reporting Requirements

**Accidents and exposures**: any laboratory accident involving biohazardous materials must be reported to the BSO and IBC promptly. For significant exposures (needle stick with HIV, splash of infectious material), immediate medical evaluation is required. Post-exposure prophylaxis (PEP) for HIV or hepatitis B must be initiated within hours to be effective.

**Changes to approved protocols**: significant changes to approved IBC protocols (new agents, new procedures at higher containment) require IBC notification or re-approval before implementation.

**Biennial reporting to NIH**: institutions must report to NIH/OSP biennially on all IBC-registered protocols and any significant incidents.

## Enforcement

The NIH Guidelines are not federal law (except for research conducted in federal laboratories). For private institutions, compliance is enforced through:
- **NIH funding conditions**: failure to maintain a compliant IBC can result in loss of all NIH funding institution-wide (a severe consequence for research universities)
- **State and local regulations**: many states have recombinant DNA regulations that parallel or exceed the NIH Guidelines
- **OSHA regulations**: the Bloodborne Pathogens Standard (29 CFR 1910.1030) is federal law enforceable by OSHA; it applies to laboratory work with HIV, HBV, and other bloodborne pathogens regardless of NIH funding status
- **CDC Select Agent Regulations**: federal regulations (42 CFR Part 73) for possession and use of Select Agents, enforced by CDC — legally binding regardless of funding source

**Consequences of non-compliance**: beyond funding loss, non-compliant research may result in institutional sanctions on the PI, publication retractions if journals discover research was conducted without IBC approval, and reputational damage to the institution.

## Case Study: IBC Protocol for a Typical Synthetic Biology Experiment

**Scenario**: a graduate student is building a toggle switch circuit in E. coli MG1655 (BSL-1 strain) using lacI and tetR repressor genes, with GFP as a reporter.

**IBC status**: this experiment is likely **exempt** under Section III-F of the NIH Guidelines (E. coli K-12 host, non-pathogenic gene products, no toxin genes).

**However**: the institution may require IBC registration for all recombinant DNA work in an institutional biosafety registration system, even for exempt experiments.

**What would change the risk level**:
- Adding a gene encoding a toxin protein: would require IBC review, potentially BSL-2
- Using a clinical isolate of E. coli (not K-12): would require IBC review at BSL-2
- Working with plasmids encoding antibiotic resistance for selection (routine): still exempt, but the resistance gene must be one appropriate for laboratory use (not a gene that would confer clinically significant resistance to a novel class)

## Why This Matters

IBCs are the operational reality of biosafety regulation — the tangible mechanism through which abstract guidelines become laboratory practice. For a researcher new to the field, the IBC is the institution to notify before starting recombinant DNA work and the source of authoritative guidance on whether a specific experiment is exempt, requires notification, or requires full approval. Understanding the IBC's role — and the BSO's role — and working with them rather than viewing them as bureaucratic obstacles is both practically essential and professionally correct. The IBC framework has successfully maintained biosafety in thousands of laboratories over five decades because it combines institutional accountability with scientific expertise at the point where decisions are actually made.
