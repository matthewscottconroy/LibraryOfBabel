# Exercises — Chapter 19

The exercises below develop the main results and test your understanding of the multi-user setting. Draw the capacity regions wherever possible — geometry helps.

**Exercise 19.1.** Compute the capacity region of the Gaussian MAC with $P_1 = P_2 = 1$ and $N = 1$ (unit noise). Draw the region. What happens as $P_i \to \infty$?

**Exercise 19.2.** State and prove the Slepian-Wolf converse: show that $R_X < H(X|Y)$ or $R_Y < H(Y|X)$ or $R_X + R_Y < H(X,Y)$ leads to positive probability of error.

**Exercise 19.3.** For binary symmetric correlated sources: $X \sim \text{Bernoulli}(1/2)$ and $Y = X \oplus E$ where $E \sim \text{Bernoulli}(\epsilon)$ independently. Compute $H(X|Y)$, $H(Y|X)$, and $H(X,Y)$. Draw the Slepian-Wolf rate region.

**Exercise 19.4.** (Wyner-Ziv) For Gaussian sources $X \sim N(0, \sigma^2)$ and side information $Y = X + Z$, $Z \sim N(0, N)$ (independent of $X$), compute the Wyner-Ziv rate-distortion function for squared-error distortion. Verify that $R_{\text{WZ}}(D) = R(D \mid Y)$.

**Exercise 19.5.** (Wiretap) For the binary wiretap channel where $Y = X \oplus N_Y$, $Z = X \oplus N_Y \oplus N_Z$ (Bernoulli noise): compute the secrecy capacity $C_s$ when $P(N_Y = 1) = \epsilon_Y < \epsilon_Z = P(N_Z = 1)$.
