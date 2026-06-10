# Chapter 5: Key Researchers

---

## Herbert Jaeger

**Affiliation:** Jacobs University Bremen (later rebranded Constructor University Bremen), Germany. Previously at GMD — German National Research Center for Information Technology, Sankt Augustin.

**Role in reservoir computing:** Jaeger is the inventor of echo state networks and one of the two founders of the reservoir computing paradigm (alongside Wolfgang Maass, who independently developed Liquid State Machines at roughly the same time). His 2001 GMD Technical Report — "The 'echo state' approach to analysing and training recurrent neural networks" — introduced the ESN, defined the echo state property, and demonstrated that simple random recurrent networks could perform temporal tasks far beyond what had been thought achievable without gradient-based training. This report, though a technical memo rather than a journal article, became one of the most influential documents in the history of machine learning, eventually accumulating thousands of citations.

**The 2001 GMD Technical Report [Jaeger2001]:** This is the foundational document. It introduces: the ESN architecture; the echo state property (definition and sufficient conditions); the offline training procedure using pseudoinverse / ridge regression; the washout period; and empirical demonstrations on chaotic time series prediction, including the Mackey-Glass system. The report is notable for its combination of mathematical rigor and practical insight — Jaeger does not merely describe the method but explains *why* it works and *when* it will fail.

**The 2001 Science paper [Jaeger2004]:** A compact version of the ESN results published in *Science* demonstrating 2400-fold improvement over previous methods on Mackey-Glass prediction and establishing ESNs as a serious alternative to LSTM for temporal tasks.

**Memory capacity [Jaeger2002memory]:** The formal analysis of the information-theoretic memory capacity of linear ESNs, establishing the bound $MC \leq N$ and showing it can be achieved with equality for certain network configurations. This paper planted the seed for a decade of theoretical work on reservoir computing capacity.

**The 2007 Science paper on pattern generation [Jaeger2007]:** Demonstrated that ESNs can learn to generate complex periodic patterns (including musical sequences) by online learning with a modified RLS algorithm. This was an important step toward using reservoirs for motor control and pattern generation, not just prediction.

**Conceptors [Jaeger2014]:** In the 2010s, Jaeger developed the theory of "conceptors" — matrix operators that can be used to store, combine, and morph patterns in a reservoir. Conceptors are essentially constrained principal projections of reservoir states during specific patterns. They provide a new perspective on reservoir memory and opened up connections to associative memory and cognitive science. The key monograph "Controlling Recurrent Neural Networks by Conceptors" (2014) is a major theoretical contribution.

**Teaching and community:** Jaeger has been a consistent and generous mentor to the reservoir computing community, writing accessible tutorials (see further reading), organizing workshops, and maintaining open-source code.

**Recommended papers:**
- [Jaeger2001] GMD Technical Report: the foundational document
- [Jaeger2002tutorial] "A tutorial on training recurrent neural networks..." (GMD Report 159): the essential how-to guide
- [Jaeger2004] "Harnessing Nonlinearity" (*Science*, 2004): landmark empirical demonstration
- [Jaeger2002memory] "Short term memory in echo state networks" (GMD Report 152): memory capacity theory
- [LukoAndJaeger2009] "Reservoir Computing Approaches to Recurrent Neural Network Training" (with Lukoševičius): comprehensive review

---

## Mantas Lukoševičius

**Affiliation:** Jacobs University Bremen; also worked at Technische Universität Berlin and other European institutions.

**Role in reservoir computing:** Lukoševičius is the author of the most widely read practical guide to ESN training [Lukoševičius2012], a landmark tutorial that distilled years of empirical experience into systematic, actionable advice. He joined Jaeger's group as a doctoral student and postdoc, and his contributions bridged the gap between the theoretical elegance of the ESN framework and the messy reality of making it work on real tasks.

**The 2012 Practical Guide [Lukoševičius2012]:** "A Practical Guide to Applying Echo State Networks" in *Lecture Notes in Computer Science* (Neural Networks: Tricks of the Trade, 2nd edition) is the go-to reference for practitioners. It covers: reservoir construction (weight initialization, scaling, sparsity); the effect of each hyperparameter (especially $\rho$, $\alpha$, input scaling, and $\lambda$); common failure modes and their diagnoses; output feedback and teacher forcing; and empirical heuristics accumulated from extensive experimentation. Every serious ESN practitioner has read this paper.

**The 2009 review [LukoAndJaeger2009]:** Co-authored with Jaeger, "Reservoir Computing Approaches to Recurrent Neural Network Training" in *Computer Science Review* is the comprehensive theoretical and historical review of the field as of 2009. It situates ESNs and LSMs within the broader landscape of recurrent network training, covers the theoretical foundations (including the connection to kernel methods and the fading memory framework), and provides an authoritative account of what was known about reservoir computing at the time.

**Contributions to training methods:** Lukoševičius analyzed the online training procedures (including RLS-based methods for online ESN learning), the effect of output feedback, and the behavior of ESNs on classification tasks. His careful experimental methodology and well-documented code became a model for the field.

**Recommended papers:**
- [Lukoševičius2012] "A Practical Guide to Applying Echo State Networks": essential reading
- [LukoAndJaeger2009] "Reservoir Computing Approaches to Recurrent Neural Network Training": comprehensive review

---

## Benjamin Schrauwen

**Affiliation:** Ghent University, Belgium. Department of Electronics and Information Systems (ELIS), research group "reservoir computing."

**Role in reservoir computing:** Schrauwen and his colleagues at Ghent — particularly Jan Dambre, David Verstraeten, and Dirk Stroobandt — were among the first to systematically study the unifying principles of reservoir computing across different physical substrates (silicon, optical, mechanical), contributing both to the theoretical foundations and to the hardware realization of reservoir computing.

**Unification of ESN and LSM [Verstraeten2007]:** The paper "An experimental unification of reservoir computing methods" (with Verstraeten, Schrauwen, and d'Haene) demonstrated empirically that ESNs and LSMs perform comparably on standard benchmarks when properly tuned, supporting the view that the reservoir computing paradigm — not the specific biological details — is what matters for computation. This paper helped establish the unified RC framework.

**Hardware and physical reservoir computing:** Schrauwen's group was instrumental in the experimental realization of reservoir computing in physical systems: photonic reservoirs, delay-line reservoirs, and analog electronic reservoirs. This work showed that reservoir computing is not merely a computational abstraction but a practical design principle for physical computing systems. The seminal paper on delay-coupled photonic reservoirs [Appeltant2011], while primarily by Appeltant, Soriano, et al., grew from the intellectual tradition established by Schrauwen's group.

**Memory-nonlinearity tradeoff [Dambre2012]:** The paper "Information Processing Capacity of Dynamical Systems" (Dambre, Verstraeten, Schrauwen, and Massar) provided a theoretical framework for quantifying the total computational capacity of a dynamical system, decomposing it into memory and nonlinear components and showing the fundamental tradeoff between them. This is directly relevant to Section 3.3's discussion of why $\rho \approx 1$ is optimal.

**Recommended papers:**
- [Verstraeten2007] "An experimental unification of reservoir computing methods"
- [Dambre2012] "Information Processing Capacity of Dynamical Systems"
- [Schrauwen2007] "An overview of reservoir computing: theory, applications, and implementations" (survey paper, covers hardware)
