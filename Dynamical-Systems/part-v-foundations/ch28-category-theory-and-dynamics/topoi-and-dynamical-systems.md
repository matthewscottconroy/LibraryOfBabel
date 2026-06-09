# 28.3 Topoi and Dynamical Systems

A topos is a category that behaves like the category of sets — it has products, exponentials, and a "subobject classifier" that plays the role of $\{0,1\}$. The power of topos theory is that it allows you to do mathematics internally, in a language that makes sense in any topos, and then transport the results between different categorical contexts.

For dynamics, the relevant topos is surprisingly natural: the category of $G$-sets for a group $G$ (or more specifically, $\mathbb{Z}$-sets, which are just sets with a distinguished automorphism). Dynamical systems live in this topos, and the internal logic of the topos knows about time.

## 28.3.1 The Topos of a Group Action

**Definition 28.3.1.** For a topological group $G$ acting on a set $X$, the *topos of $G$-sets* $\mathbf{Set}^G$ consists of sets with $G$-action. A "generalized dynamical system" is an object in this topos.

**Theorem 28.3.2 (Lawvere).** The category $\mathbf{Set}^{\mathbb{Z}}$ (sets with a $\mathbb{Z}$-action, i.e., sets with an automorphism) is a topos. Dynamical systems (in the topological sense) correspond to sheaves on this topos.

The key insight here is that the internal logic of $\mathbf{Set}^{\mathbb{Z}}$ is *temporal*: to say that a property $P$ holds "at time $n$" is to evaluate it in the copy of the topos at time $n$. The modal operators "eventually $P$" and "always $P$" arise naturally from the topos-internal logic.

**Internal Logic:** The internal language of the topos $\mathbf{Set}^{\mathbb{Z}}$ is intuitionistic — propositions like "eventually $P$ holds" and "always $P$ holds" have different truth values, corresponding to temporal logic.

This is not just a curiosity. The fact that the internal logic is intuitionistic means that proofs in this logic automatically carry computational content — they are proofs that can be run as programs. The connection back to Chapter 27 (computability) is direct: the intuitionistic logic of the dynamics topos corresponds to computable aspects of the dynamical system.

## 28.3.2 The Stone Space Functor

Boolean algebras and compact totally disconnected spaces are two sides of the same coin, via Stone duality. In the presence of a dynamical system, this duality translates between topological and algebraic descriptions.

**Definition 28.3.3.** The *Stone space* of a Boolean algebra $B$ is the compact totally disconnected space $\text{Stone}(B)$ of ultrafilters on $B$. A dynamical system $(X, f)$ with $X$ a Stone space corresponds to a Boolean algebra $B(X)$ with an automorphism $f^*$.

**Theorem 28.3.4 (Stone Duality for Dynamics).** The functor $X \mapsto C(X, \mathbb{Z}/2\mathbb{Z})$ (continuous functions to $\{0,1\}$) gives a contravariant equivalence:
$$\{\text{0-dim compact systems}\}^{op} \simeq \{\text{Boolean algebras with automorphism}\}.$$

Under this duality, subshifts correspond to finitely generated Boolean algebras with automorphism — exactly the sofic shifts.

This is a beautiful result. The sofic shifts (Chapter 12) — those defined by forbidden patterns in a labeled graph — turn out to be exactly the systems whose Boolean algebra of clopen sets is finitely generated as an algebra with automorphism. The combinatorial condition (finitely many forbidden words) is equivalent to the algebraic condition (finite generation). Stone duality is making this precise.

Next, we'll see one of the most surprising categorical results in the whole book: that entropy is a functor.
