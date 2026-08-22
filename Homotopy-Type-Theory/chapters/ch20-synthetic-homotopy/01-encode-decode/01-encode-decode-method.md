# 1.1 The Encode-Decode Method

## The Problem

Given a type $X$ with a basepoint $x_0 : X$, we want to understand the loop space:
$$\Omega(X, x_0) :\equiv (x_0 =_X x_0)$$

This is a type whose elements are loops at $x_0$ — paths from $x_0$ back to itself. The fundamental group $\pi_1(X, x_0) = \|\Omega(X, x_0)\|_0$ is the set of connected components of the loop space.

For ordinary types (like $\mathbb{N}$, $\mathsf{Bool}$, products), the loop space is easy to compute: these are sets, so all loops are $\mathsf{refl}$. But for HITs (like $S^1$, $S^2$, $K(G,1)$), the loop space can be non-trivial and interesting.

**The challenge:** The type $x_0 = x_0$ is defined in terms of the identity type, not in terms of the specific structure of $X$. How do we compute it?

**The encode-decode strategy:** Find another type $G$ that we understand and show $\Omega(X, x_0) \simeq G$. The trick is that $G$ should be simpler — ideally, a familiar algebraic structure like $\mathbb{Z}$ or a group $G$.

## The Method, Abstractly

Here's the encode-decode method in its most general form:

**Setup:** We want to show $\Omega(X, x_0) \simeq G$ for some type $G$.

**Step 1 (Code).** Define a type family:
$$\mathsf{code} : X \to \mathsf{Type}$$

such that $\mathsf{code}(x_0) :\equiv G$. The family $\mathsf{code}$ should be defined using the HIT structure of $X$ — i.e., using the non-dependent eliminator (or recursor) of $X$.

The key invariant: $\mathsf{code}(x)$ is the "group of paths from $x_0$ to $x$, encoded in $G$."

**Step 2 (Encode).** Define:
$$\mathsf{encode} : \prod_{x:X}\, (x_0 = x) \to \mathsf{code}(x)$$

by transport: $\mathsf{encode}(x, p) :\equiv \mathsf{transport}^\mathsf{code}(p, e)$ where $e : \mathsf{code}(x_0) = G$ is the "identity element" or "starting point" in $G$.

Concretely: $\mathsf{encode}(x, p)$ transports the "identity element" of $G$ along the path $p$ in $X$, using the action of $X$-paths on $G$ defined by $\mathsf{code}$.

**Step 3 (Decode).** Define:
$$\mathsf{decode} : \prod_{x:X}\, \mathsf{code}(x) \to (x_0 = x)$$

using the HIT eliminator of $X$. At the basepoint, $\mathsf{decode}(x_0) : G \to (x_0 = x_0)$ sends each element $g \in G$ to the corresponding loop.

**Step 4 (Inverse).** Show the two composites are homotopic to the identity:
- $\mathsf{encode}(x, \mathsf{decode}(x, c)) = c$ for all $c : \mathsf{code}(x)$
- $\mathsf{decode}(x, \mathsf{encode}(x, p)) = p$ for all $p : x_0 = x$

This proves $\mathsf{encode}$ and $\mathsf{decode}$ are inverses, hence equivalences.

**Conclusion.** $(x_0 = x) \simeq \mathsf{code}(x)$ for all $x$. At $x = x_0$: $\Omega(X, x_0) \simeq G$. So $\pi_1(X, x_0) \simeq \|G\|_0$ (the set of connected components of $G$).

## Why Transport Is the Key

The encode function uses transport:
$$\mathsf{encode}(x, p) :\equiv \mathsf{transport}^\mathsf{code}(p, \mathsf{id})$$

Why transport? Because transport is how paths in $X$ act on elements of fibers (i.e., elements of $\mathsf{code}(x)$). The path $p : x_0 = x$ acts on $\mathsf{code}(x_0) = G$ by transporting along $p$, giving an element of $\mathsf{code}(x)$.

When $x = x_0$ (loops), transport gives an action of $\Omega(X, x_0)$ on $G$:
$$\mathsf{transport}^\mathsf{code}(\ell) : G \to G \quad \text{for each loop } \ell : x_0 = x_0$$

If this action is "the group operation" on $G$ (multiplication by the group element corresponding to $\ell$), then transport is doing exactly what encode should do.

## The Role of Univalence

Often, the code family $\mathsf{code} : X \to \mathsf{Type}$ is defined using Univalence. For a HIT $X$ with a path constructor $\ell : x = x'$, we need to specify $\mathsf{transport}^\mathsf{code}(\ell) : \mathsf{code}(x) \to \mathsf{code}(x')$, which is an equivalence.

By Univalence, specifying this equivalence is the same as specifying a path $\mathsf{code}(x) = \mathsf{code}(x')$ in the universe. So the HIT eliminator for $X$ (into $\mathsf{Type}$) takes as input:
- A type at each point constructor
- A path in $\mathsf{Type}$ (i.e., an equivalence) at each path constructor

This is how Univalence enters the encode-decode method: it's what allows the code family to "use" the path constructors of the HIT.

## The Circle: A Preview

For $X = S^1$ and $G = \mathbb{Z}$:
- $\mathsf{code}(\mathsf{base}) :\equiv \mathbb{Z}$
- $\mathsf{transport}^\mathsf{code}(\mathsf{loop}) :\equiv \mathsf{succ}_{\mathbb{Z}}$ (the successor equivalence, viewed as a path $\mathbb{Z} = \mathbb{Z}$ via Univalence)

The encode function sends a loop $p : \mathsf{base} = \mathsf{base}$ to the integer "how many times $p$ goes around $\mathsf{loop}$" — the winding number. Transporting 0 along $p$ increments by 1 for each traversal of $\mathsf{loop}$.

The decode function sends an integer $n$ to $\mathsf{loop}^n$ — the $n$-fold concatenation of $\mathsf{loop}$ (with $\mathsf{loop}^0 = \mathsf{refl}$ and $\mathsf{loop}^{-1} = \mathsf{loop}^{-1}$).

## Connectivity and Encode-Decode

The encode-decode method has a key limitation: it works best for the *path space* $x_0 = x$, not for arbitrary identity types. To compute higher homotopy groups $\pi_n$ for $n \geq 2$, we apply the method to the loop space $\Omega^{n-1}(X, x_0)$ instead of $X$ itself.

**Higher encode-decode:**
- For $\pi_2(X)$: apply encode-decode to $\Omega(X, x_0)$ to compute $\Omega^2(X, x_0)$
- For $\pi_3(X)$: apply to $\Omega^2(X, x_0)$
- etc.

Each application requires a new code family, designed for the specific space at hand.

**The Freudenthal suspension theorem** systematizes this: it gives conditions under which $\pi_k(X) \cong \pi_{k+1}(\Sigma X)$, allowing you to use the encode-decode method at one level to compute the next.

## Naturality

The encode and decode maps are *natural* in $x$: for any path $q : x = x'$, the square:

$$\begin{array}{ccc}
(x_0 = x) & \xrightarrow{\mathsf{encode}(x)} & \mathsf{code}(x) \\
(q \circ -) \downarrow & & \downarrow \mathsf{transport}^\mathsf{code}(q) \\
(x_0 = x') & \xrightarrow{\mathsf{encode}(x')} & \mathsf{code}(x')
\end{array}$$

commutes. Similarly for decode. This naturality is essential for proving that the encode-decode maps are inverses.

**Proof of naturality.** By path induction (J) on $q$. The base case $q = \mathsf{refl}_x$ is trivial. $\square$

## Summary

The encode-decode method is:
1. **Find** a code family $\mathsf{code} : X \to \mathsf{Type}$ with $\mathsf{code}(x_0) = G$
2. **Define** $\mathsf{encode}$ by transport, $\mathsf{decode}$ by HIT elimination
3. **Verify** the round-trips using HIT elimination and computation rules
4. **Conclude** $(x_0 = x) \simeq \mathsf{code}(x)$

The beauty is that this method reduces hard homotopy-theoretic questions (what are all the loops in $X$?) to type-theoretic calculations (can we show these two maps are inverses?). And the type theory's computation rules make these calculations manageable.

The method was introduced by Voevodsky for the $\pi_1(S^1)$ computation and has since been applied to compute $\pi_2(S^2)$, $\pi_3(S^2)$ (via the Hopf fibration), and $\pi_4(S^3)$ (Brunerie's theorem). It is the most powerful single technique in synthetic homotopy theory.
