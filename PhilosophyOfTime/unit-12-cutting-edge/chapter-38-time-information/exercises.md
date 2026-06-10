# Chapter 38 Exercises: Time and Information

---

## Comprehension and Analysis

**1. The Shannon-Boltzmann Connection**
Shannon entropy and thermodynamic entropy are formally identical. Explain what this means precisely — what is the mathematical relationship? Then articulate why this formal identity is philosophically puzzling. Does the identity show that information and physical entropy are the *same thing*, or does it merely show that the same mathematical structure can be applied in two different domains? What would each answer imply?

**2. Maxwell's Demon and Landauer's Principle**
Trace the full resolution of Maxwell's Demon through Szilard (1929) and Landauer (1961). Why does the demon's sorting of molecules not violate the second law? Be precise about which step in the demon's operation generates entropy, and why. What does this tell us about the relationship between information erasure and thermodynamic irreversibility? Is Landauer's principle a conceptual claim, an empirical claim, or both?

**3. The Black Hole Information Paradox**
State the black hole information paradox carefully, making explicit what premises generate the contradiction. The argument depends on: (a) Hawking's calculation that radiation is thermal; (b) the unitarity of quantum mechanics; and (c) the assumption that information cannot escape the black hole interior. Which of these premises do different proposed resolutions challenge? Which seems most defensible to you, and why?

**4. The Page Curve**
Explain what the Page curve is and why it serves as the benchmark for resolutions of the information paradox. If the radiation from an evaporating black hole follows the Page curve, what does this imply about the relationship between the early and late radiation? What is the "Page time," and what happens to the entanglement entropy of the radiation before and after the Page time?

**5. The Island Formula**
The island formula proposes that the entropy of Hawking radiation should be calculated by including contributions from "islands" inside the black hole. What is philosophically surprising about this prescription? What does it imply about the relationship between the interior and exterior of a black hole? How does the island formula relate to the holographic principle and the Ryu-Takayanagi formula?

**6. Landauer's Principle and Reversible Computation**
Landauer's principle says that erasure has a thermodynamic cost, but logically reversible computation — computation that never erases information — need not. Charles Bennett showed that any computation can be made reversible. Does this mean that computation can in principle be made thermodynamically free? What practical obstacles prevent this? What does the possibility of reversible computation imply about the relationship between information and physical entropy?

**7. Digital Physics and Circularity**
The most pointed objection to digital physics is the circularity problem: "computation" presupposes temporal order (steps happen in sequence), so defining time as computational steps is circular. Evaluate this objection. Can digital physics respond to it? What resources are available — for instance, defining order in terms of causal rather than temporal relations? Does this response succeed, or does it merely push the problem back a level?

---

## Short Essay Questions

**8. Information Erasure and the Arrow of Time**
Landauer's principle connects information erasure to entropy production. Some philosophers and physicists have proposed that the arrow of time is fundamentally an informational phenomenon: the past-to-future direction is the direction of increasing information loss (erasure). Evaluate this proposal. Does it reduce the arrow of time to the asymmetry of information erasure, or does it presuppose the arrow of time in characterizing what counts as "erasure"? What would a successful informational account of the arrow of time need to show?

**9. Wheeler's "It from Bit"**
John Archibald Wheeler proposed that physical reality is fundamentally informational — that every physical quantity derives its existence from information-theoretic observations and registrations ("it from bit"). What does this proposal mean? How does it relate to: (a) the formal identity of Shannon and thermodynamic entropy; (b) the holographic principle; (c) digital physics? Is Wheeler's proposal a coherent philosophical position, or is it a metaphor that does not bear the weight placed on it?

**10. The Firewall Paradox**
Almheiri, Marolf, Polchinski, and Sully (the "AMPS" paper, 2012) argued that if information escapes from an evaporating black hole in Hawking radiation, there must be a "firewall" at the event horizon — a surface of high-energy radiation that destroys any infalling observer. This contradicts general relativity's prediction that infalling observers experience nothing special at the event horizon. What are the premises of the firewall argument? What does it reveal about the tensions between general relativity and quantum mechanics? How might the ER=EPR proposal (Maldacena-Susskind 2013) respond?

---

## Observational and Laboratory Exercises

**11. Landauer's Principle in Practice**
The experimental confirmation of Landauer's principle by Bérut et al. (2012) involved measuring the heat dissipated when a single colloidal particle's position (one bit of information) was reset. Research this experiment. What physical system was used? How was "one bit" operationalized? How was the heat dissipated measured? What were the main experimental challenges, and how were they addressed? What does this experiment demonstrate about the physical reality of information? Write a 600-word summary of the experimental design and results.

**12. Black Hole Thermodynamics: Tracing the History**
The thermodynamic properties of black holes were established through a series of papers from the early 1970s. Construct a timeline of the key developments: Bekenstein's entropy proposal (1973), the four laws of black hole mechanics (Bardeen, Carter, Hawking 1973), Hawking's radiation calculation (1975), and Page's entropy analysis (1993). For each development, identify: (a) the key result; (b) the conceptual innovation; (c) the connection to information theory. What does this history reveal about the relationship between thermodynamics, quantum mechanics, and gravity?

**13. Entropy Calculations**
(a) Calculate the Shannon entropy (in bits) of the following probability distributions: (i) a fair coin (two outcomes, probability 1/2 each); (ii) a fair six-sided die; (iii) a biased coin with P(heads) = 0.9; (iv) a certain outcome (P = 1 for one outcome, 0 for all others).
(b) Explain intuitively what the differences in entropy values between (i)–(iv) represent.
(c) Landauer's principle says that erasing one bit generates at least *k_B T* ln 2 of heat. At room temperature (T ≈ 293 K), calculate this minimum heat in joules and in electron-volts. How does this compare to the thermal energy *k_B T*?
(d) A modern CPU performs roughly 10¹⁰ operations per second, many of which involve information erasure. Estimate a lower bound on the entropy generated per second. Compare this to the actual heat dissipation of a modern CPU (roughly 100 watts). What does the ratio tell you?
