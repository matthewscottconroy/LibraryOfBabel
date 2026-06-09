# Chapter 01: Vector Spaces

The concept of a vector space abstracts the essential features of familiar geometric vectors — the ability to add them and to scale them — into a purely algebraic definition that applies to objects far beyond arrows in the plane. Functions, polynomials, matrices, and sequences can all be viewed as elements of vector spaces. This abstraction is powerful because any theorem proved about vector spaces in general applies to all of these objects at once.

## What the Chapter Covers

**Section 1: Axioms and Definitions** presents the eight axioms of a vector space and derives basic consequences. A vector space over a field $F$ is a set $V$ with operations of addition ($V \times V \to V$) and scalar multiplication ($F \times V \to V$) satisfying axioms including commutativity and associativity of addition, the existence of a zero vector, the existence of additive inverses, and the compatibility of scalar multiplication with field multiplication. The foundational examples are $\mathbb{R}^n$, the space $C([a,b])$ of continuous functions, the space of polynomials of degree at most $n$, and the solution space of a homogeneous linear ODE.

**Section 2: Subspaces** defines a subspace as a nonempty subset of a vector space that is closed under addition and scalar multiplication (and therefore is itself a vector space). The intersection of subspaces is a subspace; the union generally is not. The span of a set of vectors $\{v_1, \ldots, v_k\}$ — the set of all linear combinations $\alpha_1 v_1 + \cdots + \alpha_k v_k$ — is the smallest subspace containing those vectors.

**Section 3: Linear Independence** introduces the central criterion for efficiency: a set of vectors is linearly independent if no one of them can be written as a linear combination of the others, equivalently if the only solution to $\alpha_1 v_1 + \cdots + \alpha_k v_k = 0$ is $\alpha_1 = \cdots = \alpha_k = 0$. Linear independence of a set of functions is tested by the Wronskian. The $k$ solutions of a $k$-th order linear ODE are linearly independent iff their Wronskian is nonzero somewhere on the interval.

**Section 4: Bases and Dimension** defines a basis as a linearly independent spanning set and proves that any two bases of a finite-dimensional vector space have the same number of elements — this number is the **dimension** of the space. The dimension theorem for linear maps (Rank-Nullity, proved in the next chapter) follows from the theory developed here.

## Connection to Differential Equations

The solution space of the $n$-th order homogeneous linear ODE $L[y] = 0$ is an $n$-dimensional subspace of $C^n(I)$. This fact — which follows from the theory of bases and dimension together with the existence and uniqueness theorem — is what gives the general solution structure its character: once a basis (set of $n$ linearly independent solutions) is found, every solution is a unique linear combination of them.
