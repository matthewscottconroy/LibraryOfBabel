# Chapter 12: Matrix-Vector Multiplication with Light

## The Central Operation of Machine Learning

If you had to identify the single mathematical operation that modern artificial intelligence most relies on, it would be matrix-vector multiplication. A feedforward neural network layer is, at its mathematical core, a linear map: $\mathbf{y} = W\mathbf{x}$, where $W$ is a weight matrix and $\mathbf{x}$ is an input vector. A transformer's attention mechanism computes $\text{softmax}(QK^T/\sqrt{d_k})V$ — three matrix multiplications. A convolutional layer is (in its frequency-domain form) a multiplication of spectral vectors. The training process that finds the weights is itself gradient descent on a loss function whose gradients are computed by backpropagating through — matrix multiplications.

Modern AI accelerators (GPU, TPU, Cerebras, Graphcore) are therefore, at their core, matrix multiplication engines. The H100 GPU delivers 3958 TFLOPS of FP16 matrix multiply performance per chip. NVIDIA's revenue in 2024 exceeded $60 billion, almost entirely because training and running large language models requires performing astronomical numbers of matrix multiplications.

The question this chapter addresses: can optical systems perform matrix-vector multiplication more efficiently than electronics? Under specific conditions, the answer is yes — and understanding precisely when and why requires the physics developed in Units I–III and the mathematical framework of this chapter.

---

## Three Fundamental Questions

**Why matrix multiplication?** Not because it is the most fundamental operation in computer science, but because it is simultaneously (a) the computational bottleneck of modern AI, (b) a linear operation that optics can perform naturally, and (c) representable as the superposition of electromagnetic fields — which is what a coherent optical system does by default. We examine the complexity-theoretic argument for why the optical approach to matrix multiplication is physically plausible.

**How does an MZI mesh perform matrix multiplication?** The Mach-Zehnder interferometer (Chapter 7) is a unitary $2 \times 2$ operation: it takes two input optical amplitudes and produces two output amplitudes, with the operation parameterized by two phase angles. A mesh of $N(N-1)/2$ MZIs can implement any $N \times N$ unitary matrix (Reck decomposition, 1994; Clements decomposition, 2016). Non-unitary matrices are implemented by adding diagonal amplitude attenuators. Together, these operations enable any matrix-vector product on any complex vector of dimension $N$.

**What are the practical limits?** The MZI mesh is elegant in theory. In practice, it faces: thermal drift of phase shifters, finite extinction ratio of beam splitters, loss accumulation across the mesh depth, limited precision of phase setting (ENOB of the analog drive circuits), and the need for accurate calibration. We examine each limit quantitatively and compare the demonstrated performance of real photonic matrix processors to the theoretical ideal.

---

## Four Sections

**Section 12.1: The Case for Optical Linear Algebra** establishes why matrix-vector multiplication is the right target for optical processors, derives the scaling argument, and distinguishes the coherent (complex-valued) from incoherent (non-negative) approaches.

**Section 12.2: The MZI Mesh** derives the transfer matrix of a single MZI, the Reck and Clements decompositions of unitary matrices into MZI networks, and the programming procedure.

**Section 12.3: SVD Implementation and the Shen 2017 Experiment** shows how the singular value decomposition of a general (non-unitary) matrix is implemented, analyzes the 2017 MIT experiment that first demonstrated a photonic neural network on chip, and discusses commercial architectures (Lightmatter, Lightelligence, Luminous).

**Section 12.4: Wavelength-Multiplexed Architectures** covers the incoherent WDM approach using ring weight banks and broadcast-and-weight networks — a different physical implementation of matrix multiplication that trades phase sensitivity for wavelength-division parallelism.

---

## A Note on Precision

Every matrix-vector multiplication requires that the weights be encoded with sufficient precision. For a neural network inference task, 8-bit weights (256 levels) are typically adequate. For training, 16-bit precision is common. For scientific computing (solving linear systems), 64-bit (double precision) is often required.

An analog optical system encoding weights as phase angles or optical intensities has a precision limited by the signal-to-noise ratio of the optical field — roughly equivalent to the ENOB (effective number of bits) established in Chapter 9. Current photonic matrix processors achieve ENOB ≈ 5–8 bits, equivalent to ~6 bits of weight precision. This is adequate for neural network inference but not for scientific computing.

Understanding this precision-bandwidth tradeoff — and the physics that sets the ENOB ceiling — is one of the central lessons of this chapter.

---

## References

[1] Shen, Y., et al. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446. [The experiment that launched the current photonic deep learning research wave.]

[2] Reck, M., Zeilinger, A., Bernstein, H.J., & Bertani, P. (1994). "Experimental realization of any discrete unitary operator." *Physical Review Letters*, 73, 58–61. [The Reck decomposition; first proved that any unitary matrix can be decomposed into a product of $2 \times 2$ unitary matrices.]

[3] Clements, W.R., et al. (2016). "Optimal design for universal multiport interferometers." *Optica*, 3(12), 1460–1465. [The Clements decomposition; a more hardware-efficient alternative to Reck that is used in most modern photonic linear algebra implementations.]
