# Section 9.4: Forward Error Correction

Every modern optical communication system operates near the Shannon limit not by encoding information perfectly, but by deliberately adding redundancy in a structured way that allows the receiver to detect and correct errors. This is the province of **forward error correction** (FEC): channel coding schemes that add redundant bits to the transmitted data, allowing the receiver to recover the original information even when noise has corrupted some fraction of the received bits.

FEC is what transforms the theoretical promise of Shannon's capacity theorem into practical engineering reality. Shannon's theorem guarantees that reliable communication at any rate below capacity is *possible*; FEC codes are the construction that makes it *practical*.

For photonic computing, FEC has a dual relevance:

1. **System context**: The optical channels feeding and collecting data from photonic compute engines use FEC codes to ensure reliable data transfer. Understanding FEC overhead is necessary for system-level capacity calculations.

2. **Computing application**: FEC decoding is itself a computationally intensive operation — particularly for high-performance LDPC and polar codes. This has been proposed as one application domain for photonic signal processing.

**Subsection 9.4.1 — FEC Principles**: The rate-distance tradeoff, Hamming bound, hard vs. soft decision, and net coding gain.

**Subsection 9.4.2 — LDPC and Polar Codes**: The two dominant modern FEC constructions and their decoding complexity.

**Subsection 9.4.3 — FEC for Photonic Computing**: The system implications and the possibility of photonic FEC decoding.
