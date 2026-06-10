# Topological Spaces

## The Move from Metric to Open Set

Begin with something familiar. You know what it means for a function $f : \mathbb{R} \to \mathbb{R}$ to be continuous at a point $x_0$: for every $\varepsilon > 0$ there exists $\delta > 0$ such that $|x - x_0| < \delta$ implies $|f(x) - f(x_0)| < \varepsilon$. This is the classical $\varepsilon$-$\delta$ definition, and it works perfectly well for functions between metric spaces where you have a notion of distance.

But here is an observation that changes everything: this definition is equivalent to saying that the preimage of every open set in $\mathbb{R}$ is open in $\mathbb{R}$. The $\varepsilon$-$\delta$ language and the open-set language say the same thing — but the open-set language mentions no distances. It refers only to which subsets count as "open."

This is the insight that generates topology. If continuity is really about open sets rather than distances, we can define continuity for spaces that have no metric at all — as long as we specify which subsets are open. And the conditions an open-set system must satisfy to make continuity well-behaved can be read off from the metric case.

## The Axioms

**Definition.** A *topological space* is a pair $(X, \tau)$ where $X$ is a set and $\tau \subseteq \mathcal{P}(X)$ is a collection of subsets satisfying:

1. **Trivial sets are open:** $\emptyset \in \tau$ and $X \in \tau$.
2. **Arbitrary unions:** If $\{U_\alpha\}_{\alpha \in I} \subseteq \tau$, then $\bigcup_{\alpha \in I} U_\alpha \in \tau$.
3. **Finite intersections:** If $U_1, \ldots, U_n \in \tau$, then $U_1 \cap \cdots \cap U_n \in \tau$.

The collection $\tau$ is called a *topology* on $X$, and its elements are the *open sets*. A subset $C \subseteq X$ is *closed* if its complement $X \setminus C$ is open. Note the terminological warning: "open" and "closed" are not opposites. A set can be both open and closed (such sets are called *clopen*), neither, or one but not the other. In $\mathbb{R}$ with the standard topology, the interval $(0,1)$ is open but not closed; $[0,1]$ is closed but not open; $\emptyset$ and $\mathbb{R}$ are both.

Why these three axioms? The empty set and $X$ should be open because they represent the trivial situations: no constraints, and all constraints respectively. Arbitrary unions are permitted because openness is a *local* condition: if every point of $U_\alpha$ has a neighborhood contained in $U_\alpha$, then every point of the union has such a neighborhood. Only *finite* intersections are required because infinite intersections can destroy openness: in $\mathbb{R}$, each interval $(-1/n, 1/n)$ is open, but their intersection $\bigcap_{n=1}^\infty (-1/n, 1/n) = \{0\}$ is a single point, which is not open in $\mathbb{R}$.

## The Standard Examples

### The Metric Topology

Every metric space $(X, d)$ carries a natural topology: a subset $U \subseteq X$ is open if for every point $x \in U$ there exists $\varepsilon > 0$ such that the open ball $B(x, \varepsilon) = \{y \in X : d(x, y) < \varepsilon\}$ is contained in $U$. This is the *metric topology* or *topology induced by the metric*. One checks the axioms:

- $\emptyset$ is vacuously open; $X$ is open because any $\varepsilon$ works for any point.
- Unions: if $x \in \bigcup U_\alpha$ then $x \in U_\alpha$ for some $\alpha$, so there is a ball around $x$ in $U_\alpha$, hence in the union.
- Finite intersections: if $x \in U \cap V$ with $B(x, \varepsilon_1) \subseteq U$ and $B(x, \varepsilon_2) \subseteq V$, take $\varepsilon = \min(\varepsilon_1, \varepsilon_2)$.

The Euclidean topology on $\mathbb{R}^n$ is the metric topology for the standard distance $d(x,y) = \sqrt{\sum (x_i - y_i)^2}$. This is the topology you have been working with throughout ordinary analysis.

### The Discrete and Indiscrete Topologies

On any set $X$, there are two extremes:

- The *discrete topology*: $\tau = \mathcal{P}(X)$, every subset is open. This corresponds to the metric topology for the discrete metric $d(x,y) = 1$ if $x \neq y$. In the discrete topology, every function $f : X \to Y$ (to any topological space $Y$) is continuous, because preimages of open sets are always subsets of $X$, hence open.

- The *indiscrete* (or *trivial*) topology: $\tau = \{\emptyset, X\}$, as few open sets as possible. The only continuous functions $f : Y \to X$ from a Hausdorff space $Y$ into an indiscrete $X$ are constant functions. The indiscrete topology makes every function $f : X \to Y$ continuous.

The discrete topology is the *finest* (most open sets); the indiscrete is the *coarsest* (fewest open sets). Any topology on $X$ lies between these two extremes.

### The Sierpiński Space

The set $\{0, 1\}$ with topology $\tau = \{\emptyset, \{1\}, \{0,1\}\}$ is the *Sierpiński space*. It is the simplest non-trivial example of a topological space that is not Hausdorff. Its importance far exceeds its size: a continuous function $f : Y \to \{0,1\}$ (into the Sierpiński space) corresponds exactly to an open subset of $Y$, namely $f^{-1}(\{1\})$. The Sierpiński space *classifies open sets*: it is the subobject classifier in the category of topological spaces. This makes precise a connection between logic (propositions as $\{0,1\}$) and topology (predicates as open sets) that runs deep in domain theory and synthetic topology.

### The Sorgenfrey Line

The *Sorgenfrey line* (also called the lower limit topology or $\mathbb{R}_\ell$) is the real line with the topology generated by half-open intervals $[a, b)$ for $a < b$. This topology is strictly finer than the standard topology: every interval $(a,b)$ is open in $\mathbb{R}_\ell$ (write $(a,b) = \bigcup_{n=1}^\infty [a + 1/n, b)$), but $[a,b)$ is open in $\mathbb{R}_\ell$ and not open in $\mathbb{R}$.

The Sorgenfrey line is a remarkable source of counterexamples. It is first-countable, separable, and Lindelöf, but the Sorgenfrey plane $\mathbb{R}_\ell \times \mathbb{R}_\ell$ is not Lindelöf. It is regular but not second-countable. It serves as a touchstone for properties that hold in metric spaces but not in general topological spaces.

## Neighborhood Systems and Closure

A *neighborhood* of a point $x \in X$ is any open set containing $x$ (in some formulations, any set containing an open set containing $x$). The neighborhood filter $\mathcal{N}(x) = \{U \in \tau : x \in U\}$ encodes all local information about $x$.

Given a subset $A \subseteq X$:
- The *interior* $A^\circ = \text{int}(A)$ is the largest open set contained in $A$: the union of all open subsets of $A$.
- The *closure* $\overline{A} = \text{cl}(A)$ is the smallest closed set containing $A$: the intersection of all closed sets containing $A$.
- The *boundary* $\partial A = \overline{A} \setminus A^\circ$.

Equivalently: $x \in \overline{A}$ if and only if every open set containing $x$ meets $A$. The closure operator satisfies the Kuratowski axioms: $\overline{\emptyset} = \emptyset$; $A \subseteq \overline{A}$; $\overline{\overline{A}} = \overline{A}$; $\overline{A \cup B} = \overline{A} \cup \overline{B}$.

## Bases for Topologies

In practice, topologies are rarely described by listing all open sets — there are too many. Instead, one specifies a *basis*.

**Definition.** A *basis* for a topology on $X$ is a collection $\mathcal{B} \subseteq \mathcal{P}(X)$ satisfying:
1. Every point $x \in X$ is in some $B \in \mathcal{B}$.
2. If $x \in B_1 \cap B_2$ for $B_1, B_2 \in \mathcal{B}$, there exists $B_3 \in \mathcal{B}$ with $x \in B_3 \subseteq B_1 \cap B_2$.

The topology generated by $\mathcal{B}$ consists of all arbitrary unions of elements of $\mathcal{B}$. Examples:
- Open intervals $(a, b)$ form a basis for the standard topology on $\mathbb{R}$.
- Open balls $B(x, \varepsilon)$ form a basis for the metric topology on any metric space.
- Products $U \times V$ with $U$ open in $X$ and $V$ open in $Y$ form a basis for the product topology on $X \times Y$.

## The Fundamental Constructions

Every interesting topological space is built from simpler ones by three operations:

**Subspace topology.** Given $(X, \tau)$ and $A \subseteq X$, the subspace topology on $A$ is $\tau_A = \{U \cap A : U \in \tau\}$. A subset $V \subseteq A$ is open in $A$ if and only if $V = U \cap A$ for some open $U \subseteq X$. The subspace topology is the coarsest topology making the inclusion $A \hookrightarrow X$ continuous.

**Product topology.** Given $(X, \tau_X)$ and $(Y, \tau_Y)$, the product topology on $X \times Y$ is generated by the basis $\{U \times V : U \in \tau_X, V \in \tau_Y\}$. It is the coarsest topology making both projections $\pi_1 : X \times Y \to X$ and $\pi_2 : X \times Y \to Y$ continuous. This characterizes the product topology by a universal property: it is the categorical product in **Top**.

**Quotient topology.** Given $(X, \tau_X)$ and a surjection $q : X \twoheadrightarrow Y$, the quotient topology on $Y$ is $\tau_Y = \{V \subseteq Y : q^{-1}(V) \in \tau_X\}$. A set in $Y$ is open if and only if its preimage under $q$ is open. This is the finest topology making $q$ continuous. Universal property: a function $f : Y \to Z$ is continuous if and only if $f \circ q : X \to Z$ is continuous.

## The Metric Topology as Special Case

The metric topology is the natural topology induced by a distance function. But not every topology is a metric topology. Metrizable spaces (those whose topology arises from some metric) form a well-studied class; the Urysohn metrization theorem characterizes them as regular second-countable spaces. Non-metrizable spaces abound: the Zariski topology is non-metrizable (its closed sets are too sparse), the Scott topology of domain theory is non-Hausdorff, and the Sorgenfrey plane is regular but not metrizable.

The distinction matters for HoTT: the homotopy-theoretic content of a space is captured by its topology, not its specific metric. Two different metrics on the same set can give the same topology (e.g., the Euclidean metric and the taxicab metric on $\mathbb{R}^n$) and hence the same continuous maps, the same homotopy groups, the same identity types. Metrization is a red herring for homotopy theory.

## Why Open Sets Are the Right Abstraction

Open sets axiomatize exactly the structure needed to define continuity, compactness, connectedness, and the rest of the topological menagerie. Why not use something else — convergence, for instance, or neighborhood filters?

The answer is that open sets are *equivalent* to neighborhood filters (Hausdorff's original formulation), to closure operators satisfying the Kuratowski axioms, and to convergence satisfying appropriate axioms. These are all different ways of presenting the same structure. The open-set formulation wins on grounds of economy: three axioms, directly checkable, generating a rich theory.

And from the perspective of HoTT: the analogue of the open-set axioms in type theory is the structure of *propositions* in a topos. The opens of a space correspond to propositional subtypes. The topology of a type — its "shape" — is encoded in its identity types, not in any explicit collection of open subsets. The open-set axioms are the classical precursor to the univalent structure that HoTT makes synthetic.
