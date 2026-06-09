# Chapter 40 — Exercises

## Important Figures

- **Henri Cartan (1904–2008) & Samuel Eilenberg (1913–1998)** — *Homological Algebra* (1956): Ext and Tor defined via resolutions; the systematic theory of derived functors
- **Jean-Pierre Serre (1926–)** — Serre duality; local cohomology; applications of Ext and Tor in algebraic geometry and number theory
- **Alexander Grothendieck (1928–2014)** — derived categories as the "right" setting for derived functors (Tôhoku paper, 1957); cohomological $\delta$-functors; derived categories

## References and Primary Sources

- **H. Cartan & S. Eilenberg, *Homological Algebra* (Princeton, 1956)** — the founding text
- **C. Weibel, *An Introduction to Homological Algebra* (Cambridge, 1994)**, Chs. 2–3 — Ext and Tor; group cohomology
- **K. Brown, *Cohomology of Groups* (Springer, 1982)** — applications to group theory

## Examples, Applications, and Thought Experiments

- **$\text{Ext}^1_{\mathbb{Z}}(\mathbb{Z}/m\mathbb{Z}, \mathbb{Z}/n\mathbb{Z}) \cong \mathbb{Z}/\gcd(m,n)\mathbb{Z}$** — compute from the resolution $0 \to \mathbb{Z} \xrightarrow{\times m} \mathbb{Z} \to \mathbb{Z}/m\mathbb{Z} \to 0$; apply $\text{Hom}(-, \mathbb{Z}/n\mathbb{Z})$; the $\text{Ext}^1$ classifies extensions $0 \to \mathbb{Z}/n\mathbb{Z} \to E \to \mathbb{Z}/m\mathbb{Z} \to 0$; there are $\gcd(m,n)$ non-isomorphic such extensions
- **$\text{Tor}_1^{\mathbb{Z}}(\mathbb{Z}/m\mathbb{Z}, \mathbb{Z}/n\mathbb{Z}) \cong \mathbb{Z}/\gcd(m,n)\mathbb{Z}$** — same resolution, apply $- \otimes_{\mathbb{Z}} \mathbb{Z}/n\mathbb{Z}$; Tor measures the failure of flatness; $\mathbb{Z}/m\mathbb{Z}$ is not flat because $\mathbb{Z}/n\mathbb{Z} \otimes_{\mathbb{Z}} \mathbb{Z}/m\mathbb{Z}$ has torsion
- **Group cohomology** — $H^n(G, A) = \text{Ext}^n_{\mathbb{Z}[G]}(\mathbb{Z}, A)$; $H^2(G, A)$ classifies group extensions $1 \to A \to E \to G \to 1$ with $A$ abelian; $H^1(G, A)$ classifies "twisted" maps; group cohomology encodes both algebraic and topological information about $G$
- **Thought experiment: "defect measurement"** — Hom is left exact: $0 \to A \to B \to C \to 0$ gives $0 \to \text{Hom}(M,A) \to \text{Hom}(M,B) \to \text{Hom}(M,C)$; the map on the right may not be surjective; $\text{Ext}^1(M, A)$ measures this defect; $\text{Ext}^n$ measures how "far through" a long exact sequence the exactness fails; Ext is the precise bookkeeper of obstructions

## Exercises

1. Using the free resolution $0 \to \mathbb{Z} \xrightarrow{\times 4} \mathbb{Z} \to \mathbb{Z}/4\mathbb{Z} \to 0$, compute $\operatorname{Ext}^n_{\mathbb{Z}}(\mathbb{Z}/4\mathbb{Z}, \mathbb{Z}/6\mathbb{Z})$ for all $n \geq 0$. (Apply $\operatorname{Hom}_{\mathbb{Z}}(-, \mathbb{Z}/6\mathbb{Z})$ to the truncated resolution.) Verify that $\operatorname{Ext}^1_{\mathbb{Z}}(\mathbb{Z}/4\mathbb{Z}, \mathbb{Z}/6\mathbb{Z}) \cong \mathbb{Z}/\gcd(4,6)\mathbb{Z}$.

2. Using the same resolution, compute $\operatorname{Tor}_n^{\mathbb{Z}}(\mathbb{Z}/4\mathbb{Z}, \mathbb{Z}/6\mathbb{Z})$ for all $n \geq 0$. (Apply $- \otimes_{\mathbb{Z}} \mathbb{Z}/6\mathbb{Z}$ to the free part of the resolution.) Compare your answer for $\operatorname{Tor}_1$ with your answer for $\operatorname{Ext}^1$ and explain why they agree in this case.

3. Let $R = k[x]/(x^2)$ for a field $k$, and let $M = k = R/(x)$. Using the periodic free resolution $\cdots \to R \xrightarrow{x} R \xrightarrow{x} R \to k \to 0$, compute $\operatorname{Ext}^n_R(k, k)$ for all $n \geq 0$. What does the failure of these groups to vanish for large $n$ say about the projective dimension of $k$?

4. Prove that $\operatorname{Ext}^1_R(M, N)$ classifies short exact sequences $0 \to N \to E \to M \to 0$ up to isomorphism of extensions. Specifically: show that every projective presentation $P_1 \to P_0 \to M \to 0$ gives a map $\operatorname{Hom}(P_1, N) \to \operatorname{Ext}^1_R(M, N)$ (via the connecting homomorphism), and that the equivalence classes of extensions biject with elements of $\operatorname{Ext}^1_R(M, N)$.

5. Let $M$ be an $R$-module. Prove that $M$ is flat if and only if $\operatorname{Tor}_1^R(M, N) = 0$ for every $R$-module $N$. (Hint: $M$ is flat iff $M \otimes_R -$ is exact iff its first left derived functor vanishes.) Verify this for $M = \mathbb{Q}$ over $R = \mathbb{Z}$.

6. Let $G = \mathbb{Z}/2\mathbb{Z}$ act on $A = \mathbb{Z}$ by the trivial action. Compute $H^0(G, A)$, $H^1(G, A)$, and $H^2(G, A)$ using the periodic free resolution of $\mathbb{Z}$ over $\mathbb{Z}[G]$. (The resolution alternates between multiplication by $1 + \sigma$ and $1 - \sigma$, where $\sigma$ is the generator of $G$.) Interpret $H^2(G, A)$ as classifying extensions of $G$ by $A$.

7. Let $0 \to L \to M \to N \to 0$ be a short exact sequence of $R$-modules. Prove that the sequence $\cdots \to \operatorname{Ext}^n_R(A, M) \to \operatorname{Ext}^n_R(A, N) \xrightarrow{\delta} \operatorname{Ext}^{n+1}_R(A, L) \to \cdots$ is exact for any fixed $R$-module $A$. (This is the long exact sequence in the second variable for Ext; you may use the injective resolution construction.)

8. (Challenge) Prove the balance theorem for Ext: for any $R$-modules $M$ and $N$, the group computed by resolving $M$ projectively and applying $\operatorname{Hom}(-, N)$ agrees with the group computed by resolving $N$ injectively and applying $\operatorname{Hom}(M, -)$. That is, $H^n(\operatorname{Hom}(P_\bullet, N)) \cong H^n(\operatorname{Hom}(M, I^\bullet))$ for all $n$. (Hint: form the double complex $\operatorname{Hom}(P_p, I^q)$ and use the two spectral sequences of a double complex, which both converge to the same limit.)
