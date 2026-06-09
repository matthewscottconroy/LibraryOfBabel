# Exercises — Chapter 41

These exercises develop hands-on facility with Collatz orbits, the 2-adic extension, the stochastic model, and Tao's exponential sum methods. Exercise 41.4 requires reading the abstract of Tao's paper.

---

**Exercise 41.1.** Compute the Collatz orbit of $n = 27$: find the stopping time $\sigma(27)$ and the total stopping time $\tau(27)$. What is the maximum value reached?

**Exercise 41.2.** (2-Adic Extension) Express $n = 5$ as a 2-adic integer $x = 1 + 0 \cdot 2 + 1 \cdot 4 + \cdots$. Apply $\tilde{C}$ (the accelerated map) and verify it matches applying $C$ twice to $n = 5$ (since $5$ is odd).

**Exercise 41.3.** (Stochastic Model) Simulate 1000 random walks on $(0, \infty)$ starting at $\log 100$ with steps $\pm\log(3/2)$ or $-\log 2$ (each with probability 1/2). What fraction hit 0 within 100 steps? Compare to $\tau(n)$ for actual Collatz orbits starting near $100$.

**Exercise 41.4.** (Research) Read Tao's 2022 paper abstract. The key tool is "exponential sum estimates on ${\mathbb Z}_2$." Describe what an exponential sum on ${\mathbb Z}_2$ is, and why bounding it gives control over Collatz orbit densities.
