# 23.3.1 Bonding Methods: Flip-Chip, Wafer Bonding, and Transfer Printing

## Why III-V at All

Efficient light emission requires a direct bandgap: radiative recombination in GaAs or InP conserves momentum without phonon assistance, giving internal quantum efficiencies near unity, while silicon's indirect gap suppresses radiative rates by orders of magnitude (Chapter 4). The practical materials: **GaAs**-based quantum wells and VCSELs for 850 nm datacom, **InP**-based InGaAsP/InAlGaAs quantum wells for 1310/1550 nm — the gain media of every telecom laser and SOA. InP wafers, however, top out at 100–150 mm, are fragile and expensive, and III-V fabs lack silicon's scale. The entire integration agenda is: keep the III-V volume as small as possible (it supplies only gain), and let silicon do everything else.

The integration options form a ladder of increasing intimacy. This subsection covers the *assembly* approaches — attaching finished or semi-finished III-V material to silicon.

## Flip-Chip Die Bonding

The most direct approach: fully fabricate and test the laser as a discrete III-V die, then mount it face-down on the silicon photonic chip, with solder (AuSn) bumps providing mechanical attachment, electrical contact, and a heat path. Light couples from the laser facet into an on-chip edge coupler.

- **Alignment is the crux.** Edge-coupled modes of a few μm demand placement accuracy of ±0.5–1 μm (for ≲1 dB penalty) in three axes. Precision die bonders achieve this either *actively* (power the laser during placement, maximize coupled power — accurate but slow) or *passively* (machine vision plus lithographically defined mechanical stops and pedestals that register the die height — faster, the production choice).
- **Throughput is serial**: one die at a time, seconds to minutes each. Fine for transceivers; painful for a wafer of thousands of chips.
- **Strengths**: known-good dies (the laser was tested *before* attach, so yield multiplies rather than compounds), independent optimization of the III-V process, and heat extraction through the bumps rather than through the photonic chip's buried oxide (a real advantage — BOX is a thermal blanket).

Flip-chip remains the dominant approach in shipped products where laser count per chip is small.

## Die-to-Wafer and Wafer-to-Wafer Bonding

The heterogeneous approach, by contrast, bonds *unpatterned* III-V epitaxial material onto the SOI wafer early, and only then fabricates the laser — with silicon-side lithography. Two bonding chemistries dominate:

**Direct (molecular) bonding.** Both surfaces are planarized to sub-0.5 nm RMS roughness (CMP, Section 23.1.3), activated with an O₂ plasma to create hydrophilic OH-terminated surfaces, brought into contact at room temperature (van der Waals attachment), and annealed at ~250–300 °C to form covalent bonds across the interface. The low anneal temperature matters twice over: it respects the thermal budget, and it limits stress from the large thermal-expansion mismatch between InP and Si.

**Adhesive bonding.** A spun-on polymer — DVS-BCB (benzocyclobutene) is the standard — glues the III-V to the SOI with a 50–300 nm bond line, curing near 250 °C. It tolerates particles, roughness, and topography far better than direct bonding, at the cost of a polymer (with its thermal resistance) in the optical near field. This is the workhorse of the Ghent/IMEC heterogeneous platform.

In both cases, after bonding, the InP *substrate* is removed (mechanical grinding plus selective wet etch to an etch-stop layer), leaving only the few-μm epitaxial film — quantum wells and cladding — riding on silicon. The killer insight, due to the UCSB–Intel line of work that produced the first electrically pumped hybrid silicon evanescent laser [Fang et al., *Optics Express*, 2006]: because the film is unpatterned, **die placement accuracy is irrelevant** (±25 μm is fine); the laser cavity is subsequently defined by *lithography on the silicon wafer*, self-aligned to the silicon waveguides beneath. Bonding converts an assembly-precision problem into a lithography problem — and lithography is what silicon fabs do supremely well.

Die-to-wafer bonding (multiple small III-V dies tiled over a 200/300 mm SOI wafer, only where lasers are needed) economizes the expensive III-V area; wafer-to-wafer bonding maximizes throughput when full coverage is wanted.

## Micro-Transfer Printing

Micro-transfer printing (μTP) splits the difference between flip-chip's known-good-die logic and bonding's wafer-scale parallelism. On the III-V source wafer, devices ("coupons," tens×hundreds of μm) are fabricated atop a sacrificial release layer, which is then underetched so each coupon is held only by small resist tethers. A patterned **elastomeric (PDMS) stamp** presses onto an array of coupons and — exploiting the rate-dependent adhesion of the elastomer — picks them all when peeled *fast*, and releases them onto the target wafer when peeled *slow*. Placement accuracy is ~±0.5–1.5 μm (3σ), adequate for evanescent-coupling schemes and taper-assisted coupling; arrays of dozens–hundreds of coupons print in one stamp cycle.

μTP's virtues: massively parallel like wafer bonding, *sparse and material-efficient* like flip-chip (coupons can be pre-tested on-source for known-good selection), and heterogeneous in the broadest sense — the same technique prints III-V amplifiers, lithium-niobate patches, and Ge devices onto photonic wafers [see Roelkens and colleagues' III-V-on-Si program, e.g., *Laser & Photonics Reviews*, 2010, and its μTP successors]. Its challenges: release-layer epitaxy and yield of the underetch, tether engineering, and interface cleanliness — all active development areas, with commercial tooling (X-Celeprint lineage) maturing through the 2020s.

## Choosing, as a System Designer

| Method | Alignment burden | Parallelism | III-V usage | Maturity |
|---|---|---|---|---|
| Flip-chip die | ±0.5–1 μm at assembly | serial | full dies | production (transceivers) |
| Die/wafer bonding | ±25 μm placement; lithography-defined devices | wafer-scale | tiled films | production (Intel-class volume) |
| Micro-transfer printing | ±0.5–1.5 μm stamp | array-parallel | sparse coupons | pilot/emerging |

For photonic computing the decision couples to architecture: a processor needing one comb source (Chapter 7) can flip-chip it; an architecture wanting *hundreds* of distributed on-chip gain elements — optical fan-out amplifiers between mesh stages, per-channel sources for WDM weight banks — is only plausible with wafer-scale bonding or transfer printing. The next subsection follows the bonded material into working lasers, and the monolithic-growth endgame.
