# Chapter 21: Key Researchers

## Herbert Jaeger (Constructor University, Bremen)

Herbert Jaeger pioneered the application of echo state networks to temporal tasks including early speech-related benchmarks. His 2001 and 2002 technical reports established ESNs and introduced the memory capacity measure. Jaeger's group was among the first to demonstrate that RC networks could handle the temporal variability of speech without explicit alignment, using the reservoir's fading memory in place of HMM state alignment.

**Representative works**: [Jaeger2001], [Jaeger2002MC], [JaegerHaas2004]

## Benjamin Schrauwen (Ghent University)

Benjamin Schrauwen was a central figure in the Ghent University reservoir computing group and contributed substantially to speech applications. His work compared RC to HMMs on phoneme recognition tasks, established benchmark protocols, and developed the ReservoirPy software library (initially as an internal tool). Schrauwen's NIPS 2007 paper [SchrauwenEtAl2007] is one of the most-cited RC speech papers.

**Representative works**: [SchrauwenEtAl2007], [VerstraetEtAl2006], [SchrauwenEtAl2008]

## David Verstraeten (Ghent University)

David Verstraeten conducted extensive benchmarking of ESNs on speech and audio tasks, including the first systematic comparison of RC with HMMs on the TIMIT benchmark. His PhD thesis [Verstraeten2009] remains one of the most comprehensive empirical studies of RC for speech processing.

**Representative works**: [VerstraetEtAl2006], [VerstraetEtAl2007], [Verstraeten2009]

## Jochen Triesch (Frankfurt Institute for Advanced Studies)

Jochen Triesch developed intrinsic plasticity rules for reservoir neurons [TrieschEtAl2005] that adapt individual neurons' gain and bias to achieve a target output distribution. These rules improve reservoir performance on speech tasks by optimizing the operating point of each neuron for the particular input statistics of speech features.

**Representative works**: [TrieschEtAl2005], [TrieschEtAl2007]

## Jürgen Schmidhuber (IDSIA / King Abdullah University)

While best known for LSTMs, Schmidhuber's work on sequence learning problems (particularly the design of gating mechanisms for long-term dependencies) defined the competitive landscape against which RC speech systems are measured. His group's early LSTM work on speech [HochreiterSchmidhuber1997] remains the principal competitor model.

**Representative works**: [HochreiterSchmidhuber1997], [SchmidhuberEtAl2007]

## Alain de Cheveigné (CNRS / ENS Paris)

Alain de Cheveigné's work on auditory modeling and pitch perception provides the biological context for reservoir-based speech processing. His models of auditory cortex dynamics as a population of damped oscillators bear a strong resemblance to the leaky-integrator reservoir model and provide biological grounding for the architectural choices of speech-processing ESNs.

**Representative works**: [deCheveigneEtAl2019], [deCheveigneEtAl2021]

## Florian Krebs / Mantas Lukoševičius Group

The ReservoirPy library (Appendix D) and its speech processing extensions were developed by multiple contributors in the broader RC community. Lukoševičius's practical guide [Lukosevičius2012] includes speech processing examples that have served as starting points for many subsequent implementations.

**Representative works**: [Lukosevičius2012], [LukoseviciusJaeger2009]
