# Chapter 01: Residues

The residue of an analytic function at an isolated singularity is the coefficient $a_{-1}$ in the Laurent expansion around that singularity. This single number carries all the information needed to evaluate contour integrals: the residue theorem states that the integral of $f$ around a closed contour equals $2\pi i$ times the sum of the residues at all enclosed singularities. This chapter develops systematic methods for computing residues and proves the residue theorem.

## Section 01: Computing Residues

For a simple pole at $z_0$: $\mathrm{Res}(f; z_0) = \lim_{z \to z_0}(z - z_0)f(z)$.

For a quotient $p(z)/q(z)$ with a simple zero of $q$ at $z_0$: $\mathrm{Res} = p(z_0)/q'(z_0)$.

For a pole of order $m$: $\mathrm{Res}(f; z_0) = \frac{1}{(m-1)!}\lim_{z \to z_0}\frac{d^{m-1}}{dz^{m-1}}[(z-z_0)^m f(z)]$.

These formulas reduce residue computation to limits and differentiation, avoiding the need to expand the full Laurent series.

## Section 02: The Residue Theorem

**Theorem.** If $f$ is analytic on and inside a simple closed contour $C$ except at finitely many isolated singularities $z_1, \ldots, z_N$ inside $C$:
$$\oint_C f(z)\, dz = 2\pi i\sum_{k=1}^N \mathrm{Res}(f; z_k).$$

The proof proceeds by deforming $C$ to small circles around each singularity and computing each circle integral using the Laurent series.

## Learning Objectives

After this chapter, a student should be able to:

- Compute residues at poles of any order using the appropriate formula.
- Apply the residue theorem to contour integrals with multiple poles.
- Recognize when Cauchy's integral formula is a special case of the residue theorem.
- Use residues to evaluate integrals that would otherwise be intractable.
