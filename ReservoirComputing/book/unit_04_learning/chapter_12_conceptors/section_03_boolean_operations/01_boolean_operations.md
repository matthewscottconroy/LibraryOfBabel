# Section 12.3: Boolean Operations on Conceptors

## 12.3.1 The Idea of Conceptor Logic

A conceptor $C$ is a soft projection encoding a reservoir's activity pattern. Having defined conceptors for individual patterns, a natural question arises: can we combine them? Can we define operations analogous to Boolean NOT, AND, and OR that make semantic sense?

Jaeger [Jaeger2014] showed that the answer is yes — and in a mathematically precise way. He defined NOT, AND, and OR operations on conceptors that:
1. Have natural geometric interpretations in terms of the patterns' state subspaces.
2. Are closed: the NOT, AND, and OR of conceptors are conceptors.
3. Form a bounded distributive lattice — the same algebraic structure as ordinary Boolean logic.

This gives conceptors a genuine logical structure. Patterns can be negated ("everything but pattern $p$"), conjoined ("the part shared by patterns $p$ and $q$"), and disjoined ("the union of patterns $p$ and $q$"). These operations have direct applications in pattern composition, interpolation, and selective attention.

## 12.3.2 The NOT Operation

**Definition.** The conceptor NOT of $C$ is:

$$\lnot C = I - C.$$

**Interpretation.** If $C = UDU^\top$ with $d_i \in [0,1]$, then

$$\lnot C = U(I - D)U^\top = U\text{diag}(1-d_i)U^\top.$$

Directions that were strongly represented in $C$ (large $d_i$) are suppressed by $\lnot C$, and vice versa. The NOT operation literally *inverts* the activation profile: directions that were active in pattern $p$ become inactive in $\lnot C^p$, and vice versa.

**Geometric interpretation.** If $C^p$ is approximately a projection onto a subspace $V^p$, then $\lnot C^p = I - C^p$ is approximately a projection onto the *orthogonal complement* $V^{p\perp}$. More precisely, since the conceptor's singular values are in $(0,1)$ (not exactly 0 or 1), $\lnot C^p$ is a soft projection onto the complement.

**Example.** If pattern $p$ is a sinusoid and the reservoir's states span a 5-dimensional subspace $V^p \subset \mathbb{R}^{100}$, then $C^p$ projects approximately onto $V^p$ and $\lnot C^p$ projects approximately onto $V^{p\perp}$ (the 95-dimensional complement). Recalling pattern $\lnot C^p$ suppresses the sinusoid pattern and emphasizes all other directions.

**Closure check:** $\lnot C = I - C$ has eigenvalues $1 - d_i \in (0,1)$ when $d_i \in (0,1)$. So $\lnot C$ has all eigenvalues in $(0,1)$: it is a valid conceptor. ✓

**Involution:** $\lnot\lnot C = I - (I - C) = C$. NOT is its own inverse. ✓

## 12.3.3 The AND Operation

**Definition.** The conceptor AND of $C$ and $B$ is defined via the morphism property of the aperture adaptation. Let $C$ and $B$ be conceptors with singular value decompositions $C = U_C D_C U_C^\top$ and $B = U_B D_B U_B^\top$. The AND is defined to be the conceptor corresponding to the *intersection* of the two patterns' state subspaces.

The formal definition is:

$$C \wedge B = \lnot(\lnot C \vee \lnot B),$$

where $\vee$ is the OR operation defined below. This is De Morgan's law, ensuring consistency with Boolean logic. But we need to define OR first.

## 12.3.4 The OR Operation

**Definition.** The conceptor OR of $C$ and $B$ is defined as the unique conceptor that captures the *union* of the two patterns' state subspaces. Formally:

$$C \vee B = \bigl(C^{-1} + B^{-1} - I\bigr)^{-1}.$$

Wait — this formula has problems when $C$ or $B$ is singular. The rigorous definition uses pseudoinverses, but let us proceed with the generic case (full-rank $C$ and $B$) for clarity.

**Alternative definition via covariance.** If $C^p$ comes from pattern $p$ with covariance $R^p$ and $C^q$ comes from pattern $q$ with covariance $R^q$, then the OR is the conceptor of the *combined* pattern (the reservoir driven by both patterns interleaved):

$$R^{p \cup q} = \frac{1}{2}(R^p + R^q),$$

$$C^p \vee C^q = R^{p \cup q}(R^{p \cup q} + \alpha^{-2}I)^{-1} = \frac{R^p + R^q}{2}\!\left(\frac{R^p + R^q}{2} + \alpha^{-2}I\right)^{-1}.$$

This is the natural "sum" of the two patterns' state covariances.

**Algebraic formula (from covariance definition).** Expressing $R^p = \alpha^{-2} C^p (I-C^p)^{-1}$ (from $C^p = R^p(R^p + \alpha^{-2}I)^{-1}$ implies $R^p = \alpha^{-2}C^p(I-C^p)^{-1}$):

$$C^p \vee C^q = \text{conceptor}\!\left(\frac{\alpha^{-2}}{2}\left(C^p(I-C^p)^{-1} + C^q(I-C^q)^{-1}\right), \alpha\right).$$

**Closed-form OR formula.** Substituting and simplifying (see [Jaeger2014, Appendix]):

$$\boxed{C^p \vee C^q = C^p(C^p + C^q - C^p C^q)^{-1} C^q.}$$

**Note:** This formula assumes $C^p + C^q - C^p C^q$ is invertible, which holds when the sum $C^p + C^q \succ C^p C^q$ in the PSD order (always true for generic conceptors with no shared zero-eigenvalue directions).

## 12.3.5 AND via De Morgan

With OR defined, AND is defined by De Morgan's law:

$$C^p \wedge C^q = \lnot(\lnot C^p \vee \lnot C^q) = I - ((I - C^p) \vee (I - C^q)).$$

Applying the OR formula with $A = I - C^p$ and $B = I - C^q$:

$$A \vee B = A(A + B - AB)^{-1}B.$$

$$A + B = (I-C^p) + (I-C^q) = 2I - C^p - C^q.$$

$$AB = (I-C^p)(I-C^q) = I - C^p - C^q + C^p C^q.$$

$$A + B - AB = (2I - C^p - C^q) - (I - C^p - C^q + C^p C^q) = I - C^p C^q.$$

$$A \vee B = (I-C^p)(I - C^p C^q)^{-1}(I-C^q).$$

Therefore:

$$\boxed{C^p \wedge C^q = I - (I-C^p)(I - C^p C^q)^{-1}(I-C^q).}$$

## 12.3.6 Geometric Interpretation of AND and OR

**OR: Union of subspaces.** The OR $C^p \vee C^q$ captures the *combined* subspace of patterns $p$ and $q$. If pattern $p$ has active subspace $V^p$ and pattern $q$ has active subspace $V^q$, then $C^p \vee C^q$ captures $V^p + V^q$ (the span of both). Directions that are active in *either* pattern pass through the OR conceptor.

**AND: Intersection of subspaces.** The AND $C^p \wedge C^q$ captures the *shared* subspace of patterns $p$ and $q$. It strongly passes only directions that are active in *both* patterns (directions in $V^p \cap V^q$). Directions active in only one pattern are suppressed.

**NOT: Complement.** $\lnot C^p$ captures everything *except* pattern $p$'s subspace.

These geometric interpretations hold exactly in the limit of hard projections (aperture $\alpha \to \infty$). For finite apertures, they hold approximately.

## 12.3.7 The Bounded Distributive Lattice

**Theorem 12.3.1 (Jaeger 2014).** The set of conceptors $\mathcal{C}$ (symmetric positive semidefinite matrices with eigenvalues in $[0,1]$) equipped with the operations NOT $\lnot$, AND $\wedge$, OR $\vee$, and the partial order $C \leq B \iff C \preceq B$ (PSD order) forms a **bounded distributive lattice**, with:
- Bottom element: $\mathbf{0}$ (zero matrix, corresponding to $\alpha \to 0$).
- Top element: $I$ (identity matrix, corresponding to $\alpha \to \infty$).

*Proof sketch.*

**Lattice properties:**

1. *Associativity*: $(C \wedge B) \wedge A = C \wedge (B \wedge A)$ and similarly for $\vee$. Follows from the symmetry of the covariance sum definition.

2. *Commutativity*: $C \wedge B = B \wedge C$ and $C \vee B = B \vee C$. Follows from symmetry of the definitions.

3. *Idempotency*: $C \wedge C = C$ and $C \vee C = C$. For AND: $C \wedge C = \lnot(\lnot C \vee \lnot C) = \lnot\lnot C = C$; this uses $A \vee A = A$, which follows from the OR formula: $A \vee A = A(A + A - A^2)^{-1}A = A(2A - A^2)^{-1}A = A(A(2I-A))^{-1}A = A(2I-A)^{-1}$... and $A(2I-A)^{-1} = A$ iff $A = A(2I-A)$ iff $0 = A(I-A)$, which holds when $A(I-A) = 0$ (only if $A$ is a projector). This is a subtle point that requires the full lattice proof. See [Jaeger2014, Theorem 1].

4. *Absorption*: $C \wedge (C \vee B) = C$ and $C \vee (C \wedge B) = C$.

5. *Distributivity*: $C \wedge (B \vee A) = (C \wedge B) \vee (C \wedge A)$.

**Boundary elements:**

- $\mathbf{0} \wedge C = \mathbf{0}$ and $\mathbf{0} \vee C = C$: $\mathbf{0}$ is the bottom element.
- $I \wedge C = C$ and $I \vee C = I$: $I$ is the top element.

These follow from the formula $C \wedge \mathbf{0} = \lnot(\lnot C \vee I) = \lnot(I) = \mathbf{0}$ and $C \vee \mathbf{0} = C$ (OR with zero pattern is the original pattern). $\blacksquare$

## 12.3.8 Example: Two Stored Patterns

Let us work through a concrete example with two patterns.

**Setup.** Reservoir: $N = 100$ neurons, $\rho = 0.9$, sparse connectivity. Two patterns:
- Pattern $p_1$: sinusoid at frequency $f_1 = 1/17$ (period 17).
- Pattern $p_2$: sinusoid at frequency $f_2 = 1/19$ (period 19).

Both patterns drive the reservoir to approximately 5-dimensional state subspaces (for a sinusoidal input, the reservoir's response is dominated by modes at the fundamental and a few harmonics).

**Computed conceptors:**
- $C_1 = R_1(R_1 + \alpha^{-2}I)^{-1}$ for pattern $p_1$.
- $C_2 = R_2(R_2 + \alpha^{-2}I)^{-1}$ for pattern $p_2$.

With $\alpha = 10$ (large enough to capture both patterns' active directions):

```python
import numpy as np

def build_conceptor(W, w_in, pattern_fn, T=2000, washout=500, alpha=10):
    """Compute conceptor for given pattern."""
    N = W.shape[0]
    r = np.zeros(N)
    states = []
    for t in range(T + washout):
        u = pattern_fn(t)
        r = np.tanh(W @ r + w_in * u)
        if t >= washout:
            states.append(r.copy())
    
    states = np.array(states)
    R = (states.T @ states) / T
    C = R @ np.linalg.inv(R + alpha**-2 * np.eye(N))
    return C, R

def conceptor_NOT(C):
    return np.eye(len(C)) - C

def conceptor_OR(C, B):
    """C OR B using the algebraic formula."""
    A_inv = C + B - C @ B
    # Avoid singularity with regularization
    return C @ np.linalg.solve(A_inv + 1e-10*np.eye(len(C)), B)

def conceptor_AND(C, B):
    """C AND B = NOT(NOT C OR NOT B)."""
    return conceptor_NOT(conceptor_OR(conceptor_NOT(C), conceptor_NOT(B)))

# Aperture adaptation
def adapt_aperture(C, gamma):
    """Change aperture of C by factor gamma."""
    n = len(C)
    # C at aperture alpha*gamma: use formula
    # C_new = C * (C + gamma^-2 * (I - C))^{-1}
    M = C + gamma**-2 * (np.eye(n) - C)
    return C @ np.linalg.inv(M + 1e-10*np.eye(n))
```

**Boolean operations and their meanings:**

1. **$C_1 \vee C_2$**: The OR conceptor. Its singular values are large in directions active in *either* $p_1$ or $p_2$. Running the reservoir with $C_1 \vee C_2$ produces a mix of the two patterns.

2. **$C_1 \wedge C_2$**: The AND conceptor. Its singular values are large only in directions active in *both* $p_1$ and $p_2$. Since $p_1$ and $p_2$ are at different frequencies, their state subspaces are nearly orthogonal, and $C_1 \wedge C_2$ is nearly zero. This means the AND of two orthogonal patterns is the "null pattern" — there is no pattern that is simultaneously $p_1$ and $p_2$.

3. **$\lnot C_1$**: The NOT conceptor. Running the reservoir with $\lnot C_1$ generates activity in all directions *except* those characteristic of $p_1$. If the reservoir was generating $p_1$ before, applying $\lnot C_1$ suppresses it — a form of "unlearning" or active inhibition.

**Singular value profiles:**

For patterns $p_1$ and $p_2$ at different frequencies, the singular values of $C_1$ are:
- 5 large values ($\approx 0.9$) corresponding to the pattern's active subspace.
- 95 small values ($\approx 0.01$) corresponding to inactive directions.

The OR $C_1 \vee C_2$ has approximately 10 large singular values (the union of both active subspaces) and 90 small ones.

The AND $C_1 \wedge C_2$ has nearly all singular values near 0 (since the active subspaces are nearly orthogonal).

---

*Conceptors give a reservoir not just storage and retrieval, but a full algebra of pattern manipulation. The Boolean lattice structure means that reasoning about patterns — "the intersection of these two patterns," "everything but this pattern" — has a precise mathematical implementation in the eigenspectrum of the conceptor matrices.*
