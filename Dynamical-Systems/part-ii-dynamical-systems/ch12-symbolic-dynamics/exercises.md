# Exercises — Chapter 12

The problems below develop the main tools of symbolic dynamics through computation and proof. Several require working explicitly with transition matrices and Perron-Frobenius theory. The last two — especially 12.7 — are genuinely open research questions and are included to point toward the frontier.

---

**Exercise 12.1.** Show that the golden mean shift (forbidden word $\{11\}$) is a subshift of finite type. Write the transition matrix and compute $p(n)$ for small $n$ (the number of allowed words of length $n$). Verify $h_{\text{top}} = \log(1+\sqrt{5})/2$.

**Exercise 12.2.** Prove that the even shift (sequences where all runs of 0s between consecutive 1s have even length) has a sofic presentation but is not an SFT. (*Hint:* Show that any set of forbidden words that defines the even shift must be infinite.)

**Exercise 12.3.** Compute the zeta function of the golden mean shift using $\zeta(t) = 1/\det(I - tA)$. Expand as a power series and verify the coefficient of $t^n$ equals $|\text{Fix}(\sigma^n)|$ for the golden mean shift.

**Exercise 12.4.** (Parry Measure) For the golden mean shift, compute the Parry measure: find the Perron-Frobenius eigenvectors of $A = \begin{pmatrix}1&1\\1&0\end{pmatrix}$ and write the Markov transition probabilities.

**Exercise 12.5.** Let $X$ be a subshift. Show that $p_X(n+1)/p_X(n) \geq 1$ (the complexity function is nondecreasing). Show that $X$ is periodic (all orbits periodic) iff $p_X(n) = \text{const}$ for all large $n$.

**Exercise 12.6.** (Collatz Connection) Consider the Collatz map as a coding: to each $n \in \mathbb{N}$, associate the sequence of parities $c_k = n_k \pmod 2$ where $n_{k+1} = T(n_k)$. The sequence $c \in \{0,1\}^{\mathbb N}$ is the *Collatz itinerary* of $n$. What subshift does the set of all Collatz itineraries generate? Is it an SFT? A sofic shift?

**Exercise 12.7.** (Research) The *entropy of the Collatz process*: if we view the parity sequence $c_n = T^n(m) \pmod 2$ as a stationary process (under some invariant measure), what is its entropy rate? What constraints does the Collatz conjecture place on this entropy?
