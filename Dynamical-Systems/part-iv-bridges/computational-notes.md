# Computational Notes — Part IV: The Bridges

These notes implement the bridges between dynamical systems and information theory computationally. The central experiments demonstrate: the Shannon-McMillan-Breiman theorem as the ergodic theorem applied to log-probabilities; the variational principle in action; symbolic coding of hyperbolic systems; and the information-production interpretation of chaos.

---

## 1. The Shannon-McMillan-Breiman Theorem Experimentally

### Setup: Stationary Ergodic Process from a Dynamical System

Take the doubling map $D(x) = 2x \pmod 1$ with partition $\mathcal{P} = \{[0,1/2), [1/2,1)\}$. The itinerary $s_n = \mathbf{1}_{[1/2,1)}(D^n(x)) \in \{0,1\}$ is a stationary ergodic binary process (actually i.i.d. for Lebesgue-a.e. $x$).

```
def smb_experiment(f, partition_fn, x0, n=10000):
    """
    Verify Shannon-McMillan-Breiman:
    -(1/n) log mu(atom containing x) -> h(f, partition) a.s.
    """
    x = x0
    log_measure_sum = 0.0
    curve = []
    
    for k in range(n):
        # Atom measure = 2^{-k} for uniform partition refinement
        # More generally, need to track the refined atom
        atom = partition_fn(x)
        x = f(x)
        
        # Approximate: for doubling map, mu(k-refinement atom) = 2^{-k}
        log_measure_sum += log(0.5)  # each step halves the atom
        curve.append(-log_measure_sum / (k+1))
    
    return curve  # should converge to h = log(2) = 1 bit
```

**Correct implementation with atom tracking**:

```
def smb_correct(x, n, f=lambda x: 2*x % 1, base=0.5):
    """Track shrinking atom measure directly"""
    lo, hi = 0.0, 1.0
    log_probs = []
    for k in range(n):
        mid = (lo + hi) / 2
        if f.__name__ == 'doubling' or True:
            # Under doubling map, binary subdivision
            if x < mid:
                hi = mid
            else:
                lo = mid
            x = 2*x % 1
        log_probs.append(log2(hi - lo) / (k+1))
    return log_probs  # converges to -h = -1
```

**Expected**: the quantity $-\frac{1}{n}\log \mu(\text{atom of }\mathcal{P}^{(n)}\text{ containing }x)$ converges to $h_\mu(f, \mathcal{P}) = \log 2 = 1$ bit for Lebesgue-a.e. starting point.

---

## 2. The Variational Principle in Practice

### Computing h_top and Checking the Variational Principle

For an SFT with transition matrix $A$:
$$h_{\text{top}} = \log \lambda_{\text{PF}}(A) = \sup_\mu h_\mu.$$

```
def topological_entropy_sft(A):
    """Compute h_top for SFT via Perron root"""
    eigenvalues = np.linalg.eigvals(A)
    return np.log(max(np.abs(eigenvalues)))

def ks_entropy_markov(A, pi=None):
    """
    KS entropy of Markov chain with transition matrix A
    and stationary distribution pi.
    h = -sum_{i,j} pi_i A_ij log A_ij
    """
    if pi is None:
        # Find stationary distribution
        evals, evecs = np.linalg.eig(A.T)
        pi = evecs[:, np.argmax(np.abs(evals))].real
        pi /= pi.sum()
    h = 0.0
    for i in range(len(A)):
        for j in range(len(A)):
            if A[i,j] > 0:
                h -= pi[i] * A[i,j] * np.log(A[i,j])
    return h
```

**Example**: Golden mean shift ($A = [[1,1],[1,0]]$):
- $h_{\text{top}} = \log \phi \approx 0.481$ (log of golden ratio)
- Parry measure: stationary distribution of $A$ is $\pi = (1/\phi^2, 1/\phi)$ (normalized)... actually the stationary distribution for the Markov chain with matrix $A/\lambda_{\text{PF}}$ (where each row is normalized by the Perron root and eigenvector).
- Verify: $h_\mu = h_{\text{top}}$ for the Parry measure.

---

## 3. Symbolic Coding of a Chaotic Map

### Logistic Map and Its Symbolic Code

For $f_4(x) = 4x(1-x)$ with partition $\{[0,1/2), [1/2,1)\}$:

```
def logistic_code(x, n):
    """Generate symbolic itinerary under logistic map"""
    code = []
    for k in range(n):
        code.append(0 if x < 0.5 else 1)
        x = 4 * x * (1 - x)
    return code

def logistic_decode(code):
    """Recover x from first n symbols (approximately)"""
    # x = (2/pi) * arcsin(sqrt(x))^{-1}(binary expansion)
    # For doubling map coding: x = sum code[k] * 2^{-(k+1)}
    # For logistic map (conjugate to doubling via h(x) = sin^2(pi x/2)):
    # x = sin^2(pi * y / 2) where y = sum code[k] * 2^{-(k+1)}
    y = sum(bit * 2**(-k-1) for k, bit in enumerate(code))
    return np.sin(np.pi * y / 2)**2
```

**Verify conjugacy**: For a random $x \in [0,1]$, generate 50 symbols under $f_4$. Then decode the 50-symbol code to get $\hat{x}$. Verify $|x - \hat{x}| < 2^{-50} \cdot C$ (the coding is exact up to the approximation of truncating at 50 symbols).

**Information content**: The first $n$ symbols of the code for a Lebesgue-typical $x$ contain exactly $n$ bits of information (since the entropy rate is $\log 2 = 1$ bit per step). Each additional symbol of the itinerary reduces uncertainty about $x$ by exactly one binary digit.

---

## 4. Pesin's Formula Numerically

### Verifying h = sum of positive Lyapunov exponents

For the logistic map $f_4$:
- Lyapunov exponent: $\lambda = \int_0^1 \log|f_4'(x)| \frac{dx}{\pi\sqrt{x(1-x)}} = \log 2$
- KS entropy with arcsine measure: $h = \log 2$ (isomorphic to doubling map)
- Pesin's formula: $h = \lambda > 0$ ✓

```
def verify_pesin_logistic(n=100000):
    x = 0.2  # generic initial condition
    lyapunov_sum = 0.0
    for k in range(n):
        lyapunov_sum += np.log(abs(4 - 8*x))  # |f'_4(x)| = |4 - 8x|
        x = 4*x*(1-x)
    lyapunov = lyapunov_sum / n
    print(f"Lyapunov exponent: {lyapunov:.4f}")
    print(f"Expected (log 2): {np.log(2):.4f}")
```

For the Lorenz system, verify Pesin's formula:
```
# After computing Lyapunov spectrum [lambda1, lambda2, lambda3]
# For SRB measure:
# h_KS = lambda1  (only lambda1 > 0)
# h_top >= h_KS (with equality for the SRB measure)
print(f"KS entropy (Pesin) ≈ {lambda1:.4f}")
print(f"Should match topological entropy ≈ 0.906")
```

---

## 5. The Lempel-Ziv Algorithm and Entropy Rate

### LZ77/LZ78 as Universal Compressor

The Lempel-Ziv algorithm is the standard lossless compression algorithm and is *universally optimal* for stationary ergodic sources: the compression ratio converges to the entropy rate.

```
def lz78_length(sequence):
    """Return number of phrases in LZ78 parsing"""
    dictionary = {'': 0}
    current = ''
    phrase_count = 0
    for symbol in sequence:
        extended = current + str(symbol)
        if extended in dictionary:
            current = extended
        else:
            dictionary[extended] = len(dictionary)
            phrase_count += 1
            current = ''
    if current:
        phrase_count += 1
    return phrase_count

def lz_entropy_estimate(sequence):
    """
    By LZ theorem: phrase_count * log(phrase_count) / len(sequence)
    converges to h (entropy rate)
    """
    n = len(sequence)
    c = lz78_length(sequence)
    return c * np.log2(c) / n
```

**Experiments**:
1. i.i.d. fair coin: LZ entropy estimate → 1 bit/symbol.
2. i.i.d. Bernoulli(0.1): LZ estimate → $h(0.1) \approx 0.469$ bits/symbol.
3. Logistic map itinerary: LZ estimate → $\log 2 \approx 1$ bit/step.
4. Period-2 orbit of logistic map (periodic sequence): LZ estimate → 0 bits/step.

---

## 6. Chaos and Information Production

### Predictability Horizon Computation

```
def predictability_horizon(lyapunov_exponent, epsilon_initial=1e-10, 
                           epsilon_final=0.1):
    """
    Time at which error grows from epsilon_initial to epsilon_final.
    T_pred = (1/lambda) * log(epsilon_final / epsilon_initial)
    """
    return np.log(epsilon_final / epsilon_initial) / lyapunov_exponent

# For Lorenz system:
T_pred = predictability_horizon(0.906)
print(f"Predictability horizon: {T_pred:.1f} time units")
# ≈ 24 time units; with dt = 0.01, ≈ 2400 steps

# For atmosphere (lambda ≈ 0.5/day):
T_atm = predictability_horizon(0.5/24) * 24  # convert to days
print(f"Atmospheric predictability: {T_atm:.1f} days")
```

### Information Production Rate

The rate of information production by a chaotic map is its entropy rate:
- Lorenz attractor: $h_\mu \approx 0.9$ nats/time unit = $0.9/\ln 2 \approx 1.3$ bits/time unit
- Logistic map: $h_\mu = 1$ bit/iteration
- Cat map: $h_\mu = \log \lambda_+ \approx 0.962$ nats/iteration = 1.39 bits/iteration

**Computational demonstration**: Generate 1000 bits from the logistic map and feed them to a random number generator quality test (NIST SP 800-22). The logistic map output passes most tests at $\mu = 4$ (good pseudo-randomness) but fails for $\mu < \mu_\infty$ (periodic, fails randomness tests).

---

## 7. Connection to the Quiz App

Quiz questions from Part IV's computational perspective:
- "Describe the experiment that verifies the Shannon-McMillan-Breiman theorem for the doubling map. What quantity converges, to what, and at what rate?"
- "The Lempel-Ziv algorithm is universally optimal. Explain what 'universally optimal' means for a compression algorithm."
- "Pesin's formula states $h_\mu = \sum_{\lambda_i > 0} \lambda_i$ for SRB measures. Verify this for the logistic map $f_4$ by computing both sides independently."
- "For the Lorenz system with maximal Lyapunov exponent $\approx 0.9$ nats/time, compute the predictability horizon starting from measurement accuracy $10^{-8}$ and requiring prediction accuracy $0.1$."
