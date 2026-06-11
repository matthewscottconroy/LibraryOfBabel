# Placement: ch16-derived-functors.md

## Part in book/
This chapter belongs to **Part VIII: Homological Algebra** (`book/part-08-homological-algebra/`), specifically:
- `ch39-resolutions/`
- `ch40-derived-functors-ext-and-tor/`

## Sections in book/ that cover this material

| Curriculum section | Book location |
|---|---|
| Projective and injective resolutions (§16.1) | `ch39-resolutions/39.1-projective-resolutions/`, `39.2-injective-resolutions/` |
| Right and left derived functors (§16.2) | `ch40-derived-functors-ext-and-tor/40.1-derived-functors/` |
| $\mathrm{Ext}^n$ groups (§16.3) | `ch40-derived-functors-ext-and-tor/40.2-ext/` |
| $\mathrm{Tor}_n$ groups (§16.4) | `ch40-derived-functors-ext-and-tor/40.3-tor/` |
| Balance of Ext and Tor (§16.5) | `ch40-derived-functors-ext-and-tor/40.4-balance/` |
| Applications: extensions, flatness (§16.6) | `ch40-derived-functors-ext-and-tor/40.5-applications/` |

## Content in curriculum/ not fully covered in book/

- The curriculum discusses the **derived category** as a brief forward pointer at the end of §16.2, noting that derived functors are steps toward the derived category $D(\mathcal{A})$. The book does not have any section on derived categories in Part VIII. Given their centrality to modern algebra and geometry, a short introductory section (even as an "outlook" section at the end of `ch40`) would strengthen the text.
- The curriculum explicitly computes $\mathrm{Ext}^n_{\mathbb{Z}}(\mathbb{Z}/m\mathbb{Z}, \mathbb{Z}/n\mathbb{Z})$ for specific values. The book has general computation methods but fewer fully worked numerical examples. → Add to `40.2` a worked computation table.
- The curriculum computes $\mathrm{Tor}_1^R(M, N)$ for several examples (including $\mathrm{Tor}_1^\mathbb{Z}(\mathbb{Z}/m, \mathbb{Z}/n) \cong \mathbb{Z}/\gcd(m,n)$). The book has the general theory but this specific example is not fully worked out. → Add to `40.3`.
