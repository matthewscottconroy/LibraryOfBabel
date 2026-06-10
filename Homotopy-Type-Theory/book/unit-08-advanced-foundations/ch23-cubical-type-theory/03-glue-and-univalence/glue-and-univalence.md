# 23.3 The Glue Type and Univalence as a Theorem

## The Last Missing Piece

With `hcomp` and `transp` defined for function types, $\Sigma$-types, $\Pi$-types, and inductive types, cubical type theory is nearly complete. Every type former can compose and transport. Almost every type former.

The universe $\mathsf{Type}$ itself remains. And composition in the universe is exactly what is needed to prove univalence.

The naive approach — say that composing a tube of types is their colimit or some abstract limit construction — fails because it doesn't give computation rules. You would have composition in the universe, but no way to evaluate the result.

The Glue type is the solution. Instead of composing types abstractly, we introduce a concrete new type former that *records* the data needed to compose in the universe. Once Glue is defined with explicit computation rules, composition in the universe is just Glue formation. And univalence follows as a theorem — not postulated, but derived.

## The Glue Type Constructor

The Glue type takes three pieces of data:
1. A *base type* $B : \mathsf{Type}$
2. A face formula $\phi$
3. A *partial type with equivalence*: a partial type $T : [\phi \vdash \mathsf{Type}]$ and a partial equivalence $e : [\phi \vdash T \simeq B]$

From this data, we form the Glue type:

$$\mathsf{Glue}[\phi \vdash (T, e)]\, B : \mathsf{Type}$$

**Intuition.** The Glue type is the result of "replacing $B$ with $T$ on the face $\phi$, via the equivalence $e$." When you are on the face $\phi$, the type you see is $T$ (connected to $B$ by $e$). When you are off the face, you see $B$. The type smoothly interpolates by recording that $T$ and $B$ are equivalent.

**The critical analogy.** The Glue type is to the universe what `hcomp` is to an ordinary type: it fills an open box in the universe when you have partial type data on a face.

## Introduction and Elimination Rules

**Introduction** (`glue`): To construct an element of $\mathsf{Glue}[\phi \vdash (T, e)]\, B$, provide:
- A partial element $t : [\phi \vdash T]$
- A base element $b : B$
- A coherence condition: $b = e(t)$ whenever $\phi = 1$

$$\mathsf{glue}[\phi \vdash t]\, b : \mathsf{Glue}[\phi \vdash (T, e)]\, B$$

**Elimination** (`unglue`): To extract the underlying $B$-element from a Glue element $g$:

$$\mathsf{unglue}[\phi \vdash e](g) : B$$

**Computation rules** (definitional):

When $\phi = 1$ (we are on the full face):
$$\mathsf{Glue}[1 \vdash (T, e)]\, B = T$$
$$\mathsf{glue}[1 \vdash t]\, b = t$$
$$\mathsf{unglue}[1 \vdash e](t) = e(t)$$

When $\phi = 0$ (we are off all faces):
$$\mathsf{Glue}[0 \vdash (\cdot, \cdot)]\, B = B$$
$$\mathsf{glue}[0 \vdash \cdot]\, b = b$$
$$\mathsf{unglue}[0 \vdash \cdot](b) = b$$

These are definitional equalities. The type checker applies them automatically. When $\phi = 1$, a Glue type literally *is* $T$ — not propositionally equal, not equivalent — the same type.

## Composition in the Universe via Glue

With the Glue type, we can define `hcomp` for the universe:

Given a tube of types $u : (j : \mathbb{I}) \to [\phi \vdash \mathsf{Type}]$ and a base type $A_0 : \mathsf{Type}$ with $A_0 = u(0)$ when $\phi = 1$, define:

$$\mathsf{hcomp}^{\mathsf{Type}}_\phi(u, A_0) :\equiv \mathsf{Glue}[\phi \vdash (u(1), e_\phi)]\, A_0'$$

where:
- $A_0'$ is the base type transported forward (it is $A_0$ when the tube is empty)
- $e_\phi : [\phi \vdash u(1) \simeq A_0']$ is the equivalence obtained by transporting along the tube from $j = 1$ back to $j = 0$, composed with the map to $A_0'$

The details require care, but the key point is that composition in the universe is *Glue formation*. We do not abstractly "take the colimit" — we form the Glue type with the specific equivalence data that the tube provides.

This closes the circle: `hcomp` is now defined for every type former, including the universe. The theory is complete.

## Proving Univalence

**Theorem (Univalence in CCHM).** For any types $A, B : \mathsf{Type}$:

$$\mathsf{idToEquiv} : (A =_{\mathsf{Type}} B) \xrightarrow{\;\;\simeq\;\;} (A \simeq B)$$

is an equivalence. Equivalently, there is a function $\mathsf{ua} : (A \simeq B) \to (A =_{\mathsf{Type}} B)$ that is a quasi-inverse to $\mathsf{idToEquiv}$.

**Proof — Constructing ua.**

Given an equivalence $e : A \simeq B$, define:

$$\mathsf{ua}(e) :\equiv \lambda i. \;\mathsf{Glue}\!\left[(i = 0) \mapsto (A, e),\; (i = 1) \mapsto (B, \mathsf{id}_B)\right] B : A =_{\mathsf{Type}} B$$

Check the endpoints:
- At $i = 0$: the formula $(i = 0)$ holds, so $\mathsf{Glue}[1 \vdash (A, e)]\, B = A$ definitionally. ✓
- At $i = 1$: the formula $(i = 1)$ holds, so $\mathsf{Glue}[1 \vdash (B, \mathsf{id})]\, B = B$ definitionally. ✓

So $\mathsf{ua}(e)$ is a path from $A$ to $B$ in the universe.

**The computation rule.**

$$\mathsf{transport}(\mathsf{ua}(e), a) = e(a)$$

This is the crucial property. Let's understand why it holds.

$\mathsf{transport}(\mathsf{ua}(e), a) = \mathsf{transp}^{\lambda i. \, \mathsf{Glue}[\ldots]\, B}(a)$

The computation rule for `transp` applied to a Glue type says: transport through a Glue type composed of an equivalence $e$ at $i = 0$ applies the forward direction of $e$ to produce an element of $B$. Working through the rules:

$$\mathsf{transp}^{\mathsf{Glue}[(i=0) \mapsto (A,e)]\, B}(a) = \mathsf{unglue}[\mathsf{id}](\mathsf{glue}[(i=1) \mapsto \mathsf{transp}^B_1(\mathsf{unglue}[e](a))]\,(\mathsf{unglue}[e](a)))$$

Simplifying at the endpoints using the Glue computation rules: at $i = 1$, $\mathsf{unglue}[e](a) = e(a)$, and since the type at $i = 1$ is $B$ with identity equivalence, the result is $e(a)$.

The Glue type is precisely designed so that transport through it applies the underlying equivalence. This is not a coincidence — the `transp` rule for Glue types is defined to do exactly this.

**Verifying ua is a quasi-inverse.**

$\mathsf{ua}(\mathsf{idToEquiv}(p)) = p$: By path induction (which is derivable), reduce to $p = \text{refl}$. Then $\mathsf{idToEquiv}(\text{refl}) = \mathsf{id}_A$, and $\mathsf{ua}(\mathsf{id}_A) = \lambda i. \, \mathsf{Glue}[(i=0) \mapsto (A, \mathsf{id}), (i=1) \mapsto (A, \mathsf{id})]\, A$. By the Glue computation rules, when the equivalence is identity, the Glue type is $A$ at both endpoints and the path reduces to $\lambda i. \, A = \text{refl}_A$. ✓

$\mathsf{idToEquiv}(\mathsf{ua}(e)) = e$: From the computation rule, $\mathsf{transport}(\mathsf{ua}(e), -) = e(-)$ as functions. By function extensionality (itself a theorem: `funExt h = λ i x → h x i`), these are equal as equivalences. ✓

So `idToEquiv` is an equivalence. Univalence is proved. $\square$

## The Contrast with Book HoTT

In Book HoTT, the situation is:

| Item | Book HoTT | CCHM Cubical |
|------|-----------|--------------|
| `ua(e) : A = B` | Axiom (no reduction rule) | Theorem (defined via Glue) |
| `transport(ua e, a) = e(a)` | Separate axiom | Definitional (from Glue computation rules) |
| Canonicity | Unknown | Theorem |
| Computation of `transport(ua e, a)` | Stuck | Runs to `e(a)` |

The philosophical difference: in Book HoTT, we *assert* that `ua` exists and behaves a certain way. In CCHM, we *define* `ua` using Glue and prove that it behaves correctly by computation. The axiom becomes a definition, and the computation rule becomes a consequence.

## Function Extensionality Is Also a Theorem

The simplicity of the cubical path type makes function extensionality definitional:

**Theorem (funext).** For $f, g : \Pi_{x:A} B(x)$ and $h : \Pi_{x:A} f(x) =_{B(x)} g(x)$:

$$\mathsf{funExt}(h) :\equiv \lambda i. \, \lambda x. \, h(x)(i) : f =_{\Pi_{x:A} B(x)} g$$

Check: $(\lambda i. \, \lambda x. \, h(x)(i))(0) = \lambda x. \, h(x)(0) = \lambda x. \, f(x) = f$ definitionally. At $1$: similarly $g$. The proof is three characters: swap the arguments.

And $\mathsf{funExt}(h)(i)(x) = h(x)(i)$ definitionally — the computation rule holds with no proof required.

## The Glue Type as a General Colimit Device

The Glue type has a natural semantic interpretation: it computes a *pullback* in the fibration category. Given partial type data $(T, e)$ over a face $\phi$ and a base type $B$, the Glue type is the type whose elements are:

- An element of $T$ (when on the face) 
- An element of $B$ (as the "base carrier")
- Coherence: the $T$-element corresponds to its image in $B$ under $e$

This is the limit of the diagram $T \xrightarrow{e} B \xleftarrow{\text{id}} B$ restricted appropriately. The Glue type computes this limit as a type former with explicit reduction rules, which is what distinguishes it from an abstract colimit and gives it computational content.

## Canonicity Revisited

The Glue type and the resulting proof of univalence complete the picture for canonicity.

**Theorem (Full Canonicity).** In CCHM cubical type theory: every closed term $t : \mathbb{N}$ is definitionally equal to a numeral.

The proof must now handle Glue types: a closed term of type $\mathbb{N}$ might be built using `ua` (via Glue), transport through `ua`, and other univalence constructions. Canonicity says all of these eventually reduce.

Why? Because `transp` through a Glue type has a computation rule that applies the equivalence. The equivalence is a function. The function is applied to a concrete element. The result computes. And since we're targeting $\mathbb{N}$, the final result must be a numeral.

The Brunerie number: the term $\mathsf{Brunerie} : \mathbb{Z}$ is defined using $\pi_4(S^3)$, which requires transporting along paths in the universe (i.e., applying equivalences via `ua`). In Book HoTT: stuck. In Cubical Agda: the evaluation runs, applying the Glue computation rules, applying the equivalences, reducing, and returning $-2$. This is canonicity made concrete.
