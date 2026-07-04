# Section 28.1: Fundamental Physical Limits

This section asks what physics permits, independent of engineering — the floors that no clever design, no future foundry, and no amount of capital can breach. Establishing them precisely is the first discipline of an honest outlook, because the field's rhetoric routinely invokes "fundamental limits" to dignify claims that in truth collide with far nearer engineering walls. The two must be told apart.

Three floors bound classical photonic computing, one from each of the great physical theories of information:

- **Thermodynamic.** Landauer's principle sets a minimum energy $k_B T\ln 2$ for each irreversible bit erasure (Section 28.1.1).
- **Information-theoretic.** Shannon's channel capacity, applied to a shot-noise-limited optical signal, makes each additional bit of analog precision cost geometrically more energy (Section 28.1.2).
- **Quantum.** The Poissonian statistics of photon arrival impose the standard quantum limit, an irreducible noise floor even for perfect classical light (Section 28.1.3).

The section's central message is deflating and essential in equal measure: for classical photonic computing, these fundamental floors are *not* today's binding constraints. A real photonic multiply-accumulate dissipates on the order of $10^{-13}$–$10^{-12}$ J, some eight orders of magnitude above the Landauer bound; the machine is stopped long before thermodynamics by optical loss, data-converter energy, and laser efficiency — the engineering limits of Section 28.2. The fundamental limits nonetheless earn their place here for two reasons. First, they fix the ultimate ceiling — the best any photonic computer could asymptotically become — and a mature field should know its own horizon. Second, one of the three is not a distant ceiling at all. The precision–energy trade-off is an operative constraint, felt in every analog optical processor built to date: it is why photonic machine learning targets low numerical precision, and why sub-photon-per-operation demonstrations are possible only at correspondingly low effective precision. The quantum noise floor beneath it is the point at which the story of classical photonic computing ends and the story of quantum photonics (Unit VII) begins.

The three subsections take the floors in turn:

- **28.1.1** — Energy per Operation and the Landauer Limit
- **28.1.2** — The Precision–Energy Trade-off
- **28.1.3** — The Quantum Noise Floor
