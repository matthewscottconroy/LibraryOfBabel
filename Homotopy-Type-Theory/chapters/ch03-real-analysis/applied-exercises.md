# Applied Exercises

The abstract machinery of metric spaces, convergence, continuity, and homotopy finds concrete expression across a surprising range of applied domains. Epsilon-delta reasoning is not merely a formality: it is the precise language used to guarantee that numerical algorithms converge, that signal decompositions converge, that curves are drawable, and that robotic motion is feasible. The following exercises ask you to translate practical engineering and scientific problems into the language of analysis developed in this chapter, and in doing so to see that the technical definitions — Cauchy sequences, uniform continuity, connectedness, path homotopy — are not abstract for their own sake but are exactly what applied work requires.

---

## Exercise A.1: Fixed-Point Iteration and the Contraction Mapping Theorem
*Domain: Numerical Computing / Scientific Computing*

**Setup:** Many numerical problems reduce to finding a fixed point of a function — a value $x^*$ such that $f(x^*) = x^*$. For example, solving $e^{-x} = x$ is equivalent to finding the fixed point of $f(x) = e^{-x}$. The simplest algorithm is fixed-point iteration: start at some $x_0$ and repeatedly apply $f$, generating the sequence $x_0, f(x_0), f(f(x_0)), \ldots$ The hope is that this sequence converges to $x^*$.

The Banach fixed-point theorem (contraction mapping theorem) from Section 2 guarantees that if $f: X \to X$ is a *contraction* on a complete metric space $X$ (meaning there is $c < 1$ with $d(f(x), f(y)) \leq c \cdot d(x, y)$ for all $x, y$), then $f$ has a unique fixed point and fixed-point iteration converges to it from any starting point, at a geometric rate.

**Questions:**

1. Let $f(x) = e^{-x}$ on $X = [0.5, 1.0] \subset \mathbb{R}$. Verify that $f$ maps $X$ into itself (i.e., $f : X \to X$). Compute $f'(x)$ and find the contraction constant $c = \sup_{x \in [0.5, 1.0]} |f'(x)|$. Is $c < 1$? (Note: This is the mean value theorem providing a contraction constant.)

2. The Banach fixed-point theorem gives an explicit error bound: after $n$ iterations, $d(x_n, x^*) \leq \frac{c^n}{1 - c} d(x_1, x_0)$. Starting from $x_0 = 0.5$, how many iterations does this bound guarantee to achieve error less than $10^{-6}$? Implement (mentally or on paper) the first five iterations and compare the actual error to the bound.

3. Newton's method for finding zeros of $g(x) = f(x) - x$ has iteration $x_{n+1} = x_n - g(x_n)/g'(x_n)$. Show that Newton's method is fixed-point iteration for the function $F(x) = x - g(x)/g'(x)$. Under what conditions on $g$ near the fixed point does $F$ satisfy the contraction mapping hypothesis? (Hint: compute $F'(x^*)$ and show it is 0 when $g(x^*) = 0$ and $g'(x^*) \neq 0$. This gives quadratic convergence — contracting by $c^2$ each step rather than $c$.)

*Abstract concept illustrated: Banach fixed-point theorem (completeness + contraction implies unique fixed point); Cauchy sequences as the mechanism of convergence; the quantitative completeness bound giving explicit convergence rates.*

---

## Exercise A.2: Fourier Series and $L^2$ Convergence
*Domain: Signal Processing / Electrical Engineering*

**Setup:** A periodic signal $f : [0, 2\pi] \to \mathbb{R}$ can be approximated by sums of sinusoids: the Fourier series $\sum_{n=-\infty}^{\infty} \hat{f}(n) e^{inx}$ where $\hat{f}(n) = \frac{1}{2\pi} \int_0^{2\pi} f(x) e^{-inx} dx$. In signal processing, $f$ might be an audio waveform, and the Fourier coefficients $\hat{f}(n)$ are its frequency components. The central question is: in what sense does the partial sum $S_N f(x) = \sum_{|n| \leq N} \hat{f}(n) e^{inx}$ converge to $f$ as $N \to \infty$?

The correct notion of convergence here is $L^2$ convergence: $\|S_N f - f\|_{L^2}^2 = \frac{1}{2\pi}\int_0^{2\pi} |S_N f(x) - f(x)|^2 dx \to 0$. The Riesz-Fischer theorem says that the space $L^2([0, 2\pi])$ is complete (it is a Hilbert space), and the Parseval identity says $\|f\|_{L^2}^2 = \sum_{n=-\infty}^{\infty} |\hat{f}(n)|^2$.

**Questions:**

1. Consider the square wave $f(x) = 1$ for $0 < x < \pi$ and $f(x) = -1$ for $\pi < x < 2\pi$. Compute the Fourier coefficients $\hat{f}(n)$ for all $n \in \mathbb{Z}$. (You should find $\hat{f}(n) = 0$ for even $n$ and $\hat{f}(n) = \frac{2}{i\pi n}$ for odd $n$, roughly.) Use Parseval's identity to verify that $\sum |\hat{f}(n)|^2 = \|f\|_{L^2}^2 = 1$.

2. In audio processing, you truncate the Fourier series at frequency $N$ to produce a "band-limited" version of the signal. Interpret the $L^2$ norm $\|S_N f - f\|_{L^2}$ as an energy measurement (total squared error across the period). Using Parseval, express this error purely in terms of Fourier coefficients. For the square wave, write a formula for the approximation error as a function of $N$.

3. The Gibbs phenomenon: near the discontinuity of the square wave, the partial sums $S_N f$ overshoot by approximately 9%, regardless of how large $N$ is. This shows that $L^2$ convergence does not imply pointwise convergence at every point. Construct a formal argument (using the definitions of $L^2$ norm and pointwise convergence) showing that $\|f_n - f\|_{L^2} \to 0$ does not imply $f_n(x) \to f(x)$ for all $x$. (Hint: consider a sequence of functions that "spike" at a single point but shrink in $L^2$.)

*Abstract concept illustrated: Completeness of function spaces ($L^2$ is a complete metric space — a Hilbert space); the distinction between $L^2$ convergence and pointwise convergence; Cauchy sequences in infinite-dimensional metric spaces.*

---

## Exercise A.3: Bézier Curves as Parametric Paths
*Domain: Computer Graphics / CAD / Typography*

**Setup:** A quadratic Bézier curve with control points $P_0, P_1, P_2 \in \mathbb{R}^2$ is defined by:
$$\gamma(t) = (1-t)^2 P_0 + 2t(1-t) P_1 + t^2 P_2, \quad t \in [0, 1].$$
This is precisely a path in $\mathbb{R}^2$ in the sense of Section 7: a continuous function $\gamma : [0, 1] \to \mathbb{R}^2$. The curve starts at $P_0 = \gamma(0)$ and ends at $P_2 = \gamma(1)$, with $P_1$ acting as a "control point" that shapes the curvature. Bézier curves are used in every vector graphics system (PostScript, SVG, fonts) to represent smooth curves.

**Questions:**

1. Verify that $\gamma(0) = P_0$ and $\gamma(1) = P_2$. Show that $\gamma$ is continuous (in fact, smooth). Compute $\gamma'(t)$ and find the tangent vector at $t = 0$ and $t = 1$. Interpret the tangent vectors geometrically in terms of $P_0, P_1, P_2$.

2. Two Bézier curves are homotopic (with fixed endpoints) if one can be continuously deformed into the other while keeping both endpoints fixed. Given two different quadratic Bézier curves $\gamma_1$ and $\gamma_2$ in $\mathbb{R}^2$ with the same endpoints $P_0$ and $P_2$, construct an explicit homotopy $H : [0, 1] \times [0, 1] \to \mathbb{R}^2$ between them. (Hint: $H(t, s) = (1-s)\gamma_1(t) + s\gamma_2(t)$ is a natural candidate. Verify the endpoint conditions.)

3. In $\mathbb{R}^2$, any two paths with the same endpoints are homotopic (since $\mathbb{R}^2$ is simply connected: $\pi_1(\mathbb{R}^2) = 0$). Give an example of two paths in $\mathbb{R}^2 \setminus \{0\}$ (the plane with the origin removed) that are *not* homotopic. (This is the analytic content of the fact that $\pi_1(\mathbb{R}^2 \setminus \{0\}) = \mathbb{Z}$: loops are classified by their winding number around the removed point.) How would a graphics application that operates in a punctured domain need to account for this?

*Abstract concept illustrated: Paths as continuous maps $[0,1] \to X$; homotopy of paths with fixed endpoints; simple connectedness and its failure; the fundamental group as a topological invariant.*

---

## Exercise A.4: Epsilon-Delta Reasoning and Floating-Point Error
*Domain: Numerical Analysis / Floating-Point Arithmetic*

**Setup:** In floating-point arithmetic, real numbers are approximated by values from a finite set $\mathbb{F}$ (the floating-point numbers). Every real number $x$ is rounded to a nearby floating-point number $\text{fl}(x)$ with $|\text{fl}(x) - x| \leq u |x|$, where $u$ is the "machine epsilon" (for double precision, $u \approx 2.2 \times 10^{-16}$). When computing $f(x)$ numerically, the actual computation uses $\text{fl}(x)$ and intermediate rounding errors accumulate. The question of whether a numerical result is accurate is essentially a question about continuity: if $f$ is continuous at $x$, does $\text{fl}(f(\text{fl}(x))) \approx f(x)$?

**Questions:**

1. The function $f(x) = \sqrt{x+1} - \sqrt{x}$ is continuous and positive for $x \geq 0$, with $\lim_{x \to \infty} f(x) = 0$. For large $x$ (say $x = 10^{12}$) on a machine with machine epsilon $u$, the direct formula suffers severe cancellation: $\sqrt{x+1}$ and $\sqrt{x}$ are nearly equal, and their floating-point representations agree in all significant bits, leaving only noise. Use the epsilon-delta definition of continuity to diagnose this: near $x = 10^{12}$, what is $\delta$ such that $|\hat{x} - x| < \delta u \cdot x$ implies $|f(\hat{x}) - f(x)|$ is small? Show that for large $x$, the absolute continuity bound blows up in relative terms. (The fix is to rationalize: $f(x) = \frac{1}{\sqrt{x+1} + \sqrt{x}}$, which is numerically stable.)

2. The function $g(x) = \tan(x)$ near $x = \pi/2$ is discontinuous (it has a vertical asymptote). Formally: for any $M > 0$, there is $\delta > 0$ such that $|x - \pi/2| < \delta$ but $|g(x)| > M$. Translate this discontinuity statement into a practical numerical statement: if you compute $\tan(\text{fl}(\pi/2))$ in floating point (where $\text{fl}(\pi/2)$ is the nearest floating-point approximation to $\pi/2$), what can go wrong? Why does the fact that $\pi$ is irrational and $\text{fl}(\pi/2) \neq \pi/2$ actually save the computation from diverging?

3. A function $f : \mathbb{R} \to \mathbb{R}$ is *uniformly continuous* on $[a, b]$ if for every $\varepsilon > 0$ there exists $\delta > 0$ (depending only on $\varepsilon$, not on $x$) such that $|x - y| < \delta$ implies $|f(x) - f(y)| < \varepsilon$. State the theorem from Section 3 that guarantees uniform continuity for continuous functions on compact sets. Explain why this theorem is the theoretical basis for the claim that a numerical algorithm computing $f$ on $[a, b]$ with fixed floating-point precision produces uniform accuracy: the same $\delta$ works for all input points.

*Abstract concept illustrated: Epsilon-delta definitions of continuity; uniform continuity on compact sets; the Extreme Value Theorem; continuity as a quantitative stability condition for numerical computation.*

---

## Exercise A.5: Path-Connected Components and Robot Motion Planning
*Domain: Robotics / Motion Planning*

**Setup:** A robot operating in a 2D environment occupies a region of space that it cannot overlap with obstacles. The *configuration space* $C$ of the robot is the set of all feasible positions and orientations. The robot can move from configuration $c_1$ to configuration $c_2$ if and only if there is a continuous path $\gamma : [0,1] \to C$ with $\gamma(0) = c_1$ and $\gamma(1) = c_2$ — that is, if $c_1$ and $c_2$ lie in the same *path-connected component* of $C$.

Motion planning is the problem of computing such a path (or determining none exists). The theory of connectedness from Section 5 is directly relevant: the connected components of $C$ partition the set of configurations into equivalence classes of "reachable from each other."

**Questions:**

1. A robot arm consists of two rigid links of lengths $r_1 = 1$ and $r_2 = 0.8$, joined at a shoulder and elbow joint. Each joint can rotate freely through $[0, 2\pi)$. The configuration space is therefore $C = S^1 \times S^1$ (the torus). Show that the torus is connected: given any two configurations $(\theta_1, \phi_1)$ and $(\theta_2, \phi_2)$, construct an explicit continuous path in $C$ between them. (What is the fundamental group $\pi_1(C)$? How many qualitatively different loops are there?)

2. Now suppose the robot is a *rigid body* moving in the plane with a circular obstacle at the origin of radius $R = 0.5$. The configuration space is $C = \mathbb{R}^2 \setminus B(0, 0.5)$ (the plane with an open disk removed). Is $C$ connected? Is $C$ simply connected? Two paths from $c_1$ to $c_2$ avoiding the obstacle are homotopic if one can be continuously deformed into the other while always avoiding the obstacle. What does the fundamental group $\pi_1(C, c_0)$ tell you about the qualitative classes of robot motions?

3. The *potential field* approach to motion planning defines a function $U : C \to \mathbb{R}$ (the "potential") with $U(c_{\text{goal}}) = 0$ and $U(c) > 0$ elsewhere, and moves the robot in the direction of steepest descent. By the Intermediate Value Theorem (applied to the connected configuration space), if there is a path from $c_{\text{start}}$ to $c_{\text{goal}}$, then $U$ takes all values between $U(c_{\text{start}})$ and 0 along some path. However, potential field methods can get trapped at local minima. Explain how this is related to the topological structure of $C$: if $C$ is simply connected, can a "nice" potential function (with no local minima other than the goal) always be constructed? What goes wrong when $C$ has non-trivial $\pi_1$?

*Abstract concept illustrated: Path-connectedness and connected components; the fundamental group as an obstruction to homotopy of paths; the Intermediate Value Theorem on connected spaces; the difference between connected and simply connected.*

---

## Exercise A.6: Compactness and Optimal Control
*Domain: Control Systems / Optimization*

**Setup:** A control system is described by a differential equation $\dot{x}(t) = f(x(t), u(t))$, where $x(t) \in \mathbb{R}^n$ is the state and $u(t) \in U$ is the control input. An optimal control problem asks: among all control functions $u : [0, T] \to U$ satisfying given constraints, find one that minimizes a cost functional $J(u) = \int_0^T L(x(t), u(t)) dt$. The existence of an optimal control (not just an infimum) is a non-trivial analytic question — and it depends on compactness.

**Questions:**

1. Suppose the control set $U = [u_{\min}, u_{\max}]$ is a closed bounded interval (hence compact by Heine-Borel). A standard theorem (Pontryagin, Filippov) asserts that if $f$ and $L$ are continuous and $U$ is compact, then the set of achievable state trajectories is compact in an appropriate function space. State (without proof) the Extreme Value Theorem from Section 4 of this chapter. Explain why compactness of the trajectory set, combined with the Extreme Value Theorem, guarantees that the infimum of $J(u)$ over all admissible controls is attained.

2. Now drop the compactness of $U$: suppose $U = \mathbb{R}$ (unbounded controls). Give an explicit one-dimensional example where $J(u) = \int_0^1 (x(t)^2 + u(t)^2) dt$ has infimum 0 but no minimizer. (Hint: let $x(0) = 1$, $\dot{x} = u$, and use increasingly aggressive controls that drive $x$ to 0 very quickly.) What assumption of the Extreme Value Theorem fails in this case?

3. In the Pontryagin maximum principle, the optimal control $u^*(t)$ is found by maximizing the Hamiltonian $H(x, p, u) = p \cdot f(x, u) - L(x, u)$ over $u \in U$ at each time $t$. This maximization over a compact set $U$ has a solution by the Extreme Value Theorem (since $u \mapsto H(x, p, u)$ is continuous and $U$ is compact). Explain why, if $U$ were open (e.g., $U = (0, 1)$ rather than $[0, 1]$), the maximum might not be attained and the optimal control might not exist as an ordinary function. What is the relationship between this phenomenon and the completeness-versus-compactness distinction in metric space theory?

*Abstract concept illustrated: The Extreme Value Theorem (a continuous function on a compact metric space attains its maximum); Heine-Borel theorem (compact $\Leftrightarrow$ closed and bounded in $\mathbb{R}^n$); compactness as a finiteness condition that enables extrema to be attained.*
