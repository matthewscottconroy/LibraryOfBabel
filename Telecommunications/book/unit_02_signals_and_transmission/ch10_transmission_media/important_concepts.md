# Chapter 10 — Important Concepts

**A medium's properties are a function of the manufacturing art** *(chapter)* —
Not fixed facts. Glass attenuating 1,000 dB/km in 1966 attenuates 0.17 dB/km today;
Cat5e specified to 100 MHz in 1999 carries 2.5 Gb/s under 802.3bz. Anyone stating
confidently what a medium can do should be asked what year they learned it.

**Solid versus stranded** *(§10.1)* — Solid conductors for permanent horizontal
runs (better electrically, poor flex life); stranded for patch cords. Using either
in the other's role produces faults months later.

**Category as a bandwidth specification** *(§10.1)* — Cat5e is 100 MHz, Cat6 is
250 MHz, Cat6a is 500 MHz, Cat8 is 2 GHz. **Not** a data rate: the rate depends on
the encoding, which is why the same Cat5e carries 100 Mb/s, 1 Gb/s and 2.5 Gb/s
under successive standards.

**Cat6's conditional 10 Gb/s** *(§10.1)* — 55 m headline, falling further in a
tightly bundled installation, because Cat6 does not specify **alien crosstalk** and
Cat6a does.

**The cabling cost argument** *(§10.1)* — Installation labour is identical
regardless of category and dominates the total; the material difference is small.
Hence "install the best cable you can terminate properly" is an economic argument on
a fifteen-year asset, not a technical preference. The counter-argument is real:
Cat6a is harder to terminate, and a badly terminated Cat6a run performs worse than a
well-terminated Cat5e one.

**Shielding designations** *(§10.1)* — U/UTP, F/UTP, S/FTP, F/FTP. Specify shielded
for severe EMI, dense 10 Gb/s bundles, or emission-security requirements; specify
unshielded otherwise. **A shield must be earthed properly**, and one bonded at both
ends across a potential difference carries current and radiates.

**T568A and T568B** *(§10.1)* — Differ by swapping the orange and green pairs;
either works, and **consistency within an installation is the only requirement**.
Pins 3 and 6 are one pair split around pins 4 and 5 — a telephone-compatibility
artefact, and the reason wiring "straight across" produces a split pair.

**Untwist limit** *(§10.1)* — No more than 13 mm at a termination for Cat5e, less
above. Beyond it the common-mode rejection stops working locally and NEXT rises.

**PoE budgets** *(§10.1)* — 802.3af 12.95 W, 802.3at 25.5 W, 802.3bt Type 3 51 W,
Type 4 71.3 W, all measured **at the device** after cable loss. A switch's total
budget is usually far less than the sum of its ports' maximums, so the arithmetic
must be done before ordering. Dissipated power also warms the bundle, raising
attenuation.

**Coax's confinement mechanism** *(§10.2)* — The field is entirely contained between
centre conductor and shield, so external fields do not reach it. Categorically
different from twisted pair's symmetry-based rejection: coax works **unbalanced**,
referenced to ground.

**50 Ω versus 75 Ω** *(§10.2)* — 50 Ω approximates the optimum for power handling
(transmitters, antenna feeds, test equipment, Wi-Fi); 75 Ω approximates minimum
attenuation (cable television, DOCSIS, video). Mismatching them reflects 4% of the
power at each discontinuity.

**Termination and reflections** *(§10.2)* — An unterminated coax end reflects
everything, which is why bus topologies needed a terminator at each end and why a
single loose connector took down an entire 10BASE2 segment.

**Hybrid fibre-coax** *(§10.2)* — Fibre to a neighbourhood node, coax to the homes.
The coax segment is **shared** among 100–500 households, which produces the "slow at
8 p.m." pattern that DSL subscribers do not experience.

**Why coax lost the LAN** *(§10.2)* — Not electrical properties, which were good.
The bus topology was operationally fragile (one fault killed everything), changes
were disruptive, and 10BASE-T's star reused telephone cabling already in buildings.
**Operational characteristics decided it**, which is how most media decisions are
actually made.

**Total internal reflection** *(§10.3)* — Light beyond the critical angle
θ_c = arcsin(*n*₂/*n*₁) reflects entirely at the core–cladding boundary. For typical
indices, about 84.4°. **The cladding is not a coating**: it provides a controlled,
permanent boundary of known index so the fibre can be handled and buried without its
optics changing.

**Single-mode versus multimode** *(§10.3)* — Multimode's 50 or 62.5 µm core supports
many paths and therefore modal dispersion; single-mode's ~9 µm core supports one and
eliminates it. Reach: 33–400 m against 10–80 km.

**Where the fibre cost actually is** *(§10.3)* — In the **transceivers**, not the
glass. A VCSEL into a 50 µm core is cheap; a DFB laser into a 9 µm core is not.
Hence multimode in data centres, where the transceiver saving multiplies by link
count, and single-mode everywhere else.

**OM and OS grades** *(§10.3)* — OM1 33 m to OM4 400 m at 10 Gb/s, the improvement
coming entirely from grading the refractive index profile so modes travel at more
nearly equal speeds. OS2 at ≤0.4 dB/km is the long-haul standard. **Jacket colour
is a convention, not a standard** — read the printing.

**Connector polish types** *(§10.3)* — PC (−40 dB return loss), UPC (−50 dB, blue),
**APC (−60 dB, 8° angle, green)**. APC reflects light out of the core rather than
back down it. **Never mate APC to UPC**: high loss and possible ferrule damage.

**Optical loss budget** *(§10.3)* — Launch power minus receiver sensitivity gives
the budget; subtract fibre, splices, connectors and a repair allowance; what remains
is margin. Under 3 dB is marginal; over 6 dB is comfortable. **Too much power is
also a fault** — receivers saturate, and attenuators exist for it.

**Dirty connectors** *(§10.3)* — The most common fibre fault by a wide margin. A
fingerprint costs several dB against a typical 20 dB budget.

**Never look into a fibre** *(§10.3)* — 1310 and 1550 nm are invisible, so there is
no blink reflex, and long-haul power levels cause retinal damage.

**Spectrum as a regulated resource** *(§10.4)* — Free space is the one medium you do
not own. **Licensed** (exclusive, expensive, protected), **unlicensed** (free,
shared, no protection), **lightly licensed** (CBRS, 6 GHz AFC). The 2021 US C-band
auction raised \$81 billion, which calibrates what predictability is worth.

**Higher frequency: more bandwidth, less range** *(§10.4)* — From the 20 log₁₀(*f*)
term in path loss plus rising material absorption. 60 GHz's oxygen absorption
(~15 dB/km) makes it useless for distance and ideal for dense reuse.

**When free space is right** *(§10.4)* — Cable is impossible (wayleave, roads,
listed fabric), prohibitive, or too slow; or the endpoint moves. Paid for with the
loss of any guarantee, plus rain fade above ~10 GHz, Fresnel clearance, and
regulatory compliance.

**The six-question media procedure** *(§10.5)* — How far; what rate over the
installation's life; what electrical environment; does the far end need power; what
does it all cost; what are the non-technical constraints. **Answer in order**, and
the distance question alone resolves most cases.

**Over 100 m rules out twisted pair** *(§10.5)* — Not "makes it marginal". There is
no supported way to run copper Ethernet past 100 m.

**Between buildings, use fibre** *(§10.5)* — And the reason is **earthing and
lightning**, not bandwidth. Separate buildings have separate earths at different
potentials; copper conducts the difference and conducts induced surges directly into
equipment at both ends. Fibre is a dielectric. This is a safety argument that
overrides cost.

**The power question decides more than it is credited for** *(§10.5)* — Copper
delivers up to 90 W; fibre delivers none. A camera or access point at the end of
copper needs no electrician; on fibre it needs an outlet, a supply and a converter,
all of which fail eventually.

**State the condition under which the answer changes** *(§10.5)* — The sentence that
makes a design document useful to whoever inherits it, and what Chapter 72 §72.4
marks.
