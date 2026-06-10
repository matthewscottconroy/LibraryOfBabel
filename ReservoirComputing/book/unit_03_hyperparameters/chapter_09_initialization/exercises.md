# Chapter 9 Exercises

## Analytical Exercises

**Exercise 9.1 (SCR memory capacity — detailed derivation).**
For the linear SCR with $N$ neurons, weight $\rho$, single input weight $\sigma_{in}$ at neuron 1:

(a) Verify the formula $r_i(t) = \sigma_{in} \sum_{m=0}^\infty \rho^{mN + i - 1} u_{t - mN - (i-1)}$ by substituting into the SCR recursion and confirming both the self-consistency condition and boundary conditions.

(b) Compute the covariance $\mathbb{E}[r_i(t) r_j(t)]$ for $i \neq j$. Show it is exactly zero when the inputs are i.i.d. (Hint: the sum over delays for neurons $i$ and $j$ involves disjoint sets of delays.)

(c) From the diagonal covariance structure, confirm that the state covariance matrix $R_{\mathbf{rr}} = \text{diag}(\sigma_{r_1}^2, \ldots, \sigma_{r_N}^2)$ in the SCR is diagonal. (Note: this is not the same as saying the matrix is diagonal in the original basis — the SCR's weight matrix is not diagonal. The diagonal covariance holds because the neurons remember disjoint delays.)

(d) Conclude from parts (b) and (c) that $MC = \sum_i MC^{(i)} = N$ exactly, and that this bound holds for *any* $\rho \in (0,1)$.

---

**Exercise 9.2 (SCR vs. ESN memory profile).**
Compare the memory profiles $\{MC_k\}_{k=1}^{3N}$ for:
- A linear SCR with $N = 10$ neurons and $\rho = 0.95$.
- A linear random ESN with $N = 10$ neurons and spectral radius $\rho = 0.95$ (one realization).

(a) For the SCR, compute $MC_k$ analytically for $k = 1, 2, \ldots, 3N$. What pattern do you see?

(b) For the random ESN, use the formula from Section 7.2.5 with a diagonalized weight matrix. Estimate $MC_k$ for a random eigenspectrum.

(c) Both have $MC \leq N = 10$. Which has higher $MC$ for this $\rho$? What does this tell you about the efficiency of random vs. structured reservoirs?

(d) At what spectral radius $\rho$ does the ESN's total memory capacity equal the SCR's? (Recall: for the ESN, $MC \approx \rho^2 \cdot \text{const}$, while for the SCR, $MC = N$ exactly for any $\rho < 1$.) What does this imply for practical ESN design?

---

**Exercise 9.3 (IP update equations — logistic to tanh).**
Derive the IP update equations for the tanh activation $f(x) = \tanh(x)$ (not the logistic sigmoid). Use the Gaussian target distribution $q(y) = \mathcal{N}(0, \sigma_T^2)$ restricted to $(-1,1)$ (approximately valid for small $\sigma_T$).

(a) Write the KL divergence $D_{KL}(p_y \| q)$ as a function of $a$ and $b$.

(b) Compute $\partial D_{KL}/\partial b$ and $\partial D_{KL}/\partial a$, expressing the result in terms of $y = \tanh(ax+b)$.

(c) Show that the update rules simplify to:
$$\Delta b = \eta \left(-\frac{y}{\sigma_T^2} + a(1-y^2)\right), \quad \Delta a = \eta\left(\frac{1}{a} - x\frac{y}{\sigma_T^2} + ax(1-y^2)\right).$$

(d) At the equilibrium ($\Delta a = \Delta b = 0$), what is the condition on the joint statistics of $(x, y)$? Interpret this as a moment-matching condition.

---

**Exercise 9.4 (Hybrid SCR-random reservoir).**
Consider a reservoir that is a convex combination of the SCR weight matrix $W_{SCR}$ and a random matrix $W_{rand}$:

$$W = (1-\epsilon) W_{SCR} + \epsilon W_{rand},$$

where $\epsilon \in [0,1]$ and both matrices have the same spectral radius $\rho$.

(a) At $\epsilon = 0$: what is $MC$? At $\epsilon = 1$: what is $MC$?

(b) Hypothesize (based on the results of Section 9.2.5) how $MC$ changes as $\epsilon$ increases from 0. Is the relationship monotonic?

(c) What about the *nonlinear* capacity? At $\epsilon = 0$ (pure SCR, tanh activation), the nonlinear capacity is nonzero. Does it increase or decrease as $\epsilon$ increases?

(d) Based on your analysis, propose an optimal value of $\epsilon$ for a task requiring: (i) maximum linear memory; (ii) a balance of linear memory and nonlinear capacity.

---

**Exercise 9.5 (ESP and IP — stability analysis).**
After IP training converges, the gains $\{a_i\}$ and biases $\{b_i\}$ are fixed and the effective activation function for neuron $i$ is $f_i(x) = \sigma(a_i x + b_i)$ (logistic sigmoid).

(a) The Jacobian of the reservoir map with IP is $J = \text{diag}(a_i \sigma(a_i x_i + b_i)(1-\sigma(a_i x_i+b_i))) \cdot W$. Show that $\rho(J) \leq \max_i a_i \cdot \frac{1}{4} \cdot \rho(W)$.

(b) For the ESP to hold (with probability 1), we need $\rho(J) < 1$. What constraint does this place on the maximum allowable gain $\max_i a_i$?

(c) If the IP target distribution is exponential with mean $1/\mu$, the equilibrium mean activation is $\mathbb{E}[y] = 1/\mu$. For a logistic sigmoid neuron at equilibrium, estimate the mean gain $\mathbb{E}[\sigma(a_ix_i+b_i)(1-\sigma(a_i x_i+b_i)) \cdot a_i]$ as a function of $\mu$ and $a_i$.

(d) Given your estimate from (c), what constraint on $a_i$ ensures the ESP is preserved after IP training?

---

## Thought Experiments

**Thought Experiment 9.1: Why Does Extreme Simplicity Work?**

The SCR consists of $N$ neurons connected in a single ring. This is, in some sense, the simplest possible reservoir that has $N$ neurons. Yet it achieves the maximum memory capacity $MC = N$. 

(a) Can you think of an even simpler reservoir that achieves $MC = N$? (Hint: the delay line — a feedforward chain of neurons — achieves $MC = N$ trivially, since neuron $i$ holds exactly $u_{t-i}$. But is this really a "reservoir"? Does it satisfy the ESP?)

(b) The SCR's success is sometimes attributed to its uniform eigenspectrum. But consider: the delay line (all eigenvalues zero except for the one at the "end") also achieves $MC = N$. What does this suggest about the relationship between eigenspectrum uniformity and memory capacity?

(c) The SCR has very limited nonlinear capacity (for the tanh version) because the ring structure creates minimal nonlinear mixing of the delayed inputs. The delay line has essentially *no* nonlinear capacity. Does this suggest that $MC = N$ is actually a bad thing for tasks requiring nonlinearity? When would you *prefer* a reservoir with $MC < N$?

---

**Thought Experiment 9.2: Information-Theoretic Interpretation of Intrinsic Plasticity.**

(a) IP maximizes the output entropy of each neuron. Does maximizing the entropy of individual neurons automatically maximize the entropy of the joint output $H(\mathbf{r}(t))$? Why or why not?

(b) For a linear reservoir, the joint entropy $H(\mathbf{r}(t)) = \frac{1}{2}\log\det(2\pi e R_{\mathbf{rr}})$ is maximized when the state covariance matrix $R_{\mathbf{rr}}$ has maximum determinant, subject to fixed trace $\text{tr}(R_{\mathbf{rr}}) = N\sigma^2$. This is achieved when $R_{\mathbf{rr}} = \sigma^2 I$, i.e., when the states are uncorrelated and have equal variance. Does IP tend toward this goal? (Hint: IP adapts gain and bias to equalize the output variances. Does it also decorrelate?)

(c) The gap between individual entropy maximization (what IP does) and joint entropy maximization (the ideal) is a manifestation of the difference between marginal and joint distributions. Propose an extension of IP that would adapt the recurrent weights (not just the gain and bias) to also decorrelate the neuron outputs.

---

## Lab Exercises

**Lab 9.1: SCR vs. ESN Memory Capacity Comparison.**

```python
import numpy as np
import matplotlib.pyplot as plt

def build_scr(N, rho, sigma_in):
    """Build Simple Cycle Reservoir."""
    W = np.zeros((N, N))
    for i in range(1, N):
        W[i, i-1] = rho
    W[0, N-1] = rho  # close the cycle
    
    w_in = np.zeros(N)
    w_in[0] = sigma_in  # input to first neuron only
    
    return W, w_in

def build_random_esn(N, rho, sigma_in, connectivity=0.1, seed=0):
    """Build standard random ESN."""
    np.random.seed(seed)
    W = np.random.randn(N, N) * (np.random.rand(N, N) < connectivity)
    eigs = np.abs(np.linalg.eigvals(W))
    W *= rho / eigs.max()
    w_in = np.random.uniform(-sigma_in, sigma_in, N)
    return W, w_in

def measure_MC_profile(W, w_in, K=60, T=3000, washout=500, ridge=1e-5):
    """Measure MC_k for k=1..K."""
    N = W.shape[0]
    u = np.random.uniform(-1, 1, T + washout + K)
    r = np.zeros(N)
    states = []
    for t in range(T + washout + K):
        r = np.tanh(W @ r + w_in * u[t])
        if t >= washout:
            states.append(r.copy())
    states = np.array(states)
    
    MC = np.zeros(K)
    for k in range(1, K+1):
        X = states[k:k+T]
        y = u[washout:washout+T]
        W_out = np.linalg.lstsq(X.T @ X + ridge * np.eye(N), X.T @ y, rcond=None)[0]
        y_hat = X @ W_out
        MC[k-1] = np.corrcoef(y_hat, y)[0,1]**2
    return MC

N = 20
rho = 0.9

MC_scr = measure_MC_profile(*build_scr(N, rho, 0.1))
MC_esn = measure_MC_profile(*build_random_esn(N, rho, 0.1))

fig, axes = plt.subplots(1, 2, figsize=(12, 4))
axes[0].bar(range(1, len(MC_scr)+1), MC_scr, alpha=0.7, label='SCR')
axes[0].bar(range(1, len(MC_esn)+1), MC_esn, alpha=0.7, label='ESN')
axes[0].set_xlabel('Delay k')
axes[0].set_ylabel('MC_k')
axes[0].set_title('Memory capacity profile')
axes[0].legend()

axes[1].plot([0.1, 0.5, 0.7, 0.9, 0.95, 0.99],
             [sum(measure_MC_profile(*build_scr(N, r, 0.1))) for r in [0.1,0.5,0.7,0.9,0.95,0.99]],
             'b-o', label='SCR')
axes[1].plot([0.1, 0.5, 0.7, 0.9, 0.95, 0.99],
             [sum(measure_MC_profile(*build_random_esn(N, r, 0.1))) for r in [0.1,0.5,0.7,0.9,0.95,0.99]],
             'r-s', label='ESN')
axes[1].axhline(N, color='k', linestyle='--', label=f'MC = N = {N}')
axes[1].set_xlabel(r'Spectral radius $\rho$')
axes[1].set_ylabel('Total MC')
axes[1].set_title('Total memory capacity vs. spectral radius')
axes[1].legend()
plt.tight_layout()
plt.savefig('scr_vs_esn_mc.pdf')
plt.show()
```

**Tasks:**
1. For the SCR at $\rho = 0.9$, do you observe the predicted pattern in the MC profile? Identify the delay slots $k$ that each neuron is responsible for.
2. Compare total MC for SCR and ESN at several spectral radii. For what $\rho$ does the ESN approach the SCR's total MC?
3. Evaluate both architectures on a memory task: $y_t = u_{t-15}$. Which performs better? Does the memory profile predict the winner?

---

**Lab 9.2: Intrinsic Plasticity Pre-Training.**

```python
import numpy as np
import matplotlib.pyplot as plt

def ip_step(a, b, x, eta=0.001, mu=0.2):
    """One IP update step (logistic sigmoid target: exponential)."""
    y = 1.0 / (1.0 + np.exp(-(a * x + b)))
    db = eta * (1 - (2 + mu) * y + mu * y**2)
    da = eta * (1.0 / a + x * db / eta)
    a_new = np.clip(a + da, 0.01, 10.0)
    b_new = b + db
    return a_new, b_new

def pretrain_ip(W, w_in, u_train, eta=0.0005, mu=0.2):
    """Run IP pre-training, return adapted gains and biases."""
    N = W.shape[0]
    a = np.ones(N)
    b = np.zeros(N)
    r = np.zeros(N)
    
    a_history = [a.copy()]
    for ut in u_train:
        x = W @ r + w_in * ut
        for i in range(N):
            a[i], b[i] = ip_step(a[i], b[i], x[i], eta, mu)
        r = 1.0 / (1.0 + np.exp(-(a * x + b)))
        a_history.append(a.copy())
    
    return a, b, np.array(a_history)

N = 50
np.random.seed(7)
W = np.random.randn(N, N) * (np.random.rand(N, N) < 0.1)
eigs = np.abs(np.linalg.eigvals(W))
W *= 0.9 / eigs.max()
w_in = np.random.uniform(-0.5, 0.5, N)

T_train = 5000
u_train = np.random.randn(T_train)

a_final, b_final, a_hist = pretrain_ip(W, w_in, u_train)

# Plot gain convergence
plt.figure(figsize=(10, 4))
plt.plot(a_hist[:, :5])
plt.xlabel('Training step')
plt.ylabel('Gain a_i')
plt.title('IP Gain Convergence (5 neurons)')
plt.savefig('ip_convergence.pdf')

# Show output distribution before and after
r = np.zeros(N)
outputs_before = []
for ut in u_train[:1000]:
    r = np.tanh(W @ r + w_in * ut)  # standard ESN (no IP)
    outputs_before.extend(r[:5])

r = np.zeros(N)
outputs_after = []
for ut in u_train[:1000]:
    x = W @ r + w_in * ut
    r = 1.0 / (1.0 + np.exp(-(a_final * x + b_final)))
    outputs_after.extend(r[:5])

plt.figure(figsize=(8, 4))
plt.hist(outputs_before, bins=50, alpha=0.7, label='Before IP', density=True)
plt.hist(outputs_after, bins=50, alpha=0.7, label='After IP', density=True)
plt.xlabel('Neuron output')
plt.ylabel('Density')
plt.title('Output distribution before and after IP')
plt.legend()
plt.savefig('ip_output_dist.pdf')
plt.show()
```

**Tasks:**
1. Do the gains converge? To what value? Is the convergence consistent across neurons?
2. Compare the output distribution before and after IP. Has it become more exponential-like?
3. Train an ESN readout with and without IP pre-training on a nonlinear task (e.g., NARMA-10). Does IP improve performance? By how much?
4. Try different values of $\mu \in \{0.1, 0.3, 0.5, 1.0\}$. How does $\mu$ affect the equilibrium gain and the task performance?

---

## Key Concepts

See `key_concepts.md`.

## Key Researchers

See `key_researchers.md`.
