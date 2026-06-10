# Chapter 16 — Further Reading and References

---

## Essential References

### [Appeltant2011] — The Founding Paper of Single-Node RC

**Appeltant, L., Soriano, M.C., Van der Sande, G., Danckaert, J., Massar, S., Dambre, J., Schrauwen, B., Mirasso, C.R., & Fischer, I. (2011). Information processing using a single dynamical node as complex systems. *Nature Communications*, 2, 468.**

This paper introduced time-multiplexing for physical reservoir computing and demonstrated the first experimental optoelectronic single-node reservoir. The core result — that a single node with delay feedback emulates a large reservoir — opened the entire field of practical physical RC. The supplementary materials give detailed mathematical analysis of the virtual node construction.

### [Nakajima2021] — The Reference Book

**Nakajima, K. & Fischer, I. (eds.) (2021). *Reservoir Computing: Theory, Physical Implementations, and Applications*. Springer Nature.**

The most comprehensive reference for physical reservoir computing. Covers theory (Part I), photonic/optoelectronic implementations (Part II), unconventional substrates (Part III), and applications (Part IV). Essential for any serious study of physical RC. Every chapter is written by leading practitioners in its specific area.

### [Tanaka2019] — The Review

**Tanaka, G., Yamane, T., Héroux, J.B., Nakane, R., Kanazawa, N., Takeda, S., Numata, H., Nakano, D., & Hirose, A. (2019). Recent advances in physical reservoir computing: A review. *Neural Networks*, 115, 100–123.**

A comprehensive review covering photonic, spintronic, memristive, soft-robotic, and biological physical reservoirs. Includes the four-conditions framework and a systematic comparison of implementations.

---

## Time-Multiplexing

### [Larger2012]

**Larger, L., Soriano, M.C., Brunner, D., Appeltant, L., Gutiérrez, J.M., Pesquera, L., Mirasso, C.R., & Fischer, I. (2012). Photonic information processing beyond Turing: An optoelectronic implementation of reservoir computing. *Optics Express*, 20(3), 3241–3249.**

The first all-optoelectronic implementation demonstrating gigahertz-rate processing with the time-multiplexed architecture.

### [Brunner2013]

**Brunner, D., Soriano, M.C., Mirasso, C.R., & Fischer, I. (2013). Parallel photonic information processing at gigabyte per second data rates using transient states. *Nature Communications*, 4, 1364.**

Spatial (rather than temporal) multiplexing: a spatial light modulator creates many parallel nodes, achieving processing rates of 1 Gbyte/s.

---

## Benchmarks

### [Jaeger2004]

**Jaeger, H. & Haas, H. (2004). Harnessing nonlinearity: Predicting chaotic systems and saving energy in wireless communication. *Science*, 304(5667), 78–80.**

Introduced the channel equalization benchmark and demonstrated ESN performance far exceeding state-of-the-art at the time. The paper that put reservoir computing on the map for the broader machine learning community.

### [Rodan2011]

**Rodan, A. & Tino, P. (2011). Minimum complexity echo state network. *IEEE Transactions on Neural Networks*, 22(1), 131–144.**

Showed that simple ring topologies (equivalent to the Appeltant time-multiplexed system) can match random ESNs on standard benchmarks. Provides the theoretical bridge between physical single-node systems and the digital ESN literature.

---

## Diverse Physical Implementations

### [Torrejon2017]

**Torrejon, J. et al. (2017). Neuromorphic computing with nanoscale spintronic oscillators. *Nature*, 547(7664), 428–431.**

Spintronic oscillators as reservoir nodes. Demonstrates hardware neural network inference at nanoscale, with applications to vowel recognition.

### [Vandoorne2014]

**Vandoorne, K. et al. (2014). Experimental demonstration of reservoir computing on a silicon photonic chip. *Nature Communications*, 5, 3541.**

On-chip silicon photonic reservoir (covered in detail in Chapter 17).

### [Legenstein2007]

**Legenstein, R. & Maass, W. (2007). What makes a dynamical system computationally powerful? In *New Directions in Statistical Signal Processing*. MIT Press. 127–154.**

A theoretical analysis of the computational conditions for physical reservoir computing, formalized via the separation property and approximation property.
