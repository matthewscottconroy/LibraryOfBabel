# Mass Analyzers

Ask a proteomicist why they chose an Orbitrap over a triple quadrupole for their experiment, and you will learn more about mass spectrometry in five minutes than you would from reading a textbook chapter. The answer is never "because it is better" — it is always "because of what I needed to measure." The mass analyzer is the heart of the spectrometer, and the choice of analyzer is ultimately a choice about what kinds of biological questions you can answer.

The core challenge is this: ions of different m/z must be physically separated so that they can be counted individually. There is no single best way to do this. Different physical principles produce different trade-offs, and the history of mass spectrometry is largely a history of clever inventors exploiting different aspects of ion physics — electric fields, magnetic fields, oscillation frequencies, flight times — to push resolution, speed, and sensitivity in different directions.

## Quadrupole (Q)

A **quadrupole** consists of four parallel metal rods arranged in a square. Opposing rod pairs carry superimposed DC and RF (radiofrequency) voltages. Only ions within a narrow m/z window have stable oscillating trajectories and pass through to the detector; all others spiral outward and are lost. By scanning the voltages, the quadrupole scans across m/z values.

Key properties:
- **Unit resolution**: Can resolve ions differing by 1 Da.
- **Fast scan rate**: Can scan m/z 50–1200 in milliseconds.
- **Selective transmission**: Excellent for targeted experiments where specific m/z values are monitored.
- Typical standalone use: Ion source → Q → detector (single quadrupole for simple MS); or as a filter in triple-quadrupole instruments.

The quadrupole's genius is its simplicity and speed. It has no moving parts, it tolerates modest vacuum requirements, and it can switch from monitoring one m/z value to another in microseconds. For targeted experiments — measuring a defined set of known peptides or metabolites at maximum sensitivity — nothing beats a quadrupole-based system.

## Time-of-Flight (TOF)

In a **TOF** analyzer, all ions are accelerated by the same electric field and then travel through a field-free drift tube. Since all ions start with the same kinetic energy ($KE = qV = \frac{1}{2}mv^2$), heavier ions travel more slowly:

$$t = L\sqrt{\frac{m}{2qV}}$$

where $L$ is the flight path length, $V$ is the accelerating voltage. By measuring the time of arrival at the detector, m/z is calculated. **Reflectron TOF** uses an electrostatic mirror to increase effective path length and correct for small initial kinetic energy differences, improving resolution.

Key properties:
- **Resolution**: ~10,000–30,000 (reflectron TOF)
- **Mass accuracy**: ~5–20 ppm (without lock mass); better with internal calibrants
- **Wide m/z range**: No scanning needed — the entire spectrum is recorded in each acquisition pulse
- **Fast acquisition**: Spectra acquired at kHz rates
- Common combination: MALDI-TOF (bacterial identification), Q-TOF (ESI-based accurate mass)

The TOF analyzer has a beautiful elegance: it exploits one of the most basic laws of mechanics — that heavier objects move more slowly under the same force — and turns it into a measuring device. The longer the flight tube, the better the mass resolution, which is why some research TOF instruments are physically enormous.

## Orbitrap

The **Orbitrap** (invented by Alexander Makarov; commercialized by Thermo Scientific as the Fusion, Lumos, Eclipse, and Astral series) is based on electrostatic ion trapping. Ions are injected tangentially into a spindle-shaped inner electrode surrounded by an outer barrel electrode. Ions orbit around the inner electrode while simultaneously oscillating along the spindle axis — the frequency of axial oscillation depends on m/z:

$$\omega = \sqrt{\frac{q \cdot k}{m}}$$

where $k$ is a constant related to the electric field geometry. By detecting the image current induced by oscillating ions and applying a **Fourier transform**, the frequency spectrum is converted to a mass spectrum (analogous to FT-NMR). 

Key properties:
- **Ultra-high resolution**: >100,000–500,000 FWHM
- **Mass accuracy**: <2–5 ppm routinely; <1 ppm with calibration
- **Full scan sensitivity**: Records all ions simultaneously (no scanning)
- **Limitation**: Requires long acquisition times for high resolution (transient duration determines resolution); slower than quadrupole for targeted work
- Common combination: Q-Orbitrap (Orbitrap Exploris, Q Exactive) — quadrupole for precursor selection + Orbitrap for high-resolution detection

The Orbitrap is, by some measure, the most important development in biological mass spectrometry since electrospray ionization. It gave proteomicists and metabolomicists the combination they had always wanted — high resolution and high mass accuracy — in a benchtop instrument compatible with routine LC-MS workflows. Before the Orbitrap, achieving this kind of resolution required a Fourier transform ion cyclotron resonance (FT-ICR) instrument that cost millions of dollars, required a superconducting magnet, and needed a dedicated specialist to run it. The Orbitrap democratized high-resolution MS.

## Ion Trap (LTQ)

The **ion trap** (linear ion trap, LTQ) stores ions in a 3D or 2D RF field and ejects them sequentially by scanning the RF voltage. Ion traps can perform **MS^n** — multiple rounds of isolation and fragmentation of fragment ions — enabling structural elucidation. They have lower resolution than Orbitrap and TOF but are excellent at capturing and isolating ions for fragmentation.

## Triple Quadrupole (QqQ) for Targeted Quantification

The **triple quadrupole** consists of three quadrupoles in series: Q1 (precursor selection) → q2 (collision cell for fragmentation) → Q3 (product ion selection). By monitoring a specific precursor → product ion transition, this configuration achieves maximum sensitivity for targeted quantification — the **Selected Reaction Monitoring (SRM)** or **Multiple Reaction Monitoring (MRM)** mode.

SRM/MRM is the gold standard for quantitative proteomics and clinical metabolomics, offering linear dynamic range of 4–5 orders of magnitude, sub-femtomole sensitivity, and excellent reproducibility.

It turns out that the path through Q1 → q2 → Q3 is extraordinarily selective. You are not just filtering by mass; you are filtering by mass, fragmenting, and filtering again by the mass of a specific fragment. The probability that two different molecules share the same precursor mass, fragment the same way, and produce a fragment at the same product m/z is vanishingly small. This is why SRM/MRM is trusted for clinical measurements — it is essentially a molecular fingerprint verification rather than a mass measurement.

## Comparison Table

| Analyzer | Resolution | Mass Accuracy | Scan Speed | MS^n | Best Use |
|---|---|---|---|---|---|
| Quadrupole | Unit (~1000) | 100–500 ppm | Very fast | No | SRM/MRM targeted |
| TOF | 10,000–30,000 | 5–20 ppm | Very fast | Limited | Accurate mass, MALDI |
| Orbitrap | >100,000 | <5 ppm | Slow | Via trap | Untargeted, DIA |
| Ion Trap | 1,000–5,000 | 100–300 ppm | Fast | Yes (MS^n) | Structural |
| Triple Quad (QqQ) | Unit | 100–500 ppm | Very fast | 2 stages | Targeted quantification |

## Why This Matters

The choice of mass analyzer — or hybrid combination — is the primary determinant of whether an experiment can identify unknowns (requires high resolution/accuracy), quantify targets with high sensitivity (requires QqQ or Orbitrap), or characterize structural details (requires MS^n); matching the analyzer to the biological question is a core competency of proteomics and metabolomics experimental design. When you encounter a proteomics dataset labeled "Q Exactive" or a metabolomics study performed on an "Agilent triple quad," the instrument name is already telling you something about the design philosophy of the experiment and the likely depth, accuracy, and quantitative range of the resulting data. Learn to read those instrument names as biological clues.
