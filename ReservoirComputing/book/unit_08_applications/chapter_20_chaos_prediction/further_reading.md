# Chapter 20 — Further Reading and References

---

## Essential References

### [Pathak2018]

**Pathak, J., Hunt, B., Girvan, M., Lu, Z., & Ott, E. (2018). Model-free prediction of large spatiotemporally chaotic systems from data: A reservoir computing approach. *Physical Review Letters*, 120(2), 024102.**

The landmark paper. Shows 8 Lyapunov times VPT on the KS equation, introduces the parallel reservoir architecture, and provides Lyapunov spectrum estimation from the trained model. Required reading.

### [Pathak2017]

**Pathak, J., Lu, Z., Hunt, B.R., Girvan, M., & Ott, E. (2017). Using machine learning to replicate chaotic attractors and calculate Lyapunov exponents from data. *Chaos*, 27(12), 121102.**

The companion paper focused on the Lorenz system. Demonstrates attractor replication, VPT calculation, and Lyapunov exponent estimation from RC models.

### [Gauthier2021]

**Gauthier, D.J., Bollt, E., Griffith, A., & Barbosa, W.A.S. (2021). Next generation reservoir computing. *Nature Communications*, 12, 5564.**

The NVAR paper (Chapter 15), providing the Lorenz comparison.

---

## Takens' Theorem and Delay Embeddings

### [Takens1981]

**Takens, F. (1981). Detecting strange attractors in turbulence. In *Dynamical Systems and Turbulence, Warwick 1980*. Lecture Notes in Mathematics, 898. Springer. 366–381.**

The original embedding theorem. Not easy reading, but essential for understanding the theoretical foundation.

### [Sauer1991]

**Sauer, T., Yorke, J.A., & Casdagli, M. (1991). Embedology. *Journal of Statistical Physics*, 65(3–4), 579–616.**

A more accessible and general treatment of embedding theorems, extending Takens to include noisy observations and multiple observations. Highly recommended.

### [Kantz1997]

**Kantz, H. & Schreiber, T. (1997). *Nonlinear Time Series Analysis*. Cambridge University Press.**

The standard textbook for nonlinear time series analysis, including delay embedding, Lyapunov exponent estimation, and attractor reconstruction. Essential background for Chapter 20.

---

## Chaotic Systems and Lyapunov Exponents

### [Ott2002]

**Ott, E. (2002). *Chaos in Dynamical Systems*, 2nd ed. Cambridge University Press.**

A comprehensive textbook on chaos theory by one of the key researchers in Chapter 20. Covers attractors, Lyapunov exponents, fractal dimensions, and controlling chaos. Required background for understanding the Pathak et al. results in depth.

### [Lorenz1963]

**Lorenz, E.N. (1963). Deterministic nonperiodic flow. *Journal of Atmospheric Sciences*, 20(2), 130–141.**

The original Lorenz paper. Essential historical context.

---

## Follow-Up Work

### [Pathak2018b]

**Pathak, J., Wikner, A., Fussell, R., Chandra, S., Hunt, B.R., Girvan, M., & Ott, E. (2018). Hybrid forecasting of chaotic processes: Using machine learning in conjunction with a knowledge-based model. *Chaos*, 28(4), 041101.**

The hybrid model approach: combines a physical model with a reservoir. Achieves VPT > 12 Lyapunov times on the KS equation by using an imperfect model to pre-process features. Important follow-up.

### [Lu2018]

**Lu, Z., Hunt, B.R., & Ott, E. (2018). Attractor reconstruction by machine learning. *Chaos*, 28(6), 061104.**

Demonstrates that reservoir computers can learn the attractor topology (not just predict the time series) — the reservoir's closed-loop trajectory lies on a reconstructed attractor that is topologically equivalent to the true attractor.

### [Vlachas2020]

**Vlachas, P.R., Pathak, J., Hunt, B.R., Sapsis, T.P., Girvan, M., Ott, E., & Koumoutsakos, P. (2020). Backpropagation algorithms and reservoir computing in recurrent neural networks for the forecasting of complex spatiotemporal dynamics. *Neural Networks*, 126, 191–217.**

A careful comparison of RC, trained LSTM, and hybrid methods for spatiotemporal chaos. Shows that while LSTM can outperform RC for some systems with sufficient training data, RC remains competitive and much faster to train.
