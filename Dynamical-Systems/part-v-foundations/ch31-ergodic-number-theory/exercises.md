# Exercises — Chapter 31

These problems develop the tools of ergodic number theory through computation and proof. Exercise 31.4 is the most concrete — just iterate the Collatz map and count — but it builds intuition for the heuristic bounds discussed in Section 31.4.

---

**Exercise 31.1.** (Weyl) Prove that the sequence $(n^2\alpha \pmod 1)$ is equidistributed for irrational $\alpha$ using the van der Corput trick: apply Weyl's criterion to show $\frac{1}{N}\sum e^{2\pi ikn^2\alpha} \to 0$.

**Exercise 31.2.** (Normal Numbers) Show that if $x$ is normal in base 2, then $2x \pmod 1$ is also normal in base 2. Conclude that the set of normal numbers is invariant under the doubling map.

**Exercise 31.3.** (Furstenberg Correspondence) Apply the Furstenberg correspondence principle to the set of odd numbers $A = \{1, 3, 5, 7, \ldots\}$ (density $1/2$). What is the corresponding MPT and set $B$? Does $A$ contain arithmetic progressions of length 3?

**Exercise 31.4.** (Collatz) Compute the first 20 iterates of $C$ starting from $n = 27$. How many steps to reach 1? Compare to the heuristic prediction $O(\log n)$.
