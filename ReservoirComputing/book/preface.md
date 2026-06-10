# Preface

## The Machine That Forgets Just Enough

---

There is a paradox at the heart of reservoir computing that is worth appreciating before we say anything technical.

A reservoir computer learns by not learning — at least, not in the way we usually mean. Its recurrent connections, which carry information forward through time, are set at random before training begins and are never adjusted. What it does learn — a single linear transformation from its high-dimensional state to its output — is the simplest kind of learning imaginable. You write down a matrix equation. You solve it. You are done.

And yet this system, so primitive in its training procedure, can predict chaotic attractors, process speech, control robots, model motor cortex, and approximate any fading-memory functional to any desired accuracy. The gap between the simplicity of the method and the richness of what it can compute is what makes reservoir computing intellectually delightful. It suggests that much of what we think is required for temporal computation — the careful gradient descent, the vanishing-gradient engineering, the architectural cleverness of gating mechanisms — is not actually required. The dynamics do the work. The training is bookkeeping.

This book exists because, despite a body of excellent papers and a fine practical tutorial by Lukoševičius (2012), there is no single text that takes a reader from absolute first principles — from "what is a dynamical system?" and "why can't a feedforward network handle time?" — all the way to the frontiers of quantum reservoir computing, biological substrates, and the open theoretical problems of the field. We have tried to write that text.

---

## What This Book Is

This is a textbook for graduate students and advanced undergraduates in machine learning, computational neuroscience, dynamical systems, and related fields. It assumes mathematical maturity — comfort with linear algebra, calculus, and basic probability — but does not assume prior knowledge of dynamical systems, control theory, neuroscience, or reservoir computing itself.

Every important result is derived. Proofs are stepped through line by line. We do not say "it can be shown that" and move on; we show it. When a proof is beyond our scope, we say so explicitly, give an intuitive argument, and provide a precise reference.

Every claim about biology, cognition, or philosophy is cited. This is deliberate and important. Reservoir computing sits at the intersection of machine learning, neuroscience, and dynamical systems — fields with very different standards of evidence and very different cultures of assertion. We have tried to maintain the highest standards across all three. When we say "it has been observed that cortical microcircuits have random-looking recurrent connectivity" (a claim relevant to the biological motivation for reservoir computing), we cite the neuroscience literature from which this claim comes. When we describe the DishBrain experiments (neurons in a culture dish learning to play Pong), we are careful to state what was demonstrated, what the authors interpret, and what remains debated. The reader who finds a claim suspicious should find, in the associated citation, a primary source that can be checked.

The exercises are designed to be genuine. Conceptual exercises test understanding that cannot be demonstrated by plugging into formulas. Thought experiments are open-ended questions with no single correct answer, designed to develop the kind of intuition that mathematics alone cannot build. Lab exercises are computational experiments — observe the phenomenon yourself before taking it on faith. Programming projects are substantial: they take hours to days, not minutes, and they are designed to produce something a person could show a colleague and feel proud of.

---

## What This Book Is Not

This book is not a comprehensive survey of the reservoir computing literature. The field is large, moving fast, and has produced thousands of papers. We have chosen depth over breadth: where we cover something, we cover it fully, but we have not tried to cover everything.

This book is not biased toward any single application domain. Reservoir computing is a general-purpose temporal computing paradigm, and we have tried to reflect its generality. Applications chapters (Unit VIII) are introductions to domains, not exhaustive treatments.

This book is not politically neutral on contested questions, but it is epistemically honest. Where the evidence is good, we say so confidently. Where it is preliminary, we say so clearly. We have tried to distinguish between "this is well-established" and "this is one interpretation of the evidence" throughout.

---

## How to Use This Book

**If you are new to dynamical systems and machine learning:** Begin at Chapter 1 and read sequentially through Chapter 7. This will give you the foundation. Then choose an application domain from Unit VIII that interests you, and read that chapter. Return to Units III–IV for the engineering and training details as you need them.

**If you are an ML practitioner who knows RNNs but not reservoir computing:** Begin at Chapter 3 (Section 3.7) and read Chapter 4 carefully. Then read Chapter 5 for the full ESN treatment. Chapter 8 for hyperparameters. Pick up theory from Unit II as needed.

**If you are a neuroscientist:** Read Chapter 6 (Liquid State Machines) early. Then Chapter 24 (neuroscience applications). Chapter 11 (FORCE learning) is directly relevant to motor cortex models. Chapter 29 (ergodic theory) is optional but illuminating.

**If you are a physicist or engineer interested in physical implementations:** Unit VII is your primary destination. Read Chapter 16 first for the general framework, then the specific substrate chapters as interest dictates.

**If you are a mathematician:** Unit IX contains the full proofs. Read it alongside the corresponding chapters in Units I–IV for the theoretical context.

---

## A Note on Software

All programming exercises in this book can be completed using Python with NumPy, SciPy, and Matplotlib — the standard scientific Python stack. For reservoir computing specifically, we recommend **ReservoirPy** (Trouvain et al., 2020), an open-source library that implements ESNs, deep reservoirs, and online learning with a clean API. Appendix D provides a comprehensive tutorial.

Where we use spiking neural networks (Liquid State Machines in Chapter 6), we use **Brian2** (Stimberg et al., 2019). For quantum reservoir computing experiments (Chapter 31), we use **Qiskit** or **QuTiP**. All are freely available.

---

## Acknowledgments

This book draws on the work of many researchers who have built reservoir computing from the ground up over 25 years. We have tried to acknowledge them carefully throughout — in the Key Researchers sections of each chapter, in the Further Reading annotations, and in the citations that accompany every technical claim. Any errors in attribution are unintentional and we welcome corrections.

---

## A Last Word Before We Begin

The best reservoir computing paper ever written — the most important, the most read, the most cited — is a 33-page technical report from 2001 by a researcher named Herbert Jaeger at the German National Research Center for Information Technology. It is not a polished journal article. It is handwritten-feeling, discursive, full of asides and honest uncertainty. And it describes, with remarkable clarity, a simple idea that turned out to be profound.

That idea is: let the network's memory be random, and let learning do only what learning is good at. The elegance is not in the architecture. It is in the restraint.

We hope this book conveys some of that spirit.

---

*Begin.*
