**Corresponds to:** Chapter 11 — Online Learning: RLS, Kalman Filters, and Adaptive Readouts (Unit 4)

**Prerequisites:** Chapter 5 (ESN basics), matrix inversion lemma (Sherman-Morrison-Woodbury), Chapter 9 (benchmarks)

**Learning Objectives:**
- Implement Recursive Least Squares (RLS) as an efficient online update for the ESN readout weights
- Understand the matrix inversion lemma: the RLS update avoids re-solving the full normal equations at each step
- Compare batch ridge regression to online RLS: understand when they agree and when they differ
- Apply RLS to a non-stationary task and observe its adaptive tracking ability
- Understand the forgetting factor λ_f in RLS and its role in tracking distributional shift
