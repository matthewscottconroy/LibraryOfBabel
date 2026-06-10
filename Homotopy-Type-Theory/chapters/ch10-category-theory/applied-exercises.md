# Applied Exercises

Category theory is often described as "abstract nonsense" — a pejorative that has become a badge of honor. Its power comes from operating at a level of abstraction where the same argument works simultaneously in many different mathematical contexts. The exercises below make this concrete: they ask you to apply categorical concepts to functional programming (monads, functors), database theory (schemas as categories), universal algebra (Lawvere theories), order theory (functors between preorders), and data structure design (presheaves as versioned data). In each case, the categorical framework reveals structure that would be obscured by a more elementary treatment.

---

## Exercise B.1: Functors and Natural Transformations in Haskell
*Domain: Functional Programming / Type Theory*

**Setup:** In Haskell, the `Functor` typeclass models functors from the category **Hask** (whose objects are Haskell types and morphisms are Haskell functions) to itself. A `Functor f` instance provides `fmap :: (a -> b) -> f a -> f b`, which must satisfy the functor laws: `fmap id = id` and `fmap (g . h) = fmap g . fmap h`. A natural transformation between functors `f` and `g` is a polymorphic function `nat :: forall a. f a -> g a` that commutes with `fmap`.

**Questions:**

1. **Verify the functor laws** for `Maybe`, `[]` (list), and `Either e` (for a fixed type `e`). For each, write out what `fmap id = id` and `fmap (g . h) = fmap g . fmap h` say concretely. Are these provable by computation alone (i.e., definitional equalities in Haskell's type checker), or do they require reasoning about program behavior?

2. **Natural transformations.** The functions `safeHead :: [a] -> Maybe a` and `maybeToList :: Maybe a -> [a]` are natural transformations. Verify the naturality square: for any function `f :: a -> b`, `fmap f . safeHead = safeHead . fmap f` (and similarly for `maybeToList`). Write out the naturality square as a commutative diagram, labeling the objects (types) and morphisms (functions).

3. **The category of functors.** Natural transformations between `Functor` instances compose: if `alpha :: forall a. f a -> g a` and `beta :: forall a. g a -> h a`, then `beta . alpha :: forall a. f a -> h a` is a natural transformation. Show that this composition is associative and has identity natural transformations (the `id` function, interpreted polymorphically). Conclude that Haskell `Functor`s and their natural transformations form a category.

4. (Extension) The functor `(->) r` (functions from a fixed type `r`) is both a `Functor` and a `Monad`. Show that it is also a `Comonad` (with `extract :: (r -> a) -> a` and `duplicate :: (r -> a) -> r -> r -> a`). In categorical terms, `(->) r` is the functor part of the product-exponential adjunction; identify the unit and counit of this adjunction.

*Abstract concept illustrated: Functors and natural transformations (Sections 1 and 2); the functor category; adjunctions via the product-exponential.*

---

## Exercise B.2: Monads as Monoids in the Category of Endofunctors
*Domain: Functional Programming / Abstract Algebra*

**Setup:** A monad on a category $\mathcal{C}$ is a monoid in the monoidal category of endofunctors $\mathcal{C} \to \mathcal{C}$, where the monoidal product is functor composition. Concretely, a monad is a triple $(T, \eta, \mu)$ where $T : \mathcal{C} \to \mathcal{C}$ is an endofunctor, $\eta : \mathsf{Id} \Rightarrow T$ is the unit (a natural transformation), and $\mu : T \circ T \Rightarrow T$ is the multiplication (join), satisfying the monad laws: $\mu \circ T\eta = \mathsf{id}_T = \mu \circ \eta T$ and $\mu \circ T\mu = \mu \circ \mu T$.

In Haskell, a monad is given by `return :: a -> m a` (the unit $\eta$) and `(>>=) :: m a -> (a -> m b) -> m b` (bind, from which join is `join :: m (m a) -> m a`, `join = (>>= id)`).

**Questions:**

1. **The list monad.** For the list monad `[]`, identify $T$, $\eta$ (`return`), and $\mu$ (`join`). What do the monad laws say concretely? Verify that `join . return = id`, `join . fmap return = id`, and `join . join = join . fmap join` for lists, by computing both sides on a concrete example like `[[1,2],[3]]`.

2. **Every adjunction gives a monad.** Given the free-forgetful adjunction $F \dashv U : \mathbf{Mon} \to \mathbf{Set}$ between the category of monoids and sets, the composite $UF : \mathbf{Set} \to \mathbf{Set}$ is a monad. Show that $UF(S)$ is the free monoid on $S$ (the list type `[S]`), that the unit $\eta_S : S \to UF(S)$ is `\x -> [x]` (singleton list), and that the multiplication $\mu_S : UF(UF(S)) \to UF(S)$ is `concat` (list of lists to list). Verify that the monad laws hold.

3. **The state monad.** The state monad `State s a = s -> (a, s)` models stateful computation. Show that it arises from the adjunction $(-) \times s \dashv (-)^s$ (the product-exponential adjunction in $\mathbf{Set}$). Identify the unit and counit of this adjunction, and show how they yield the `return` and `join` of the state monad.

4. (Extension) The Kleisli category of a monad $(T, \eta, \mu)$ has the same objects as $\mathcal{C}$ but morphisms $A \to_{\mathbf{Kl}} B$ are maps $A \to TB$ in $\mathcal{C}$. In Haskell, these are "Kleisli arrows" `a -> m b`. Show that the Kleisli composition `(>=>)` (fish operator: `(f >=> g) x = f x >>= g`) makes Kleisli arrows into a category. Show that every monad gives a functor from the Kleisli category back to $\mathcal{C}$, given by the Eilenberg-Moore resolution.

*Abstract concept illustrated: Monads (Section 6); adjunctions give monads; the Kleisli construction.*

---

## Exercise B.3: Database Schemas as Categories
*Domain: Database Theory / Information Architecture*

**Setup:** David Spivak observed that a relational database schema can be modeled as a category: objects are tables, morphisms are foreign-key relationships, and the fact that foreign keys compose (if `Orders.CustomerID` points to `Customers.CustomerID` and `Customers.RegionID` points to `Regions.RegionID`, then we have a composed morphism from `Orders` to `Regions`) means that schemas naturally form categories. A database *instance* (the actual data) is a functor $I : \mathbf{Schema} \to \mathbf{Set}$ assigning to each table a set of rows, and to each foreign key a function between those sets. A morphism of database instances is a natural transformation.

**Questions:**

1. **A simple schema.** Consider a schema with three tables: `Department`, `Employee`, `Project`. There are foreign keys: `Employee.DeptID → Department`, `Project.LeadID → Employee`, and `Project.DeptID → Department`. Draw this as a category (nodes = tables, arrows = foreign keys). Write out all composites that exist (including identities). Is this category freely generated, or does it have non-trivial equations?

2. **A database instance.** Define a concrete instance of the above schema: assign sets $I(\text{Department}) = \{d_1, d_2\}$, $I(\text{Employee}) = \{e_1, e_2, e_3\}$, $I(\text{Project}) = \{p_1, p_2\}$, and functions for each foreign key. Verify that your instance satisfies the natural transformation condition: if two paths of morphisms in the schema have the same source and target, the corresponding functions must compose to the same result (this is the *commutativity constraint* for the functor).

3. **Schema morphisms and data migration.** A morphism of schemas $F : \mathbf{S}_1 \to \mathbf{S}_2$ (a functor between the schema categories) induces three operations on instances: pullback $F^*$ (restricting an $\mathbf{S}_2$-instance to an $\mathbf{S}_1$-instance), and left and right adjoints $F_!$ and $F_*$ (extending an $\mathbf{S}_1$-instance to an $\mathbf{S}_2$-instance). The triple $(F_! \dashv F^* \dashv F_*)$ is the data migration adjunction. Construct a small example: define schemas $\mathbf{S}_1$ (one table `Person` with a self-foreign-key `parentOf`) and $\mathbf{S}_2$ (two tables `Adult` and `Child` with a foreign key `Child.parentID → Adult`), a functor $F : \mathbf{S}_1 \to \mathbf{S}_2$ (map both tables to `Person`), and compute $F^*$ of a specific $\mathbf{S}_2$-instance.

4. (Extension) The category of all schemas (categories) and all schema morphisms (functors) is the category $\mathbf{Cat}$. The Yoneda lemma applied to $\mathbf{Cat}$ says that a schema $\mathbf{S}$ is determined up to equivalence by the functor it represents: the category $[\mathbf{S}, \mathbf{Set}]$ of all instances. Show that two schemas $\mathbf{S}_1 \simeq \mathbf{S}_2$ are equivalent if and only if their instance categories $[\mathbf{S}_1, \mathbf{Set}] \simeq [\mathbf{S}_2, \mathbf{Set}]$ are equivalent. This is a direct application of the Yoneda lemma (via the Morita equivalence theorem for categories).

*Abstract concept illustrated: Categories as abstract structures; functors as models/instances; natural transformations as instance morphisms; adjunctions as data migration.*

---

## Exercise B.4: The Free-Forgetful Adjunction for Groups and Lists
*Domain: Abstract Algebra / Functional Programming*

**Setup:** One of the most important patterns in mathematics and programming is the free-forgetful adjunction: for any algebraic structure (group, monoid, ring, vector space), there is a "free" construction that builds the most general structure on a given set, and a "forgetful" functor that discards the structure. The adjunction $F \dashv U$ (free functor left adjoint to forgetful functor) captures the universal property of free objects.

**Questions:**

1. **Free groups.** The free group $F(S)$ on a set $S$ is the group whose elements are words in the alphabet $S \cup S^{-1}$, with the group operation being concatenation and reduction. Verify that $F$ is a functor $\mathbf{Set} \to \mathbf{Grp}$: for a function $f : S \to T$, define $F(f) : F(S) \to F(T)$ and check functoriality. Show that the universal property of $F(S)$ is exactly the unit-counit adjunction: $\mathsf{Hom}_{\mathbf{Grp}}(F(S), G) \cong \mathsf{Hom}_{\mathbf{Set}}(S, U(G))$, where the bijection sends a group homomorphism $\phi : F(S) \to G$ to the function $s \mapsto \phi(s)$.

2. **Lists as the free monoid.** Show that the list type `[a]` in Haskell is the free monoid on the type `a`: it satisfies the universal property that any function `f :: a -> m` where `m` is a monoid extends uniquely to a monoid homomorphism `foldMap f :: [a] -> m`. Write this as an adjunction $L \dashv U : \mathbf{Mon}_{\mathbf{Hask}} \to \mathbf{Hask}$ (between Haskell monoids and Haskell types). Identify the unit $\eta_a : a \to [a]$ as `singleton` and the counit $\epsilon_M : [M] \to M$ (for a monoid `M`) as `mconcat`.

3. **The triangle identities.** Every adjunction $F \dashv U$ satisfies the triangle identities: $U\epsilon \circ \eta U = \mathsf{id}_U$ and $\epsilon F \circ F\eta = \mathsf{id}_F$. Work out what these say concretely for the free-group adjunction: $U\epsilon_{F(S)} \circ \eta_{U(F(S))} = \mathsf{id}_{U(F(S))}$ says that applying the unit and then the counit-applied-to-forgetful is the identity. Verify this by computing both sides on a specific word in $F(S)$.

4. (Extension) The monad associated to the free-forgetful adjunction for groups is the monad $T = UF : \mathbf{Set} \to \mathbf{Set}$. Show that $T(S)$ consists of reduced words over $S \cup S^{-1}$, that the unit $\eta_S$ is the inclusion of generators, and that the multiplication $\mu_S : T(T(S)) \to T(S)$ is concatenation-and-reduction of words-of-words. Verify the monad laws. This monad is sometimes called the "group monad."

*Abstract concept illustrated: Adjunctions (Section 5); the free-forgetful pattern; the unit-counit formulation; the monad of an adjunction.*

---

## Exercise B.5: Functors Between Preorders as Monotone Functions
*Domain: Order Theory / Logic*

**Setup:** A preorder $(P, \leq)$ is a category whose objects are elements of $P$ and with at most one morphism $x \to y$ (present if $x \leq y$, absent otherwise). A functor between preorders is exactly a monotone function. This makes order theory a special case of category theory, and categorical concepts — limits, colimits, adjunctions — specialize to familiar order-theoretic concepts.

**Questions:**

1. **Categorical structure of preorders.** Show that a product $x \times y$ in a preorder category is the meet (greatest lower bound) $x \wedge y$, and a coproduct $x + y$ is the join (least upper bound) $x \vee y$. A terminal object is a top element $\top$, an initial object is a bottom element $\bot$. Verify each claim by checking the relevant universal properties.

2. **Galois connections as adjunctions.** A Galois connection between posets $(P, \leq_P)$ and $(Q, \leq_Q)$ is a pair of monotone functions $f : P \to Q$ and $g : Q \to P$ such that $f(p) \leq_Q q \iff p \leq_P g(q)$. Show that a Galois connection is exactly an adjunction $f \dashv g$ in the category of preorders. Work out the example: let $P = Q = \mathcal{P}(\{1,2,3\})$ (power set ordered by inclusion), $f(S) = S$ (identity), $g(S) = S$ — this is the trivial case. For a nontrivial example, let $P = \mathbb{Z}$, $Q = \mathbb{R}$, $f = \lceil - \rceil$ (ceiling) and $g = \lfloor - \rfloor$ (floor), and verify the adjunction.

3. **The logic of a Heyting algebra.** An intuitionistic propositional logic is modeled by a Heyting algebra: a poset with meet, join, bottom, and an implication $\Rightarrow$ such that $a \wedge b \leq c \iff a \leq b \Rightarrow c$ (i.e., $(-) \wedge b \dashv b \Rightarrow (-)$). This is the adjunction that models currying in a CCC restricted to a preorder. Verify that the Heyting algebra $\{0, 1\}$ (classical logic, ordered by $0 \leq 1$) satisfies all the axioms, and that the set of open subsets of a topological space (ordered by inclusion) forms a Heyting algebra where $U \Rightarrow V$ is the interior of $(U^c \cup V)$.

4. (Extension) The quantifiers $\forall$ and $\exists$ in first-order logic are, respectively, the right and left adjoints of the substitution functor $f^* : \mathsf{Sub}(Y) \to \mathsf{Sub}(X)$ (restricting predicates along $f : X \to Y$). Work out this adjunction explicitly in the case where $X = A \times B$ and $Y = A$ (the projection $\pi : A \times B \to A$): show that $\exists_\pi$ gives $\exists_{b:B} P(a, b)$ and $\forall_\pi$ gives $\forall_{b:B} P(a, b)$, and verify the adjunction bijection $P \leq_{\mathsf{Sub}(A \times B)} f^*(Q) \iff \exists_\pi P \leq_{\mathsf{Sub}(A)} Q$ for specific small sets $A$ and $B$.

*Abstract concept illustrated: Categories as general structures; limits and colimits; adjunctions; the connection between logic and adjunctions (Lawvere's thesis).*

---

## Exercise B.6: Presheaves as Versioned Data Structures
*Domain: Software Engineering / Distributed Systems*

**Setup:** A presheaf on a category $\mathcal{C}$ is a functor $F : \mathcal{C}^{op} \to \mathbf{Set}$. When $\mathcal{C}$ is a poset, a presheaf is a system of data sets indexed by the poset, with restriction maps going "downward." This is precisely the structure of version-controlled data: the poset represents a version history, the data at each version is the set of facts that hold at that version, and morphisms represent "forgetting" or "restriction" as you move to an earlier version.

**Questions:**

1. **A version history as a category.** Let $\mathcal{C}$ be the poset $\{v_1 \leq v_2 \leq v_3\}$ representing three versions of a codebase. Define a presheaf $F : \mathcal{C}^{op} \to \mathbf{Set}$ where $F(v_i)$ is the set of files that exist at version $v_i$, and restriction maps $F(v_j \to v_i) : F(v_j) \to F(v_i)$ (for $v_i \leq v_j$) represent the "file existed in the earlier version" relationship. Write out a concrete example with a few files, and verify that $F$ satisfies the functoriality conditions: $F(\mathsf{id}) = \mathsf{id}$ and $F(g \circ f) = F(f) \circ F(g)$.

2. **Natural transformations as version migrations.** If $F$ and $G$ are two presheaves on $\mathcal{C}$ (two "data systems" over the same version history), a natural transformation $\alpha : F \Rightarrow G$ is a "version-compatible migration": for each version $v$, a function $\alpha_v : F(v) \to G(v)$, such that migrations commute with restrictions. Define a second presheaf $G$ on the same version history from Question 1 (perhaps tracking test results rather than files), and define a natural transformation from $F$ to $G$ (mapping files to their test results). Verify the naturality square.

3. **The Yoneda Lemma for version data.** Fix a version $v_k \in \mathcal{C}$. The representable presheaf $\mathsf{Hom}(-, v_k) : \mathcal{C}^{op} \to \mathbf{Set}$ sends each version $v_i$ to the set of morphisms from $v_i$ to $v_k$ in $\mathcal{C}$ — in the poset, this is a one-element set if $v_i \leq v_k$ and empty otherwise. The Yoneda lemma says that natural transformations $\mathsf{Hom}(-, v_k) \Rightarrow F$ are in bijection with elements of $F(v_k)$. Work out this bijection concretely: given an element $x \in F(v_k)$ (a datum that exists at version $v_k$), construct the corresponding natural transformation, and verify it is natural.

4. (Extension) In a distributed version control system (like Git), the version history is not a linear poset but a DAG (directed acyclic graph), which can be modeled as a category with morphisms corresponding to "commit reachability." A merge commit corresponds to a limit (specifically a pullback) in this category. Sketch how to model a Git repository as a presheaf on a commit DAG, with the restriction maps corresponding to "what you can see from a given commit." What does the sheaf condition (as a constraint on presheaves) correspond to in this data model?

*Abstract concept illustrated: Presheaves as set-valued functors on $\mathcal{C}^{op}$; the Yoneda lemma; limits as data merge operations; the density theorem.*
