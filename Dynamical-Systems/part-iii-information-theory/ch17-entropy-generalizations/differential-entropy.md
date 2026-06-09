# 17.4 Differential Entropy

Everything so far has been about discrete distributions — finite or countable alphabets. But the world is full of continuous signals: audio waveforms, physical measurements, the position of a particle in space. Can we define entropy for continuous random variables?

The naive answer is to take the limit of discrete entropy as the quantization gets finer. But this limit diverges — the entropy of a continuous variable is infinite if you insist on exact precision. The right approach is different: we simply define entropy for continuous distributions directly, using an integral instead of a sum.

**Definition 17.4.1 (Differential Entropy).** For a continuous random variable $X$ with density $f(x)$, the *differential entropy* is:
$$h(X) = -\int f(x)\log f(x)\,dx.$$

This looks like the Shannon entropy formula, with the sum replaced by an integral. But there is an important warning:

**Warning:** Differential entropy is *not* the limit of discrete entropy as the quantization gets finer: it can be negative (e.g., $X \sim U[0, 1/2]$ has $h(X) = -\log 2 < 0$). It is not invariant under smooth reparametrization.

These are fundamental differences from the discrete case. A uniform distribution on a very short interval has high precision (small quantization error) but negative differential entropy. Under a change of variables $Y = g(X)$, differential entropy transforms as $h(Y) = h(X) + E[\log |g'(X)|]$ — it picks up a Jacobian term, unlike discrete entropy which is invariant under relabeling.

What differential entropy *is* good for is comparing distributions, computing mutual information between continuous variables, and finding maximum entropy distributions. The differences $h(X) - h(X|Y) = I(X;Y)$ (mutual information) are invariant under reparametrization, even though the individual terms are not.

**Examples:**
- $X \sim N(\mu, \sigma^2)$: $h(X) = \frac{1}{2}\log(2\pi e \sigma^2)$
- $X \sim U[a, b]$: $h(X) = \log(b-a)$
- $X \sim \text{Exp}(\lambda)$: $h(X) = 1 - \log\lambda$

Among these, the Gaussian stands out for a reason:

**Theorem 17.4.2 (Gaussian Maximizes Entropy).** Among all distributions with fixed mean $\mu$ and variance $\sigma^2$, the Gaussian $N(\mu, \sigma^2)$ maximizes differential entropy.

This theorem is deeply connected to the central limit theorem: the Gaussian is the "most random" distribution for fixed variance, in the sense of maximizing entropy. It is the distribution of maximum uncertainty given only first- and second-moment constraints. This connection between Gaussians, entropy, and central limit theorems runs throughout probability theory and appears in information geometry (Chapter 20) as the statement that the Gaussian is the maximum entropy distribution in its exponential family.

The Gaussian entropy formula $h(N(\mu, \sigma^2)) = \frac{1}{2}\log(2\pi e \sigma^2)$ also appears in Shannon's channel capacity formula for Gaussian channels: $C = \frac{1}{2}\log(1 + P/N)$ is the difference of two Gaussian entropies (output minus noise). In that formula, the $2\pi e$ cancels, leaving only the signal-to-noise ratio. This is not a coincidence — Gaussian channels are solved by Gaussian inputs precisely because Gaussian distributions maximize differential entropy.
