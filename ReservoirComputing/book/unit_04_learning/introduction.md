# Unit IV: Learning in the Reservoir — Beyond Linear Readouts

---

> *"The readout is not the prison of the reservoir. It is the window through which we see what the reservoir knows."*

---

## The Linear Assumption

The canonical reservoir computer has a linear readout. This is not arbitrary: the linear readout is what makes reservoir training tractable (a convex optimization problem with a closed-form solution), interpretable (the readout weights tell you which reservoir nodes matter), and theoretically analyzable (we can compute its generalization error exactly).

But linearity is an assumption, and assumptions can be wrong. For some tasks, the reservoir state contains the relevant information in a form that no linear function can extract. For some tasks, we need not just prediction but uncertainty. For some tasks, we want to train not just the readout but the reservoir itself.

This unit relaxes the linear assumption in three directions:

1. **More powerful readouts** — nonlinear, probabilistic, and kernel-based.
2. **Training the reservoir** — the FORCE algorithm and its relatives.
3. **Structured memory** — conceptors, which allow a single reservoir to store and recall multiple learned patterns.

---

## The Four Chapters

**Chapter 10** surveys readout architectures beyond ridge regression. We develop polynomial, kernel, and Bayesian readouts — each with its theory and its use cases. We examine output feedback, which turns a prediction reservoir into a generative one. These are largely modular extensions: swap out the readout, keep the reservoir.

**Chapter 11** presents FORCE learning — the algorithm that shocked the reservoir computing community in 2009. FORCE trains the recurrent weights of the reservoir itself using a clever online least-squares update. The result: networks that generate complex, precisely timed patterns that no fixed-reservoir system can produce. We give the full derivation, analyze what FORCE-trained networks actually learn, and examine their role in computational neuroscience.

**Chapter 12** develops conceptors — Herbert Jaeger's elegant extension of reservoir computing to multi-pattern memory. A conceptor is a matrix that encodes which subspace of reservoir state space corresponds to a particular learned pattern. Boolean operations on conceptors allow patterns to be combined, excluded, and interpolated. We prove the algebraic structure, work through examples, and discuss the deep questions about memory and cognition that conceptors raise.

---

## A Thematic Thread

There is a philosophical thread running through this unit. Each chapter is, in a different way, asking: *how much can we do with the reservoir's representation?* Linear readouts exploit the first-order structure. Kernel readouts exploit the metric structure. FORCE training modifies the representation itself. Conceptors organize multiple representations. Together they illustrate that the reservoir is not a fixed black box but a flexible substrate that can be engaged in many ways — and that the right level of intervention depends on the task, the data, and the constraints of the application.

---

*We begin where the standard tutorial ends.*
