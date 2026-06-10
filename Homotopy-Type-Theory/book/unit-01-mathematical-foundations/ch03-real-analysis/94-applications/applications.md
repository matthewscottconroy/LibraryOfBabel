# Applications: Real Analysis

## 1. Machine Learning and Optimization

Modern machine learning is built on analysis. Training a neural network means finding the minimum of a loss function L(θ) over a high-dimensional parameter space θ ∈ ℝⁿ. The tools are gradient descent (and its variants), which require continuity and differentiability of L.

The theoretical foundation: gradient descent converges to a local minimum when the loss function is smooth (Lipschitz continuous gradient). Convergence rates depend on properties like *L-smoothness* (the gradient is Lipschitz continuous with constant L) and *strong convexity* (the function curves upward at a controlled rate). These are metric-space analytic properties.

The Banach Fixed Point theorem appears in proving convergence of iterative algorithms: the update step θ ↦ θ - η∇L(θ) is a contraction mapping (for small enough learning rate η), so the sequence of parameter values converges to the fixed point — the optimal parameters.

Compactness ensures that optimization over a compact set achieves its minimum (Extreme Value Theorem). Most practical neural network training is over non-compact spaces, which is why local minima and saddle points are significant concerns. The non-compactness means the minimum may not be achieved.

## 2. Signal Processing and Function Spaces

Fourier analysis — decomposing signals into frequency components — lives in the function space L²(ℝ) (square-integrable functions), which is a complete inner product space (Hilbert space). The Fourier transform is an isometric bijection L²(ℝ) → L²(ℝ) (Plancherel's theorem).

The practical consequence: digital signal processing (audio compression, image processing, telecommunications) works in these function spaces. MP3 audio compression represents audio as a time-frequency decomposition, discarding components below a perceptual threshold. JPEG image compression uses a discrete cosine transform (a Fourier-like decomposition on a finite grid).

The analysis that makes this work: completeness of L²(ℝ) ensures that truncating the Fourier series (keeping only some frequency components) produces a sequence of approximations converging to the original signal. The convergence is in L² norm — the sum of squared errors approaches zero. This is the completeness of the space doing the work.

## 3. Partial Differential Equations: Existence and Uniqueness

The heat equation, wave equation, and Schrödinger equation are all partial differential equations (PDEs). Proving that these equations have solutions — and that the solutions are unique — uses the analytic tools of this chapter.

**The heat equation** ∂u/∂t = Δu (where Δ is the Laplacian) on a bounded domain Ω ⊆ ℝⁿ with boundary conditions. To prove existence of a solution:

1. Discretize: find solutions uₙ to a discrete approximation.
2. Show (uₙ) is an equicontinuous and uniformly bounded family.
3. Apply the Arzelà-Ascoli theorem: extract a uniformly convergent subsequence.
4. Show the limit satisfies the original PDE.

The Arzelà-Ascoli theorem is the compactness argument that makes the existence proof work. Without completeness (to make limits exist) and compactness (to extract convergent subsequences), the argument fails.

**Sobolev spaces** — completions of smooth functions in norms measuring both the function and its derivatives — are the natural function spaces for PDEs. The key theorem: a function in the Sobolev space W^{k,p} is actually k-times differentiable (in an appropriate sense) if p and k are large enough (Sobolev embedding theorems). These are the analytic tools of mathematical physics.

## 4. Computer Graphics and Homotopy

In computer animation and game rendering, smooth deformations of 3D objects are modeled using homotopy theory. A *morph* from one shape to another is a continuous family of shapes: a homotopy between the initial and final configurations.

*Level sets* and *marching cubes* algorithms extract surfaces from volumetric data by finding connected components of a function's level sets — an application of connectedness theory.

*Bezier curves* and *NURBS* (non-uniform rational B-splines) are parameterized paths in 3D space, used to model smooth curves and surfaces. The continuity of these paths (which is ensured by their construction from smooth basis functions) corresponds to the analytic definition of path continuity.

*Topological data analysis (TDA)* uses homotopy-theoretic tools — specifically, persistent homology — to detect "shape" features in high-dimensional data. It computes how the connectivity and hole structure of a dataset changes as you vary a parameter (a distance threshold). The fundamental group and higher homology groups appear as invariants of the data's shape.

## 5. Economics: Fixed Points and Equilibria

**Nash's theorem** (1951): every finite game has a Nash equilibrium — a strategy profile where no player can improve by unilaterally changing strategy. The proof uses the *Brouwer fixed point theorem*: every continuous function from a convex compact set (like the simplex of mixed strategies) to itself has a fixed point.

The Brouwer fixed point theorem requires:
- Continuity of the map (the "best response" map is continuous by the maximum theorem).
- Compactness of the domain (the space of mixed strategies is compact — a product of simplices).
- Convexity of the domain.

This is metric space analysis in service of game theory and economics. More generally, the *Kakutani fixed point theorem* (a version for set-valued maps) is the main tool for proving the existence of equilibria in more complex economic models.

**Walras's theorem**: in a general equilibrium model with multiple goods, prices adjust until markets clear. The existence of an equilibrium price vector uses fixed-point arguments in the space of prices — again, compactness and continuity of the excess demand function.

## 6. Homotopy in Robotics and Motion Planning

A robot's configuration space C is the set of all possible positions of the robot (positions and orientations of all joints). Motion planning asks: is there a path in C from the starting configuration to the goal configuration, avoiding obstacles?

The *topological complexity* of a configuration space measures how many "regions" the space of path pairs decomposes into — a homotopy-theoretic invariant. Higher topological complexity means motion planning is harder algorithmically.

Obstacle avoidance: the free space C_free = C \ obstacles has a specific topology. If C_free is path-connected, the robot can always reach its goal. If C_free has holes, some pairs of configurations may be separated, and the fundamental group records which loops in configuration space are "obstructed" by obstacles.

Continuous motion (paths in C_free) corresponds exactly to the analytic notion of a path — a continuous function from [0,1] into the configuration space. The homotopy between two motions is a continuous deformation of one motion into the other. The analytic tools of continuity and the topological tools of path-homotopy work together.

## 7. Constructive Mathematics and Computable Analysis

In computable analysis, a real number x is *computable* if there is an algorithm that, given ε > 0, outputs a rational q with |x - q| < ε. Computable real numbers are dense in ℝ.

A function f: ℝ → ℝ is *computable* if there is an algorithm that, given a computable representation of x and ε > 0, outputs a rational approximation to f(x) within ε. The key theorem: every computable function on a compact interval is uniformly computable — the modulus of uniform continuity is itself computable.

This connects the analytic notion of uniform continuity (the Heine-Cantor theorem) to computability theory: on compact domains, continuity is not just uniform but *uniformly computable*. Compactness is the bridge between analysis and computation.

In HoTT's constructive setting, the real numbers are defined as a HIT with computable Cauchy sequences as generators. The analytic theorems — completeness, compactness of [0,1], the IVT — all hold in this constructive setting, but their proofs must be constructive: they must exhibit witnesses, not just assert existence. The IVT, for instance, requires finding the point where the function equals c — and constructively, you must describe an algorithm for approximating it.
