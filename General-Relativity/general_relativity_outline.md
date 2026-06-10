# The Road to General Relativity: A Complete Concept Map
## From First Principles to the Frontiers of Gravitational Physics

---

# UNIT I: Logic, Proof, and the Language of Mathematics

*Unit Introduction: Before any physics can be done, one must command the language in which physics is written. This unit establishes the logical and set-theoretic foundations that underpin all subsequent mathematics. Mastery here is non-negotiable: every theorem in differential geometry and every derivation in general relativity rests on the ideas introduced in these chapters.*

---

## Chapter 1: Logic and Proof

*Chapter Introduction: Mathematics is distinguished from other disciplines by its insistence on proof. This chapter introduces propositional and predicate logic, the grammar of mathematical argument, and the major proof strategies a student will use throughout this curriculum.*

### 1.1 Propositional Logic
- 1.1.1 Propositions, truth values, and truth tables
- 1.1.2 Logical connectives: conjunction, disjunction, negation, implication, biconditional
- 1.1.3 Tautologies, contradictions, and logical equivalence
- 1.1.4 Laws of logic: De Morgan's laws, distributivity, contrapositive

### 1.2 Predicate Logic
- 1.2.1 Predicates and quantifiers: universal and existential
- 1.2.2 Nested quantifiers and their meaning
- 1.2.3 Negating quantified statements
- 1.2.4 Free and bound variables

### 1.3 Methods of Proof
- 1.3.1 Direct proof
- 1.3.2 Proof by contrapositive
- 1.3.3 Proof by contradiction
- 1.3.4 Proof by cases
- 1.3.5 Mathematical induction: weak and strong forms
- 1.3.6 Well-ordering principle and its equivalence to induction

### 1.4 Common Proof Pitfalls
- 1.4.1 Circular reasoning
- 1.4.2 Unverified assumptions
- 1.4.3 Quantifier errors

---

**Key Concepts:** proposition, predicate, quantifier, tautology, contradiction, direct proof, contrapositive, contradiction, induction

**Important Figures:** Aristotle, Gottfried Wilhelm Leibniz, George Boole, Gottlob Frege, Bertrand Russell, Alfred North Whitehead

**Additional Reading and Primary Sources:**
- Euclid, *Elements* (Books I–II)
- Russell & Whitehead, *Principia Mathematica* (selections)
- Velleman, *How to Prove It*
- Hammack, *Book of Proof* (open access)

**Exercises:**
1. Construct truth tables for all sixteen binary connectives.
2. Prove that √2 is irrational using proof by contradiction.
3. Prove by induction that the sum of the first n integers equals n(n+1)/2.
4. Negate the statement: "For every ε > 0 there exists δ > 0 such that |f(x) - L| < ε whenever 0 < |x - a| < δ."

**Thought Experiments:**
- The Liar's Paradox: "This statement is false." Why does self-reference break propositional logic?
- Hilbert's Program: What would it mean for mathematics to be complete and consistent? Why did Gödel's incompleteness theorems end this dream?

---

## Chapter 2: Set Theory

*Chapter Introduction: Set theory provides the common foundation for all modern mathematics. The structures that arise in differential geometry — manifolds, vector spaces, topological spaces — are all built from sets. This chapter covers naive and axiomatic set theory to the degree needed for the rest of the curriculum.*

### 2.1 Naive Set Theory
- 2.1.1 Sets, elements, and membership
- 2.1.2 Subset, superset, equality of sets
- 2.1.3 Set operations: union, intersection, complement, difference
- 2.1.4 Power set and Cartesian product
- 2.1.5 Russell's paradox and the need for axioms

### 2.2 Axiomatic Set Theory (ZFC Overview)
- 2.2.1 Axiom of extensionality
- 2.2.2 Axiom of pairing and union
- 2.2.3 Axiom of infinity
- 2.2.4 Axiom of choice and its equivalents (Zorn's lemma, well-ordering theorem)

### 2.3 Relations and Functions
- 2.3.1 Ordered pairs and Cartesian products
- 2.3.2 Relations: reflexive, symmetric, transitive, antisymmetric
- 2.3.3 Equivalence relations and partitions
- 2.3.4 Partial and total orders
- 2.3.5 Functions: injective, surjective, bijective
- 2.3.6 Composition and inverse functions
- 2.3.7 Indexed families and the axiom of choice

### 2.4 Cardinality and Infinity
- 2.4.1 Finite sets and counting
- 2.4.2 Countable and uncountable sets
- 2.4.3 Cantor's diagonal argument
- 2.4.4 Cantor's theorem: |P(A)| > |A|
- 2.4.5 Cardinal arithmetic; beth and aleph numbers (survey)

---

**Key Concepts:** set, element, subset, function, relation, equivalence class, cardinality, countability, Cartesian product, axiom of choice

**Important Figures:** Georg Cantor, Richard Dedekind, Ernst Zermelo, Abraham Fraenkel, Bertrand Russell, John von Neumann, Kurt Gödel

**Additional Reading and Primary Sources:**
- Cantor, *Contributions to the Founding of the Theory of Transfinite Numbers*
- Halmos, *Naive Set Theory*
- Enderton, *Elements of Set Theory*

**Exercises:**
1. Prove that the rational numbers are countable.
2. Prove that the real numbers are uncountable (Cantor's diagonal argument).
3. Show that the power set of a set with n elements has 2ⁿ elements.
4. Give an example of an equivalence relation on ℝ and describe the resulting partition.

**Thought Experiments:**
- Hilbert's Hotel: How can a fully occupied infinite hotel accommodate infinitely many new guests?
- What does it mean for two infinite sets to have the same "size"?

---

## Chapter 3: Number Systems

*Chapter Introduction: The progression from natural numbers to real and complex numbers is both a historical journey and a conceptual one. Each extension resolves a limitation of the previous system and opens new mathematical territory. The real numbers, in particular, are the arena of calculus.*

### 3.1 Natural Numbers and Integers
- 3.1.1 Peano axioms
- 3.1.2 Arithmetic operations and their properties
- 3.1.3 Integer arithmetic; divisibility and primes
- 3.1.4 The division algorithm; greatest common divisor

### 3.2 Rational Numbers
- 3.2.1 Construction from integers as equivalence classes of pairs
- 3.2.2 Density of the rationals
- 3.2.3 Failure to capture all lengths: √2 ∉ ℚ

### 3.3 Real Numbers
- 3.3.1 Dedekind cuts construction
- 3.3.2 Cauchy sequences construction
- 3.3.3 The completeness axiom and least upper bounds
- 3.3.4 Archimedean property
- 3.3.5 Decimal representations

### 3.4 Complex Numbers
- 3.4.1 Definition as ordered pairs of reals
- 3.4.2 Algebraic operations; complex conjugate and modulus
- 3.4.3 The complex plane; polar form
- 3.4.4 Euler's formula: e^(iθ) = cos θ + i sin θ
- 3.4.5 Roots of unity
- 3.4.6 Fundamental theorem of algebra (statement)

---

**Key Concepts:** Peano axioms, rational density, Dedekind cut, completeness, least upper bound, complex plane, Euler's formula

**Important Figures:** Giuseppe Peano, Richard Dedekind, Augustin-Louis Cauchy, Carl Friedrich Gauss, Leonhard Euler

**Additional Reading and Primary Sources:**
- Dedekind, *Continuity and Irrational Numbers*
- Rudin, *Principles of Mathematical Analysis*, Ch. 1
- Needham, *Visual Complex Analysis*, Ch. 1

**Exercises:**
1. Construct √2 as a Dedekind cut.
2. Prove the density of rationals: between any two reals there is a rational.
3. Compute all fifth roots of unity and plot them in the complex plane.
4. Prove that ℂ is algebraically closed (sketch).

**Thought Experiments:**
- Why does the number line "feel" continuous if it has gaps (the irrationals filling in the rationals)?
- What does it mean geometrically to multiply two complex numbers?

---

# UNIT II: Mathematical Foundations — Calculus

*Unit Introduction: Calculus is the mathematics of change and accumulation. It is the indispensable engine of theoretical physics. This unit develops single-variable calculus rigorously, then extends to series, which are essential for perturbative methods throughout physics.*

---

## Chapter 4: Single-Variable Differential Calculus

*Chapter Introduction: The derivative captures the instantaneous rate of change of a function. This concept, once made rigorous, becomes the local linear approximation of a map — an idea that generalizes all the way to the tangent space of a manifold in differential geometry.*

### 4.1 Limits and Continuity
- 4.1.1 Informal notion of a limit
- 4.1.2 Epsilon-delta definition of a limit
- 4.1.3 Limit laws and computation
- 4.1.4 One-sided limits; limits at infinity
- 4.1.5 Continuity at a point and on an interval
- 4.1.6 The intermediate value theorem
- 4.1.7 Extreme value theorem

### 4.2 The Derivative
- 4.2.1 Definition as a limit of a difference quotient
- 4.2.2 Geometric interpretation: slope of the tangent line
- 4.2.3 Differentiability implies continuity
- 4.2.4 Differentiation rules: power, product, quotient, chain
- 4.2.5 Derivatives of elementary functions
- 4.2.6 Higher-order derivatives and notation

### 4.3 Applications of the Derivative
- 4.3.1 Mean value theorem and its corollaries
- 4.3.2 Local extrema and critical points
- 4.3.3 Concavity and inflection points
- 4.3.4 L'Hôpital's rule
- 4.3.5 Newton's method

### 4.4 Implicit Differentiation and Related Rates
- 4.4.1 Implicit differentiation
- 4.4.2 Inverse function theorem (single-variable)
- 4.4.3 Derivatives of inverse trigonometric functions
- 4.4.4 Related rates problems

---

**Key Concepts:** limit, continuity, derivative, differentiability, mean value theorem, chain rule, implicit differentiation

**Important Figures:** Isaac Newton, Gottfried Wilhelm Leibniz, Augustin-Louis Cauchy, Karl Weierstrass, Bernard Bolzano

**Additional Reading and Primary Sources:**
- Newton, *Method of Fluxions* (historical)
- Leibniz, *Nova Methodus* (historical)
- Spivak, *Calculus*
- Apostol, *Calculus*, Vol. 1

**Exercises:**
1. Using the epsilon-delta definition, prove lim_{x→2} (3x - 1) = 5.
2. Differentiate f(x) = x^x using logarithmic differentiation.
3. Prove the mean value theorem from Rolle's theorem.
4. Find all critical points and classify them for f(x) = x⁴ - 4x³.

**Thought Experiments:**
- If velocity is the derivative of position, what is the physical meaning of the second derivative? The third (jerk)?
- Zeno's paradoxes of motion: how does the limit definition of the derivative resolve the paradox of Achilles and the tortoise?

---

## Chapter 5: Single-Variable Integral Calculus

*Chapter Introduction: Integration is both the inverse of differentiation and a means of computing accumulated quantities. The Riemann integral formalizes the area under a curve; the fundamental theorem of calculus reveals the deep duality between the two operations.*

### 5.1 The Riemann Integral
- 5.1.1 Partitions, Riemann sums, upper and lower sums
- 5.1.2 Definition of the Riemann integral
- 5.1.3 Integrability: continuous functions are integrable
- 5.1.4 Properties of the definite integral

### 5.2 The Fundamental Theorem of Calculus
- 5.2.1 FTC Part I: the derivative of the integral
- 5.2.2 FTC Part II: evaluation of definite integrals via antiderivatives
- 5.2.3 Antiderivatives and indefinite integrals

### 5.3 Techniques of Integration
- 5.3.1 Substitution (change of variables)
- 5.3.2 Integration by parts
- 5.3.3 Trigonometric integrals and substitutions
- 5.3.4 Partial fractions
- 5.3.5 Reduction formulas

### 5.4 Improper Integrals
- 5.4.1 Infinite limits of integration
- 5.4.2 Integrands with singularities
- 5.4.3 Comparison tests for convergence

### 5.5 Applications of Integration
- 5.5.1 Area between curves
- 5.5.2 Arc length
- 5.5.3 Volumes of revolution
- 5.5.4 Physical applications: work, center of mass

---

**Key Concepts:** Riemann sum, definite integral, fundamental theorem of calculus, antiderivative, improper integral, arc length

**Important Figures:** Bernhard Riemann, Newton, Leibniz, Cauchy, Henri Lebesgue

**Additional Reading and Primary Sources:**
- Riemann, *On the Representability of a Function by a Trigonometric Series* (historical)
- Spivak, *Calculus*, Chapters 13–19
- Apostol, *Calculus*, Vol. 1

**Exercises:**
1. Evaluate ∫₀^∞ e^{-x²} dx using a double integral trick.
2. Compute the arc length of y = x^{3/2} from x = 0 to x = 4.
3. Prove that every continuous function on [a,b] is Riemann integrable.
4. Evaluate ∫ x² ln(x) dx by parts.

**Thought Experiments:**
- The integral as an infinite sum of infinitesimal rectangles: why does making the rectangles infinitely thin give an exact answer?
- Can you have a function that is integrable but not differentiable everywhere?

---

## Chapter 6: Sequences, Series, and Approximation

*Chapter Introduction: Infinite series are the backbone of analytical methods in physics. Taylor series allow local approximation of any smooth function by a polynomial — this is the mathematical engine behind perturbation theory, the weak-field limit of general relativity, and countless other tools.*

### 6.1 Sequences
- 6.1.1 Definition and convergence
- 6.1.2 Limit theorems for sequences
- 6.1.3 Monotone convergence theorem
- 6.1.4 Subsequences and Bolzano-Weierstrass theorem
- 6.1.5 Cauchy sequences and completeness

### 6.2 Infinite Series
- 6.2.1 Partial sums and convergence
- 6.2.2 Geometric series; telescoping series
- 6.2.3 Divergence test
- 6.2.4 Comparison, limit comparison, and integral tests
- 6.2.5 Ratio and root tests
- 6.2.6 Alternating series test; absolute and conditional convergence
- 6.2.7 Rearrangement theorem (Riemann)

### 6.3 Power Series and Taylor Series
- 6.3.1 Power series: radius and interval of convergence
- 6.3.2 Term-by-term differentiation and integration
- 6.3.3 Taylor and Maclaurin series
- 6.3.4 Taylor's theorem with remainder
- 6.3.5 Common Taylor series: e^x, sin x, cos x, ln(1+x), (1+x)^α
- 6.3.6 Binomial series
- 6.3.7 Applications: limits, approximate computation

### 6.4 Fourier Series (Introduction)
- 6.4.1 Periodic functions and trigonometric polynomials
- 6.4.2 Fourier coefficients
- 6.4.3 Convergence of Fourier series (statement)
- 6.4.4 Parseval's identity

---

**Key Concepts:** convergence, Cauchy sequence, power series, radius of convergence, Taylor series, Fourier series, remainder estimate

**Important Figures:** Brook Taylor, Colin Maclaurin, Jean-Baptiste Joseph Fourier, Niels Henrik Abel, Karl Weierstrass

**Additional Reading and Primary Sources:**
- Fourier, *The Analytical Theory of Heat* (historical, selections)
- Rudin, *Principles of Mathematical Analysis*, Chapters 3, 7–8
- Apostol, *Calculus*, Vol. 1, Chapters 10–11

**Exercises:**
1. Derive the Taylor series for e^x from the definition and prove it converges everywhere.
2. Use the binomial series to derive an approximation for (1+x)^{1/2} valid for small x — this will appear again in the weak-field limit of GR.
3. Find the Fourier series of the square wave and sketch partial sums.
4. Prove that the radius of convergence of Σ aₙxⁿ is R = 1/limsup|aₙ|^{1/n}.

**Thought Experiments:**
- Why does the Taylor series of some functions (like e^{-1/x²}) fail to equal the function everywhere even though the series converges?
- What does it mean physically that a square wave can be built from pure sinusoids?

---

# UNIT III: Mathematical Foundations — Multivariable and Vector Mathematics

*Unit Introduction: Physics happens in space, and space has more than one dimension. This unit develops calculus for functions of several variables, introduces vector calculus (the language of electromagnetism and fluid mechanics), and establishes linear algebra as the algebraic framework for all of the above. These tools are the direct precursor to the tensor calculus used in general relativity.*

---

## Chapter 7: Multivariable Calculus

*Chapter Introduction: Extending calculus to higher dimensions requires care: limits become directional, derivatives become matrices, and integration requires new techniques. Every concept here has a direct analogue in the differential geometry of curved spacetime.*

### 7.1 Euclidean Space and Geometry
- 7.1.1 ℝⁿ as a vector space; norms and distances
- 7.1.2 Dot product and angles
- 7.1.3 Lines, planes, and hyperplanes
- 7.1.4 Open and closed sets in ℝⁿ; neighborhoods

### 7.2 Functions of Several Variables
- 7.2.1 Graphs and level sets
- 7.2.2 Limits in ℝⁿ; path dependence
- 7.2.3 Continuity

### 7.3 Partial and Directional Derivatives
- 7.3.1 Partial derivatives; notation ∂f/∂xᵢ
- 7.3.2 Geometric interpretation of partial derivatives
- 7.3.3 Higher-order partials; Clairaut's theorem on equality of mixed partials
- 7.3.4 Directional derivative
- 7.3.5 The gradient vector ∇f

### 7.4 The Total Derivative and Jacobian
- 7.4.1 Total derivative as a linear map
- 7.4.2 The Jacobian matrix
- 7.4.3 Chain rule in matrix form
- 7.4.4 Implicit function theorem
- 7.4.5 Inverse function theorem in ℝⁿ

### 7.5 Optimization in Several Variables
- 7.5.1 Critical points: gradient equals zero
- 7.5.2 Second derivative test: Hessian matrix
- 7.5.3 Constrained optimization and Lagrange multipliers

### 7.6 Multiple Integrals
- 7.6.1 Double integrals over rectangles and general regions
- 7.6.2 Iterated integrals and Fubini's theorem
- 7.6.3 Change of variables formula; Jacobian determinant
- 7.6.4 Polar, cylindrical, and spherical coordinates
- 7.6.5 Triple integrals; applications

---

**Key Concepts:** gradient, Jacobian, Hessian, chain rule, Clairaut's theorem, implicit function theorem, multiple integral, change of variables

**Important Figures:** Leibniz, Cauchy, Carl Gustav Jacob Jacobi, Alexis Clairaut, Joseph-Louis Lagrange

**Additional Reading and Primary Sources:**
- Spivak, *Calculus on Manifolds*
- Apostol, *Calculus*, Vol. 2
- Hubbard & Hubbard, *Vector Calculus, Linear Algebra, and Differential Forms*

**Exercises:**
1. Find and classify all critical points of f(x,y) = x³ - 3xy².
2. Use Lagrange multipliers to find the point on the ellipse x²/4 + y²/9 = 1 closest to (1,2).
3. Evaluate ∫∫ e^{-(x²+y²)} dx dy over all of ℝ² by converting to polar coordinates.
4. Verify Clairaut's theorem for f(x,y) = sin(xy).

**Thought Experiments:**
- A function can have all partial derivatives equal to zero at a point yet not have a local minimum or maximum there (saddle point). How does the Hessian capture this?
- The Jacobian determinant measures how volumes scale under coordinate transformations — why does this matter for integrals?

---

## Chapter 8: Vector Calculus

*Chapter Introduction: Vector calculus fuses multivariable calculus with the algebra of vectors. The gradient, divergence, curl, and the integral theorems of Green, Stokes, and Gauss are the mathematical backbone of classical electromagnetism — and the latter two theorems are special cases of the generalized Stokes theorem on differential forms.*

### 8.1 Vector Fields
- 8.1.1 Definition and examples in ℝ² and ℝ³
- 8.1.2 Visualization: field lines
- 8.1.3 Conservative vector fields; potential functions
- 8.1.4 Curl and irrotational fields
- 8.1.5 Divergence and solenoidal fields

### 8.2 Line Integrals
- 8.2.1 Line integrals of scalar functions
- 8.2.2 Line integrals of vector fields: work
- 8.2.3 Path independence and the gradient theorem
- 8.2.4 Exact and inexact differentials

### 8.3 Surface Integrals
- 8.3.1 Parametrized surfaces; surface area
- 8.3.2 Flux of a vector field through a surface
- 8.3.3 Orientation of surfaces

### 8.4 The Integral Theorems
- 8.4.1 Green's theorem in the plane
- 8.4.2 Stokes' theorem: ∮∂S F·dr = ∬S (∇×F)·dS
- 8.4.3 Gauss's (divergence) theorem: ∯∂V F·dS = ∭V (∇·F) dV
- 8.4.4 Physical interpretations and applications
- 8.4.5 The theorems as special cases of the generalized Stokes theorem (preview)

### 8.5 Coordinate Systems and Differential Operators
- 8.5.1 Gradient, divergence, curl, and Laplacian in Cartesian coordinates
- 8.5.2 In cylindrical coordinates
- 8.5.3 In spherical coordinates
- 8.5.4 General orthogonal curvilinear coordinates

---

**Key Concepts:** gradient, divergence, curl, Laplacian, line integral, flux, Green's theorem, Stokes' theorem, divergence theorem, conservative field, exact differential

**Important Figures:** George Green, George Gabriel Stokes, Carl Friedrich Gauss, Hermann von Helmholtz, Lord Kelvin

**Additional Reading and Primary Sources:**
- Green, *Essay on the Application of Mathematical Analysis to the Theories of Electricity and Magnetism* (historical)
- Schey, *div, grad, curl, and all that*
- Griffiths, *Introduction to Electrodynamics*, Mathematical Introduction

**Exercises:**
1. Verify Stokes' theorem for F = (y, -x, 0) over the upper hemisphere of the unit sphere.
2. Compute the divergence and curl of F = (x², xy, xyz) in Cartesian coordinates.
3. Express the Laplacian in spherical coordinates from scratch.
4. Show that a conservative field has zero curl.

**Thought Experiments:**
- Why does the divergence theorem "count" sources and sinks inside a volume by measuring the net flow through its surface?
- A field with zero curl is locally conservative — but globally, topology can obstruct this. What is a physical example?

---

## Chapter 9: Linear Algebra

*Chapter Introduction: Linear algebra is the study of linear maps and their structure. In physics, it is everywhere: the state of a system lives in a vector space; symmetries are linear transformations; tensors are multilinear maps. This chapter is foundational for everything that follows — especially for the tensor analysis used in general relativity.*

### 9.1 Vector Spaces
- 9.1.1 Axiomatic definition: field, vector addition, scalar multiplication
- 9.1.2 Examples: ℝⁿ, polynomial spaces, function spaces, matrices
- 9.1.3 Subspaces
- 9.1.4 Span, linear independence, basis
- 9.1.5 Dimension; finite and infinite-dimensional spaces

### 9.2 Linear Maps
- 9.2.1 Definition and examples
- 9.2.2 Kernel and image; rank-nullity theorem
- 9.2.3 Matrix representation of a linear map
- 9.2.4 Composition and matrix multiplication
- 9.2.5 Change of basis; similarity transformations

### 9.3 Systems of Linear Equations
- 9.3.1 Gaussian elimination and row reduction
- 9.3.2 Row echelon and reduced row echelon form
- 9.3.3 Existence and uniqueness of solutions
- 9.3.4 LU decomposition

### 9.4 Determinants
- 9.4.1 Axiomatic definition via alternating multilinear forms
- 9.4.2 Computation: cofactor expansion, row operations
- 9.4.3 Properties: multiplicativity, transpose, invertibility criterion
- 9.4.4 Cramer's rule

### 9.5 Inner Product Spaces
- 9.5.1 Inner products; norm and orthogonality
- 9.5.2 Gram-Schmidt orthogonalization
- 9.5.3 Orthogonal projections; least squares
- 9.5.4 Orthonormal bases; Fourier series as an inner product expansion

### 9.6 Eigenvalues and Eigenvectors
- 9.6.1 Definition; characteristic polynomial
- 9.6.2 Diagonalization; conditions for diagonalizability
- 9.6.3 Symmetric matrices: spectral theorem (real eigenvalues, orthogonal eigenvectors)
- 9.6.4 Quadratic forms and their classification

### 9.7 Dual Spaces and Bilinear Forms
- 9.7.1 The dual space V*; covectors (linear functionals)
- 9.7.2 Dual basis; natural pairing between V and V*
- 9.7.3 Bilinear and sesquilinear forms
- 9.7.4 Symmetric bilinear forms; signature; Sylvester's law of inertia
- 9.7.5 The metric as a non-degenerate symmetric bilinear form
- 9.7.6 Index and signature: Euclidean vs. pseudo-Riemannian signatures — preview of the Lorentz metric (signature -+++)

---

**Key Concepts:** vector space, basis, dimension, linear map, kernel, image, rank-nullity, determinant, eigenvalue, inner product, dual space, bilinear form, signature, metric

**Important Figures:** Arthur Cayley, James Joseph Sylvester, Hermann Grassmann, Giuseppe Peano, Emmy Noether, David Hilbert

**Additional Reading and Primary Sources:**
- Halmos, *Finite-Dimensional Vector Spaces*
- Axler, *Linear Algebra Done Right*
- Strang, *Introduction to Linear Algebra*
- Grassmann, *Die Lineale Ausdehnungslehre* (historical)

**Exercises:**
1. Prove the rank-nullity theorem.
2. Find the eigenvalues and eigenvectors of a 3×3 symmetric matrix; verify the spectral theorem.
3. Construct the dual basis for a basis of ℝ³ and verify the natural pairing.
4. Classify the quadratic form Q = 2x² + 3y² - z² - 2xz; find its signature.

**Thought Experiments:**
- A linear map can be represented by many different matrices (one for each choice of basis), yet "is" only one transformation. What is the invariant content of a linear map?
- A metric with signature (-+++) assigns negative "length-squared" to some vectors. What does this mean physically?

---

# UNIT IV: Mathematical Foundations — Advanced Analysis and Topology

*Unit Introduction: The mathematics of general relativity lives on smooth manifolds with a metric — objects that require a sophisticated analytical and topological foundation. This unit develops ordinary and partial differential equations (the equations of physics), complex analysis (a powerful tool for solving them), and the real analysis and topology needed to make rigorous sense of manifolds.*

---

## Chapter 10: Ordinary Differential Equations

*Chapter Introduction: Ordinary differential equations (ODEs) describe the time evolution of physical systems. Newton's second law is an ODE; the geodesic equation of general relativity is a system of ODEs. Mastery of ODEs, both analytical and qualitative, is essential.*

### 10.1 First-Order ODEs
- 10.1.1 Classification: linear, nonlinear, separable, exact
- 10.1.2 Separable equations
- 10.1.3 Linear first-order equations; integrating factors
- 10.1.4 Exact equations and integrating factors
- 10.1.5 Existence and uniqueness: Picard-Lindelöf theorem

### 10.2 Second-Order Linear ODEs
- 10.2.1 Homogeneous equations with constant coefficients
- 10.2.2 The characteristic equation; real and complex roots
- 10.2.3 Particular solutions: method of undetermined coefficients; variation of parameters
- 10.2.4 Reduction of order
- 10.2.5 The Wronskian; linear independence of solutions

### 10.3 Systems of ODEs
- 10.3.1 First-order linear systems; matrix form
- 10.3.2 Eigenvalue method
- 10.3.3 Phase plane analysis; fixed points and stability
- 10.3.4 Linearization near fixed points

### 10.4 Series Solutions
- 10.4.1 Power series solutions; ordinary points
- 10.4.2 Regular singular points; Frobenius method
- 10.4.3 Bessel's equation and Bessel functions
- 10.4.4 Legendre's equation and Legendre polynomials

### 10.5 Sturm-Liouville Theory
- 10.5.1 Sturm-Liouville eigenvalue problems
- 10.5.2 Orthogonality of eigenfunctions
- 10.5.3 Completeness and eigenfunction expansions

---

**Key Concepts:** ODE, linearity, superposition, Wronskian, existence and uniqueness, phase plane, eigenvalue, Frobenius method, Legendre polynomials, Bessel functions, Sturm-Liouville

**Important Figures:** Leonhard Euler, Joseph-Louis Lagrange, Augustin-Louis Cauchy, Charles-Émile Picard, Friedrich Bessel, Adrien-Marie Legendre, Jacques Sturm, Joseph Liouville

**Additional Reading and Primary Sources:**
- Tenenbaum & Pollard, *Ordinary Differential Equations*
- Coddington & Levinson, *Theory of Ordinary Differential Equations*
- Arnold, *Ordinary Differential Equations*

**Exercises:**
1. Solve y'' + 4y = sin(2t) using variation of parameters.
2. Find the series solution of Legendre's equation (1-x²)y'' - 2xy' + n(n+1)y = 0 about x=0.
3. Analyze the phase portrait of the nonlinear system ẋ = y, ẏ = -sin(x) (the pendulum).
4. State and prove the Picard-Lindelöf theorem.

**Thought Experiments:**
- The pendulum equation ẍ = -(g/L)sin(θ) is nonlinear. For small θ, sin(θ) ≈ θ and the system is linear. What does the phase portrait look like globally vs. locally?
- What does it mean for a solution to an ODE to be "unique"? Can you construct a physical situation where uniqueness fails?

---

## Chapter 11: Partial Differential Equations

*Chapter Introduction: The equations of physics — wave equation, heat equation, Laplace's equation, Einstein's field equations — are all partial differential equations (PDEs). This chapter develops the classical theory of linear PDEs, which provides context for understanding the much harder nonlinear Einstein equations.*

### 11.1 Classification of PDEs
- 11.1.1 Order, linearity, and homogeneity
- 11.1.2 Classification of second-order linear PDEs: elliptic, parabolic, hyperbolic
- 11.1.3 Characteristic curves and the method of characteristics
- 11.1.4 Well-posedness: existence, uniqueness, continuous dependence

### 11.2 The Three Prototype PDEs
- 11.2.1 Laplace's equation ∇²u = 0: harmonic functions, mean value property
- 11.2.2 The heat equation ∂u/∂t = k∇²u: fundamental solution, maximum principle
- 11.2.3 The wave equation ∂²u/∂t² = c²∇²u: d'Alembert's formula, finite speed of propagation

### 11.3 Separation of Variables
- 11.3.1 Separation on rectangular domains
- 11.3.2 Fourier series solutions
- 11.3.3 Separation in spherical coordinates: spherical harmonics
- 11.3.4 Separation in cylindrical coordinates: Bessel functions

### 11.4 Green's Functions
- 11.4.1 The concept of Green's function as an impulse response
- 11.4.2 Green's function for the Laplacian in ℝ³
- 11.4.3 Green's functions for wave and heat equations

### 11.5 Distributions and Generalized Functions
- 11.5.1 The Dirac delta "function" as a distribution
- 11.5.2 Derivatives of distributions
- 11.5.3 Fourier transforms of distributions

---

**Key Concepts:** elliptic/parabolic/hyperbolic classification, characteristics, Laplace equation, wave equation, heat equation, separation of variables, spherical harmonics, Green's function, Dirac delta

**Important Figures:** Jean le Rond d'Alembert, Joseph Fourier, Siméon Denis Poisson, George Green, Pierre-Simon Laplace, Paul Dirac, Laurent Schwartz

**Additional Reading and Primary Sources:**
- Evans, *Partial Differential Equations*
- Strauss, *Partial Differential Equations: An Introduction*
- Schwartz, *Théorie des distributions* (historical)

**Exercises:**
1. Solve the wave equation on [0,L] with fixed endpoints using separation of variables.
2. Derive d'Alembert's solution to the 1D wave equation by a change of variables.
3. Find the Green's function for -∇²u = f on all of ℝ³.
4. Show that the spherical harmonics Yₗᵐ are eigenfunctions of the angular momentum operator L².

**Thought Experiments:**
- The wave equation propagates information at finite speed c; the heat equation propagates information instantaneously. What does this mean for causality? How does general relativity handle this?
- Why is the solution to an elliptic equation determined everywhere by its boundary values, while a hyperbolic equation requires initial data?

---

## Chapter 12: Complex Analysis

*Chapter Introduction: Complex analysis — the calculus of functions of a complex variable — is one of the most beautiful and useful areas of mathematics. Analyticity imposes powerful constraints: a function's behavior near any point completely determines it everywhere. Contour integration provides tools for evaluating real integrals that appear throughout physics.*

### 12.1 Analytic Functions
- 12.1.1 Complex differentiability and the Cauchy-Riemann equations
- 12.1.2 Analytic and holomorphic functions
- 12.1.3 Harmonic functions and their connection to analytic functions
- 12.1.4 Elementary analytic functions: e^z, sin z, cos z, log z

### 12.2 Complex Integration
- 12.2.1 Contour integrals
- 12.2.2 Cauchy's theorem
- 12.2.3 Cauchy's integral formula and its consequences
- 12.2.4 Morera's theorem; Liouville's theorem

### 12.3 Series and Singularities
- 12.3.1 Power series for analytic functions; radius of convergence
- 12.3.2 Laurent series
- 12.3.3 Classification of isolated singularities: removable, pole, essential
- 12.3.4 Residues

### 12.4 The Residue Theorem and Applications
- 12.4.1 The residue theorem
- 12.4.2 Evaluation of real integrals using residues
- 12.4.3 The argument principle; Rouché's theorem

### 12.5 Conformal Mappings
- 12.5.1 Conformality: angle-preserving maps
- 12.5.2 Möbius transformations
- 12.5.3 The Riemann mapping theorem (statement)
- 12.5.4 Applications to potential theory and 2D fluid flow

---

**Key Concepts:** Cauchy-Riemann equations, analytic/holomorphic function, contour integral, Cauchy's theorem, Laurent series, singularity, residue, conformal mapping

**Important Figures:** Augustin-Louis Cauchy, Bernhard Riemann, Karl Weierstrass, Henri Poincaré, Édouard Goursat

**Additional Reading and Primary Sources:**
- Ahlfors, *Complex Analysis*
- Needham, *Visual Complex Analysis*
- Riemann, *Foundations of a General Theory of Functions* (historical)

**Exercises:**
1. Evaluate ∫_{-∞}^{∞} 1/(1+x²) dx using residues.
2. Find the Laurent series of 1/(z²(z-1)) valid for 0 < |z| < 1.
3. Prove the fundamental theorem of algebra using Liouville's theorem.
4. Verify Cauchy-Riemann equations for f(z) = z² and for f(z) = |z|².

**Thought Experiments:**
- Liouville's theorem says a bounded entire function must be constant. What does this imply about the behavior of analytic functions at infinity?
- Conformal maps preserve angles locally. Why does the Mercator projection preserve angles but distort areas?

---

## Chapter 13: Real Analysis and Point-Set Topology

*Chapter Introduction: Real analysis makes calculus rigorous, and topology abstracts the notions of "nearness" and "continuity" from their dependence on any particular distance function. These ideas culminate in the definition of a topological manifold — the stage on which all of differential geometry and general relativity is performed.*

### 13.1 Metric Spaces
- 13.1.1 Definition; examples: ℝⁿ, function spaces, discrete metric
- 13.1.2 Open and closed sets; interior, closure, boundary
- 13.1.3 Convergence; continuity in metric spaces
- 13.1.4 Completeness; Cauchy sequences; Banach spaces
- 13.1.5 Compactness: sequential and open cover definitions; Heine-Borel theorem
- 13.1.6 Connectedness and path-connectedness

### 13.2 Topological Spaces
- 13.2.1 Axiomatic definition of a topology
- 13.2.2 Bases and sub-bases
- 13.2.3 Continuous maps; homeomorphisms
- 13.2.4 Product topology; subspace topology; quotient topology
- 13.2.5 Separation axioms: T1, Hausdorff (T2), normal
- 13.2.6 Compactness in topological spaces; Tychonoff's theorem

### 13.3 Connectedness and Homotopy
- 13.3.1 Connected and path-connected spaces
- 13.3.2 Homotopy of paths; fundamental group π₁
- 13.3.3 Simply connected spaces
- 13.3.4 Covering spaces (overview)

### 13.4 Uniform Convergence and Function Spaces
- 13.4.1 Pointwise vs. uniform convergence
- 13.4.2 Uniform convergence and continuity; integration; differentiation
- 13.4.3 The space C([a,b]) with the sup norm
- 13.4.4 Arzelà-Ascoli theorem; equicontinuity

### 13.5 Introduction to Differential Topology
- 13.5.1 Topological manifolds: definition and examples
- 13.5.2 Charts, atlases, and coordinate changes
- 13.5.3 Smooth manifolds (preview of Unit VIII)
- 13.5.4 Orientability

---

**Key Concepts:** metric space, topological space, open/closed set, compactness, Heine-Borel, completeness, Hausdorff, homeomorphism, fundamental group, topological manifold, chart, atlas

**Important Figures:** Georg Cantor, Maurice Fréchet, Felix Hausdorff, Henri Poincaré, L.E.J. Brouwer, Andrey Tikhonov

**Additional Reading and Primary Sources:**
- Rudin, *Principles of Mathematical Analysis*, Chapters 2–4
- Munkres, *Topology*
- Hocking & Young, *Topology*

**Exercises:**
1. Prove the Heine-Borel theorem: a subset of ℝⁿ is compact iff it is closed and bounded.
2. Prove that a continuous image of a compact set is compact.
3. Show that the circle S¹ and the interval [0,1] are not homeomorphic.
4. Compute the fundamental group of the torus T² = S¹ × S¹.

**Thought Experiments:**
- The surface of a donut (torus) and the surface of a mug with a handle are homeomorphic. What does this mean? What property are they sharing?
- General relativity is a theory of spacetime topology as well as geometry. Why does the topology of spacetime matter for physics?

---

# UNIT V: Classical Mechanics

*Unit Introduction: Classical mechanics is the physical theory that describes the motion of macroscopic bodies under the influence of forces. It is the first physics theory the student will master, and its reformulations — the Lagrangian and Hamiltonian — provide the conceptual framework for all of modern theoretical physics, including quantum mechanics and general relativity. The variational principles developed here will reappear as the principle of extremal action in deriving the geodesic equation.*

---

## Chapter 14: Newtonian Mechanics

*Chapter Introduction: Newton's laws define the classical paradigm: forces cause acceleration, and the world evolves deterministically from initial conditions. This chapter establishes the formalism and applies it to the central-force problem — gravity in its Newtonian form — which general relativity will ultimately replace.*

### 14.1 Newton's Laws
- 14.1.1 The concept of an inertial frame of reference
- 14.1.2 Newton's First Law: the law of inertia
- 14.1.3 Newton's Second Law: F = ma; mass as inertia
- 14.1.4 Newton's Third Law: action and reaction
- 14.1.5 Galilean relativity and transformations between inertial frames

### 14.2 Kinematics
- 14.2.1 Position, velocity, acceleration vectors
- 14.2.2 Kinematics in polar, cylindrical, spherical coordinates
- 14.2.3 Relative motion

### 14.3 Forces and Dynamics
- 14.3.1 Fundamental forces: gravity, normal, friction, tension
- 14.3.2 Momentum and impulse; conservation of linear momentum
- 14.3.3 Work and kinetic energy; work-energy theorem
- 14.3.4 Conservative forces and potential energy
- 14.3.5 Conservation of energy

### 14.4 Angular Momentum and Rotational Dynamics
- 14.4.1 Angular momentum of a particle
- 14.4.2 Torque; conservation of angular momentum
- 14.4.3 Central forces; conservation of angular momentum implies planar motion

### 14.5 Newtonian Gravity
- 14.5.1 Newton's law of universal gravitation
- 14.5.2 Gravitational potential energy
- 14.5.3 The shell theorem
- 14.5.4 Gravitational field and gravitational potential
- 14.5.5 Tidal forces: differential gravity across a finite body
- 14.5.6 Poisson's equation for gravity: ∇²Φ = 4πGρ

### 14.6 The Kepler Problem
- 14.6.1 Two-body problem; reduction to one-body problem
- 14.6.2 Conservation laws and effective potential
- 14.6.3 Orbit equation: conic sections
- 14.6.4 Kepler's three laws derived from Newton's theory
- 14.6.5 Escape velocity and bound orbits

### 14.7 Non-Inertial Frames
- 14.7.1 Pseudo-forces in accelerating frames
- 14.7.2 Rotating frames: Coriolis and centrifugal forces
- 14.7.3 The Foucault pendulum

---

**Key Concepts:** inertial frame, Newton's laws, momentum, angular momentum, conservative force, potential, Kepler's laws, tidal force, Poisson's equation, non-inertial frame, Coriolis force

**Important Figures:** Isaac Newton, Johannes Kepler, Galileo Galilei, Robert Hooke, Henry Cavendish, Léon Foucault

**Additional Reading and Primary Sources:**
- Newton, *Philosophiæ Naturalis Principia Mathematica* (Books I–III selections)
- Kepler, *Astronomia Nova* (historical)
- Kleppner & Kolenkow, *An Introduction to Mechanics*
- Symon, *Mechanics*

**Exercises:**
1. Derive Kepler's second law (equal areas) from conservation of angular momentum.
2. Compute the tidal force on the near and far sides of an object in free fall toward a planet.
3. Derive the orbit equation for a central force F = -k/r² using the conserved energy and angular momentum.
4. Show that Poisson's equation ∇²Φ = 4πGρ reduces to Laplace's equation outside a mass distribution.

**Thought Experiments:**
- Einstein's elevator: if you are in a sealed room accelerating upward at g, can you tell by any mechanical experiment that you are not in a gravitational field? What does this suggest?
- Tidal forces cannot be "transformed away" by going to a freely falling frame — they represent genuine differential gravity. Why is this the seed of the idea of spacetime curvature?

---

## Chapter 15: Lagrangian Mechanics

*Chapter Introduction: The Lagrangian formulation recasts mechanics not in terms of forces but in terms of a single scalar function — the Lagrangian L = T - V. Motion is determined by the principle of stationary action. This reformulation is far more powerful than Newton's approach: it handles constraints naturally, generalizes to field theories, and provides the direct bridge to the geodesic principle of general relativity.*

### 15.1 The Calculus of Variations
- 15.1.1 Functionals; the action functional
- 15.1.2 Derivation of the Euler-Lagrange equation
- 15.1.3 Boundary conditions; natural and fixed
- 15.1.4 Constrained variation; isoperimetric problems
- 15.1.5 Multiple degrees of freedom; the Euler-Lagrange equations in n dimensions

### 15.2 The Principle of Least Action
- 15.2.1 Hamilton's principle: δS = 0
- 15.2.2 Equivalence with Newton's laws (for conservative forces)
- 15.2.3 Generalized coordinates and constraints
- 15.2.4 Holonomic and non-holonomic constraints

### 15.3 The Lagrangian and Equations of Motion
- 15.3.1 Lagrangian for a particle in a potential: L = T - V
- 15.3.2 Examples: simple pendulum, double pendulum, particle on a rotating ring
- 15.3.3 Lagrangian in curvilinear coordinates
- 15.3.4 Lagrangian for a charged particle in an electromagnetic field

### 15.4 Symmetries and Conservation Laws: Noether's Theorem
- 15.4.1 Cyclic (ignorable) coordinates and conserved momenta
- 15.4.2 Noether's theorem: every continuous symmetry has a corresponding conservation law
- 15.4.3 Time translation symmetry → conservation of energy
- 15.4.4 Spatial translation symmetry → conservation of momentum
- 15.4.5 Rotational symmetry → conservation of angular momentum

### 15.5 Constraints and the Method of Lagrange Multipliers
- 15.5.1 Lagrange multipliers in mechanics: constraint forces
- 15.5.2 Applications: bead on a wire, particle on a sphere

---

**Key Concepts:** action functional, Euler-Lagrange equation, Hamilton's principle, generalized coordinates, cyclic coordinates, Noether's theorem, conservation law, symmetry

**Important Figures:** Joseph-Louis Lagrange, William Rowan Hamilton, Emmy Noether, Leonhard Euler, Pierre Louis Maupertuis

**Additional Reading and Primary Sources:**
- Lagrange, *Mécanique Analytique* (historical)
- Noether, *Invariante Variationsprobleme* (1918) — primary source
- Goldstein, Poole & Safko, *Classical Mechanics*
- Landau & Lifshitz, *Mechanics*, Vol. 1

**Exercises:**
1. Derive the Euler-Lagrange equations for a particle in a central potential using polar coordinates.
2. Use Noether's theorem to derive the conservation of angular momentum from rotational symmetry.
3. Set up the Lagrangian for a double pendulum and derive the equations of motion.
4. Find the shortest curve between two points on a sphere (the geodesic) using the calculus of variations.

**Thought Experiments:**
- The geodesic equation in general relativity is the Euler-Lagrange equation for a specific Lagrangian involving the spacetime metric. In what sense is free fall "extremizing" something?
- Noether's theorem says conservation laws come from symmetries. What conserved quantity corresponds to the symmetry of spacetime under Lorentz boosts?

---

## Chapter 16: Hamiltonian Mechanics and Phase Space

*Chapter Introduction: The Hamiltonian formulation trades generalized velocities for generalized momenta, doubling the dimension of the state space to a 2n-dimensional "phase space." This geometric perspective reveals deep structures — symplectic geometry, canonical transformations, the Poisson bracket — that prefigure the algebraic structures of quantum mechanics and the symplectic structure of phase space in GR.*

### 16.1 The Legendre Transform and the Hamiltonian
- 16.1.1 The Legendre transform: from L to H
- 16.1.2 Hamilton's equations: q̇ = ∂H/∂p, ṗ = -∂H/∂q
- 16.1.3 H as total energy for conservative systems
- 16.1.4 Phase space and phase portraits

### 16.2 Canonical Transformations
- 16.2.1 Generating functions and canonical transformations
- 16.2.2 Preservation of Hamilton's equations; symplectic condition
- 16.2.3 Action-angle variables
- 16.2.4 Liouville's theorem on phase space volume preservation

### 16.3 The Poisson Bracket
- 16.3.1 Definition and properties of the Poisson bracket
- 16.3.2 Equations of motion in Poisson bracket form: ḟ = {f, H}
- 16.3.3 Constants of motion; commuting observables
- 16.3.4 Poisson bracket algebra and its quantization (preview of QM)

### 16.4 Hamilton-Jacobi Theory
- 16.4.1 The Hamilton-Jacobi equation
- 16.4.2 Complete integrals and the connection to optics (eikonal equation)
- 16.4.3 Separation of variables; solving the Kepler problem
- 16.4.4 WKB approximation (preview of QM)

### 16.5 Integrable Systems
- 16.5.1 Definition of integrability; constants of motion in involution
- 16.5.2 Liouville-Arnold theorem; KAM theorem (statement)
- 16.5.3 Chaos in non-integrable systems; sensitive dependence on initial conditions

---

**Key Concepts:** Hamiltonian, canonical transformation, symplectic structure, phase space, Liouville's theorem, Poisson bracket, Hamilton-Jacobi equation, integrability, KAM theorem

**Important Figures:** William Rowan Hamilton, Carl Gustav Jacob Jacobi, Joseph Liouville, Siméon Denis Poisson, Vladimir Arnold, Andrey Kolmogorov, Jürgen Moser

**Additional Reading and Primary Sources:**
- Hamilton, *On a General Method in Dynamics* (historical)
- Landau & Lifshitz, *Mechanics*, Ch. 7–8
- Arnold, *Mathematical Methods of Classical Mechanics*
- Goldstein, Poole & Safko, *Classical Mechanics*, Chs. 8–10

**Exercises:**
1. Derive Hamilton's equations from Hamilton's principle applied to the Hamiltonian action.
2. Verify that the transformation q → p, p → -q is canonical and find the generating function.
3. Use the Hamilton-Jacobi equation to solve for the orbits of a particle in a central potential.
4. Compute {L_x, L_y} using the Poisson bracket and show it equals L_z.

**Thought Experiments:**
- In the Hamiltonian formulation, q and p are treated symmetrically. What is the physical meaning of the "momentum" coordinate in a field theory?
- Liouville's theorem says phase space "volume" is preserved. What does this say about the reversibility of classical mechanics? How does this connect to the second law of thermodynamics?

---

## Chapter 17: Rigid Body Dynamics and Continuum Mechanics

*Chapter Introduction: Rigid bodies and continuous media extend mechanics from point particles to extended objects. The stress tensor introduced here for continua — describing internal forces per unit area in a deformable medium — is the direct precursor to the stress-energy tensor in general relativity.*

### 17.1 Rigid Body Kinematics
- 17.1.1 Degrees of freedom of a rigid body
- 17.1.2 Euler angles; rotation matrices
- 17.1.3 Angular velocity as a vector; instantaneous axis of rotation
- 17.1.4 Body and space frames

### 17.2 Rigid Body Dynamics
- 17.2.1 Inertia tensor; principal axes and principal moments
- 17.2.2 Euler's equations of rigid body motion
- 17.2.3 Torque-free motion; the tennis racket theorem
- 17.2.4 The symmetric top; precession and nutation

### 17.3 Introduction to Continuum Mechanics
- 17.3.1 The continuum hypothesis; material and spatial descriptions
- 17.3.2 Stress tensor: definition as flux of momentum
- 17.3.3 Strain tensor: symmetric part of the displacement gradient
- 17.3.4 Equations of motion: ∂_t(ρvᵢ) = ∂_j σ^{ij} + fᵢ (Cauchy's equation)
- 17.3.5 Fluid mechanics: Euler and Navier-Stokes equations

---

**Key Concepts:** inertia tensor, Euler angles, principal axes, stress tensor, strain tensor, Cauchy's equation, Navier-Stokes equations

**Important Figures:** Leonhard Euler, Siméon Denis Poisson, Augustin-Louis Cauchy, George Gabriel Stokes, Claude-Louis Navier

**Additional Reading and Primary Sources:**
- Goldstein, Poole & Safko, *Classical Mechanics*, Chs. 4–5
- Landau & Lifshitz, *Theory of Elasticity*
- Landau & Lifshitz, *Fluid Mechanics*

**Exercises:**
1. Compute the inertia tensor for a uniform solid sphere and verify isotropy.
2. Derive Euler's equations from the Lagrangian for a rigid body.
3. Write the Navier-Stokes equations in index notation and identify each term's physical meaning.

**Thought Experiments:**
- The stress tensor σ^{ij} describes the force per unit area exerted across a surface element. How does this generalize in general relativity to the stress-energy tensor T^{μν}, which includes energy density, momentum density, and pressure as components?

---

# UNIT VI: Classical Electromagnetism

*Unit Introduction: Electromagnetism is the first field theory mastered in a physics curriculum, and it is the direct inspiration for the mathematical language of general relativity. Maxwell unified electricity, magnetism, and light into one theory described by partial differential equations. The reformulation of electromagnetism using 4-vectors and differential forms in the context of special relativity will serve as a template for understanding how GR is formulated.*

---

## Chapter 18: Electrostatics

*Chapter Introduction: Electrostatics governs the forces between stationary electric charges. The fundamental law (Coulomb's) is closely analogous to Newton's gravitational law — and many of the tools developed here (potentials, Gauss's law, Poisson's equation) will be reused directly when studying Newtonian gravity and its relativistic generalization.*

### 18.1 Coulomb's Law and the Electric Field
- 18.1.1 Electric charge; conservation and quantization
- 18.1.2 Coulomb's law
- 18.1.3 The principle of superposition
- 18.1.4 The electric field E; field lines

### 18.2 Gauss's Law
- 18.2.1 Electric flux; Gauss's law in integral form
- 18.2.2 Gauss's law in differential form: ∇·E = ρ/ε₀
- 18.2.3 Applications: planar, cylindrical, spherical symmetry

### 18.3 Electric Potential
- 18.3.1 Work done by the electric field; path independence
- 18.3.2 Electric potential V; E = -∇V
- 18.3.3 Poisson's equation: ∇²V = -ρ/ε₀
- 18.3.4 Laplace's equation in charge-free regions
- 18.3.5 Multipole expansion: monopole, dipole, quadrupole terms

### 18.4 Conductors and Capacitors
- 18.4.1 Electrostatics in conductors; boundary conditions
- 18.4.2 Capacitance; energy stored in electric fields
- 18.4.3 Method of images

### 18.5 Dielectrics
- 18.5.1 Polarization P; bound charges
- 18.5.2 Displacement field D; Gauss's law in matter
- 18.5.3 Linear dielectrics: D = εE

---

**Key Concepts:** Coulomb's law, electric field, Gauss's law, electric potential, Poisson's equation, multipole expansion, boundary conditions, dielectric

**Important Figures:** Charles-Augustin de Coulomb, Carl Friedrich Gauss, Michael Faraday, Siméon Denis Poisson, George Green

**Additional Reading and Primary Sources:**
- Griffiths, *Introduction to Electrodynamics*, Chs. 2–4
- Jackson, *Classical Electrodynamics*, Ch. 1–3
- Faraday, *Experimental Researches in Electricity* (historical)

**Exercises:**
1. Use Gauss's law to find the electric field inside and outside a uniformly charged solid sphere.
2. Solve Laplace's equation inside a hollow sphere with a specified potential on the surface (Legendre polynomial expansion).
3. Compute the dipole moment of two equal and opposite charges and write the far-field potential.

**Thought Experiments:**
- Electrostatics satisfies Poisson's equation ∇²Φ = -ρ/ε₀; Newtonian gravity satisfies ∇²Φ_grav = 4πGρ. How close is the analogy? Where does it break down?

---

## Chapter 19: Magnetostatics and Electromagnetic Induction

*Chapter Introduction: Moving charges create magnetic fields; changing magnetic fields create electric forces. Faraday's law and Ampere's law complete the picture before Maxwell's unification.*

### 19.1 The Magnetic Field
- 19.1.1 Magnetic force on moving charges: the Lorentz force F = q(E + v×B)
- 19.1.2 Biot-Savart law
- 19.1.3 Ampere's law in integral and differential form: ∇×B = μ₀J
- 19.1.4 The vector potential A; B = ∇×A
- 19.1.5 No magnetic monopoles: ∇·B = 0

### 19.2 Electromagnetic Induction
- 19.2.1 Faraday's law: ∮E·dl = -dΦ_B/dt
- 19.2.2 Lenz's law
- 19.2.3 Differential form: ∇×E = -∂B/∂t
- 19.2.4 Inductance; energy stored in magnetic fields

### 19.3 Magnetic Materials
- 19.3.1 Magnetization M; bound currents
- 19.3.2 H field; Ampere's law in matter
- 19.3.3 Dia-, para-, and ferromagnetism

---

**Key Concepts:** Lorentz force, Biot-Savart law, Ampere's law, Faraday's law, vector potential, magnetic flux, electromagnetic induction, H field

**Important Figures:** Hans Christian Ørsted, André-Marie Ampère, Michael Faraday, Jean-Baptiste Biot, Félix Savart

**Additional Reading and Primary Sources:**
- Griffiths, *Introduction to Electrodynamics*, Chs. 5–7
- Faraday, *Experimental Researches in Electricity*, Series I (1831)

**Exercises:**
1. Compute the magnetic field on the axis of a circular current loop using the Biot-Savart law.
2. Use Ampere's law to find B inside and outside an infinite solenoid.
3. Derive the equation for the mutual inductance of two coaxial loops.

**Thought Experiments:**
- Faraday discovered that a changing magnetic flux induces an EMF. Einstein later showed that a "magnetic field" in one frame can be a "pure electric field" in another frame. How does special relativity unify these phenomena?

---

## Chapter 20: Maxwell's Equations and Electromagnetic Waves

*Chapter Introduction: James Clerk Maxwell's addition of the displacement current ∂E/∂t to Ampere's law completed the theory of electromagnetism and, as a consequence, predicted the existence of electromagnetic waves traveling at the speed of light. This is one of the greatest theoretical discoveries in the history of physics and the direct historical trigger for special relativity.*

### 20.1 Maxwell's Equations
- 20.1.1 The four Maxwell equations in differential form:
  - ∇·E = ρ/ε₀
  - ∇·B = 0
  - ∇×E = -∂B/∂t
  - ∇×B = μ₀J + μ₀ε₀ ∂E/∂t
- 20.1.2 The displacement current: ε₀ ∂E/∂t
- 20.1.3 Maxwell's equations in integral form
- 20.1.4 Maxwell's equations in matter

### 20.2 Electromagnetic Waves
- 20.2.1 Derivation of the wave equation from Maxwell's equations
- 20.2.2 Speed of light: c = 1/√(μ₀ε₀)
- 20.2.3 Plane wave solutions; polarization
- 20.2.4 Energy in electromagnetic waves: the Poynting vector S = E×B/μ₀
- 20.2.5 Radiation pressure

### 20.3 Potentials and Gauge Freedom
- 20.3.1 Scalar and vector potentials: E = -∇V - ∂A/∂t, B = ∇×A
- 20.3.2 Gauge transformations: A → A + ∇χ, V → V - ∂χ/∂t
- 20.3.3 Lorenz gauge: ∇·A + (1/c²)∂V/∂t = 0
- 20.3.4 Coulomb gauge
- 20.3.5 The inhomogeneous wave equations for potentials

---

**Key Concepts:** Maxwell's equations, displacement current, electromagnetic wave, speed of light, Poynting vector, scalar/vector potential, gauge transformation, Lorenz gauge

**Important Figures:** James Clerk Maxwell, Heinrich Hertz, Michael Faraday, André-Marie Ampère

**Additional Reading and Primary Sources:**
- Maxwell, *A Treatise on Electricity and Magnetism* (1873, selections)
- Maxwell, *On Physical Lines of Force* (1861)
- Griffiths, *Introduction to Electrodynamics*, Chs. 8–10
- Jackson, *Classical Electrodynamics*, Chs. 6–7

**Exercises:**
1. Derive the electromagnetic wave equation from Maxwell's equations in vacuum.
2. Compute the Poynting vector for a plane electromagnetic wave and verify it equals the energy flux.
3. Show that the Lorenz gauge decouples the wave equations for A and V.
4. Verify that Maxwell's equations are invariant under gauge transformations.

**Thought Experiments:**
- Maxwell's equations predict light travels at speed c. But relative to what frame? This apparent contradiction drove Einstein to special relativity.
- In the Lorenz gauge, the equations for A and V have the same form as each other. What symmetry does this reflect?

---

## Chapter 21: Radiation and Retarded Potentials

*Chapter Introduction: When charges accelerate, they radiate electromagnetic energy. The retarded potentials formalize the idea that electromagnetic influences propagate at finite speed. The mathematical structure here — the retarded Green's function, radiation from a compact source — is closely paralleled by gravitational wave emission in linearized general relativity.*

### 21.1 Retarded Potentials
- 21.1.1 The retarded Green's function for the wave operator
- 21.1.2 Retarded potentials (Jefimenko's equations)
- 21.1.3 Physical interpretation: signals travel at c; retarded time

### 21.2 The Liénard-Wiechert Potentials
- 21.2.1 Potentials for a point charge in arbitrary motion
- 21.2.2 Fields of an accelerated charge
- 21.2.3 Radiation from an accelerated charge: Larmor formula

### 21.3 Multipole Radiation
- 21.3.1 Electric dipole radiation
- 21.3.2 Magnetic dipole and electric quadrupole radiation
- 21.3.3 Radiation reaction force; Abraham-Lorentz force

---

**Key Concepts:** retarded potential, retarded time, Liénard-Wiechert potential, Larmor formula, multipole radiation, electric dipole radiation, radiation reaction

**Important Figures:** Alfred-Marie Liénard, Emil Wiechert, Joseph Larmor, Max Abraham, Hendrik Lorentz

**Additional Reading and Primary Sources:**
- Jackson, *Classical Electrodynamics*, Chs. 6, 9, 14
- Griffiths, *Introduction to Electrodynamics*, Ch. 11

**Exercises:**
1. Derive the Larmor formula for the power radiated by an accelerated point charge.
2. Compute the radiation from an oscillating electric dipole and find the angular distribution.
3. Estimate the power emitted by gravitational waves from a binary star system by analogy with electromagnetic quadrupole radiation.

**Thought Experiments:**
- Gravitational waves are emitted by oscillating mass quadrupoles, just as electromagnetic waves are emitted by oscillating charge dipoles. Why must it be quadrupole and not dipole for gravity? (Hint: think about conservation laws.)

---

# UNIT VII: Special Relativity

*Unit Introduction: Special relativity is the crown of 19th-century physics and the indispensable foundation for general relativity. Einstein's 1905 postulates — that the laws of physics are the same in all inertial frames and that the speed of light is constant — force a radical revision of concepts of space, time, and simultaneity. The mathematical structure that emerges (Minkowski spacetime, 4-vectors, tensors) is the direct springboard into the curved spacetime of general relativity.*

---

## Chapter 22: Historical Background and Motivation

*Chapter Introduction: No scientific revolution appears from nowhere. This chapter traces the physical and conceptual tensions in late-19th-century physics that made special relativity inevitable: the failure to detect the luminiferous ether, the incompatibility between Newtonian mechanics and Maxwell's electromagnetism, and the desperate theoretical attempts to reconcile them.*

### 22.1 The Newtonian World and Galilean Relativity
- 22.1.1 Galilean transformations
- 22.1.2 Galilean addition of velocities
- 22.1.3 Absolute space and time in Newton's view
- 22.1.4 The success of classical mechanics; Newton's bucket

### 22.2 Maxwell's Equations and the Problem of the Ether
- 22.2.1 The luminiferous ether hypothesis
- 22.2.2 Prediction: Maxwell's equations are covariant only in the ether frame
- 22.2.3 The Michelson-Morley experiment (1887)
- 22.2.4 Other ether drift experiments

### 22.3 Attempts to Save Classical Mechanics
- 22.3.1 Lorentz contraction hypothesis
- 22.3.2 Fitzgerald contraction
- 22.3.3 Lorentz's electron theory and local time
- 22.3.4 Poincaré's contributions: the principle of relativity; group structure of transformations

### 22.4 Einstein's 1905 Paper
- 22.4.1 "On the Electrodynamics of Moving Bodies" — structure and argument
- 22.4.2 The two postulates
- 22.4.3 Rejection of the ether; new definitions of simultaneity

---

**Key Concepts:** Galilean transformation, luminiferous ether, Michelson-Morley experiment, Lorentz contraction, principle of relativity, simultaneity

**Important Figures:** Albert Michelson, Edward Morley, Hendrik Lorentz, Henri Poincaré, Albert Einstein, George FitzGerald, Woldemar Voigt

**Additional Reading and Primary Sources:**
- Einstein, *On the Electrodynamics of Moving Bodies* (1905) — English translation (primary source)
- Lorentz, *Electromagnetic Phenomena in a System Moving with Any Velocity Smaller Than That of Light* (1904)
- Poincaré, *On the Dynamics of the Electron* (1906)
- Miller, *Albert Einstein's Special Theory of Relativity* (historical study)

**Exercises:**
1. Compute the expected fringe shift in the Michelson-Morley experiment and compare to the null result.
2. Show that Lorentz transformations form a group.

**Thought Experiments:**
- Einstein's famous thought experiment: riding alongside a light beam. What would you see? Why does this contradict Maxwell's equations?
- What precisely does it mean for two events to be simultaneous? Why is simultaneity frame-dependent?

---

## Chapter 23: Postulates and the Structure of Spacetime

*Chapter Introduction: From two postulates, Einstein derived a complete kinematics. The key conceptual move is to define simultaneity operationally using light signals, from which the Lorentz transformation follows inevitably. The geometry of spacetime — four-dimensional, with a distinguished "Minkowski metric" of signature (-+++) — is the mathematical framework that captures everything.*

### 23.1 The Two Postulates
- 23.1.1 The principle of relativity: laws of physics are the same in all inertial frames
- 23.1.2 The constancy of the speed of light: c is the same in all inertial frames
- 23.1.3 Operational definition of simultaneity via light signals

### 23.2 Spacetime and Events
- 23.2.1 Events as points in spacetime ℝ⁴
- 23.2.2 The spacetime interval: ds² = -c²dt² + dx² + dy² + dz²
- 23.2.3 Timelike, spacelike, and null intervals
- 23.2.4 The light cone; causal structure

### 23.3 The Minkowski Metric
- 23.3.1 The metric tensor η_{μν} = diag(-1,+1,+1,+1)
- 23.3.2 Raising and lowering indices with η
- 23.3.3 Invariance of the interval as the defining property of Lorentz transformations
- 23.3.4 Minkowski spacetime as a pseudo-Riemannian manifold

### 23.4 Causality in Minkowski Spacetime
- 23.4.1 Causal future and causal past
- 23.4.2 Absolute vs. relative concepts: what all observers agree on
- 23.4.3 The impossibility of faster-than-light signaling

---

**Key Concepts:** spacetime interval, Minkowski metric, light cone, timelike/spacelike/null, inertial frame, causality, pseudo-Riemannian metric

**Important Figures:** Albert Einstein, Hermann Minkowski, Henri Poincaré

**Additional Reading and Primary Sources:**
- Minkowski, *Space and Time* (1908 lecture) — primary source
- Einstein, *Relativity: The Special and General Theory* (1920)
- Taylor & Wheeler, *Spacetime Physics*

**Exercises:**
1. Show that ds² is invariant under Lorentz transformations.
2. Classify the following pairs of events as timelike, spacelike, or null: (a) two events separated by 3 m in space and 10 ns in time; (b) a flash of light at the origin and its detection 1 m away after 1/c seconds.
3. Draw the light cone for an event and identify which events can causally influence it.

**Thought Experiments:**
- Minkowski's declaration: "Henceforth, space by itself, and time by itself, are doomed to fade away into mere shadows, and only a kind of union of the two will preserve an independent reality." What did he mean?
- If two observers disagree about the time ordering of two spacelike-separated events, does this violate causality?

---

## Chapter 24: Lorentz Transformations and Kinematic Effects

*Chapter Introduction: The Lorentz transformation is the precise rule for converting spacetime coordinates between inertial frames. It implies the famous kinematic effects — time dilation and length contraction — that have been confirmed by experiment to extraordinary precision.*

### 24.1 Deriving the Lorentz Transformation
- 24.1.1 Derivation from the two postulates using Einstein's operational procedure
- 24.1.2 The Lorentz boost along one axis
- 24.1.3 The Lorentz factor γ = 1/√(1 - v²/c²)
- 24.1.4 The full Lorentz transformation in matrix form
- 24.1.5 The Poincaré group: combining boosts and spatial rotations and translations

### 24.2 Time Dilation
- 24.2.1 Proper time τ along a worldline
- 24.2.2 Time dilation: moving clocks run slow, Δt = γΔτ
- 24.2.3 Experimental tests: muon lifetime, GPS corrections, Hafele-Keating experiment

### 24.3 Length Contraction
- 24.3.1 Proper length
- 24.3.2 Length contraction: moving rods are shorter, L = L₀/γ
- 24.3.3 Lorentz contraction as a geometric effect in spacetime

### 24.4 The Relativity of Simultaneity
- 24.4.1 Formal derivation from the Lorentz transformation
- 24.4.2 The train-and-lightning thought experiment
- 24.4.3 Order of events depends on frame for spacelike separations

### 24.5 Addition of Velocities
- 24.5.1 Relativistic velocity addition formula
- 24.5.2 Speed of light as invariant maximum
- 24.5.3 Aberration of light

### 24.6 The Twin Paradox
- 24.6.1 Setup: one twin stays, one accelerates away and returns
- 24.6.2 Resolution: asymmetry due to acceleration
- 24.6.3 Spacetime diagrams and proper time along worldlines

---

**Key Concepts:** Lorentz transformation, Lorentz factor γ, proper time, time dilation, length contraction, relativity of simultaneity, velocity addition, twin paradox, Poincaré group

**Important Figures:** Albert Einstein, Hendrik Lorentz, Paul Langevin, Hermann Minkowski, Henri Poincaré

**Additional Reading and Primary Sources:**
- Einstein, *On the Electrodynamics of Moving Bodies* (1905) — Sections 1–4
- Minkowski, *Space and Time* (1908)
- Taylor & Wheeler, *Spacetime Physics*

**Exercises:**
1. Two spaceships pass each other at v = 0.8c. A clock on ship A reads Δτ = 10 s for an interval. What does ship B observe?
2. Derive the relativistic velocity addition formula from the Lorentz transformation.
3. Compute the Lorentz contraction of a proton traveling at 0.9999c in the LHC frame.
4. Resolve the twin paradox using spacetime diagrams, computing the proper time along each worldline.

**Thought Experiments:**
- The ladder paradox: a ladder longer than a garage can fit inside the garage due to length contraction. In the ladder frame, the garage is even shorter. How is this consistent?
- If simultaneity is relative, can you reverse the order of cause and effect by changing frames? Why or why not?

---

## Chapter 25: Four-Vectors and Relativistic Dynamics

*Chapter Introduction: The power of special relativity is best expressed in the language of 4-vectors — objects with one time component and three space components that transform covariantly under the Lorentz group. The 4-vector framework is the precursor to full tensor calculus and will be embedded in the general covariant tensor formalism of general relativity.*

### 25.1 Four-Vectors
- 25.1.1 Contravariant 4-vectors: Aᵘ = (A⁰, A¹, A², A³)
- 25.1.2 Covariant components: Aᵤ = η_{μν} Aᵛ (index lowering)
- 25.1.3 The Lorentz invariant scalar product: AᵘAᵤ = η_{μν}AᵘAᵛ
- 25.1.4 The position 4-vector xᵘ = (ct, x, y, z)
- 25.1.5 The 4-velocity uᵘ = dxᵘ/dτ; normalization uᵘuᵤ = -c²

### 25.2 Relativistic Kinematics
- 25.2.1 4-momentum: pᵘ = m uᵘ = (E/c, p)
- 25.2.2 Energy-momentum relation: E² = (pc)² + (mc²)²
- 25.2.3 Rest mass; massless particles (photons): E = pc
- 25.2.4 4-momentum conservation
- 25.2.5 Relativistic collisions and decay processes

### 25.3 Relativistic Dynamics
- 25.3.1 The relativistic equation of motion: dpᵘ/dτ = Fᵘ
- 25.3.2 The 4-force Fᵘ and its properties
- 25.3.3 Work-energy theorem in relativistic form
- 25.3.4 The Lorentz force 4-vector
- 25.3.5 Motion in a uniform magnetic field; relativistic cyclotron frequency

### 25.4 The Stress-Energy Tensor for a Perfect Fluid (Preview)
- 25.4.1 Need for a tensor to describe energy and momentum density of a continuous medium
- 25.4.2 T^{μν} for a perfect fluid: T^{μν} = (ρ + p/c²)uᵘuᵛ + p η^{μν}
- 25.4.3 Conservation: ∂ᵤ T^{μν} = 0

---

**Key Concepts:** 4-vector, Lorentz scalar, 4-velocity, 4-momentum, energy-momentum relation, mass-energy equivalence, 4-force, stress-energy tensor

**Important Figures:** Albert Einstein, Herman Minkowski, Max Planck, Paul Dirac

**Additional Reading and Primary Sources:**
- Einstein, *Does the Inertia of a Body Depend Upon Its Energy Content?* (1905) — primary source (E = mc²)
- Misner, Thorne & Wheeler, *Gravitation*, Ch. 2
- Taylor & Wheeler, *Spacetime Physics*, Chs. 7–9

**Exercises:**
1. Verify that 4-velocity uᵘ satisfies uᵘuᵤ = -c².
2. A particle of mass m decays at rest into two photons. Find the energy and 3-momentum of each photon.
3. Show that the energy-momentum relation E² = p²c² + m²c⁴ follows from pᵘpᵤ = -m²c².
4. Compute the stress-energy tensor T^{μν} for a pressureless dust cloud (p = 0) and verify ∂ᵤT^{μν} = 0 implies conservation of energy and momentum.

**Thought Experiments:**
- Einstein's famous E = mc² paper asked: if a body emits radiation, does its mass change? Why is mass-energy equivalence the natural conclusion of special relativity?
- In classical mechanics, mass is the source of gravity. In GR, the source is T^{μν}. What new sources of gravity does special relativity add?

---

## Chapter 26: Relativistic Electrodynamics

*Chapter Introduction: Maxwell's equations are already Lorentz-covariant — they were written in their "correct" form before Einstein's work. Expressing them in 4-vector and tensor notation makes this manifest and provides the first example of a relativistic field theory. The electromagnetic field tensor F_{μν} is the prototype for covariant field descriptions in general relativity.*

### 26.1 Covariant Formulation of Electrodynamics
- 26.1.1 The 4-current density: Jᵘ = (cρ, J)
- 26.1.2 The 4-potential: Aᵘ = (V/c, A)
- 26.1.3 Gauge invariance: Aᵘ → Aᵘ + ∂ᵘχ
- 26.1.4 Lorenz gauge: ∂ᵤAᵘ = 0; wave equation □Aᵘ = -μ₀Jᵘ

### 26.2 The Electromagnetic Field Tensor
- 26.2.1 Definition: F_{μν} = ∂ᵤAᵥ - ∂ᵥAᵤ
- 26.2.2 Components of F_{μν} in terms of E and B fields
- 26.2.3 Lorentz transformation of E and B fields
- 26.2.4 The dual tensor *F^{μν}; the two Lorentz invariants of the field: F_{μν}F^{μν} = 2(B² - E²/c²), *F_{μν}F^{μν} = -4E·B/c

### 26.3 Maxwell's Equations in Covariant Form
- 26.3.1 Inhomogeneous equations: ∂ᵤF^{μν} = μ₀Jᵛ
- 26.3.2 Homogeneous equations (Bianchi identity): ∂_{[μ}F_{νρ]} = 0
- 26.3.3 Conservation of charge: ∂ᵤJᵘ = 0

### 26.4 The Electromagnetic Stress-Energy Tensor
- 26.4.1 Definition: T^{μν}_{EM} = F^{μα}F^ν_α - (1/4)η^{μν}F_{αβ}F^{αβ} (in units μ₀=1)
- 26.4.2 Components: energy density, Poynting vector, Maxwell stress tensor
- 26.4.3 Conservation: ∂ᵤT^{μν}_{EM} = -F^{μν}Jᵥ

---

**Key Concepts:** 4-potential, 4-current, electromagnetic field tensor F_{μν}, Maxwell's equations in covariant form, Bianchi identity, electromagnetic stress-energy tensor, gauge invariance

**Important Figures:** James Clerk Maxwell, Albert Einstein, Hermann Minkowski, Arnold Sommerfeld

**Additional Reading and Primary Sources:**
- Misner, Thorne & Wheeler, *Gravitation*, Ch. 3
- Jackson, *Classical Electrodynamics*, Ch. 11–12
- Griffiths, *Introduction to Electrodynamics*, Ch. 12

**Exercises:**
1. Show that F_{μν} is antisymmetric and has 6 independent components; identify them as components of E and B.
2. Derive the Lorentz transformation of E and B fields from the transformation of F_{μν}.
3. Verify that ∂ᵤF^{μν} = μ₀Jᵛ reproduces the inhomogeneous Maxwell equations.
4. Show that ∂ᵤT^{μν}_{EM} = -F^{μν}Jᵥ expresses the rate of work done by the field on currents.

**Thought Experiments:**
- A charge at rest creates only an electric field. In a frame where the charge is moving, there is a magnetic field. In what sense are E and B "the same thing viewed from different frames"?
- The electromagnetic Bianchi identity ∂_{[μ}F_{νρ]} = 0 implies the homogeneous Maxwell equations (∇·B = 0, ∇×E = -∂B/∂t). In GR, the Bianchi identity for the Riemann tensor plays an analogous role. What is it?

---

# UNIT VIII: Differential Geometry

*Unit Introduction: Differential geometry is the mathematical language of general relativity. Whereas special relativity occurs on flat Minkowski spacetime, general relativity requires a manifold that can be curved — a pseudo-Riemannian manifold where the geometry varies from point to point in response to matter and energy. This unit builds the full machinery: smooth manifolds, tensors, connections, curvature, and geodesics. Every concept here has a direct physical interpretation in general relativity.*

---

## Chapter 27: Smooth Manifolds

*Chapter Introduction: A manifold is a space that locally looks like Euclidean space but may have a complicated global structure. The Earth's surface is a manifold: locally flat maps suffice, but no single flat map can cover the globe without distortion. Spacetime in general relativity is a 4-dimensional Lorentzian manifold.*

### 27.1 Topological Manifolds
- 27.1.1 Definition: locally homeomorphic to ℝⁿ
- 27.1.2 Charts and atlases
- 27.1.3 Coordinate changes (transition functions)
- 27.1.4 Examples: Sⁿ, torus T², projective spaces, Lie groups
- 27.1.5 Hausdorff and second-countable conditions

### 27.2 Smooth Manifolds
- 27.2.1 Smooth atlas: transition functions are C^∞ diffeomorphisms
- 27.2.2 Smooth functions on a manifold: f: M → ℝ
- 27.2.3 Smooth maps between manifolds: F: M → N
- 27.2.4 Diffeomorphisms; when are two manifolds "the same"?

### 27.3 Submanifolds and Immersions
- 27.3.1 Submanifolds; embedded and immersed submanifolds
- 27.3.2 Regular values and level sets (implicit function theorem for manifolds)
- 27.3.3 Examples: S² as level set of x²+y²+z²=1; curves and surfaces in ℝ³

### 27.4 Orientability
- 27.4.1 Orientation of a vector space
- 27.4.2 Orientable vs. non-orientable manifolds
- 27.4.3 The Möbius strip and Klein bottle as non-orientable examples
- 27.4.4 Physical significance of orientability in spacetime

---

**Key Concepts:** topological manifold, chart, atlas, smooth manifold, transition function, diffeomorphism, smooth function, submanifold, orientability

**Important Figures:** Bernhard Riemann, Hermann Weyl, Élie Cartan, Hassler Whitney, John Milnor

**Additional Reading and Primary Sources:**
- Riemann, *On the Hypotheses Which Lie at the Foundations of Geometry* (1854) — primary source
- Lee, *Introduction to Smooth Manifolds*
- Spivak, *A Comprehensive Introduction to Differential Geometry*, Vol. 1
- Misner, Thorne & Wheeler, *Gravitation*, Ch. 9

**Exercises:**
1. Show that the unit sphere S² is a smooth manifold by constructing an atlas with two charts (stereographic projections).
2. Prove that the composition of two diffeomorphisms is a diffeomorphism.
3. Show that S¹ is orientable but the Möbius band is not.

**Thought Experiments:**
- Spacetime in GR is a 4D manifold. What does it mean to say spacetime "looks locally like ℝ⁴"? What is the physical content of this local flatness?
- The non-trivial global topology of spacetime — wormholes, closed timelike curves — is a separate question from its local curvature. What is the relationship between topology and physics in GR?

---

## Chapter 28: Tangent and Cotangent Spaces

*Chapter Introduction: At each point of a manifold, there is a vector space of "velocities" — the tangent space. Its dual is the cotangent space of covectors (or 1-forms). These structures are the building blocks for all tensors. Understanding that vectors are derivations (differential operators) rather than "arrows" is a key conceptual step.*

### 28.1 Curves and Tangent Vectors
- 28.1.1 Smooth curves on manifolds: γ: ℝ → M
- 28.1.2 Tangent vectors as equivalence classes of curves
- 28.1.3 Tangent vectors as derivations: X: C^∞(M) → ℝ
- 28.1.4 The tangent space T_pM: its structure as a vector space
- 28.1.5 Coordinate basis vectors: ∂/∂xᵘ
- 28.1.6 Change of basis under coordinate transformation: ∂/∂x'ᵘ = (∂xᵛ/∂x'ᵘ) ∂/∂xᵛ

### 28.2 The Tangent Bundle
- 28.2.1 The tangent bundle TM as the disjoint union of all tangent spaces
- 28.2.2 Vector fields as sections of TM
- 28.2.3 The Lie bracket of vector fields: [X,Y]
- 28.2.4 Commuting and non-commuting vector fields
- 28.2.5 Flows of vector fields; integral curves

### 28.3 Cotangent Vectors and 1-Forms
- 28.3.1 The cotangent space T*_pM as dual of T_pM
- 28.3.2 Covectors (1-forms); the coordinate basis dxᵘ
- 28.3.3 The natural pairing ⟨ω, X⟩: T*_pM × T_pM → ℝ
- 28.3.4 The differential of a function: df as a 1-form
- 28.3.5 Change of basis for covectors: the dual transformation rule

### 28.4 The Cotangent Bundle and Differential 1-Forms
- 28.4.1 The cotangent bundle T*M
- 28.4.2 1-forms as sections of T*M
- 28.4.3 Pullback of 1-forms along smooth maps

---

**Key Concepts:** tangent space, cotangent space, tangent vector as derivation, coordinate basis, tangent bundle, Lie bracket, flow of a vector field, 1-form, differential, dual transformation law

**Important Figures:** Bernhard Riemann, Élie Cartan, Charles Ehresmann, Hassler Whitney

**Additional Reading and Primary Sources:**
- Lee, *Introduction to Smooth Manifolds*, Chs. 3–4, 11
- Spivak, *Calculus on Manifolds*
- Carroll, *Spacetime and Geometry*, Ch. 2
- Misner, Thorne & Wheeler, *Gravitation*, Ch. 9

**Exercises:**
1. Verify that the Lie bracket [X,Y] is indeed a vector field (derivation) by checking the Leibniz rule.
2. Show that the coordinate basis {∂/∂xᵘ} and the dual basis {dxᵘ} satisfy ⟨dxᵘ, ∂/∂xᵛ⟩ = δᵘᵥ.
3. Compute the Lie bracket of X = x∂/∂x and Y = ∂/∂y in ℝ².
4. If F: M → N is smooth and ω is a 1-form on N, write out the pullback F*ω in local coordinates.

**Thought Experiments:**
- A vector field on a manifold is a rule that assigns a velocity to each point. What is the physical interpretation of the flow of the vector field? What is the flow of the 4-velocity field in spacetime?
- The coordinate transformation law for vectors (Aᵘ → ∂x'ᵘ/∂xᵛ Aᵛ) and covectors (ωᵤ → ∂xᵛ/∂x'ᵘ ωᵥ) are inverse transposes of each other. What is the geometrical meaning of this distinction?

---

## Chapter 29: Tensors on Manifolds

*Chapter Introduction: Tensors are multilinear maps that take vectors and covectors as inputs and return real numbers. They are the natural objects of differential geometry because they transform in a well-defined way under coordinate changes. The metric tensor, the Riemann curvature tensor, and the Einstein tensor are all tensors of various types.*

### 29.1 Tensors at a Point
- 29.1.1 Definition: a tensor of type (r,s) as a multilinear map
- 29.1.2 Tensor product: T ⊗ S constructs higher-rank tensors
- 29.1.3 Components of a tensor in a coordinate basis
- 29.1.4 Transformation law: Tᵘ'₁...ᵘ'ᵣᵥ'₁...ᵥ'ₛ = (∂x'ᵘ'₁/∂xᵃ¹)...(∂xᵇ¹/∂x'ᵛ'₁)... T^{a₁...aᵣ}_{b₁...bₛ}
- 29.1.5 Contraction of indices: reduces rank by 2
- 29.1.6 The metric as a (0,2) tensor; raising and lowering indices

### 29.2 Symmetric and Antisymmetric Tensors
- 29.2.1 Symmetrization and antisymmetrization
- 29.2.2 Symmetric tensors of type (0,2): the metric
- 29.2.3 Antisymmetric tensors of type (0,p): p-forms
- 29.2.4 The Levi-Civita symbol ε_{μ₁...μₙ} and the volume form

### 29.3 Tensor Fields
- 29.3.1 Tensor bundles; tensor fields as smooth sections
- 29.3.2 Tensor fields in local coordinates
- 29.3.3 Algebraic operations on tensor fields: sums, products, contractions

### 29.4 The Metric Tensor
- 29.4.1 A Riemannian metric: symmetric, positive definite
- 29.4.2 A pseudo-Riemannian (Lorentzian) metric: signature (-+++)
- 29.4.3 Metric components gᵘᵥ; the inverse metric gᵘᵥ: gᵘᵅgᵅᵥ = δᵘᵥ
- 29.4.4 Arc length and proper time from the metric: ds² = gᵘᵥ dxᵘ dxᵛ
- 29.4.5 The metric determines angles, distances, and causal structure

---

**Key Concepts:** tensor, tensor product, contraction, transformation law, symmetric and antisymmetric tensors, differential form, Levi-Civita symbol, metric tensor, Riemannian vs. Lorentzian signature

**Important Figures:** Gregorio Ricci-Curbastro, Tullio Levi-Civita, Élie Cartan, Bernhard Riemann

**Additional Reading and Primary Sources:**
- Ricci & Levi-Civita, *Methods of the Absolute Differential Calculus* (1900) — primary source
- Carroll, *Spacetime and Geometry*, Ch. 1–2
- Misner, Thorne & Wheeler, *Gravitation*, Chs. 3, 9–10
- Wald, *General Relativity*, Ch. 2

**Exercises:**
1. Show that the transformation law for tensor components is consistent with the earlier transformation laws for vectors and covectors.
2. Compute the components of gᵘᵥ in polar coordinates (r,θ) from the known flat metric ds² = dr² + r²dθ².
3. Show that the contraction of a symmetric tensor with an antisymmetric tensor vanishes.
4. Compute ε_{μνρσ} ε^{μνρσ} in 4 dimensions.

**Thought Experiments:**
- "A tensor is something that transforms like a tensor." What is the content of this statement? Why is this transformation law physically essential?
- The fact that tensor equations that hold in one coordinate system hold in all of them is the basis for the principle of general covariance. What is the physical meaning of this principle?

---

## Chapter 30: Differential Forms and Integration

*Chapter Introduction: Differential forms are antisymmetric tensors — the natural objects to integrate over submanifolds. The exterior derivative generalizes the gradient, curl, and divergence, and the generalized Stokes theorem unifies Green's, Stokes', and Gauss's theorems into one statement. This machinery is essential for understanding conserved currents, the Bianchi identities, and the global structure of spacetime.*

### 30.1 Differential Forms
- 30.1.1 The exterior algebra Λ^p(T*_pM)
- 30.1.2 p-forms in local coordinates; the wedge product ∧
- 30.1.3 Basis for p-forms: dxᵘ¹ ∧ ... ∧ dxᵘᵖ
- 30.1.4 The algebra of forms: Ω*(M)

### 30.2 The Exterior Derivative
- 30.2.1 Definition of d: Ωᵖ → Ωᵖ⁺¹
- 30.2.2 Properties: linearity, Leibniz rule for wedge product, d² = 0
- 30.2.3 d applied to 0-forms: recovers the gradient
- 30.2.4 d applied to 1-forms in ℝ³: recovers curl and divergence
- 30.2.5 Closed and exact forms; the Poincaré lemma

### 30.3 Integration of Differential Forms
- 30.3.1 Orientation and the volume form
- 30.3.2 Integration of an n-form over an oriented n-manifold
- 30.3.3 Manifolds with boundary; induced orientation

### 30.4 The Generalized Stokes Theorem
- 30.4.1 Statement: ∫_{∂M} ω = ∫_M dω
- 30.4.2 Special cases: fundamental theorem of calculus, Green's, Stokes', divergence theorems
- 30.4.3 de Rham cohomology: H^p(M) = (closed p-forms)/(exact p-forms)
- 30.4.4 Physical meaning: cohomology detects topological obstructions

### 30.5 Hodge Duality
- 30.5.1 The Hodge star operator *: Ωᵖ → Ωⁿ⁻ᵖ
- 30.5.2 The codifferential δ = (-1)^{np+n+1} * d *
- 30.5.3 The Laplace-Beltrami operator Δ = dδ + δd
- 30.5.4 Harmonic forms; Hodge decomposition theorem

---

**Key Concepts:** differential form, wedge product, exterior derivative, closed/exact form, Stokes theorem, de Rham cohomology, Hodge star, Laplace-Beltrami operator

**Important Figures:** Élie Cartan, Georges de Rham, William Vallance Douglas Hodge, Henri Poincaré

**Additional Reading and Primary Sources:**
- Cartan, *Differential Forms* (historical)
- de Rham, *Variétés différentiables* (historical)
- Flanders, *Differential Forms with Applications to the Physical Sciences*
- Misner, Thorne & Wheeler, *Gravitation*, Chs. 4–5

**Exercises:**
1. Verify that d(dω) = 0 for any smooth form ω.
2. Express the Maxwell equations ∂ᵤF^{μν} = μ₀Jᵛ and ∂_{[μ}F_{νρ]} = 0 in terms of differential forms: dF = 0 and d*F = μ₀J.
3. Compute the de Rham cohomology H^1(S¹) and interpret it physically.
4. Using the Hodge star on ℝ³, show that * maps 1-forms to 2-forms in the same way that the curl maps vectors to vectors.

**Thought Experiments:**
- The Bianchi identity for the Riemann tensor can be written as d(Riemann) = 0 (schematically). In what sense is this a statement about the "closure" of curvature?
- de Rham cohomology measures the topology of a manifold via the failure of closed forms to be exact. Why would the topology of spacetime have physical consequences?

---

## Chapter 31: Connections and Covariant Derivatives

*Chapter Introduction: On a manifold, there is no canonical way to compare vectors at different points. A connection provides this structure — it defines what it means to "parallel transport" a vector along a curve. The covariant derivative is the infinitesimal version of this comparison. In general relativity, the metric uniquely determines a connection (the Levi-Civita connection), whose curvature is the gravitational field.*

### 31.1 The Problem with Ordinary Derivatives
- 31.1.1 Why partial derivatives of tensor components do not form a tensor
- 31.1.2 The need for a connection to differentiate tensors

### 31.2 Affine Connections
- 31.2.1 Definition of a connection ∇ on a manifold
- 31.2.2 Connection coefficients (Christoffel symbols of second kind) Γᵅᵘᵥ
- 31.2.3 The covariant derivative of a vector field: ∇ᵥW
- 31.2.4 Transformation law for Γᵅᵘᵥ: why it is not a tensor
- 31.2.5 Covariant derivative of arbitrary tensors
- 31.2.6 Covariant divergence; the contracted Bianchi identity

### 31.3 Parallel Transport
- 31.3.1 Definition: a vector field V is parallel transported along γ if ∇_{γ̇}V = 0
- 31.3.2 Path-dependence of parallel transport: measure of curvature
- 31.3.3 Holonomy and the holonomy group

### 31.4 The Levi-Civita Connection
- 31.4.1 Metric compatibility: ∇g = 0
- 31.4.2 Torsion tensor Tᵅᵘᵥ = Γᵅᵘᵥ - Γᵅᵥᵘ
- 31.4.3 The fundamental theorem of Riemannian geometry: unique torsion-free, metric-compatible connection
- 31.4.4 Christoffel symbols from the metric: Γᵅᵘᵥ = (1/2)gᵅσ(∂ᵘgᵥσ + ∂ᵥgᵘσ - ∂σgᵘᵥ)

---

**Key Concepts:** covariant derivative, connection, Christoffel symbols, parallel transport, holonomy, torsion, metric compatibility, Levi-Civita connection, Ricci's theorem (∇g = 0)

**Important Figures:** Gregorio Ricci-Curbastro, Tullio Levi-Civita, Élie Cartan, Jan Arnoldus Schouten, Hermann Weyl

**Additional Reading and Primary Sources:**
- Levi-Civita, *The Absolute Differential Calculus* (1927)
- Carroll, *Spacetime and Geometry*, Ch. 3
- Wald, *General Relativity*, Ch. 3
- Misner, Thorne & Wheeler, *Gravitation*, Ch. 10

**Exercises:**
1. Compute the Christoffel symbols for the 2-sphere S² with metric ds² = dθ² + sin²θ dφ².
2. Show that the covariant derivative of a (1,1) tensor has the form ∇ᵅTᵘᵥ = ∂ᵅTᵘᵥ + ΓᵘᵅσTσᵥ - ΓσᵅᵥTᵘσ.
3. Parallel transport a vector around a small closed loop on S² and compute the angle by which it rotates.
4. Derive the Christoffel symbols from the condition ∇g = 0 and zero torsion.

**Thought Experiments:**
- Parallel transport a vector along a great circle on S², return it to the starting point by a different great circle: it points in a different direction. What does this holonomy tell you about the curvature of S²?
- In special relativity, the "connection" is zero (flat spacetime, Minkowski metric in Cartesian coordinates). What does this mean for geodesics?

---

## Chapter 32: Curvature

*Chapter Introduction: Curvature is the central concept of differential geometry and the mathematical heart of general relativity. The Riemann curvature tensor encodes how parallel transport is path-dependent; its contractions — the Ricci tensor and Ricci scalar — appear directly in Einstein's field equations. The Weyl tensor carries information about the "free gravitational field" not determined by local matter.*

### 32.1 The Riemann Curvature Tensor
- 32.1.1 Definition from the failure of covariant derivatives to commute: [∇ᵘ, ∇ᵥ]Wᵅ = Rᵅ_{βμν}Wᵝ
- 32.1.2 Components in terms of Christoffel symbols:
  Rᵅ_{βμν} = ∂ᵘΓᵅᵥβ - ∂ᵥΓᵅᵘβ + ΓᵅᵘσΓσᵥβ - ΓᵅᵥσΓσᵘβ
- 32.1.3 Geometric interpretation: rotation of a vector under parallel transport around an infinitesimal loop
- 32.1.4 Symmetries of the Riemann tensor: R_{αβγδ} = -R_{βαγδ} = -R_{αβδγ} = R_{γδαβ}
- 32.1.5 Number of independent components: 20 in 4D

### 32.2 Algebraic Identities for Riemann
- 32.2.1 The first Bianchi identity: R_{α[βγδ]} = 0
- 32.2.2 The second Bianchi identity (differential): ∇_{[μ}R_{ν ρ]αβ} = 0
- 32.2.3 Consequences of the contracted Bianchi identity: ∇ᵘGᵘᵥ = 0

### 32.3 The Ricci Tensor and Ricci Scalar
- 32.3.1 Ricci tensor: Rᵘᵥ = Rᵅ_{μαν} (contraction of Riemann)
- 32.3.2 Symmetry of Ricci tensor: Rᵘᵥ = Rᵥᵘ
- 32.3.3 Ricci scalar: R = gᵘᵥRᵘᵥ
- 32.3.4 Physical interpretation: Ricci tensor measures the convergence of geodesic families (geodesic deviation)

### 32.4 The Weyl Tensor
- 32.4.1 Decomposition of the Riemann tensor: Riemann = Weyl + Ricci part + Scalar part
- 32.4.2 The Weyl tensor Cᵅβγδ: traceless part of Riemann
- 32.4.3 Physical meaning: Weyl curvature = gravitational waves and tidal forces not tied to local matter (free gravitational field)
- 32.4.4 Conformally flat manifolds: Weyl = 0

### 32.5 The Einstein Tensor
- 32.5.1 The Einstein tensor: Gᵘᵥ = Rᵘᵥ - (1/2)gᵘᵥR
- 32.5.2 Conservation: ∇ᵘGᵘᵥ = 0 (Bianchi identity)
- 32.5.3 Significance: Gᵘᵥ is the only divergence-free combination of the metric and its first two derivatives that is linear in second derivatives (Lovelock's theorem preview)

### 32.6 Sectional, Ricci, and Scalar Curvatures
- 32.6.1 Sectional curvature K(u,v)
- 32.6.2 Spaces of constant curvature: sphere (K>0), Euclidean (K=0), hyperbolic (K<0)
- 32.6.3 Schur's theorem: constant sectional curvature implies constant Ricci curvature

---

**Key Concepts:** Riemann tensor, Bianchi identities, Ricci tensor, Ricci scalar, Weyl tensor, Einstein tensor, sectional curvature, geodesic deviation, free gravitational field

**Important Figures:** Bernhard Riemann, Gregorio Ricci-Curbastro, Tullio Levi-Civita, Albert Einstein, Hermann Weyl, Luigi Bianchi

**Additional Reading and Primary Sources:**
- Riemann, *On the Hypotheses Which Lie at the Foundations of Geometry* (1854) — primary source
- Carroll, *Spacetime and Geometry*, Ch. 3
- Wald, *General Relativity*, Ch. 3
- Misner, Thorne & Wheeler, *Gravitation*, Chs. 11–13

**Exercises:**
1. Compute all non-zero components of the Riemann tensor for S² with the standard metric.
2. Show that in 2 dimensions, the Riemann tensor has only one independent component (the Gaussian curvature K = R/2).
3. Verify the contracted Bianchi identity: ∇ᵘGᵘᵥ = 0.
4. Show that the Weyl tensor is traceless: gᵘᵛCᵘαβᵥ = 0.

**Thought Experiments:**
- The Riemann tensor vanishes iff spacetime is (locally) flat. In what sense is the Riemann tensor the "gravitational field" in GR?
- In a vacuum (T_{μν} = 0), Einstein's equations imply Rᵘᵥ = 0, but not necessarily R_{αβγδ} = 0. How can spacetime be curved in vacuum? (Think: gravitational waves, tidal forces from distant sources.)

---

## Chapter 33: Geodesics and the Exponential Map

*Chapter Introduction: Geodesics are the straightest possible curves on a manifold — the paths that parallel-transport their own tangent vectors. In a Riemannian manifold, they are locally length-minimizing. In a Lorentzian manifold (spacetime), timelike geodesics maximize proper time and represent the worldlines of freely falling particles. The geodesic equation is the equation of motion in GR.*

### 33.1 The Geodesic Equation
- 33.1.1 Definition: a curve γ(τ) is a geodesic if ∇_{γ̇}γ̇ = 0
- 33.1.2 Geodesic equation in coordinates: d²xᵘ/dτ² + Γᵘᵥσ (dxᵛ/dτ)(dxσ/dτ) = 0
- 33.1.3 Affine parametrization; relationship between different affine parameters
- 33.1.4 Geodesics from the variational principle: extremize ∫gᵘᵥ(dxᵘ/dλ)(dxᵛ/dλ) dλ

### 33.2 Examples of Geodesics
- 33.2.1 Geodesics on ℝⁿ: straight lines
- 33.2.2 Geodesics on S²: great circles
- 33.2.3 Geodesics in Minkowski spacetime: straight worldlines = inertial motion
- 33.2.4 Preview: geodesics in Schwarzschild spacetime (Chapter 41)

### 33.3 Geodesic Deviation and the Jacobi Equation
- 33.3.1 One-parameter families of geodesics; the deviation vector
- 33.3.2 The Jacobi (geodesic deviation) equation:
  D²Jᵘ/dτ² = -Rᵘ_{νρσ} (dxᵛ/dτ) Jρ (dxσ/dτ)
- 33.3.3 Physical meaning: tidal forces as geodesic deviation
- 33.3.4 Focal points (conjugate points) and their significance

### 33.4 The Exponential Map
- 33.4.1 Definition: exp_p: T_pM → M maps tangent vectors to geodesics
- 33.4.2 Local diffeomorphism near p; normal coordinates
- 33.4.3 In normal coordinates: gᵘᵥ(p) = ηᵘᵥ, Γᵅᵘᵥ(p) = 0
- 33.4.4 Geodesic completeness; the Hopf-Rinow theorem

### 33.5 The Raychaudhuri Equation
- 33.5.1 Expansion, shear, and twist of a congruence of geodesics
- 33.5.2 The Raychaudhuri equation: dθ/dτ = -θ²/3 - σᵘᵥσᵘᵥ + ωᵘᵥωᵘᵥ - Rᵘᵥuᵘuᵛ
- 33.5.3 Focusing theorem: positive energy conditions cause geodesic convergence
- 33.5.4 Applications to singularity theorems (preview)

---

**Key Concepts:** geodesic, geodesic equation, affine parameter, geodesic deviation, Jacobi equation, tidal forces, exponential map, normal coordinates, Raychaudhuri equation, expansion/shear/twist

**Important Figures:** Bernhard Riemann, Tullio Levi-Civita, Amal Kumar Raychaudhuri, Heinz Hopf, Willi Rinow

**Additional Reading and Primary Sources:**
- Raychaudhuri, *Relativistic Cosmology* (1955) — primary source for Raychaudhuri equation
- Carroll, *Spacetime and Geometry*, Ch. 3
- Wald, *General Relativity*, Ch. 3, 9
- Misner, Thorne & Wheeler, *Gravitation*, Ch. 10–11

**Exercises:**
1. Derive the geodesic equation from the variational principle δ∫ds = 0.
2. Find the geodesics on S² using the Euler-Lagrange equations for the metric ds² = dθ² + sin²θ dφ².
3. Derive the Jacobi equation from the definition of the Riemann tensor.
4. Show that in normal coordinates centered at p, the Christoffel symbols vanish at p.

**Thought Experiments:**
- In normal coordinates, spacetime is locally flat at a point. What is the physical interpretation: is spacetime actually flat there, or just "looks flat" in this special frame? (Relevance: the equivalence principle.)
- Two nearby geodesics that are initially parallel will deviate due to curvature. In GR, what physical phenomenon does this describe? Think about the tide-raising force of the Moon.

---

# UNIT IX: Foundations of General Relativity

*Unit Introduction: With all the mathematical machinery in place, we are ready to formulate Einstein's theory of general relativity. This unit develops the physical motivations — the equivalence principle, the inadequacy of special relativity for gravity — and then states the field equations. Every chapter in this unit is a direct encounter with one of the deepest ideas in physics.*

---

## Chapter 34: Physical Motivation and the Equivalence Principle

*Chapter Introduction: General relativity begins with a physical insight, not a mathematical one: the observation that inertial mass and gravitational mass are equal. Einstein elevated this coincidence to a fundamental principle and used it to argue that gravity is not a force at all, but a manifestation of curved spacetime. This chapter traces that argument carefully.*

### 34.1 The Equality of Inertial and Gravitational Mass
- 34.1.1 Inertial mass: resistance to acceleration (F = mᵢa)
- 34.1.2 Gravitational mass: coupling to gravity (F = mₘ g)
- 34.1.3 Equivalence of mᵢ and mₘ: the Eötvös experiments
- 34.1.4 Historical tests: Galileo, Newton, Eötvös, modern precision tests
- 34.1.5 Why Newtonian gravity predicts mᵢ = mₘ but does not explain it

### 34.2 Einstein's Elevator: The Weak Equivalence Principle
- 34.2.1 Uniformly accelerating frame ≡ uniform gravitational field (locally)
- 34.2.2 The Weak Equivalence Principle (WEP): all test bodies fall the same way in a gravitational field
- 34.2.3 Implications: in free fall, no local mechanical experiment detects gravity
- 34.2.4 Free-fall frames as locally inertial frames

### 34.3 The Einstein Equivalence Principle (EEP)
- 34.3.1 Extension to all local physics, not just mechanics
- 34.3.2 EEP: in free fall, all local non-gravitational experiments give the same result as in special relativity
- 34.3.3 Gravitational redshift derived from EEP
- 34.3.4 Clock rates in a gravitational field
- 34.3.5 The Pound-Rebka experiment (1959)

### 34.4 Gravity Must Curve Spacetime
- 34.4.1 Argument: EEP + special relativity → light bends in gravitational fields → spacetime is curved
- 34.4.2 The deflection of light as a test
- 34.4.3 Mach's principle and Einstein's motivation
- 34.4.4 Limits of the equivalence principle: tidal forces cannot be transformed away

### 34.5 The Strong Equivalence Principle
- 34.5.1 SEP: includes self-gravitating bodies and gravitational experiments
- 34.5.2 Nordtvedt effect; post-Newtonian tests
- 34.5.3 Theories that violate SEP

---

**Key Concepts:** inertial mass, gravitational mass, weak equivalence principle, Einstein equivalence principle, strong equivalence principle, free-fall frame, gravitational redshift, tidal forces

**Important Figures:** Galileo Galilei, Isaac Newton, Roland von Eötvös, Albert Einstein, Robert Pound, Glen Rebka

**Additional Reading and Primary Sources:**
- Einstein, *The Foundation of the General Theory of Relativity* (1916) — primary source
- Einstein, *On the Influence of Gravitation on the Propagation of Light* (1911)
- Will, *Theory and Experiment in Gravitational Physics*

**Exercises:**
1. Derive the gravitational redshift formula Δν/ν = gh/c² from the equivalence principle.
2. Estimate the deflection of starlight passing the Sun using the equivalence principle (get half the GR answer; explain why).
3. Calculate the gravitational time dilation between sea level and a GPS satellite at altitude 20,200 km.

**Thought Experiments:**
- Einstein's elevator: you are in a sealed elevator accelerating at g. You fire a laser horizontally. It curves downward. In the freely-falling frame, the laser goes straight. Reconcile this.
- If gravity is just curved spacetime, what happens to Newtonian action-at-a-distance? How does GR propagate gravitational influences?

---

## Chapter 35: The Einstein Field Equations

*Chapter Introduction: The field equations G_{μν} = 8πG/c⁴ T_{μν} are the heart of general relativity. They relate the geometry of spacetime (the Einstein tensor G_{μν}, encoding curvature) to the distribution of matter and energy (the stress-energy tensor T_{μν}). This chapter derives them heuristically and presents them precisely, then interprets their content.*

### 35.1 Requirements for a Relativistic Theory of Gravity
- 35.1.1 Must reduce to Poisson's equation ∇²Φ = 4πGρ in the Newtonian limit
- 35.1.2 Source must be a tensor (T_{μν}), not just ρ
- 35.1.3 The field must be a symmetric rank-2 tensor
- 35.1.4 Equations must be generally covariant

### 35.2 Heuristic Derivation
- 35.2.1 Analogy: Poisson's equation → R_{μν} = κ T_{μν}?
- 35.2.2 Problem: ∇^μ T_{μν} = 0 requires ∇^μ R_{μν} = 0, but this fails
- 35.2.3 The Einstein tensor G_{μν} = R_{μν} - ½g_{μν}R is divergence-free by Bianchi
- 35.2.4 Einstein's equation: G_{μν} = (8πG/c⁴) T_{μν}

### 35.3 The Cosmological Constant
- 35.3.1 Einstein's original motivation: static universe
- 35.3.2 G_{μν} + Λg_{μν} = (8πG/c⁴) T_{μν}
- 35.3.3 Λ as vacuum energy; modern interpretation
- 35.3.4 "The greatest blunder": Hubble expansion and reconsideration

### 35.4 The Action Principle: Einstein-Hilbert Action
- 35.4.1 The Hilbert action: S = (c⁴/16πG) ∫R √(-g) d⁴x + S_matter
- 35.4.2 Variation of √(-g): δ√(-g) = -½√(-g) g_{μν} δg^{μν}
- 35.4.3 Variation of R: the Palatini identity
- 35.4.4 Deriving G_{μν} = (8πG/c⁴) T_{μν} from δS/δg^{μν} = 0

### 35.5 Physical Content of the Field Equations
- 35.5.1 Ten equations (symmetric 4×4 tensor), four of which are constraints
- 35.5.2 The equations as conditions relating curvature to matter
- 35.5.3 Conservation of energy-momentum: ∇^μ T_{μν} = 0 as a consequence
- 35.5.4 Counting degrees of freedom: the gravitational field has 2 independent propagating modes (gravitational waves)

### 35.6 The Newtonian Limit
- 35.6.1 Weak field and slow motion approximations
- 35.6.2 g_{00} ≈ -(1 + 2Φ/c²); recovery of ∇²Φ = 4πGρ
- 35.6.3 The constant (8πG/c⁴) fixed by matching to Newton

---

**Key Concepts:** Einstein field equations, Einstein tensor, stress-energy tensor, cosmological constant, Einstein-Hilbert action, Newtonian limit, degrees of freedom

**Important Figures:** Albert Einstein, David Hilbert, Élie Cartan, Hermann Weyl, Arthur Eddington

**Additional Reading and Primary Sources:**
- Einstein, *The Foundation of the General Theory of Relativity* (1916) — primary source (Annalen der Physik)
- Hilbert, *The Foundations of Physics* (1915) — primary source
- Wald, *General Relativity*, Ch. 4
- Carroll, *Spacetime and Geometry*, Ch. 4
- Misner, Thorne & Wheeler, *Gravitation*, Chs. 17–21

**Exercises:**
1. Vary the Einstein-Hilbert action to derive the vacuum Einstein equations G_{μν} = 0.
2. Show that the trace of the Einstein equations in 4D gives R = -(8πG/c⁴)T, where T = g^{μν}T_{μν}.
3. Verify the Newtonian limit: show g_{00} = -(1+2Φ/c²) satisfies the linearized Einstein equations when T_{00} = ρc².
4. Show that the Bianchi identity ∇^μ G_{μν} = 0 implies ∇^μ T_{μν} = 0.

**Thought Experiments:**
- Hilbert submitted a paper with the correct field equations in essentially the same week as Einstein. Who deserves credit, and why does this matter for the history of physics?
- The field equations are ten nonlinear coupled PDEs. Why are they so hard to solve, and why are exact solutions so rare and precious?

---

## Chapter 36: The Stress-Energy Tensor

*Chapter Introduction: The right-hand side of Einstein's equations is the stress-energy tensor T_{μν}, which encodes all the energy, momentum, pressure, and stress of matter and fields. Understanding its form for different types of matter is essential for solving the field equations in physical situations.*

### 36.1 General Properties of T_{μν}
- 36.1.1 T^{μν}: symmetric (μ↔ν), units of energy density
- 36.1.2 Components: T^{00} = energy density, T^{0i} = momentum density / energy flux, T^{ij} = momentum flux (stress)
- 36.1.3 Conservation: ∇_μ T^{μν} = 0

### 36.2 Dust (Pressureless Perfect Fluid)
- 36.2.1 T^{μν} = ρ u^μ u^ν
- 36.2.2 Conservation equations: continuity and geodesic equation

### 36.3 Perfect Fluid
- 36.3.1 T^{μν} = (ρ + p/c²)u^μ u^ν + p g^{μν}
- 36.3.2 Equation of state p = p(ρ)
- 36.3.3 Special cases: radiation (p = ρ/3), stiff fluid, dark energy (p = -ρ)

### 36.4 Electromagnetic Field
- 36.4.1 T^{μν}_{EM} = F^{μα}F^ν_α - ¼g^{μν}F_{αβ}F^{αβ}
- 36.4.2 Traceless: T^μ_μ = 0 for EM field
- 36.4.3 Coupling EM to gravity: Einstein-Maxwell equations

### 36.5 The Scalar Field
- 36.5.1 Lagrangian: ℒ = -½(∂_μφ)(∂^μφ) - V(φ)
- 36.5.2 Stress-energy tensor for a scalar field
- 36.5.3 Klein-Gordon equation in curved spacetime

### 36.6 Energy Conditions
- 36.6.1 Weak energy condition (WEC): T_{μν}t^μt^ν ≥ 0 for timelike t^μ
- 36.6.2 Null energy condition (NEC)
- 36.6.3 Strong energy condition (SEC): needed for Raychaudhuri focusing
- 36.6.4 Dominant energy condition
- 36.6.5 Violations: quantum fields, dark energy, exotic matter

---

**Key Concepts:** stress-energy tensor, dust, perfect fluid, equation of state, energy conditions, WEC, NEC, SEC, traceless EM stress tensor

**Important Figures:** Albert Einstein, Hermann Weyl, Lev Landau, Evgeny Lifshitz

**Additional Reading and Primary Sources:**
- Landau & Lifshitz, *Classical Theory of Fields*, Ch. 4
- Wald, *General Relativity*, Ch. 4
- Misner, Thorne & Wheeler, *Gravitation*, Chs. 5, 22

**Exercises:**
1. Verify ∇_μ T^{μν} = 0 for a perfect fluid and show it implies the Euler equation and continuity equation.
2. Compute T^{μν} for an isotropic radiation field and verify the trace equals zero.
3. Show that the WEC implies the NEC but not vice versa.

**Thought Experiments:**
- Why should pressure contribute to gravity in GR when it does not in Newtonian gravity? What is the physical meaning of the off-diagonal components T^{0i}?
- Exotic matter with negative energy density violates the WEC. Why would such matter be needed for traversable wormholes, and why do we suspect it cannot exist classically?

---

## Chapter 37: The Geodesic Equation as the Equation of Motion

*Chapter Introduction: In GR, freely falling test particles move along geodesics of the spacetime metric. This is not a separate postulate but a consequence of the field equations (via the conservation of the stress-energy tensor). This chapter develops this connection and explores the physics of free fall as curved-spacetime geometry.*

### 37.1 Free Fall as Geodesic Motion
- 37.1.1 Derivation from ∇_μ T^{μν} = 0 for dust: worldlines are geodesics
- 37.1.2 The geodesic equation: d²x^μ/dτ² + Γ^μ_{νρ}(dx^ν/dτ)(dx^ρ/dτ) = 0
- 37.1.3 Physical interpretation: Γ^μ_{νρ} terms are the "gravitational force" in the chosen coordinates
- 37.1.4 These terms vanish in locally inertial (free-fall) coordinates: the equivalence principle

### 37.2 Conserved Quantities Along Geodesics
- 37.2.1 Killing vectors: ξ^μ with ∇_{(μ}ξ_{ν)} = 0
- 37.2.2 Conservation: p_μ ξ^μ = const along a geodesic
- 37.2.3 Spherical symmetry → two Killing vectors (∂_t and ∂_φ) → energy and angular momentum conserved
- 37.2.4 General method for finding constants of geodesic motion

### 37.3 Massive and Massless Geodesics
- 37.3.1 Timelike geodesics: g_{μν}(dx^μ/dτ)(dx^ν/dτ) = -c²; proper time parametrization
- 37.3.2 Null geodesics: g_{μν}(dx^μ/dλ)(dx^ν/dλ) = 0; affine parameter
- 37.3.3 Spacelike geodesics and their physical meaning

### 37.4 Motion of Extended Bodies
- 37.4.1 The Mathisson-Papapetrou-Dixon equations for spinning bodies
- 37.4.2 Spin-orbit coupling and precession
- 37.4.3 The self-force problem and radiation reaction

---

**Key Concepts:** geodesic equation, free fall, Christoffel symbols as gravitational force, Killing vector, conserved quantities, timelike/null geodesics, Mathisson-Papapetrou equations

**Important Figures:** Albert Einstein, Myron Mathisson, Achille Papapetrou, William Dixon, Wilhelm de Sitter

**Additional Reading and Primary Sources:**
- Einstein, Infeld & Hoffmann, *The Gravitational Equations and the Problem of Motion* (1938)
- Wald, *General Relativity*, Ch. 3
- Carroll, *Spacetime and Geometry*, Ch. 3

**Exercises:**
1. Show that the Killing equation ∇_{(μ}ξ_{ν)} = 0 implies p_μξ^μ is conserved along any geodesic.
2. Find all Killing vectors of Minkowski spacetime (there are 10: translations, Lorentz boosts, rotations).
3. For a geodesic in a static, spherically symmetric spacetime, use the two Killing vectors to reduce the geodesic equation to an effective one-dimensional problem.

**Thought Experiments:**
- A geodesic is the "straightest possible" path. Why does a freely falling astronaut feel weightless even though they are "accelerating" in Newtonian terms?
- Killing vectors are the GR generalization of conservation laws. What is the analog of Noether's theorem here?


---

# UNIT X: Exact Solutions and Classical Tests

*Unit Introduction: Solving the Einstein field equations exactly is extremely difficult — they are ten coupled nonlinear PDEs. Yet a handful of exact solutions have been found that describe the most physically important spacetimes: the vacuum field outside a spherical mass (Schwarzschild), the spacetime around a rotating mass (Kerr), and the spacetime of the Universe (FLRW). This unit develops the major solutions and the observational tests that confirm GR.*

---

## Chapter 38: The Schwarzschild Solution

*Chapter Introduction: Karl Schwarzschild found the first exact solution to Einstein's field equations just weeks after Einstein published his theory in 1915, from the trenches of World War I. The Schwarzschild metric describes the unique spherically symmetric vacuum spacetime. It is the basis for understanding weak-field GR (planetary orbits, light bending) and strong-field GR (black holes).*

### 38.1 Derivation by Birkhoff's Theorem
- 38.1.1 Symmetry assumptions: static and spherically symmetric vacuum
- 38.1.2 The general spherically symmetric metric
- 38.1.3 Solving the vacuum Einstein equations G_{μν} = 0
- 38.1.4 Birkhoff's theorem: the unique solution is the Schwarzschild metric
- 38.1.5 The Schwarzschild metric: ds² = -(1-2GM/c²r)c²dt² + (1-2GM/c²r)⁻¹dr² + r²dΩ²

### 38.2 Properties of the Schwarzschild Metric
- 38.2.1 Asymptotic flatness: reduces to Minkowski as r → ∞
- 38.2.2 The Schwarzschild radius r_s = 2GM/c²
- 38.2.3 Coordinate singularity at r = r_s vs. physical singularity at r = 0
- 38.2.4 The Newtonian limit: weak field reduces to Φ = -GM/r

### 38.3 Geodesics in Schwarzschild Spacetime
- 38.3.1 Timelike geodesics: two constants of motion (energy E, angular momentum L)
- 38.3.2 The effective potential V_eff(r) for radial motion
- 38.3.3 Circular orbits; innermost stable circular orbit (ISCO) at r = 6GM/c²
- 38.3.4 Null geodesics; the photon sphere at r = 3GM/c²
- 38.3.5 Radial free fall: coordinate time vs. proper time

### 38.4 Gravitational Redshift and Time Dilation
- 38.4.1 Gravitational redshift: 1 + z = (1-2GM/c²r)^{-1/2}
- 38.4.2 Comparison with EEP prediction
- 38.4.3 Experimental verification: Pound-Rebka, GPS

---

**Key Concepts:** Schwarzschild metric, Birkhoff's theorem, Schwarzschild radius, coordinate vs. curvature singularity, effective potential, ISCO, photon sphere, gravitational redshift

**Important Figures:** Karl Schwarzschild, George Birkhoff, Johannes Droste

**Additional Reading and Primary Sources:**
- Schwarzschild, *On the Gravitational Field of a Mass Point According to Einstein's Theory* (1916) — primary source
- Carroll, *Spacetime and Geometry*, Ch. 5
- Wald, *General Relativity*, Ch. 6
- Misner, Thorne & Wheeler, *Gravitation*, Chs. 23–25

**Exercises:**
1. Derive the Schwarzschild metric by solving G_{μν} = 0 with a static spherically symmetric ansatz.
2. Find the effective potential for radial timelike geodesics and identify the ISCO.
3. Compute the proper time for a test particle to fall from r = 10GM/c² to r = 0, and compare with coordinate time.
4. Derive the gravitational redshift formula from the Schwarzschild metric.

**Thought Experiments:**
- Schwarzschild found this solution while serving on the Russian front in WWI and died months later. He did not know it described a "black hole." What did he think it described?
- An observer far from the Schwarzschild mass watches a test particle fall toward r_s. The particle's image becomes infinitely redshifted and appears to "freeze" at r_s. Meanwhile, the freely falling observer reaches r_s in finite proper time. How do we reconcile these two descriptions?

---

## Chapter 39: Classical Tests of General Relativity

*Chapter Introduction: Einstein proposed three observational tests of general relativity, all in the weak-field, slow-motion limit of the Schwarzschild solution. The dramatic confirmation of the bending of light by the Sun in 1919 made Einstein world-famous. This chapter derives all three tests and introduces modern precision tests.*

### 39.1 The Precession of Mercury's Perihelion
- 39.1.1 Newtonian orbit: closed ellipses (Bertrand's theorem)
- 39.1.2 The Schwarzschild correction to the effective potential
- 39.1.3 Derivation of perihelion advance: Δφ = 6πGM/(c²a(1-e²)) per orbit
- 39.1.4 Application to Mercury: 43 arcseconds/century
- 39.1.5 Historical background: Le Verrier's 1859 anomaly; failed Newtonian explanations

### 39.2 The Deflection of Light by the Sun
- 39.2.1 Null geodesic equation in Schwarzschild geometry
- 39.2.2 Derivation of deflection angle: δφ = 4GM/(c²b), where b = impact parameter
- 39.2.3 This is twice the Newtonian prediction
- 39.2.4 The 1919 Eddington expedition; confirmation
- 39.2.5 Modern tests: VLBI, quasar deflection

### 39.3 Gravitational Redshift (Repeat in Context)
- 39.3.1 Derivation from Schwarzschild metric
- 39.3.2 Pound-Rebka experiment
- 39.3.3 Gravity Probe A (1976)

### 39.4 The Shapiro Time Delay
- 39.4.1 Radar signal round-trip time in Schwarzschild geometry
- 39.4.2 Shapiro delay: Δt = (4GM/c³)[ln(4r_e r_r/b²) + 1]
- 39.4.3 Viking landers test (1976); Cassini test (2003)

### 39.5 Geodetic and Frame-Dragging Precession
- 39.5.1 Geodetic (de Sitter) precession: a gyroscope in orbit precesses
- 39.5.2 Lense-Thirring (frame-dragging) precession from Kerr geometry
- 39.5.3 Gravity Probe B (2004–2011): confirmation

### 39.6 Post-Newtonian Formalism (PPN)
- 39.6.1 The parameterized post-Newtonian (PPN) framework
- 39.6.2 PPN parameters γ and β
- 39.6.3 Observational constraints; GR passes all tests

---

**Key Concepts:** perihelion advance, light deflection, Shapiro delay, geodetic precession, Lense-Thirring effect, PPN framework

**Important Figures:** Albert Einstein, Arthur Eddington, Irwin Shapiro, Willem de Sitter, Josef Lense, Hans Thirring, Urbain Le Verrier

**Additional Reading and Primary Sources:**
- Einstein, *Explanation of the Perihelion Motion of Mercury from the General Theory of Relativity* (1915)
- Shapiro, *Fourth Test of General Relativity* (1964)
- Will, *Theory and Experiment in Gravitational Physics*
- Everitt et al., *Gravity Probe B: Final Results* (2011)

**Exercises:**
1. Derive the relativistic perihelion advance formula Δφ = 6πGM/(c²a(1-e²)) from the Schwarzschild geodesic equation.
2. Show that the GR light deflection 4GM/(c²b) is twice the prediction from a "Newtonian" calculation treating light as a massive particle.
3. Estimate the Shapiro delay for a radar signal bounced off Venus at superior conjunction.

**Thought Experiments:**
- Eddington's 1919 eclipse observations had significant measurement uncertainties. Were they truly a decisive test of GR, or did they play a role as much sociological as scientific?
- GPS satellites must account for both special relativistic time dilation (satellites move fast: clocks slow) and gravitational time dilation (satellites are higher up: clocks fast). Which effect dominates, and what would happen if GR corrections were not applied?

---

## Chapter 40: Black Holes in Schwarzschild Spacetime

*Chapter Introduction: When a mass is compressed below its Schwarzschild radius, the coordinate singularity at r = r_s becomes an event horizon — a one-way surface from which nothing, not even light, can escape. This chapter develops the full causal structure of the Schwarzschild black hole, introduces the Kruskal-Szekeres extension, and begins the study of black hole physics.*

### 40.1 The Event Horizon
- 40.1.1 Definition: the event horizon as a null surface (the boundary of the causal past of future null infinity)
- 40.1.2 Ingoing and outgoing null geodesics at r = r_s
- 40.1.3 Nothing can escape from r < r_s
- 40.1.4 The event horizon is a global, not a local, concept

### 40.2 Eddington-Finkelstein Coordinates
- 40.2.1 Ingoing Eddington-Finkelstein coordinates: removing the coordinate singularity
- 40.2.2 Radial null geodesics in these coordinates
- 40.2.3 Outgoing Eddington-Finkelstein coordinates

### 40.3 Kruskal-Szekeres Coordinates and the Maximal Extension
- 40.3.1 Motivation: find a coordinate system regular everywhere except r = 0
- 40.3.2 Kruskal-Szekeres coordinates (T, X)
- 40.3.3 The maximal extension: four regions (exterior, interior, white hole, parallel exterior)
- 40.3.4 The Einstein-Rosen bridge (wormhole)
- 40.3.5 The Penrose diagram of the Schwarzschild black hole

### 40.4 The Physical Singularity at r = 0
- 40.4.1 Curvature invariants diverge at r = 0: physical singularity
- 40.4.2 Tidal forces become infinite at the singularity (spaghettification)
- 40.4.3 The singularity is spacelike in Schwarzschild: it lies in the future of infalling observers

### 40.5 Black Hole Formation: Gravitational Collapse
- 40.5.1 Oppenheimer-Snyder model of pressureless dust collapse
- 40.5.2 Formation of the event horizon before the singularity
- 40.5.3 No-hair theorem (conjecture): stationary black holes characterized by M, J, Q only
- 40.5.4 Price's theorem: perturbations radiate away; black holes settle to Kerr-Newman

---

**Key Concepts:** event horizon, Eddington-Finkelstein coordinates, Kruskal-Szekeres coordinates, Penrose diagram, maximal extension, white hole, Einstein-Rosen bridge, gravitational collapse, no-hair theorem

**Important Figures:** Karl Schwarzschild, David Finkelstein, Martin Kruskal, George Szekeres, J. Robert Oppenheimer, Hartland Snyder, John Wheeler, Roger Penrose, Richard Price

**Additional Reading and Primary Sources:**
- Oppenheimer & Snyder, *On Continued Gravitational Contraction* (1939) — primary source
- Kruskal, *Maximal Extension of Schwarzschild Metric* (1960)
- Wheeler, *Geons, Black Holes, and Quantum Foam* (autobiography)
- Carroll, *Spacetime and Geometry*, Ch. 5
- Wald, *General Relativity*, Ch. 6, 12

**Exercises:**
1. Show that r = r_s is a coordinate singularity in Schwarzschild coordinates but not in Kruskal-Szekeres coordinates.
2. Draw the Penrose diagram for the maximally extended Schwarzschild spacetime and label all four regions.
3. Compute the tidal force on an astronaut of height 2 m falling radially into a stellar-mass (10 M☉) black hole. At what radius is the force dangerously large?
4. Derive the Eddington-Finkelstein metric from the Schwarzschild metric by the coordinate change v = t + r + r_s ln|r/r_s - 1|.

**Thought Experiments:**
- An observer falling freely into a black hole crosses the event horizon in finite proper time and feels nothing special at the crossing (tidal forces are small for a large black hole). Yet an outside observer never sees them cross. Are these descriptions contradictory?
- The no-hair theorem says a black hole has no "hair" beyond M, J, Q. But a collapsing star had enormous complexity — baryons, radiation, magnetic fields. Where did all this information go? (Preview of the information paradox.)

---

## Chapter 41: The Reissner-Nordström Solution

*Chapter Introduction: Adding electric charge to the Schwarzschild solution gives the Reissner-Nordström metric — the unique solution for a spherically symmetric, electrically charged mass. It introduces new features: two horizons (outer and inner) and a timelike singularity that makes the causal structure richer.*

### 41.1 Derivation and the Metric
- 41.1.1 Einstein-Maxwell equations: G_{μν} = 8πG T^{EM}_{μν}
- 41.1.2 The Reissner-Nordström metric: ds² = -Δ(r)dt² + Δ(r)⁻¹dr² + r²dΩ², where Δ = 1 - 2GM/(c²r) + GQ²/(4πε₀c⁴r²)
- 41.1.3 Parameters: mass M, charge Q
- 41.1.4 The electromagnetic field: radial electric field

### 41.2 Horizon Structure
- 41.2.1 Outer horizon r₊ and inner (Cauchy) horizon r₋
- 41.2.2 Sub-extremal (M > M_ext), extremal (M = M_ext), and super-extremal (M < M_ext) cases
- 41.2.3 The extremal RN black hole: r₊ = r₋ = GM/c²

### 41.3 Causal Structure
- 41.3.1 Penrose diagram of the Reissner-Nordström spacetime
- 41.3.2 The inner horizon as a Cauchy horizon; instability (mass inflation)
- 41.3.3 Timelike singularity at r = 0
- 41.3.4 Can an observer escape the singularity?

---

**Key Concepts:** Reissner-Nordström metric, outer/inner horizon, extremal black hole, Cauchy horizon, mass inflation, timelike singularity

**Important Figures:** Hans Reissner, Gunnar Nordström, Brandon Carter

**Additional Reading and Primary Sources:**
- Reissner, *On the Eigengravitation of the Electrical Masses according to Einstein's Theory* (1916)
- Carroll, *Spacetime and Geometry*, Ch. 6
- Wald, *General Relativity*, Ch. 12

**Exercises:**
1. Find the two horizons r± = GM/c² ± √((GM/c²)² - GQ²/(4πε₀c⁴)) and classify cases.
2. Show that the extremal RN solution has T = 0 surface gravity (extremal black holes have zero Hawking temperature).

**Thought Experiments:**
- In the super-extremal case (Q too large), there is no horizon — a "naked singularity." Why does the cosmic censorship conjecture forbid this? Is there a physical mechanism?

---

## Chapter 42: The Kerr Solution and Rotating Black Holes

*Chapter Introduction: Most black holes in nature are rotating. The Kerr metric (1963) describes the unique vacuum spacetime around a rotating, uncharged mass — one of the most important exact solutions in GR. Its rich physics (frame dragging, ergosphere, Penrose process) makes it central to astrophysics and high-energy physics.*

### 42.1 Derivation and the Kerr Metric
- 42.1.1 The difficulty: rotating solutions break spherical symmetry → axial symmetry only
- 42.1.2 Boyer-Lindquist coordinates
- 42.1.3 The Kerr metric in Boyer-Lindquist form
- 42.1.4 Parameters: mass M, specific angular momentum a = J/Mc
- 42.1.5 Limiting cases: a → 0 gives Schwarzschild; a = M gives extremal Kerr

### 42.2 Frame Dragging and the Ergosphere
- 42.2.1 Frame dragging: zero angular momentum observers (ZAMOs) are dragged around
- 42.2.2 Angular velocity of dragging: Ω(r,θ)
- 42.2.3 The static limit surface vs. the event horizon
- 42.2.4 The ergosphere: region between static limit and outer horizon
- 42.2.5 Dragging of inertial frames; Lense-Thirring effect (weak-field version)

### 42.3 Horizon Structure and Penrose Diagram
- 42.3.1 Outer and inner horizons of Kerr
- 42.3.2 Ring singularity at r = 0, θ = π/2
- 42.3.3 Penrose diagram; parallels to RN
- 42.3.4 Extremal Kerr: a = GM/c²

### 42.4 The Penrose Process
- 42.4.1 Negative energy orbits in the ergosphere
- 42.4.2 The Penrose process: extracting rotational energy from a black hole
- 42.4.3 Energy extraction limited by a ≤ 0 (irreducible mass)
- 42.4.4 Maximum efficiency: ~20.7%

### 42.5 Geodesics in Kerr Spacetime
- 42.5.1 Four constants of motion: rest mass, energy, angular momentum, Carter constant
- 42.5.2 The Carter constant: hidden symmetry (Killing tensor)
- 42.5.3 Equatorial orbits; ISCO for Kerr
- 42.5.4 Photon orbits; the shadow of a Kerr black hole

### 42.6 Superradiance
- 42.6.1 Superradiant scattering of waves from the ergosphere
- 42.6.2 Analogy with stimulated emission
- 42.6.3 Relationship to the Penrose process
- 42.6.4 Superradiant instabilities; black hole bombs

---

**Key Concepts:** Kerr metric, Boyer-Lindquist coordinates, frame dragging, ergosphere, static limit, Penrose process, Carter constant, superradiance, ISCO in Kerr

**Important Figures:** Roy Kerr, Brandon Carter, Roger Penrose, Robert Boyer, Richard Lindquist

**Additional Reading and Primary Sources:**
- Kerr, *Gravitational Field of a Spinning Mass as an Example of Algebraically Special Metrics* (1963) — primary source
- Carter, *Global Structure of the Kerr Family of Gravitational Fields* (1968)
- Penrose, *Extraction of Rotational Energy from a Black Hole* (1969)
- Misner, Thorne & Wheeler, *Gravitation*, Ch. 33

**Exercises:**
1. Show that the Kerr metric reduces to the Schwarzschild metric as a → 0.
2. Find the ergosphere boundaries (g_{tt} = 0 surface) of the Kerr metric.
3. Compute the maximum efficiency of the Penrose process and show it requires a = GM/c² (extremal Kerr).
4. Show that the ISCO radius for prograde equatorial orbits in Kerr is smaller than 6GM/c² (Schwarzschild value) and approaches GM/c² as a → GM/c².

**Thought Experiments:**
- A black hole has only three "hairs" (M, J, Q). Yet it formed from a collapsing star with arbitrary complexity. Why doesn't GR allow the black hole to retain a memory of the star's shape, composition, or quantum state?
- The Penrose process extracts energy from the rotation of a black hole. What would happen if you kept doing this until a = 0? What is the minimum mass that must remain (the irreducible mass)?

---

## Chapter 43: Other Exact Solutions

*Chapter Introduction: Beyond the Schwarzschild and Kerr families, a number of other exact solutions illuminate different aspects of general relativity — plane gravitational waves, the Gödel universe, de Sitter spacetime, and others.*

### 43.1 The Kerr-Newman Solution
- 43.1.1 The unique charged, rotating black hole
- 43.1.2 Parameters: M, J, Q
- 43.1.3 The no-hair theorem: Kerr-Newman is the most general stationary black hole

### 43.2 De Sitter and Anti-de Sitter Spacetimes
- 43.2.1 De Sitter: maximally symmetric solution with Λ > 0; exponential expansion
- 43.2.2 Anti-de Sitter: maximally symmetric solution with Λ < 0; relevance to AdS/CFT
- 43.2.3 Penrose diagrams; causal structure

### 43.3 Plane Gravitational Waves
- 43.3.1 Exact pp-wave spacetimes
- 43.3.2 The Brinkmann form of the metric
- 43.3.3 Geodesics in a pp-wave background; the sandwich wave

### 43.4 The Gödel Universe
- 43.4.1 Gödel's rotating dust universe (1949)
- 43.4.2 Closed timelike curves and the violation of global causality
- 43.4.3 Physical plausibility and Hawking's chronology protection conjecture

### 43.5 Wormholes
- 43.5.1 Morris-Thorne traversable wormholes
- 43.5.2 Exotic matter requirement
- 43.5.3 The Einstein-Rosen bridge (non-traversable)
- 43.5.4 ER = EPR conjecture (overview)

---

**Key Concepts:** Kerr-Newman solution, de Sitter space, anti-de Sitter space, pp-wave, Gödel universe, closed timelike curves, traversable wormhole, exotic matter

**Important Figures:** Roy Kerr, Ezra Newman, Willem de Sitter, Kurt Gödel, Morris & Thorne, Juan Maldacena, Leonard Susskind

**Additional Reading and Primary Sources:**
- Gödel, *An Example of a New Type of Cosmological Solutions of Einstein's Field Equations* (1949)
- Morris & Thorne, *Wormholes in Spacetime and Their Use for Interstellar Travel* (1988)
- Misner, Thorne & Wheeler, *Gravitation*, Ch. 35

**Exercises:**
1. Show that de Sitter spacetime satisfies G_{μν} + Λg_{μν} = 0 with R_{μν} = Λg_{μν}.
2. Identify closed timelike curves in the Gödel metric.
3. Explain why the Morris-Thorne wormhole requires exotic matter that violates the NEC.

**Thought Experiments:**
- Gödel showed that GR admits solutions with closed timelike curves (time travel). Does this mean GR predicts time travel is possible, or does it mean the Gödel solution is physically unrealized?
- The ER = EPR conjecture (Maldacena-Susskind) suggests that quantum entanglement is dual to wormholes. What would this imply about the information paradox?


---

# UNIT XI: Gravitational Waves

*Unit Introduction: Gravitational waves are ripples in the curvature of spacetime, propagating at the speed of light and produced by accelerating masses. Predicted by Einstein in 1916 and detected directly for the first time by LIGO in 2015, gravitational waves have opened an entirely new window onto the Universe. This unit develops the theory from linearized GR through sources and detection.*

---

## Chapter 44: Linearized General Relativity

*Chapter Introduction: When spacetime is only slightly curved, the metric can be written as the Minkowski metric plus a small perturbation. The linearized Einstein equations govern this perturbation and are the starting point for the theory of gravitational waves, the post-Newtonian approximation, and gravitational wave detection.*

### 44.1 The Linearized Metric Perturbation
- 44.1.1 g_{μν} = η_{μν} + h_{μν}, where |h_{μν}| ≪ 1
- 44.1.2 Linearized curvature tensors: Γ^α_{μν}, R_{μνρσ}, R_{μν}, G_{μν} to first order in h
- 44.1.3 Gauge invariance: linearized diffeomorphisms x^μ → x^μ + ξ^μ

### 44.2 The Lorenz Gauge and Wave Equation
- 44.2.1 The trace-reversed perturbation: h̄_{μν} = h_{μν} - ½η_{μν}h
- 44.2.2 Lorenz (harmonic) gauge: ∂^μ h̄_{μν} = 0
- 44.2.3 The linearized Einstein equations become: □h̄_{μν} = -16πG/c⁴ T_{μν}

### 44.3 Vacuum Gravitational Waves: The Transverse-Traceless Gauge
- 44.3.1 In vacuum: □h̄_{μν} = 0 → wave solutions
- 44.3.2 Plane wave solutions: h_{μν} = A_{μν} e^{ik_α x^α}, with k^μ k_μ = 0
- 44.3.3 Further gauge freedom: transverse-traceless (TT) gauge
- 44.3.4 TT gauge: h^{TT}_{μν} has only two independent polarizations
- 44.3.5 The two polarizations: h_+ and h_×

### 44.4 Physical Effect of Gravitational Waves
- 44.4.1 Geodesic deviation in a GW background
- 44.4.2 Effect on a ring of test particles: stretching and squeezing
- 44.4.3 h_+ polarization: stretches x, squeezes y
- 44.4.4 h_× polarization: same but rotated 45°
- 44.4.5 Characteristic strain and amplitude

### 44.5 Energy Carried by Gravitational Waves
- 44.5.1 The Isaacson stress-energy tensor for gravitational waves
- 44.5.2 Averaging over many wavelengths: ⟨T^{μν}_{GW}⟩
- 44.5.3 Energy flux and luminosity

---

**Key Concepts:** metric perturbation, linearized Einstein equations, Lorenz gauge, TT gauge, gravitational wave polarizations h_+ and h_×, geodesic deviation, Isaacson stress-energy tensor

**Important Figures:** Albert Einstein, Max Born, Arthur Eddington, Richard Isaacson, Rainer Weiss, Kip Thorne, Barry Barish

**Additional Reading and Primary Sources:**
- Einstein, *Approximative Integration of the Field Equations of Gravitation* (1916) — primary source
- Einstein, *On Gravitational Waves* (1918) — primary source
- Carroll, *Spacetime and Geometry*, Ch. 7
- Maggiore, *Gravitational Waves: Theory and Experiments*, Vol. 1

**Exercises:**
1. Derive the linearized Riemann tensor R_{μνρσ}^{(1)} to first order in h_{μν}.
2. Show that the two polarization states h_+ and h_× are the only physical degrees of freedom in TT gauge.
3. Calculate the strain h = ΔL/L produced by a 1 kHz gravitational wave with amplitude A at distance 100 Mpc.
4. Derive the Isaacson stress-energy tensor by expanding the Einstein equations to second order.

**Thought Experiments:**
- Einstein initially doubted his own prediction of gravitational waves, at one point thinking they were a gauge artifact. What is the physical argument that they carry real energy?
- A gravitational wave passing through a ring of particles changes the distances between them but not the coordinates. How would you actually detect this with a physical detector?

---

## Chapter 45: Sources of Gravitational Waves

*Chapter Introduction: The quadrupole formula gives the power emitted in gravitational waves by a compact source whose internal dynamics are non-relativistic. This formula governs binary star systems, compact object mergers, and other astrophysical sources.*

### 45.1 The Quadrupole Formula
- 45.1.1 Retarded solution to □h̄_{μν} = -16πG/c⁴ T_{μν}
- 45.1.2 Far-field approximation: multipole expansion
- 45.1.3 No monopole or dipole gravitational radiation (mass/momentum conservation)
- 45.1.4 The quadrupole moment tensor: I_{ij} = ∫ρ x_i x_j d³x
- 45.1.5 The quadrupole formula: h_{ij}^{TT} = (2G/c⁴r) Ï_{ij}^{TT} (second time derivative)

### 45.2 Gravitational Wave Luminosity
- 45.2.1 The Einstein quadrupole luminosity formula: L_GW = G/(5c⁵) ⟨⟨Ï_{ij} Ï^{ij}⟩⟩
- 45.2.2 Application to a binary system
- 45.2.3 Orbital decay; inspiral timescale
- 45.2.4 The Hulse-Taylor binary pulsar: first indirect detection of GW

### 45.3 Astrophysical Sources
- 45.3.1 Compact binary inspirals: neutron star–neutron star, black hole–black hole, NS–BH
- 45.3.2 The chirp mass M_c = (m₁m₂)^{3/5}/(m₁+m₂)^{1/5}
- 45.3.3 Post-Newtonian waveforms; the frequency evolution
- 45.3.4 Core-collapse supernovae
- 45.3.5 Continuous wave sources: rotating neutron stars (pulsars)
- 45.3.6 Stochastic gravitational wave background

### 45.4 Numerical Relativity and Binary Merger Waveforms
- 45.4.1 The three phases: inspiral, merger, ringdown
- 45.4.2 Effective one-body (EOB) formalism
- 45.4.3 Quasi-normal modes during ringdown

---

**Key Concepts:** quadrupole formula, gravitational wave luminosity, chirp mass, binary inspiral, Hulse-Taylor pulsar, quasi-normal modes, ringdown

**Important Figures:** Albert Einstein, Russell Hulse, Joseph Taylor, Kip Thorne, Frans Pretorius

**Additional Reading and Primary Sources:**
- Hulse & Taylor, *Discovery of a Pulsar in a Binary System* (1975) — Nobel-prize paper
- Peters, *Gravitational Radiation and the Motion of Two Point Masses* (1964)
- Maggiore, *Gravitational Waves*, Vol. 1, Chs. 3–4

**Exercises:**
1. Compute the gravitational wave luminosity of the Earth-Sun system. How long until the orbit decays?
2. For a double neutron star binary with m₁ = m₂ = 1.4 M☉ and initial separation 1 AU, estimate the inspiral time using Peters' formula.
3. Show that gravitational wave emission carries energy and angular momentum, causing the orbit to shrink and circularize.

**Thought Experiments:**
- The Hulse-Taylor pulsar indirectly detected gravitational waves through orbital decay. Why was this compelling evidence even before direct detection?
- A binary black hole merger emits ~3 solar masses of energy in gravitational waves in ~0.2 seconds. How does this compare to the luminosity of the observable Universe?

---

## Chapter 46: Detection of Gravitational Waves

*Chapter Introduction: Direct detection of gravitational waves required measuring changes in length of 4 km to a precision of 1/10,000 the diameter of a proton. The achievement of LIGO in 2015 opened the era of gravitational wave astronomy.*

### 46.1 Principles of Interferometric Detection
- 46.1.1 Michelson interferometer principle
- 46.1.2 How a GW changes the arm lengths: ΔL/L = h/2
- 46.1.3 Fabry-Pérot cavities and power recycling
- 46.1.4 Signal recycling; quantum noise limit

### 46.2 Noise Sources
- 46.2.1 Seismic noise and vibration isolation
- 46.2.2 Thermal noise (mirror and suspension)
- 46.2.3 Quantum noise: shot noise and radiation pressure noise
- 46.2.4 Standard quantum limit; squeezed light

### 46.3 LIGO, Virgo, KAGRA, and LISA
- 46.3.1 LIGO: Livingston and Hanford sites
- 46.3.2 GW150914: first direct detection (Sept. 14, 2015)
- 46.3.3 Virgo and KAGRA: global detector network; sky localization
- 46.3.4 LISA: space-based interferometer; millihertz band

### 46.4 Gravitational Wave Astronomy
- 46.4.1 The gravitational wave spectrum: audio band, nHz band, mHz band
- 46.4.2 Pulsar timing arrays: nanohertz GW background (NANOGrav 2023)
- 46.4.3 Multi-messenger astronomy: GW170817 (binary neutron star + GRB + optical kilonova)
- 46.4.4 Tests of GR with gravitational waves
- 46.4.5 Black hole spectroscopy: testing the no-hair theorem with ringdown

---

**Key Concepts:** laser interferometer, strain sensitivity, Fabry-Pérot cavity, shot noise, standard quantum limit, squeezed light, LIGO, GW150914, multi-messenger astronomy, pulsar timing array

**Important Figures:** Rainer Weiss, Kip Thorne, Barry Barish (Nobel 2017), Joseph Weber, Ronald Drever, Nergis Mavalvala

**Additional Reading and Primary Sources:**
- Abbott et al. (LIGO), *Observation of Gravitational Waves from a Binary Black Hole Merger* (2016) — primary source
- Weiss, Nobel Lecture (2017)
- Maggiore, *Gravitational Waves*, Vol. 1, Chs. 9–10

**Exercises:**
1. Estimate the strain from GW150914 (two ~30 M☉ BHs merging at ~400 Mpc). Compare to the measured h ≈ 10⁻²¹.
2. Explain why two detectors are needed to distinguish a GW from local noise, and why three are needed for sky localization.
3. Calculate the frequency of gravitational waves at the ISCO of a binary black hole merger with total mass 60 M☉.

**Thought Experiments:**
- A gravitational wave from GW150914 changed the length of the 4 km LIGO arm by ~10⁻¹⁸ m. This is smaller than the classical radius of a proton (~10⁻¹⁵ m). How is such a measurement even conceivable?
- GW170817 allowed the speed of gravitational waves to be constrained to within 10⁻¹⁵ of the speed of light. What alternative theories of gravity does this rule out?


---

# UNIT XII: Relativistic Cosmology

*Unit Introduction: Cosmology is the study of the Universe as a whole — its large-scale structure, evolution, and fate. General relativity, applied to a homogeneous and isotropic universe, yields the Friedmann equations that govern the cosmic expansion. This unit develops the standard cosmological model from the FLRW metric through inflation.*

---

## Chapter 47: The FLRW Metric and Friedmann Equations

*Chapter Introduction: If the Universe is homogeneous and isotropic on large scales (the cosmological principle), its metric is uniquely determined to be the Friedmann-Lemaître-Robertson-Walker (FLRW) form. The Einstein equations applied to this metric yield the Friedmann equations — the equations of motion for the scale factor a(t) that describes cosmic expansion.*

### 47.1 The Cosmological Principle
- 47.1.1 Homogeneity and isotropy of the Universe on large scales
- 47.1.2 Observational evidence: CMB, galaxy surveys
- 47.1.3 The Copernican principle

### 47.2 The FLRW Metric
- 47.2.1 Derivation: maximally symmetric 3-spaces; comoving coordinates
- 47.2.2 ds² = -c²dt² + a(t)²[dr²/(1-kr²) + r²dΩ²], k ∈ {-1, 0, +1}
- 47.2.3 The scale factor a(t) and its physical meaning
- 47.2.4 Curvature parameter k: closed (k=+1), flat (k=0), open (k=-1)
- 47.2.5 Comoving vs. physical coordinates

### 47.3 The Friedmann Equations
- 47.3.1 Computing G_{μν} for the FLRW metric
- 47.3.2 T_{μν} for a perfect fluid: ρ(t) and p(t)
- 47.3.3 The first Friedmann equation: (ȧ/a)² = 8πGρ/3 - kc²/a² + Λc²/3
- 47.3.4 The second Friedmann equation (Raychaudhuri): ä/a = -4πG(ρ + 3p/c²)/3 + Λc²/3
- 47.3.5 The fluid equation: ρ̇ + 3(ȧ/a)(ρ + p/c²) = 0
- 47.3.6 The Hubble parameter H(t) = ȧ/a; H₀ ≈ 67-73 km/s/Mpc

### 47.4 Cosmological Redshift
- 47.4.1 Photon propagation in FLRW: null geodesics
- 47.4.2 Cosmological redshift: 1 + z = a(t₀)/a(t_emit)
- 47.4.3 Distinction from Doppler redshift
- 47.4.4 Luminosity distance and angular diameter distance

### 47.5 Equations of State
- 47.5.1 Matter (dust): p = 0, ρ ∝ a⁻³
- 47.5.2 Radiation: p = ρ/3, ρ ∝ a⁻⁴
- 47.5.3 Cosmological constant / dark energy: p = -ρc², ρ = const

---

**Key Concepts:** cosmological principle, FLRW metric, scale factor, comoving coordinates, Hubble parameter, Friedmann equations, Raychaudhuri equation, cosmological redshift, equation of state

**Important Figures:** Alexander Friedmann, Georges Lemaître, Howard Robertson, Arthur Walker, Edwin Hubble, Georges Lemaître

**Additional Reading and Primary Sources:**
- Friedmann, *On the Curvature of Space* (1922) — primary source
- Lemaître, *A Homogeneous Universe of Constant Mass and Increasing Radius* (1927) — primary source
- Carroll, *Spacetime and Geometry*, Ch. 8
- Weinberg, *Cosmology*

**Exercises:**
1. Derive the FLRW metric by requiring maximal symmetry of constant-time hypersurfaces.
2. Derive the two Friedmann equations from the Einstein equations with a perfect fluid source.
3. Show that for matter domination (p=0, k=0): a(t) ∝ t^{2/3}.
4. Calculate the age of a flat, matter-only universe with H₀ = 70 km/s/Mpc.

**Thought Experiments:**
- The cosmological redshift of distant galaxies is often described as a Doppler shift, but it is really a stretching of the wavelength by the expanding universe. What is the difference, and does it matter?
- If k = 0 (flat universe), the universe is spatially infinite. Yet it has a finite age. How can we observe only a finite region (the observable universe)?

---

## Chapter 48: Cosmological Models and Observations

*Chapter Introduction: Different combinations of matter, radiation, and dark energy produce dramatically different cosmological histories. This chapter surveys the major cosmological eras, the evidence for dark matter and dark energy, and the observational probes that constrain the cosmological parameters.*

### 48.1 The Standard Model of Cosmology (ΛCDM)
- 48.1.1 Components: Λ (dark energy, ~68%), cold dark matter (~27%), baryons (~5%), radiation (trace)
- 48.1.2 The density parameter Ω = ρ/ρ_crit; critical density ρ_crit = 3H²/8πG
- 48.1.3 Ω_total = Ω_m + Ω_Λ + Ω_k = 1 (flat universe)
- 48.1.4 Cosmological epochs: radiation domination, matter domination, Λ domination

### 48.2 The Cosmic Microwave Background
- 48.2.1 Recombination at z ≈ 1100; photon decoupling
- 48.2.2 The blackbody spectrum: T₀ = 2.725 K
- 48.2.3 CMB temperature anisotropies: δT/T ~ 10⁻⁵
- 48.2.4 Power spectrum: acoustic peaks
- 48.2.5 CMB polarization: E-modes and B-modes
- 48.2.6 COBE, WMAP, Planck results

### 48.3 Evidence for Dark Matter
- 48.3.1 Rotation curves of galaxies (Rubin et al.)
- 48.3.2 Gravitational lensing (bullet cluster)
- 48.3.3 Large-scale structure and CMB acoustic peaks
- 48.3.4 Candidates: WIMPs, axions, sterile neutrinos
- 48.3.5 Direct and indirect detection experiments

### 48.4 Evidence for Dark Energy and Accelerated Expansion
- 48.4.1 Type Ia supernovae as standard candles (Perlmutter, Schmidt, Riess — Nobel 2011)
- 48.4.2 Hubble diagram; deceleration parameter q₀
- 48.4.3 The cosmological constant interpretation; vacuum energy
- 48.4.4 The cosmological constant problem: why is Λ so small?
- 48.4.5 Alternative models: quintessence, modified gravity

### 48.5 Distances and Cosmological Observables
- 48.5.1 Comoving distance, physical distance, luminosity distance, angular diameter distance
- 48.5.2 Baryon acoustic oscillations as a standard ruler
- 48.5.3 The Hubble tension: H₀ discrepancy between CMB and local measurements

---

**Key Concepts:** ΛCDM, density parameter, critical density, CMB, recombination, dark matter, dark energy, Type Ia supernovae, cosmological constant problem, Hubble tension

**Important Figures:** Edwin Hubble, Vera Rubin, Saul Perlmutter, Brian Schmidt, Adam Riess, George Smoot, John Mather, Jim Peebles

**Additional Reading and Primary Sources:**
- Perlmutter et al., *Measurements of Ω and Λ from 42 High-Redshift Supernovae* (1999)
- Riess et al., *Observational Evidence from Supernovae for an Accelerating Universe* (1998)
- Planck Collaboration, *Planck 2018 Results* (2020)
- Weinberg, *Cosmology*; Ryden, *Introduction to Cosmology*

**Exercises:**
1. For a flat ΛCDM universe with Ω_m = 0.3, Ω_Λ = 0.7, H₀ = 70 km/s/Mpc, compute the age of the universe numerically.
2. Estimate the comoving distance to the CMB last scattering surface (z ≈ 1100).
3. Show that q₀ = Ω_m/2 - Ω_Λ < 0 for the current universe, implying acceleration.

**Thought Experiments:**
- The cosmological constant problem: quantum field theory predicts a vacuum energy density ~10¹²⁰ times larger than the observed dark energy density. Why is this the "worst prediction in physics"?
- If dark energy is a cosmological constant, the universe will expand forever and reach a "heat death." What does this imply for the long-term fate of black holes, galaxies, and life?

---

## Chapter 49: The Hot Big Bang and Thermal History

*Chapter Introduction: Running the Friedmann equations backward in time, the Universe was once in an extremely hot, dense state — the Big Bang. The thermal history — nucleosynthesis, recombination, neutrino decoupling — is one of the great successes of modern cosmology.*

### 49.1 The Early Universe
- 49.1.1 Entropy conservation in an expanding universe
- 49.1.2 Temperature as a function of scale factor: T ∝ 1/a
- 49.1.3 Thermal equilibrium and freeze-out
- 49.1.4 The particle content of the early universe: quarks, leptons, bosons

### 49.2 Nucleosynthesis (BBN)
- 49.2.1 Neutron-proton ratio at freeze-out (T ≈ 1 MeV)
- 49.2.2 Formation of light nuclei: D, ³He, ⁴He, ⁷Li
- 49.2.3 Primordial abundances as a probe of Ω_b h²
- 49.2.4 Agreement with observations

### 49.3 Recombination and Decoupling
- 49.3.1 Hydrogen recombination at T ≈ 0.3 eV (z ≈ 1100)
- 49.3.2 Photon decoupling; mean free path diverges
- 49.3.3 Formation of the CMB
- 49.3.4 Neutrino decoupling (T ≈ 2 MeV)

### 49.4 Structure Formation
- 49.4.1 Jeans instability and gravitational collapse
- 49.4.2 Growth of perturbations in different eras
- 49.4.3 The matter power spectrum
- 49.4.4 Nonlinear collapse: Press-Schechter formalism; halo formation

---

**Key Concepts:** thermal equilibrium, freeze-out, Big Bang nucleosynthesis, primordial abundances, recombination, photon decoupling, Jeans instability, structure formation, matter power spectrum

**Important Figures:** George Gamow, Ralph Alpher, Robert Herman, Robert Wagoner, Steven Weinberg, Phillip Peebles

**Additional Reading and Primary Sources:**
- Alpher, Bethe & Gamow, *The Origin of Chemical Elements* (1948)
- Weinberg, *The First Three Minutes* (1977)
- Kolb & Turner, *The Early Universe*

**Exercises:**
1. Estimate the freeze-out temperature for neutrons using the condition Γ ~ H (weak interaction rate equals Hubble rate).
2. Calculate the primordial ⁴He mass fraction Y_p ≈ 0.25 from the neutron-proton ratio at freeze-out.
3. Show that T ∝ a⁻¹ in a radiation-dominated universe.

**Thought Experiments:**
- The Big Bang is not an explosion in space but an expansion of space itself. What is the difference? Was there a "center" of the Big Bang?
- BBN occurred ~1-3 minutes after the Big Bang. Why was the window for helium synthesis so brief?

---

## Chapter 50: Inflationary Cosmology

*Chapter Introduction: Inflation — a period of exponential expansion in the very early universe — resolves the flatness and horizon problems of the standard Big Bang, and provides the mechanism for generating the primordial density perturbations observed in the CMB.*

### 50.1 Problems with the Standard Big Bang
- 50.1.1 The horizon problem: why is the CMB so uniform?
- 50.1.2 The flatness problem: why is Ω so close to 1?
- 50.1.3 The monopole problem: too many topological defects

### 50.2 The Inflationary Paradigm
- 50.2.1 A period of quasi-exponential expansion: a ∝ e^{Ht}
- 50.2.2 Driven by a scalar field (inflaton) with a flat potential: slow-roll conditions
- 50.2.3 Resolution of the horizon problem
- 50.2.4 Resolution of the flatness problem

### 50.3 Slow-Roll Inflation
- 50.3.1 Scalar field equations in FLRW: φ̈ + 3Hφ̇ + V'(φ) = 0
- 50.3.2 Slow-roll parameters ε = -Ḣ/H², η
- 50.3.3 Inflation ends when ε ≈ 1; reheating

### 50.4 Quantum Fluctuations and the Primordial Spectrum
- 50.4.1 Quantum fluctuations of the inflaton field
- 50.4.2 Stretching of quantum modes to super-Hubble scales
- 50.4.3 Nearly scale-invariant power spectrum: n_s ≈ 1
- 50.4.4 Primordial gravitational waves: tensor-to-scalar ratio r
- 50.4.5 Observational constraints from Planck: n_s ≈ 0.965, r < 0.036

---

**Key Concepts:** horizon problem, flatness problem, inflaton, slow-roll inflation, de Sitter expansion, primordial perturbations, scale-invariant spectrum, tensor-to-scalar ratio

**Important Figures:** Alan Guth, Andrei Linde, Paul Steinhardt, Andreas Albrecht, Alexei Starobinsky, Stephen Hawking, James Bardeen

**Additional Reading and Primary Sources:**
- Guth, *Inflationary Universe: A Possible Solution to the Horizon and Flatness Problems* (1981) — primary source
- Linde, *A New Inflationary Universe Scenario* (1982)
- Planck 2018 inflation paper
- Baumann, *Cosmology* (DAMTP lecture notes, open access)

**Exercises:**
1. Show that exponential expansion a ∝ e^{Ht} solves the horizon problem by computing the comoving Hubble radius 1/(aH).
2. For chaotic inflation V(φ) = ½m²φ², compute the slow-roll parameters ε and η and find when inflation ends.
3. Estimate the number of e-folds of inflation required to solve the flatness problem.

**Thought Experiments:**
- Eternal inflation: in many models, inflation never fully ends globally — quantum fluctuations continuously nucleate new inflating regions. Does this imply the existence of a multiverse?
- The primordial gravitational wave background from inflation has not yet been detected. What would its detection tell us about the energy scale of inflation?


---

# UNIT XIII: Advanced Formulations of General Relativity

*Unit Introduction: General relativity can be formulated in several mathematically distinct but physically equivalent ways. Each formulation illuminates different aspects of the theory — its initial value structure, its symmetries, its connections to other theories. This unit develops the Hamiltonian (ADM) formulation, the action principle, the Newman-Penrose formalism, spinors, and Penrose diagrams for global causal analysis.*

---

## Chapter 51: The Initial Value Formulation (ADM)

*Chapter Introduction: In the ADM (Arnowitt-Deser-Misner) formulation, spacetime is foliated into spacelike hypersurfaces, and the Einstein equations are split into constraint equations (on each slice) and evolution equations (propagating between slices). This "3+1" decomposition is the foundation of numerical relativity.*

### 51.1 The 3+1 Decomposition
- 51.1.1 Foliating spacetime by spacelike hypersurfaces Σ_t
- 51.1.2 The lapse function N and shift vector N^i
- 51.1.3 The ADM metric: ds² = -N²dt² + γ_{ij}(dx^i + N^i dt)(dx^j + N^j dt)
- 51.1.4 The intrinsic metric γ_{ij} of each slice
- 51.1.5 The extrinsic curvature K_{ij}: how each slice curves in spacetime

### 51.2 The Gauss-Codazzi Equations
- 51.2.1 Relations between 4D curvature and intrinsic/extrinsic curvature of the slice
- 51.2.2 The Gauss equation: ^{(4)}R_{abcd} in terms of ^{(3)}R_{abcd} and K_{ab}
- 51.2.3 The Codazzi-Mainardi equation

### 51.3 The ADM Constraints and Evolution Equations
- 51.3.1 Hamiltonian constraint: ^{(3)}R + K² - K_{ij}K^{ij} = 16πGρ
- 51.3.2 Momentum constraints: D_j(K^{ij} - γ^{ij}K) = 8πGJ^i
- 51.3.3 Evolution equations for γ_{ij} and K_{ij}

### 51.4 The ADM Hamiltonian
- 51.4.1 The ADM action
- 51.4.2 Canonical momenta: π^{ij} conjugate to γ_{ij}
- 51.4.3 The ADM Hamiltonian is a sum of constraints
- 51.4.4 ADM mass and momentum for asymptotically flat spacetimes

### 51.5 The Initial Value Problem
- 51.5.1 Freely specifiable initial data: (γ_{ij}, K_{ij}) subject to constraints
- 51.5.2 Conformal decomposition; York-Lichnerowicz method
- 51.5.3 Well-posedness of the Cauchy problem for GR

---

**Key Concepts:** 3+1 decomposition, lapse and shift, intrinsic curvature, extrinsic curvature, Gauss-Codazzi equations, Hamiltonian constraint, momentum constraint, ADM mass, initial value problem

**Important Figures:** Richard Arnowitt, Stanley Deser, Charles Misner, James York, Yvonne Choquet-Bruhat

**Additional Reading and Primary Sources:**
- Arnowitt, Deser & Misner, *The Dynamics of General Relativity* (1962) — primary source
- Choquet-Bruhat & Geroch, *Global Aspects of the Cauchy Problem in General Relativity* (1969)
- Gourgoulhon, *3+1 Formalism in General Relativity* (open access)
- Baumgarte & Shapiro, *Numerical Relativity*

**Exercises:**
1. Derive the Gauss equation relating the 4D and 3D Riemann tensors in terms of the extrinsic curvature.
2. Show that the ADM Hamiltonian for GR reduces to a pure boundary term on-shell (the constraints are satisfied).
3. Verify that the ADM mass of the Schwarzschild spacetime equals M.

**Thought Experiments:**
- The Hamiltonian of GR is a sum of constraints — it vanishes on-shell. What does this mean for the notion of "time" in quantum gravity?
- The initial value problem requires data on a Cauchy surface. Does every spacetime admit a Cauchy surface? What topologies prevent this?

---

## Chapter 52: The Action Principle and Variational Methods

*Chapter Introduction: The Einstein-Hilbert action is the unique generally covariant action (up to Lovelock's theorem) that gives second-order field equations linear in the second derivatives of the metric. This chapter explores the action principle in depth, including boundary terms, the Palatini formulation, and Lovelock gravity.*

### 52.1 The Einstein-Hilbert Action Revisited
- 52.1.1 S_{EH} = (1/16πG) ∫(R - 2Λ)√(-g) d⁴x
- 52.1.2 Variation of g^{μν}, √(-g), and R
- 52.1.3 The Gibbons-Hawking-York boundary term
- 52.1.4 Why the boundary term matters: well-posedness of the variational problem

### 52.2 The Palatini Formulation
- 52.2.1 Treating g_{μν} and Γ^α_{μν} as independent variables
- 52.2.2 Varying S with respect to Γ: recovers metric compatibility (Levi-Civita connection)
- 52.2.3 Varying S with respect to g: recovers Einstein's equations
- 52.2.4 First-order formalism: the vierbein (tetrad) formulation

### 52.3 Lovelock's Theorem
- 52.3.1 Statement: in 4D, the only divergence-free symmetric tensor built from g and its derivatives, at most quadratic in second derivatives, is G_{μν} + Λg_{μν}
- 52.3.2 Implication: GR (with Λ) is essentially unique in 4D
- 52.3.3 Lovelock gravity in higher dimensions: Gauss-Bonnet terms

### 52.4 The Tetrad (Vierbein) Formulation
- 52.4.1 Local Lorentz frames: tetrads e^a_μ
- 52.4.2 Metric from the tetrad: g_{μν} = η_{ab} e^a_μ e^b_ν
- 52.4.3 The spin connection ω^{ab}_μ
- 52.4.4 Curvature 2-form from the spin connection
- 52.4.5 Action in tetrad form: the Palatini action

---

**Key Concepts:** Einstein-Hilbert action, Gibbons-Hawking-York term, Palatini formalism, Lovelock's theorem, tetrad/vierbein, spin connection, local Lorentz invariance

**Important Figures:** David Hilbert, Albert Einstein, Tullio Levi-Civita, Élie Cartan, David Lovelock, Gary Gibbons, Stephen Hawking, James York

**Additional Reading and Primary Sources:**
- Hilbert, *The Foundations of Physics* (1915)
- Lovelock, *The Einstein Tensor and Its Generalizations* (1971)
- Wald, *General Relativity*, Appendix E
- Misner, Thorne & Wheeler, *Gravitation*, Ch. 21

**Exercises:**
1. Derive the Gibbons-Hawking-York boundary term and explain why it is necessary.
2. Perform the Palatini variation of S_{EH} with respect to Γ^α_{μν} and show it implies ∇_α g_{μν} = 0.
3. State Lovelock's theorem precisely and outline its proof.

**Thought Experiments:**
- The uniqueness of GR in 4D (Lovelock's theorem) is remarkable. What changes in higher dimensions, and why should we care? (Hint: string theory lives in 10D.)

---

## Chapter 53: Penrose Diagrams and Global Causal Structure

*Chapter Introduction: Penrose diagrams (conformal diagrams) map infinite spacetimes to finite regions, making their global causal structure visible. They are an essential tool for understanding black holes, cosmology, and the precise formulation of singularity theorems.*

### 53.1 Conformal Compactification
- 53.1.1 Conformal transformations of the metric: g̃_{μν} = Ω² g_{μν}
- 53.1.2 Conformal invariance of null geodesics
- 53.1.3 Penrose's conformal compactification technique

### 53.2 Penrose Diagram of Minkowski Spacetime
- 53.2.1 Null coordinates (u,v); compactification to finite ranges
- 53.2.2 Conformal boundary: i⁰ (spatial infinity), i⁺ (future timelike infinity), i⁻ (past timelike infinity), ℐ⁺ (future null infinity), ℐ⁻ (past null infinity)
- 53.2.3 Causal structure visible at a glance

### 53.3 Penrose Diagrams of Key Spacetimes
- 53.3.1 Schwarzschild black hole (maximally extended)
- 53.3.2 Reissner-Nordström black hole
- 53.3.3 Kerr black hole
- 53.3.4 De Sitter spacetime; cosmological horizons
- 53.3.5 Anti-de Sitter spacetime; the AdS boundary
- 53.3.6 FLRW cosmological spacetimes

### 53.4 Definitions from Conformal Diagrams
- 53.4.1 Asymptotic flatness from the conformal boundary structure
- 53.4.2 Black hole region: J⁻(ℐ⁺)^c
- 53.4.3 Event horizon as ∂J⁻(ℐ⁺)
- 53.4.4 Cauchy surfaces and global hyperbolicity

---

**Key Concepts:** conformal transformation, conformal compactification, Penrose diagram, conformal boundary, future/past null infinity, event horizon as boundary, global hyperbolicity, Cauchy surface

**Important Figures:** Roger Penrose, Brandon Carter, Robert Geroch

**Additional Reading and Primary Sources:**
- Penrose, *Zero Rest-Mass Fields Including Gravitation: Asymptotic Behaviour* (1965)
- Hawking & Ellis, *The Large Scale Structure of Space-Time*, Ch. 5–6
- Carroll, *Spacetime and Geometry*, Appendix H

**Exercises:**
1. Construct the Penrose diagram of Minkowski spacetime step by step.
2. Draw the Penrose diagram for the Schwarzschild black hole and identify the four regions.
3. Draw the Penrose diagram for de Sitter spacetime and identify the cosmological horizon.

**Thought Experiments:**
- In the Penrose diagram of Schwarzschild, two observers in the left and right exterior regions cannot communicate. What is the physical meaning of the two exterior regions?
- Future null infinity ℐ⁺ is "where photons go." The information paradox asks: do photons from matter that fell into a black hole ever reach ℐ⁺? What does the Penrose diagram suggest?

---

## Chapter 54: Spinors in General Relativity and the Newman-Penrose Formalism

*Chapter Introduction: Spinors are the "square roots" of vectors — objects with half-integer transformation properties under rotations. They are needed to describe fermions (electrons, quarks) in curved spacetime. The Newman-Penrose formalism uses spinors and a null tetrad to cast the Einstein equations into a form well-suited for studying gravitational radiation and the algebraic classification of spacetimes.*

### 54.1 Two-Component Spinors
- 54.1.1 SL(2,ℂ) as the covering group of the Lorentz group
- 54.1.2 Weyl spinors: ξ^A (undotted) and η^{Ȧ} (dotted)
- 54.1.3 Spinor indices: raising and lowering with ε_{AB}
- 54.1.4 The van der Waerden symbols: converting between spinor and tensor indices

### 54.2 Spinors in Curved Spacetime
- 54.2.1 Spin structures on a manifold
- 54.2.2 The spinor covariant derivative
- 54.2.3 The Dirac equation in curved spacetime

### 54.3 The Newman-Penrose Formalism
- 54.3.1 The null tetrad {l^μ, n^μ, m^μ, m̄^μ}: null frame
- 54.3.2 The spin coefficients (12 complex scalars): κ, σ, ρ, τ, ε, γ, α, β, π, ν, μ, λ
- 54.3.3 The Weyl scalars Ψ₀, Ψ₁, Ψ₂, Ψ₃, Ψ₄ and their physical interpretation
- 54.3.4 The NP field equations (Bianchi, Ricci, and commutation relations)

### 54.4 Algebraic Classification of Spacetimes (Petrov Classification)
- 54.4.1 Principal null directions; the Weyl tensor as a symmetric spinor
- 54.4.2 Petrov types: I, II, D, III, N, O
- 54.4.3 Physical examples: Schwarzschild (type D), plane waves (type N), Kerr (type D)
- 54.4.4 The Goldberg-Sachs theorem: algebraically special spacetimes and shear-free geodesic null congruences

---

**Key Concepts:** spinor, SL(2,ℂ), Weyl spinor, spin connection in curved space, Newman-Penrose formalism, null tetrad, spin coefficients, Weyl scalars, Petrov classification, Goldberg-Sachs theorem

**Important Figures:** Élie Cartan, Ezra Newman, Roger Penrose, Ivor Robinson, Andrzej Trautman, Alexei Petrov

**Additional Reading and Primary Sources:**
- Newman & Penrose, *An Approach to Gravitational Radiation by a Method of Spin Coefficients* (1962) — primary source
- Penrose & Rindler, *Spinors and Space-Time*, Vols. 1–2
- Chandrasekhar, *The Mathematical Theory of Black Holes*

**Exercises:**
1. Verify that the van der Waerden symbols σ^{μ}_{AȦ} convert correctly between vector and spinor indices.
2. For the Schwarzschild metric, construct the Newman-Penrose null tetrad and compute the non-zero spin coefficients.
3. Identify the Petrov type of Minkowski spacetime (type O), Schwarzschild (type D), and plane gravitational waves (type N).

**Thought Experiments:**
- The Weyl scalar Ψ₄ describes outgoing gravitational radiation. Why is it Ψ₄ specifically (and not Ψ₀) that LIGO measures, given that LIGO is far from the source?


---

# UNIT XIV: Quantum Effects in Curved Spacetime

*Unit Introduction: When quantum field theory is placed on a curved spacetime background (without quantizing the metric itself), profound new phenomena emerge. The Unruh effect shows that acceleration creates a thermal bath. Hawking radiation shows that black holes evaporate. These results place GR in deep tension with quantum mechanics, leading to the information paradox — one of the central open problems in theoretical physics.*

---

## Chapter 55: Quantum Field Theory in Curved Spacetime

*Chapter Introduction: QFT in curved spacetime treats the metric as a fixed classical background and quantizes matter fields propagating on it. The absence of a global Killing vector means there is no unique notion of "particle" — different observers define particles differently. This observer-dependence is the root of the Unruh and Hawking effects.*

### 55.1 Review of QFT in Flat Spacetime
- 55.1.1 Canonical quantization of the scalar field in Minkowski space
- 55.1.2 Mode expansion; creation and annihilation operators
- 55.1.3 The Fock space; vacuum state; particle states
- 55.1.4 The Feynman propagator

### 55.2 Scalar Field in Curved Spacetime
- 55.2.1 The Klein-Gordon equation in curved spacetime: (□ - m² - ξR)φ = 0
- 55.2.2 Minimal vs. conformal coupling (ξ = 0 vs. ξ = 1/6)
- 55.2.3 Mode functions; the Klein-Gordon inner product
- 55.2.4 Ambiguity of the vacuum: no preferred decomposition into positive/negative frequency modes without a global timelike Killing vector

### 55.3 Bogoliubov Transformations
- 55.3.1 Two sets of modes related by a Bogoliubov transformation: ā_k = α_{kk'} a_{k'} + β*_{kk'} a†_{k'}
- 55.3.2 Particle creation: if β ≠ 0, the vacuum of one observer contains particles for another
- 55.3.3 The number density of particles seen by a new observer

### 55.4 Particle Creation in Time-Dependent Backgrounds
- 55.4.1 Particle creation in an expanding universe
- 55.4.2 The spectrum of created particles
- 55.4.3 Cosmological particle production and dark matter

### 55.5 The Stress-Energy Tensor in Curved Spacetime
- 55.5.1 The quantum stress-energy tensor ⟨T_{μν}⟩
- 55.5.2 Renormalization in curved spacetime; UV divergences
- 55.5.3 Trace anomaly (conformal anomaly): ⟨T^μ_μ⟩ ≠ 0 for conformal fields in curved space

---

**Key Concepts:** QFT in curved spacetime, Klein-Gordon equation, mode function, vacuum ambiguity, Bogoliubov transformation, particle creation, renormalized stress-energy tensor, trace anomaly

**Important Figures:** Leonard Parker, Yakov Zeldovich, Alexei Starobinsky, Paul Davies, William Unruh, Stephen Hawking

**Additional Reading and Primary Sources:**
- Parker, *Particle Creation in Expanding Universes* (1969) — primary source
- Birrell & Davies, *Quantum Fields in Curved Space*
- Wald, *Quantum Field Theory in Curved Spacetime and Black Hole Thermodynamics*

**Exercises:**
1. Show that the Bogoliubov coefficients satisfy |α_k|² - |β_k|² = 1 (unitarity of Bogoliubov transformation).
2. Compute the particle number density ⟨N_k⟩ = |β_k|² for the vacuum of an "in" observer measured by an "out" observer.
3. Derive the trace anomaly for a conformally coupled scalar field in 4D curved spacetime.

**Thought Experiments:**
- There is no single preferred vacuum state in curved spacetime. Does this mean quantum mechanics is ambiguous in GR? How does this relate to the measurement problem?
- Particle creation in an expanding universe produces particles from "nothing." Where does the energy come from?

---

## Chapter 56: The Unruh Effect

*Chapter Introduction: An observer uniformly accelerating through flat Minkowski spacetime perceives the Minkowski vacuum as a thermal bath of particles at temperature T = ℏa/(2πck_B). This Unruh effect demonstrates that the particle content of a state is observer-dependent — a quantum observer-dependence that has no classical analog.*

### 56.1 Rindler Spacetime
- 56.1.1 Uniformly accelerating observers in Minkowski spacetime
- 56.1.2 Rindler coordinates (τ, ξ, x⊥)
- 56.1.3 The Rindler metric: ds² = -(aξ)² dτ² + dξ² + dx⊥²
- 56.1.4 The Rindler horizon: the acceleration horizon

### 56.2 The Unruh Effect
- 56.2.1 Mode decomposition in Rindler vs. Minkowski coordinates
- 56.2.2 Bogoliubov transformation between Minkowski and Rindler modes
- 56.2.3 The Unruh temperature: T_U = ℏa/(2πck_B)
- 56.2.4 The Minkowski vacuum as a thermal state for a Rindler observer
- 56.2.5 The KMS condition and thermal Green's functions

### 56.3 Physical Implications
- 56.3.1 Relation between the Unruh effect and the equivalence principle
- 56.3.2 Detectability of the Unruh effect; experimental proposals
- 56.3.3 Connection to Hawking radiation (preview)

---

**Key Concepts:** Rindler spacetime, acceleration horizon, Unruh temperature, Bogoliubov transformation, KMS condition, thermal state, vacuum ambiguity

**Important Figures:** William Unruh, Paul Davies, Stephen Fulling, Robert Wald

**Additional Reading and Primary Sources:**
- Unruh, *Notes on Black-Hole Evaporation* (1976) — primary source
- Davies, *Scalar Production in Schwarzschild and Rindler Metrics* (1975)
- Birrell & Davies, *Quantum Fields in Curved Space*, Ch. 4

**Exercises:**
1. Derive the Rindler metric from Minkowski by the coordinate transformation T = ξ sinh(aτ), X = ξ cosh(aτ).
2. Show that the Bogoliubov transformation between Minkowski and Rindler modes yields a thermal distribution with T = ℏa/(2πck_B).
3. For an acceleration a = 10²⁰ m/s², compute the Unruh temperature. What would be needed to measure it directly?

**Thought Experiments:**
- Two observers — one inertial, one uniformly accelerating — describe the same quantum state differently: one sees a vacuum, the other sees a thermal bath. Who is "right"? What does this say about the concept of a particle?
- By the equivalence principle, an observer in a gravitational field (at rest) is equivalent to an accelerating observer. Does this mean observers at rest on the Earth's surface should see an Unruh thermal effect due to gravity?

---

## Chapter 57: Hawking Radiation

*Chapter Introduction: Stephen Hawking's 1974 discovery that black holes emit thermal radiation — and will eventually evaporate — is one of the most important theoretical results of the 20th century. It combines GR, quantum mechanics, and thermodynamics in a single calculation, and raises profound questions about information and unitarity.*

### 57.1 The Hawking Effect: Setup
- 57.1.1 Gravitational collapse forming a black hole
- 57.1.2 The "in" vacuum before collapse and the "out" state after horizon formation
- 57.1.3 Mode functions on the collapsing background

### 57.2 Derivation of Hawking Radiation
- 57.2.1 Bogoliubov coefficients between in-modes and out-modes
- 57.2.2 The tracing argument: modes behind the horizon are inaccessible
- 57.2.3 The thermal spectrum: ⟨N_ω⟩ = [exp(ℏω/k_B T_H) - 1]⁻¹
- 57.2.4 The Hawking temperature: T_H = ℏc³/(8πGMk_B)
- 57.2.5 The Planck spectrum of emitted radiation

### 57.3 Black Hole Evaporation
- 57.3.1 Power emitted: L = σ A T_H⁴ ∝ M⁻²
- 57.3.2 Mass loss rate: Ṁ = -α/M²
- 57.3.3 Evaporation time: t_evap ∝ M³
- 57.3.4 For a solar mass BH: t_evap ~ 10⁶⁶ years (astrophysically irrelevant)
- 57.3.5 For a primordial BH: M* ~ 5×10¹¹ kg evaporates now

### 57.4 The Surface Gravity and the General Formula
- 57.4.1 Surface gravity κ for stationary black holes
- 57.4.2 General formula: T_H = ℏκ/(2πck_B)
- 57.4.3 Kerr black hole: κ depends on a; extremal BH has T = 0

### 57.5 Hawking Radiation as a Tunneling Process
- 57.5.1 Parikh-Wilczek tunneling picture
- 57.5.2 Virtual pairs at the horizon; pair separation by tidal effects
- 57.5.3 Conceptual cartoon vs. the rigorous Bogoliubov derivation

---

**Key Concepts:** Hawking temperature, Hawking radiation, black hole evaporation, surface gravity κ, thermal spectrum, Bogoliubov transformation, pair production near horizon

**Important Figures:** Stephen Hawking, Jacob Bekenstein, William Unruh, Robert Wald, Mukund Rangamani

**Additional Reading and Primary Sources:**
- Hawking, *Black Hole Explosions?* (1974) — primary source (Nature letter)
- Hawking, *Particle Creation by Black Holes* (1975) — primary source (detailed paper)
- Wald, *Quantum Field Theory in Curved Spacetime and Black Hole Thermodynamics*

**Exercises:**
1. Compute the Hawking temperature for a 10 M☉ black hole.
2. Derive the evaporation time for a black hole of initial mass M₀ by solving Ṁ = -α/M².
3. Compute the Hawking temperature of an extremal Kerr black hole (a = GM/c²) from its surface gravity.
4. Estimate the mass of a primordial black hole whose evaporation time equals the age of the Universe.

**Thought Experiments:**
- Hawking radiation is thermal. Thermal radiation carries no information. But the black hole formed from a pure quantum state (a collapsing star). If Hawking radiation is thermal, where did the information go? This is the information paradox.
- The Hawking temperature T_H = ℏc³/(8πGMk_B) goes up as M goes down. A small black hole is hotter. Does this mean black hole evaporation is runaway once it starts?

---

## Chapter 58: Black Hole Thermodynamics and the Information Paradox

*Chapter Introduction: The four laws of black hole mechanics — discovered before Hawking radiation — are perfect analogs of the laws of thermodynamics. Hawking radiation identifies the Bekenstein-Hawking entropy S = A/(4ℓ_P²) as a real thermodynamic entropy, with deep implications for quantum gravity and the holographic principle.*

### 58.1 The Four Laws of Black Hole Mechanics
- 58.1.1 Zeroth law: surface gravity κ is constant over the horizon of a stationary black hole
- 58.1.2 First law: dM = (κ/8πG) dA + Ω_H dJ + Φ dQ (analogue of dU = T dS - p dV)
- 58.1.3 Second law (Hawking area theorem): dA/dt ≥ 0 in classical GR
- 58.1.4 Third law: κ → 0 is unattainable (extremal BH)

### 58.2 Bekenstein-Hawking Entropy
- 58.2.1 Bekenstein's entropy conjecture: S_BH ∝ A
- 58.2.2 Hawking's confirmation: S_BH = Ak_Bc³/(4Gℏ) = A/(4ℓ_P²)
- 58.2.3 The Planck length ℓ_P = √(ℏG/c³) ≈ 1.6 × 10⁻³⁵ m
- 58.2.4 Generalized second law: d(S_BH + S_matter)/dt ≥ 0

### 58.3 The Information Paradox
- 58.3.1 Statement: a pure state collapses into a black hole; Hawking radiation is thermal (mixed state). Unitarity violated?
- 58.3.2 Hawking's original argument for information loss
- 58.3.3 Possible resolutions:
  - Information is encoded in subtle correlations in Hawking radiation (complementarity)
  - Information is stored in a remnant
  - Information is lost (non-unitary quantum gravity)
- 58.3.4 Black hole complementarity (Susskind-'t Hooft)
- 58.3.5 Firewalls (AMPS argument, 2012)
- 58.3.6 The Page curve and unitarity
- 58.3.7 Island formula and the replica trick (2019–present)

### 58.4 The Holographic Principle and Black Hole Entropy
- 58.4.1 Bekenstein bound: S ≤ 2πER/(ℏc)
- 58.4.2 The holographic principle: physics in a region is encoded on its boundary
- 58.4.3 The Bousso entropy bound (covariant entropy bound)
- 58.4.4 Holography as a principle of quantum gravity (preview of AdS/CFT)

---

**Key Concepts:** laws of black hole mechanics, Bekenstein-Hawking entropy, Planck length, generalized second law, information paradox, unitarity, black hole complementarity, firewall, Page curve, holographic principle, Bekenstein bound

**Important Figures:** Jacob Bekenstein, Stephen Hawking, Don Page, Leonard Susskind, Gerard 't Hooft, Ahmed Almheiri, Juan Maldacena, Geoff Penington

**Additional Reading and Primary Sources:**
- Bekenstein, *Black Holes and Entropy* (1973) — primary source
- Bardeen, Carter & Hawking, *The Four Laws of Black Hole Mechanics* (1973) — primary source
- Hawking, *Breakdown of Predictability in Gravitational Collapse* (1976)
- Almheiri, Mahajan, Maldacena & Zhao, *The Page Curve of Hawking Radiation from Semiclassical Geometry* (2019)
- Penington, *Entanglement Wedge Reconstruction and the Information Paradox* (2019)

**Exercises:**
1. Verify the first law of black hole mechanics dM = (κ/8πG)dA + Ω_H dJ for the Kerr metric.
2. Compute the Bekenstein-Hawking entropy of a 10 M☉ black hole in units of k_B.
3. Compute the Bekenstein bound for a 1 kg book of radius 10 cm.
4. Describe the AMPS firewall argument and identify the three postulates that cannot simultaneously be true.

**Thought Experiments:**
- The Bekenstein-Hawking entropy S = A/4 is proportional to area, not volume. A box of gas has entropy proportional to volume. Why is a black hole different? What does this say about the number of degrees of freedom of a black hole?
- Black hole complementarity says no observer sees both the information falling into the black hole AND the information in Hawking radiation. Is this consistent with quantum mechanics? Where does it break down (the AMPS argument)?


---

# UNIT XV: Singularity Theorems and Global Methods

*Unit Introduction: The singularity theorems of Penrose and Hawking prove, from very general assumptions, that singularities are inevitable inside black holes and at the Big Bang. These theorems use the global machinery of Lorentzian geometry — causal structure, trapped surfaces, geodesic focusing — developed in Units VIII and XIII. They are among the greatest achievements in mathematical GR.*

---

## Chapter 59: Trapped Surfaces and Singularity Theorems

*Chapter Introduction: A trapped surface is a compact spacelike 2-surface from which both ingoing and outgoing null rays are converging. Penrose's 1965 theorem shows that the existence of such a surface, combined with the null energy condition, implies the existence of a singularity. This was the first proof that singularities are not merely artifacts of special symmetry.*

### 59.1 Causal Structure Preliminaries
- 59.1.1 Causal future J⁺(S) and chronological future I⁺(S)
- 59.1.2 Achronal sets; Cauchy developments
- 59.1.3 Global hyperbolicity: existence of Cauchy surfaces
- 59.1.4 Cauchy horizons and their instability

### 59.2 Trapped Surfaces
- 59.2.1 Definition: compact spacelike 2-surface S where both null expansions θ_± < 0
- 59.2.2 Physical picture: light rays from S converge in both directions
- 59.2.3 Existence of trapped surfaces implies gravitational collapse is well underway
- 59.2.4 The apparent horizon: outermost trapped surface

### 59.3 The Penrose Singularity Theorem (1965)
- 59.3.1 Statement: if (1) spacetime is globally hyperbolic, (2) the NEC holds, (3) a trapped surface exists, then spacetime is geodesically incomplete (a singularity exists)
- 59.3.2 Key ingredients: Raychaudhuri equation + NEC → geodesic focusing
- 59.3.3 Existence of a trapped surface + focusing → geodesic incompleteness
- 59.3.4 Physical meaning: singularities inside black holes are inevitable

### 59.4 The Hawking-Penrose Singularity Theorem (1970)
- 59.4.1 Statement: conditions on the SEC and initial data for the Universe → the Big Bang singularity is inevitable
- 59.4.2 Comparison with the Penrose theorem
- 59.4.3 The strong energy condition and its role

### 59.5 The Hawking Area Theorem
- 59.5.1 Statement: under the NEC, the area of the event horizon of a black hole never decreases: dA/dt ≥ 0
- 59.5.2 Proof via the Raychaudhuri equation and focusing
- 59.5.3 Violations by quantum effects (Hawking radiation)

### 59.6 Cosmic Censorship Conjectures
- 59.6.1 Weak cosmic censorship: naked singularities do not form from generic regular initial data
- 59.6.2 Strong cosmic censorship: spacetime is globally hyperbolic (no Cauchy horizons from generic data)
- 59.6.3 Status: neither has been proven; counterexamples in special cases
- 59.6.4 Evidence from numerical simulations

---

**Key Concepts:** trapped surface, apparent horizon, geodesic incompleteness, Penrose singularity theorem, Hawking-Penrose theorem, area theorem, cosmic censorship, null energy condition

**Important Figures:** Roger Penrose, Stephen Hawking, Robert Geroch, Robert Wald

**Additional Reading and Primary Sources:**
- Penrose, *Gravitational Collapse and Space-Time Singularities* (1965) — primary source (Nobel-prize work)
- Hawking & Penrose, *The Singularities of Gravitational Collapse and Cosmology* (1970) — primary source
- Hawking & Ellis, *The Large Scale Structure of Space-Time*
- Wald, *General Relativity*, Ch. 9–12

**Exercises:**
1. Show, using the Raychaudhuri equation and the NEC (R_{μν}k^μk^ν ≥ 0), that an initially converging null congruence reaches a focal point in finite affine parameter.
2. State the precise hypotheses of the Penrose singularity theorem and identify which hypothesis corresponds to the physical assumption of "collapse has started."
3. Prove the Hawking area theorem from the Raychaudhuri equation and the NEC.
4. Give an example of a spacetime where the SEC is violated and singularities are avoided.

**Thought Experiments:**
- The Penrose theorem proves geodesic incompleteness, not that matter densities or curvatures diverge. Why are these different? Is geodesic incompleteness the right definition of a "singularity"?
- Quantum effects (Hawking radiation) violate the NEC and allow the horizon area to decrease. Does this invalidate the Penrose theorem? What does it say about the ultimate fate of black holes?

---

## Chapter 60: Global Methods and Causal Topology

*Chapter Introduction: The global structure of spacetime — its topology, causal properties, and behavior at infinity — constrains what physics can occur. This chapter surveys the tools of global analysis in GR: asymptotic flatness, positivity of energy, and the topology of black holes.*

### 60.1 Asymptotic Flatness
- 60.1.1 Definition via conformal compactification: spacetime admits smooth conformal boundary
- 60.1.2 Asymptotically flat spacetimes: properties and examples
- 60.1.3 ADM mass, momentum, and angular momentum from the asymptotic metric

### 60.2 The Positive Energy Theorem
- 60.2.1 Statement: for asymptotically flat spacetimes satisfying the dominant energy condition, the ADM mass M ≥ 0, with equality iff spacetime is flat
- 60.2.2 The Witten spinor proof (1981)
- 60.2.3 The Schoen-Yau geometric proof (1979)
- 60.2.4 Physical significance: gravity cannot create negative-energy isolated systems

### 60.3 Topology of Black Holes
- 60.3.1 Hawking's topology theorem: cross-sections of the event horizon are topologically S² (in 4D, under NEC)
- 60.3.2 Exotic topologies in higher dimensions: black rings (Emparan-Reall, 2002)
- 60.3.3 Uniqueness theorems: the Kerr-Newman solution is the unique stationary black hole (in 4D)

### 60.4 Causal Dynamics and Topological Censorship
- 60.4.1 Topological censorship: no topology change of spatial sections visible to asymptotic observers
- 60.4.2 Forbidden wormhole traversal (Friedman, Schleich & Witt, 1993)
- 60.4.3 Implications for time travel

---

**Key Concepts:** asymptotic flatness, ADM mass, positive energy theorem, Witten spinor, topology of black holes, uniqueness theorem, topological censorship

**Important Figures:** Roger Penrose, Stephen Hawking, Richard Schoen, Shing-Tung Yau, Edward Witten, Robert Geroch

**Additional Reading and Primary Sources:**
- Schoen & Yau, *On the Proof of the Positive Mass Conjecture in General Relativity* (1979)
- Witten, *A New Proof of the Positive Energy Theorem* (1981)
- Hawking & Ellis, *The Large Scale Structure of Space-Time*, Ch. 9
- Wald, *General Relativity*, Ch. 11–12

**Exercises:**
1. State the positive energy theorem precisely and explain why it requires the dominant energy condition.
2. Sketch the Witten spinor proof of the positive energy theorem (identify the key steps).
3. Show that the Schwarzschild ADM mass equals the parameter M appearing in the metric.

**Thought Experiments:**
- The positive energy theorem says isolated gravitating systems cannot have negative total energy. What would negative ADM mass imply physically? What would you observe far from such an object?
- Topological censorship forbids wormhole traversal for any observer who remains outside a black hole. What does this say about the viability of wormhole-based faster-than-light travel?


---

# UNIT XVI: Frontiers of General Relativity

*Unit Introduction: This final unit surveys the active research frontiers where general relativity meets other areas of physics, mathematics, and experiment. Numerical relativity computes exact solutions to the full nonlinear equations. Modified gravity theories extend or replace GR. Quantum gravity attempts to reconcile GR with quantum mechanics. The unit closes with a survey of open problems — the questions that will occupy gravitational physicists for decades to come.*

---

## Chapter 61: Numerical Relativity

*Chapter Introduction: For most physically interesting situations — binary black hole mergers, neutron star collisions, gravitational collapse — the full nonlinear Einstein equations must be solved numerically. Numerical relativity develops the computational and mathematical methods to do this reliably and has been essential for LIGO science.*

### 61.1 Challenges of Numerical GR
- 61.1.1 The constraint equations and their preservation under evolution
- 61.1.2 Gauge freedom and coordinate singularities
- 61.1.3 The physical singularity problem: excision and punctures
- 61.1.4 Stability of numerical schemes for hyperbolic equations

### 61.2 The BSSN Formulation
- 61.2.1 Baumgarte-Shapiro-Shibata-Nakamura variables
- 61.2.2 Conformal decomposition of the spatial metric
- 61.2.3 Hyperbolicity and stability advantages over ADM
- 61.2.4 Gauge conditions: the "moving puncture" method

### 61.3 Breakthrough: Binary Black Hole Mergers
- 61.3.1 Pretorius's 2005 breakthrough: first stable long-term BBH simulation
- 61.3.2 The Goddard group (moving punctures) and independent confirmation
- 61.3.3 Inspiral, merger, ringdown waveforms
- 61.3.4 Comparison with LIGO data; calibration of post-Newtonian and EOB models

### 61.4 Other Applications of Numerical Relativity
- 61.4.1 Neutron star mergers: r-process nucleosynthesis, kilonova, short GRB
- 61.4.2 Gravitational collapse and core-collapse supernovae
- 61.4.3 Critical phenomena in gravitational collapse (Choptuik)
- 61.4.4 Cosmological simulations in GR

---

**Key Concepts:** BSSN formulation, moving punctures, excision, gauge conditions, constraint preservation, binary black hole merger simulation

**Important Figures:** Frans Pretorius, Manuela Campanelli, John Baker, Matthew Choptuik, Masaru Shibata, Thomas Baumgarte, Stuart Shapiro

**Additional Reading and Primary Sources:**
- Pretorius, *Evolution of Binary Black-Hole Spacetimes* (2005) — primary source
- Campanelli et al., *Accurate Evolutions of Orbiting Black-Hole Binaries* (2006)
- Baumgarte & Shapiro, *Numerical Relativity: Solving Einstein's Equations on the Computer*

**Exercises:**
1. Describe the constraint damping problem in ADM evolution and how BSSN addresses it.
2. What is the Choptuik phenomenon in spherical gravitational collapse? What are the universal critical exponents?
3. Explain the moving puncture method: why does it allow black hole singularities to be evolved numerically?

**Thought Experiments:**
- The first successful BBH simulation (Pretorius 2005) ran for only ~1 month on a supercomputer. What was the main technical breakthrough? Why did this take 40 years after the ADM equations were written down?

---

## Chapter 62: Modified Theories of Gravity

*Chapter Introduction: While GR passes every classical test, there are theoretical motivations (dark energy, dark matter, quantum gravity) and observational tensions (Hubble tension) that drive the study of modified gravity theories. Each modification must reduce to GR in some limit while altering predictions in another — testable deviations are the goal.*

### 62.1 Scalar-Tensor Theories
- 62.1.1 Brans-Dicke theory: replacing G with a scalar field
- 62.1.2 The Brans-Dicke parameter ω_{BD}; solar system constraints ω_{BD} > 40,000
- 62.1.3 General scalar-tensor theories: Horndeski gravity (most general with second-order EOM)
- 62.1.4 Galileon theories; screening mechanisms (Vainshtein, chameleon)

### 62.2 f(R) Gravity
- 62.2.1 Replacing R with f(R) in the action
- 62.2.2 Equivalence to scalar-tensor theory (via conformal transformation)
- 62.2.3 Starobinsky inflation: f(R) = R + R²/(6M²)
- 62.2.4 Cosmological applications; dark energy models

### 62.3 Higher-Curvature Gravity
- 62.3.1 Gauss-Bonnet gravity in 4D (topological) vs. higher dimensions
- 62.3.2 Lovelock gravity; ghost-free higher-curvature theories
- 62.3.3 Ghost instabilities and the Ostrogradski problem

### 62.4 Massive Gravity and Bigravity
- 62.4.1 Adding a mass to the graviton
- 62.4.2 The vDVZ discontinuity and the Vainshtein mechanism
- 62.4.3 The Fierz-Pauli mass term; de Rham-Gabadadze-Tolley (dRGT) massive gravity
- 62.4.4 Hassan-Rosen bigravity: two interacting metrics

### 62.5 Tests of Modified Gravity
- 62.5.1 Solar system tests: PPN parameters
- 62.5.2 Gravitational wave tests: speed of gravity, polarization modes
- 62.5.3 Cosmological tests: growth of structure, ISW effect
- 62.5.4 Strong-field tests: black hole shadows, quasi-normal modes

---

**Key Concepts:** Brans-Dicke theory, scalar-tensor gravity, Horndeski theory, f(R) gravity, Gauss-Bonnet term, Lovelock gravity, massive gravity, Vainshtein mechanism, graviton mass

**Important Figures:** Carl Brans, Robert Dicke, David Lovelock, Alexei Starobinsky, Claudia de Rham, Gregory Gabadadze, Andrew Tolley

**Additional Reading and Primary Sources:**
- Brans & Dicke, *Mach's Principle and a Relativistic Theory of Gravitation* (1961)
- Starobinsky, *A New Type of Isotropic Cosmological Models Without Singularity* (1980)
- de Rham, *Massive Gravity* (2014, review article)
- Will, *Theory and Experiment in Gravitational Physics*

**Exercises:**
1. Show that Brans-Dicke theory is equivalent to GR with a minimally coupled scalar field via a conformal transformation.
2. Derive the field equations for f(R) gravity and show they are fourth order in the metric in general.
3. State the Horndeski conditions for a scalar-tensor theory to have second-order equations of motion.

**Thought Experiments:**
- If GR is "unique" (Lovelock's theorem), why do modified gravity theories exist? What assumption(s) of Lovelock's theorem do they violate?
- The detection of GW170817 constrained the graviton speed to c_{gw}/c = 1 ± 10⁻¹⁵. What does this rule out among modified gravity theories?

---

## Chapter 63: Approaches to Quantum Gravity

*Chapter Introduction: Quantum gravity — the theory that reconciles GR and quantum mechanics — remains one of the great unsolved problems in physics. Multiple approaches exist, each with partial successes and serious obstacles. This chapter surveys the leading candidates: string theory, loop quantum gravity, and several smaller programs.*

### 63.1 The Problem of Quantum Gravity
- 63.1.1 Why GR and QM are incompatible: non-renormalizability of perturbative quantum GR
- 63.1.2 The Planck scale: ℓ_P, t_P, m_P
- 63.1.3 What a theory of quantum gravity must explain: singularity resolution, information paradox, black hole entropy microscopics

### 63.2 String Theory
- 63.2.1 Strings as the fundamental objects; the string tension
- 63.2.2 Modes of a string; massless spin-2 mode = graviton
- 63.2.3 Supersymmetry and supergravity
- 63.2.4 Five consistent superstring theories in 10D; M-theory in 11D
- 63.2.5 T-duality, S-duality, and the web of dualities
- 63.2.6 Extra dimensions and compactification (Kaluza-Klein)
- 63.2.7 D-branes and their role in black hole microscopics (Strominger-Vafa 1996)

### 63.3 AdS/CFT Correspondence
- 63.3.1 Maldacena's 1997 conjecture: Type IIB string theory on AdS₅ × S⁵ ≡ 𝒩=4 SYM in 4D
- 63.3.2 The holographic dictionary
- 63.3.3 Black holes in AdS: Hawking-Page transition; dual to deconfinement
- 63.3.4 Applications: quark-gluon plasma, condensed matter (AdS/CMT)
- 63.3.5 Entanglement entropy and the Ryu-Takayanagi formula

### 63.4 Loop Quantum Gravity
- 63.4.1 The Ashtekar variables: connection and tetrad as configuration space
- 63.4.2 Loop representation; spin networks as quantum states of geometry
- 63.4.3 Area and volume operators: discrete spectra at the Planck scale
- 63.4.4 Spin foam models: path integral version of LQG
- 63.4.5 Loop quantum cosmology: Big Bounce replacing Big Bang singularity

### 63.5 Other Approaches
- 63.5.1 Causal dynamical triangulations (CDT)
- 63.5.2 Causal set theory
- 63.5.3 Asymptotic safety
- 63.5.4 Non-commutative geometry (Connes)
- 63.5.5 Twistor theory (Penrose)

---

**Key Concepts:** Planck scale, non-renormalizability, string theory, graviton, supersymmetry, M-theory, AdS/CFT, holographic principle, loop quantum gravity, spin network, area quantization, Ashtekar variables

**Important Figures:** John Schwarz, Michael Green, Edward Witten, Juan Maldacena, Abhay Ashtekar, Lee Smolin, Carlo Rovelli, Roger Penrose, Alain Connes

**Additional Reading and Primary Sources:**
- Maldacena, *The Large N Limit of Superconformal Field Theories and Supergravity* (1997) — primary source
- Strominger & Vafa, *Microscopic Origin of the Bekenstein-Hawking Entropy* (1996)
- Ashtekar, *New Variables for Classical and Quantum Gravity* (1986)
- Penrose, *Twistor Algebra* (1967)
- Rovelli, *Quantum Gravity* (Cambridge, 2004)
- Polchinski, *String Theory*, Vols. 1–2

**Exercises:**
1. Estimate the Planck mass, length, and time from dimensional analysis using G, ℏ, and c.
2. Show that the Ryu-Takayanagi formula S = A_{min}/(4G_N) for entanglement entropy in AdS/CFT is dimensionally consistent with the Bekenstein-Hawking formula.
3. Compute the minimum eigenvalue of the area operator in LQG: A_min = 4πγℓ_P²√3 (where γ is the Barbero-Immirzi parameter).

**Thought Experiments:**
- String theory predicts 10 spacetime dimensions but we observe 4. What happens to the other 6? Is there observational evidence one way or the other?
- AdS/CFT is a duality between a gravitational theory in the bulk and a non-gravitational theory on the boundary. Does this mean gravity is "not fundamental"? What does it mean for spacetime to "emerge" from a non-gravitational theory?

---

## Chapter 64: Open Problems and Current Research

*Chapter Introduction: General relativity, despite its extraordinary success, leaves profound questions unanswered. This chapter surveys the major open problems — mathematical, physical, and observational — that define the frontier of gravitational research in the 21st century.*

### 64.1 Mathematical Open Problems
- 64.1.1 The weak and strong cosmic censorship conjectures (Penrose)
- 64.1.2 The non-linear stability of the Kerr solution
- 64.1.3 The non-linear stability of Minkowski spacetime (Christodoulou-Klainerman 1993; partial result)
- 64.1.4 Uniqueness theorems for black holes in higher dimensions
- 64.1.5 Existence and regularity of solutions to the Einstein equations with matter

### 64.2 Quantum Gravity Open Problems
- 64.2.1 The information paradox: ultimate resolution
- 64.2.2 The black hole interior: what happens beyond the horizon?
- 64.2.3 The singularity problem: is the Big Bang singularity physical or an artifact?
- 64.2.4 The cosmological constant problem
- 64.2.5 The measurement of Hawking radiation (direct detection)

### 64.3 Observational Frontiers
- 64.3.1 Next-generation gravitational wave detectors: Einstein Telescope, Cosmic Explorer
- 64.3.2 Space-based detectors: LISA (launch ~2035)
- 64.3.3 Pulsar timing arrays and the nanohertz GW background (NANOGrav, PPTA, EPTA)
- 64.3.4 The Event Horizon Telescope and black hole imaging
- 64.3.5 Tests of GR in the strong-field regime: EHT, gravitational wave spectroscopy
- 64.3.6 Multi-messenger gravitational wave astronomy

### 64.4 Theoretical Frontiers
- 64.4.1 Soft theorems, asymptotic symmetries, and the BMS group
- 64.4.2 The information paradox and the island formula
- 64.4.3 Holography beyond AdS: de Sitter holography; celestial holography
- 64.4.4 Gravitational entropy: Wald's Noether charge entropy
- 64.4.5 Gravitational memory effects and their detection
- 64.4.6 Connections between black hole physics and quantum information theory

### 64.5 The Dark Universe
- 64.5.1 The nature of dark matter: beyond ΛCDM candidates
- 64.5.2 The nature of dark energy: dynamical vs. cosmological constant
- 64.5.3 The Hubble tension: new physics or systematic error?
- 64.5.4 Primordial gravitational waves: window on inflation
- 64.5.5 GR modifications in cosmology: future surveys (Euclid, DESI, Rubin LSST)

---

**Key Concepts:** cosmic censorship, Kerr stability, non-linear stability of Minkowski, information paradox, BMS group, gravitational memory, holography, Hubble tension, next-generation detectors

**Important Figures:** Roger Penrose, Demetrios Christodoulou, Sergiu Klainerman, Andrew Strominger, Malcolm Perry, Stephen Hawking, Juan Maldacena, Geoff Penington, Ahmed Almheiri

**Additional Reading and Primary Sources:**
- Christodoulou & Klainerman, *The Global Non-Linear Stability of the Minkowski Space* (1993)
- Strominger, *Lectures on the Infrared Structure of Gravity and Gauge Theory* (2017)
- Event Horizon Telescope Collaboration, *First M87 Event Horizon Telescope Results* (2019)
- Almheiri et al., *The Entropy of Bulk Quantum Fields and the Entanglement Wedge* (2019)

**Exercises:**
1. State the Christodoulou-Klainerman theorem on the stability of Minkowski spacetime and identify the key smallness condition on initial data.
2. Describe the BMS group and its three classes of symmetries: supertranslations, superrotations, and superboosts. What physical observables do they correspond to?
3. Explain the gravitational memory effect: what happens to a ring of test particles after a gravitational wave passes? How does this differ from the oscillatory effect?

**Thought Experiments:**
- If the Kerr black hole is non-linearly stable (as conjectured), then any perturbation eventually rings down and the spacetime settles to a Kerr solution. What does this say about the no-hair theorem?
- The BMS group is infinite-dimensional — it is the symmetry group of asymptotically flat spacetimes at null infinity. Does this mean GR has infinitely many conservation laws? What are the corresponding "charges"?

---

# Appendix A: Essential Formulary

*A reference compendium of the most important equations in the curriculum, organized by unit.*

## A.1 Differential Geometry
- Christoffel symbols: Γᵅ_{μν} = ½g^{αβ}(∂_μg_{νβ} + ∂_νg_{μβ} - ∂_βg_{μν})
- Riemann tensor: R^α_{βμν} = ∂_μΓ^α_{νβ} - ∂_νΓ^α_{μβ} + Γ^α_{μσ}Γ^σ_{νβ} - Γ^α_{νσ}Γ^σ_{μβ}
- Ricci tensor: R_{μν} = R^α_{μαν}
- Ricci scalar: R = g^{μν}R_{μν}
- Einstein tensor: G_{μν} = R_{μν} - ½g_{μν}R
- Bianchi identity: ∇^μG_{μν} = 0

## A.2 Special Relativity
- Minkowski metric: ds² = -c²dt² + dx² + dy² + dz²
- Lorentz factor: γ = (1-v²/c²)^{-1/2}
- Energy-momentum: E² = p²c² + m²c⁴
- 4-velocity normalization: u^μu_μ = -c²

## A.3 General Relativity
- Einstein field equations: G_{μν} + Λg_{μν} = (8πG/c⁴)T_{μν}
- Geodesic equation: d²x^μ/dτ² + Γ^μ_{νρ}(dx^ν/dτ)(dx^ρ/dτ) = 0
- Einstein-Hilbert action: S = (c⁴/16πG)∫(R-2Λ)√(-g)d⁴x
- Geodesic deviation: D²J^μ/dτ² = -R^μ_{νρσ}ṫ^νJ^ρṫ^σ

## A.4 Schwarzschild and Kerr
- Schwarzschild: ds² = -(1-r_s/r)c²dt² + (1-r_s/r)^{-1}dr² + r²dΩ², r_s = 2GM/c²
- ISCO (Schwarzschild): r_ISCO = 6GM/c²; photon sphere: r_ph = 3GM/c²

## A.5 Black Hole Thermodynamics
- Hawking temperature: T_H = ℏc³/(8πGMk_B) = ℏκ/(2πck_B)
- Bekenstein-Hawking entropy: S_BH = k_BAc³/(4Gℏ) = A/(4ℓ²_P)
- First law: dM = (κ/8πG)dA + Ω_H dJ + Φ_H dQ

## A.6 Cosmology
- FLRW metric: ds² = -c²dt² + a²(t)[dr²/(1-kr²) + r²dΩ²]
- Friedmann equation: (ȧ/a)² = 8πGρ/3 + Λc²/3 - kc²/a²
- Hubble parameter: H = ȧ/a

---

# Appendix B: Recommended Textbooks by Level

## Introductory
- Hartle, *Gravity: An Introduction to Einstein's General Relativity* (physics first approach)
- Schutz, *A First Course in General Relativity*
- Taylor & Wheeler, *Spacetime Physics* (special relativity)

## Intermediate
- Carroll, *Spacetime and Geometry: An Introduction to General Relativity*
- D'Inverno, *Introducing Einstein's Relativity*
- Hobson, Efstathiou & Lasenby, *General Relativity: An Introduction for Physicists*

## Advanced
- Wald, *General Relativity* (mathematically rigorous)
- Misner, Thorne & Wheeler, *Gravitation* (encyclopedic)
- Chandrasekhar, *The Mathematical Theory of Black Holes*
- Hawking & Ellis, *The Large Scale Structure of Space-Time*

## Cosmology
- Weinberg, *Gravitation and Cosmology*
- Weinberg, *Cosmology*
- Baumann, *Cosmology* (DAMTP lectures, open access)
- Kolb & Turner, *The Early Universe*

## Mathematical Methods
- Lee, *Introduction to Smooth Manifolds*
- Spivak, *A Comprehensive Introduction to Differential Geometry*, Vols. 1–5
- Penrose & Rindler, *Spinors and Space-Time*, Vols. 1–2

## Quantum Gravity and Black Holes
- Birrell & Davies, *Quantum Fields in Curved Space*
- Wald, *Quantum Field Theory in Curved Spacetime and Black Hole Thermodynamics*
- Polchinski, *String Theory*, Vols. 1–2
- Rovelli, *Quantum Gravity*

---

# Appendix C: Chronological Timeline of Key Developments

| Year | Development | Figure(s) |
|------|-------------|-----------|
| 1687 | *Principia Mathematica*: Newton's laws and gravity | Newton |
| 1788 | *Mécanique Analytique*: Lagrangian mechanics | Lagrange |
| 1833 | Hamiltonian mechanics | Hamilton |
| 1859 | Anomalous perihelion of Mercury reported | Le Verrier |
| 1865 | Maxwell's equations | Maxwell |
| 1873 | *Treatise on Electricity and Magnetism* | Maxwell |
| 1854 | Riemann's lecture on geometry of space | Riemann |
| 1887 | Michelson-Morley experiment (null result) | Michelson, Morley |
| 1900 | Absolute differential calculus | Ricci, Levi-Civita |
| 1905 | Special relativity; E = mc² | Einstein |
| 1907 | Minkowski spacetime | Minkowski |
| 1915 | General relativity field equations | Einstein; Hilbert |
| 1916 | Schwarzschild solution | Schwarzschild |
| 1916 | First prediction of gravitational waves | Einstein |
| 1918 | Noether's theorem | Noether |
| 1919 | Light deflection confirmed (Eddington expedition) | Eddington et al. |
| 1922 | Friedmann cosmological solutions | Friedmann |
| 1927 | Lemaître: expanding universe | Lemaître |
| 1929 | Hubble: recession of galaxies | Hubble |
| 1939 | Gravitational collapse (Oppenheimer-Snyder) | Oppenheimer, Snyder |
| 1948 | Big Bang nucleosynthesis theory | Alpher, Gamow, Herman |
| 1955 | Raychaudhuri equation | Raychaudhuri |
| 1960 | Kruskal extension of Schwarzschild | Kruskal |
| 1963 | Kerr solution | Kerr |
| 1965 | Penrose singularity theorem | Penrose |
| 1965 | CMB discovered | Penzias, Wilson |
| 1969 | Penrose process | Penrose |
| 1970 | Hawking-Penrose theorem | Hawking, Penrose |
| 1971 | Wheeler names "black hole" widespread | Wheeler |
| 1973 | Laws of black hole mechanics | Bardeen, Carter, Hawking |
| 1973 | Bekenstein entropy | Bekenstein |
| 1974 | Hawking radiation | Hawking |
| 1974 | Binary pulsar (indirect GW detection) | Hulse, Taylor |
| 1975 | Unruh effect | Unruh |
| 1976 | Shapiro delay measured (Viking) | Shapiro et al. |
| 1979 | Positive energy theorem | Schoen, Yau |
| 1980 | f(R) inflation (Starobinsky) | Starobinsky |
| 1981 | Inflationary cosmology | Guth, Linde, Albrecht, Steinhardt |
| 1986 | Ashtekar variables for LQG | Ashtekar |
| 1993 | Global stability of Minkowski | Christodoulou, Klainerman |
| 1994 | Numerical LQG; spin networks | Rovelli, Smolin |
| 1995 | M-theory | Witten |
| 1996 | Black hole entropy microscopics | Strominger, Vafa |
| 1997 | AdS/CFT correspondence | Maldacena |
| 1998 | Accelerating expansion of Universe | Perlmutter; Riess, Schmidt |
| 2005 | Binary BBH simulation | Pretorius |
| 2006 | Moving punctures BBH simulation | Campanelli et al.; Baker et al. |
| 2011 | Gravity Probe B results | Everitt et al. |
| 2015 | GW150914: first direct GW detection | LIGO |
| 2017 | GW170817: binary neutron star + EM counterpart | LIGO/Virgo + telescopes |
| 2019 | First black hole image (M87*) | Event Horizon Telescope |
| 2019 | Island formula for Page curve | Almheiri et al.; Penington |
| 2020 | Penrose Nobel Prize (singularity theorems) | Royal Swedish Academy |
| 2022 | Sgr A* black hole image | Event Horizon Telescope |
| 2023 | Evidence for nHz GW background | NANOGrav and PTAs |

---

*End of outline. This document covers the complete conceptual journey from propositional logic and set theory through the frontiers of quantum gravity and gravitational wave astronomy. The progression is designed so that each unit builds on all previous units, and no concept is invoked before it has been carefully introduced.*
