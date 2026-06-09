# DIY Biology and Community Laboratory Biosafety

In 2008, a group of biology enthusiasts in Brooklyn rented space in a former industrial building, pooled their money, and opened Genspace — the first community biology laboratory in New York City. Some of the founding members had PhDs; others had never taken a college biology course. What they shared was a conviction that biological experimentation should not be the exclusive province of universities and pharmaceutical companies. Genspace installed an autoclave, set up safety protocols, and began offering classes in gel electrophoresis and PCR to anyone who wanted to learn. It was, in many ways, a radical act: the deliberate democratization of tools that had previously required institutional affiliation to access. A decade and a half later, Genspace is still operating, and the community biology movement it helped launch spans hundreds of spaces worldwide. The question of how to think about biosafety in that context — outside the institutional frameworks that regulate academic research — is one that synthetic biology is still working through.

**DIY biology** (do-it-yourself biology, also called community biology or biohacking) refers to biological experimentation conducted outside traditional institutional settings — in community laboratories, garages, hackerspaces, and homes — often by individuals without formal scientific training. The DIY biology movement emerged in the late 2000s alongside open-source hardware and maker culture, applying similar principles of accessible tools, shared knowledge, and community learning to life sciences. Understanding DIY biology is important for synthetic biologists both because it represents a legitimate democratization of science and because it raises distinct biosafety and biosecurity considerations.

## Origins of the DIY Biology Movement

**2008–2010**: Several founding community biology spaces opened, including **Genspace** (New York City) and **BioCurious** (San Jose, CA). These were explicitly modeled on the hackerspace concept: shared membership-funded laboratories with open access to tools, peer mentorship, and a culture of experimentation.

**2010**: The DIYbio.org network was established as a global coordination platform, publishing a **Code of Ethics** that emphasized safety, transparency, and responsible practice. Key principles:
- Transparency: share methods, results, and open questions
- Safety: use established biosafety practices; avoid working with dangerous organisms
- Community: contribute to open access knowledge
- Non-maleficence: do not create or deploy agents intended to harm

**2010s expansion**: the movement grew globally. Community labs in Europe (LabBiotech.eu, La Paillasse in France, biohack.me in Germany), Australia, and Asia. The availability of affordable CRISPR kits (e.g., The Odin's ~$150 CRISPR bacterial kit) significantly lowered barriers.

**Current landscape**: hundreds of community biology spaces worldwide, varying from well-equipped community labs (BSL-1 to limited BSL-2) to informal home operations.

## Biosafety in Community Laboratories

Reputable community biology laboratories implement biosafety standards appropriate to their work:

**EBRC/iGEM community lab standards**: the Engineering Biology Research Consortium (EBRC) and iGEM have published community laboratory biosafety recommendations that align with federal guidelines for institutions:
- Work limited to BSL-1 organisms (E. coli K-12, S. cerevisiae, B. subtilis)
- Autoclave or equivalent decontamination of all biological waste before disposal
- Personnel training in biosafety before using biological materials
- No work with human clinical samples unless BSL-2 appropriate protocols are followed

**Community lab membership agreements**: most established community labs require members to:
- Complete biosafety training (typically an online module + in-person orientation)
- Sign a membership agreement committing to biosafety protocols
- Submit a project proposal reviewed by a lab safety committee before beginning biological work
- Report accidents or near-misses to the lab safety officer

**Regulatory status**: community labs in the U.S. generally operate under the same NIH Guidelines as universities — they are exempt for BSL-1 work with standard organisms and do not require IBC registration for exempt experiments. However, they must comply with:
- Local health department regulations (varies by city/state)
- CDC/USDA Select Agent regulations if they possessed any listed agents (they generally do not)
- OSHA laboratory standard for chemical safety
- EPA regulations for disposal of biological waste

## The Legitimacy of DIY Biology

The mainstream scientific community's view of DIY biology has evolved from skepticism to qualified acceptance:

**Legitimate contributions**: community labs have contributed to:
- Open-source biology tools (OpenPCR thermocycler, MinION sequencing workshops)
- Citizen science projects (microbiome sampling, water quality testing)
- Low-cost diagnostic development (paper-based colorimetric assays)
- Science education for underserved communities (community labs in lower-income urban areas)
- Citizen biomonitoring for environmental hazards

**Concerns**:
- **Oversight gap**: community labs operate without the institutional frameworks (IBC, BSO, IRB) that provide oversight in universities. Safety depends heavily on community culture and self-regulation.
- **Skill heterogeneity**: community members range from PhD scientists seeking workspace to curious hobbyists with no scientific background. Biosafety depends on training quality and culture.
- **Self-experimentation**: some DIY biologists have engaged in self-experimentation with genetic modification tools (Josiah Zayner's public self-injection with CRISPR plasmids in 2017). While the actual safety risk was low (the experiment was unlikely to work as hoped), it highlighted gaps in oversight.

## The Josiah Zayner Case and Self-Experimentation

In 2017, Josiah Zayner — founder of The Odin, a company selling DIY biology kits — injected himself with a CRISPR construct targeting the myostatin gene (muscle growth regulator) at a live conference. This was widely covered in media.

**Safety assessment**: the actual risk was very low — the injection used a non-viral construct with low delivery efficiency, targeting a gene in muscle cells, with no realistic path to significant editing. The experiment was theatric rather than dangerous.

**Regulatory response**: the California Department of Public Health sent a cease and desist letter; the FDA issued a statement that gene therapy products (including DIY CRISPR kits marketed for human use) were subject to regulation. The FDA has authority over biologics for human use, including CRISPR constructs.

**Broader impact**: the incident prompted discussion within the DIY biology community about the appropriate limits of self-experimentation and whether the community should proactively self-regulate more visible extreme cases.

## Biosecurity Risks from DIY Biology

The biosecurity concern with DIY biology is not primarily that community labs will accidentally create dangerous pathogens (the organisms they work with are generally not pathogenic). The concern is more specific:

**Knowledge proliferation**: tutorials, protocols, and educational materials that lower technical barriers for legitimate practitioners also lower barriers for individuals with malicious intent. The DIY biology community generally accepts this as an inherent feature of open knowledge, arguing that information availability is necessary for the defensive side (biosecurity researchers, diagnosticians) as well as the offensive side.

**Screening gap**: IGSC screening applies to orders from legitimate synthesis companies. DIY biologists could potentially use lower-cost, less-regulated synthesis providers (particularly international ones) to access sequences that IGSC members would screen. This is a recognized gap in the gene synthesis screening system.

**Lack of institutional oversight**: without IBC review, there is no formal checkpoint at which a DIY biologist's project plan is reviewed for dual-use potential. Self-reporting to law enforcement or biosecurity authorities is not a cultural norm.

## Community Self-Governance

The DIY biology community has developed self-governance mechanisms:

**DIYbio Code of Ethics**: non-binding but influential statement of principles emphasizing safety, transparency, and non-maleficence.

**Community lab safety committees**: most established labs have informal safety review for member projects.

**FBI engagement**: beginning around 2012, the FBI's Weapons of Mass Destruction Directorate began engaging proactively with the DIY biology community — attending community lab events, developing relationships, and providing biosafety resources — rather than treating it as purely a surveillance target. This community policing approach is viewed as more effective than surveillance for identifying genuine threats.

**Global DIYbio organization**: the diaspora of DIYbio.org local chapters provides peer accountability across the community.

## Why This Matters

DIY biology represents both a genuine democratization of biological tools — expanding access to techniques that were previously available only to well-funded research institutions — and a genuine governance challenge. The biosafety and biosecurity frameworks developed for academic and industrial research do not straightforwardly apply to community settings. Working out how to extend appropriate oversight to community biology — without destroying its open, accessible character — is an active policy challenge. For synthetic biologists, DIY biology matters as a community they will interact with (through iGEM, community lab partnerships, and open-source tool development), as a policy space they should help shape, and as a reminder that the tools they develop do not stay inside research institutions. The design of tools that are safe by default — that cannot easily be repurposed for harm — is a form of responsible innovation that benefits both the research community and the broader public.
