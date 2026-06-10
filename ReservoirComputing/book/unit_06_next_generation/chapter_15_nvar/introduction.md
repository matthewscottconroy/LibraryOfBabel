# Chapter 15: Next-Generation Reservoir Computing — NVAR and the Random Feature Perspective

---

> *"The best model of a cat is another cat — or possibly the same cat."*
> — Norbert Wiener (paraphrased)

> *"Sometimes the reservoir you need is just a polynomial."*

---

## Chapter Introduction

In 2021, Daniel Gauthier and colleagues published a paper in *Nature Communications* with a striking result: a method they called the Next-Generation Reservoir Computer (NG-RC or NVAR) could reproduce the predictive performance of a large Echo State Network on the Lorenz system — using a fraction of the computational resources and requiring no hidden state at all [Gauthier2021]. The method is conceptually simple: take the last few observed inputs, form all polynomial combinations up to some degree, and train a linear readout on these polynomial features. No reservoir. No recurrent dynamics. Just feature engineering followed by linear regression.

This provocation deserves a careful response. Is NVAR actually a reservoir computer? Why does it work so well? And when does it fail?

The answers illuminate something deep about what reservoir computing is really doing. NVAR is a special case of the Volterra series approach (Chapter 1), and the Volterra series is, in turn, a special case of a kernel method applied to time series. The reservoir computer, it turns out, is also a kernel method — one that uses random features to approximate the kernel implicitly. The connection runs through the Rahimi-Recht random features theorem [Rahimi2007], which establishes that random projections followed by nonlinear activations approximate any shift-invariant kernel. Both NVAR and ESN are methods for approximately computing the action of a temporal kernel, differing in which kernel they target and how they approximate it.

Understanding this connection — NVAR as exact polynomial kernel evaluation, ESN as random-feature approximation of a smooth kernel — clarifies the strengths and weaknesses of each approach and points toward hybrid methods that combine the best of both.

---

## What You Will Learn

- The NVAR architecture: polynomial feature construction, the precise algorithm, and exact reproduction of Gauthier et al. 2021 results
- The Volterra series connection: NVAR as finite-order Volterra approximation
- The random features theorem (Rahimi & Recht 2007): ESN as an instance
- When NVAR outperforms ESN and vice versa: a principled comparison
- The kernel perspective: both NVAR and ESN as kernel methods on time series

---

## Prerequisites

This chapter requires Chapter 1 (Volterra series and fading memory), Chapter 5 (ESN), and basic knowledge of kernel methods (kernel functions, Mercer's theorem). The random features section requires basic probability (moment calculations, concentration inequalities at the level of a brief conceptual introduction).
