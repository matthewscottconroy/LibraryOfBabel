# Photonic Computing: From First Principles to the Frontiers of Light-Speed Information Processing

## A Comprehensive Textbook

---

### About This Book

This book is an invitation to think about computation in an entirely new way.

For the better part of a century, computing has meant electrons. Electrons shuttling through silicon, flipping transistors, representing bits as voltage levels, accumulating in capacitors, rushing along copper traces etched into circuit boards. The story of the digital age is, at its deepest level, the story of learning to control electrons with extraordinary precision — and it is one of the most successful engineering programs in human history.

But electrons have limitations. They have mass. They carry charge. They interact with each other and with the lattice of atoms through which they move, dissipating energy as heat at every step. As transistors have shrunk to the scale of a few atoms, those limitations have become not engineering problems to be solved but physical laws to be reckoned with. The era of easy exponential scaling — Moore's Law — is ending.

Into this moment arrives the photon.

The photon is massless. It carries no electric charge, so it produces no resistive heating as it travels. It moves at the fundamental speed limit of the universe. Two photon beams can cross each other in free space without the slightest interaction — no crosstalk, no signal degradation. And crucially, a single optical fiber the thickness of a human hair can carry more information per second than all the copper cables that crossed the Atlantic Ocean in the 20th century combined.

These properties make light a profoundly attractive medium for computation. But the path from "light is fast" to "light computes better than silicon" is neither obvious nor guaranteed. It requires understanding, at a deep and rigorous level, what computation actually demands from its physical substrate, what physics enables and what it constrains, and where the genuine engineering challenges lie.

This book makes that journey. It starts at the very beginning — with Maxwell's equations, with the nature of electromagnetic waves, with the quantum mechanical structure of light itself — and it travels all the way to the current research frontier: to photonic chips performing matrix multiplication for artificial intelligence, to diffractive optical networks built from plastic, to quantum photonic processors that exploit entanglement to solve problems no classical computer can touch.

---

### Who This Book Is For

This book is written for anyone who wants to *understand* photonic computing — not merely to know its vocabulary, but to grasp why its physics permits what it permits, why its engineering makes the choices it makes, and what questions remain genuinely open.

The minimum background assumed is:
- **Calculus** through multivariable (partial derivatives, line and surface integrals)
- **Linear algebra** (vectors, matrices, eigenvalues, unitary transformations)
- **Introductory physics** (mechanics and basic electricity and magnetism)
- **Programming** in Python at a basic level

Everything beyond this is developed in the text. The mathematical tools are introduced when they are needed, always motivated by the physical question they answer, never as abstract machinery deployed without explanation.

The book is intentionally suitable for:
- Advanced undergraduates in physics, electrical engineering, or computer science
- Graduate students entering the field from adjacent areas
- Researchers from one subfield (say, machine learning or quantum information) who need to understand the photonic hardware landscape
- Engineers at photonic computing companies who want deeper theoretical grounding
- Anyone intellectually curious enough to want to understand one of the most interesting intersections of physics and technology in the early 21st century

---

### How to Use This Book

The book is organized into ten units. The units are not independent — each builds on what came before. Unit I (Chapters 1–3) establishes classical electromagnetism and wave optics from first principles. This foundation is not decorative; it is the literal bedrock on which every later topic stands. A reader tempted to skip it because they "already know Maxwell's equations" is encouraged to at least read the introductions to the sections, which frame the material in ways specific to the needs of photonic computing.

Within each chapter:

- The **chapter introduction** situates the topic in the larger narrative, explaining why it matters and what questions it answers.
- Each **section** opens with an orientation that explains the scope and structure of what follows.
- Each **subsection** contains the principal mathematical and conceptual content, developed fully from stated assumptions.
- **Exercises** at the end of each chapter include three types: *Mathematical* (derivation and calculation), *Conceptual* (understanding and reasoning), and *Lab/Experimental* (hands-on investigation, whether in simulation or physical experiment).
- **Important Concepts** summarizes the key ideas in compressed form — useful for review, not for first encounter.
- **Important Researchers** briefly profiles the people behind the discoveries.
- **Further Reading and References** provides a curated path for going deeper.

### A Note on Citations

This book treats its citations seriously. In a field as young and rapidly evolving as photonic computing, it is easy for claims to outrun their evidence. Wherever a claim is specific to photonic computing or quantum photonics — rather than classical physics or well-established engineering — we provide a citation to the primary research from which it derives. Readers are encouraged to view these citations not as formalities but as *invitations*: the papers are the primary sources of the field, and reading them is irreplaceable.

In the mathematical sections, derivations are given in full. No formula is deployed without justification. Where a result is presented without derivation (because it would require more mathematical machinery than this book develops), this is explicitly stated and a reference is given.

---

### The Philosophy of This Book

This book takes a particular philosophical stance: that understanding the *reasons* behind engineering choices is more valuable than memorizing the choices themselves. The photonic computing community has made many decisions — about encoding information in phase vs. amplitude, about coherent vs. incoherent optical computation, about using MZI meshes vs. ring resonator weight banks, about pursuing fault-tolerant quantum photonic computers at cryogenic temperatures — and each of these decisions has physical, mathematical, and engineering justifications that are often not explained in the primary papers.

We explain them here. When the community has made a choice that is genuinely contested or where the jury is still out, we say so.

We also try, in the foundational chapters, to engage with what is philosophically prior: *What is computation?* *What physical properties does a computing substrate need?* *What does it mean to "perform matrix multiplication" with light?* These are not frivolous questions. Confusion about them is responsible for much of the hype and much of the skepticism that surrounds photonic computing, and clarity about them is the precondition for evaluating the field honestly.

---

### The Structure at a Glance

| Unit | Chapters | Theme |
|------|----------|-------|
| I | 1–3 | The nature of light: classical electromagnetism, wave optics, light-matter interaction |
| II | 4–5 | The laser and photodetectors: the engines of photonic computing |
| III | 6–8 | Guided-wave photonics, silicon photonics, photonic crystals |
| IV | 9–10 | Information theory and optical communications |
| V | 11–14 | Classical photonic computing: from Fourier optics to diffractive neural networks |
| VI | 15–16 | Neuromorphic photonics: spiking networks, optical synapses |
| VII | 17–22 | Quantum photonics and quantum computing with light |
| VIII | 23–24 | Fabrication and simulation tools |
| IX | 25 | Benchmarking and the computing landscape |
| X | 26–28 | Industry, research groups, and the frontiers |

---

### Conventions and Notation

Throughout this book:
- SI units are used exclusively.
- Complex exponentials use the engineering convention $e^{-i\omega t}$ for harmonic time dependence (positive frequency = positive wavevector). When the physics convention $e^{+i\omega t}$ is used (as in some quantum optics texts), it is explicitly noted.
- Vectors are written in **bold**: **E**, **B**, **k**.
- Operators (in quantum mechanics) are written with a hat: $\hat{H}$, $\hat{a}$, $\hat{a}^\dagger$.
- The Dirac notation $\langle \phi | \psi \rangle$ is used for quantum inner products.
- $\hbar = h/2\pi$ is the reduced Planck constant.
- $c = 2.998 \times 10^8$ m/s is the speed of light in vacuum.
- $\varepsilon_0 = 8.854 \times 10^{-12}$ F/m is the permittivity of free space.
- $\mu_0 = 4\pi \times 10^{-7}$ H/m is the permeability of free space.
- Natural logarithms are written $\ln$; base-2 logarithms (information-theoretic) are written $\log_2$.

---

*Let us begin.*
