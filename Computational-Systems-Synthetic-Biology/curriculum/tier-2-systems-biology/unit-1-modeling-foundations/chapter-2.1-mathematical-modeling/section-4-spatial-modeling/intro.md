# Section 4: Spatial Modeling

All of the models we have built so far share a hidden assumption: the cell is a well-mixed bag. Every molecule is equally likely to encounter every other molecule, regardless of position. Concentrations are uniform. Diffusion is instantaneous.

For small molecules in bacteria, this is an excellent approximation: an *E. coli* cell is about 1 µm across, and small molecules diffuse across it in milliseconds — much faster than most reaction timescales. The well-mixed assumption holds.

But consider a mammalian cell, 10–15 µm across. Or a neuron, with a dendrite extending hundreds of micrometers from the cell body. Or a developing embryo, where cells at the anterior end must know they are different from cells at the posterior end. In these contexts, spatial organization is not a detail — it is the phenomenon itself. The *Drosophila* embryo does not just happen to have a head; it has a head because Bicoid protein forms a spatial gradient that tells anterior cells to become head tissue and posterior cells not to. You cannot model this with ODEs.

This section introduces the spatial modeling frameworks needed to describe biological systems where position matters.

**When spatial structure matters** (subsection 4.1) provides the conceptual and quantitative criterion for when to add spatial detail. The reaction-diffusion length scale $\ell = \sqrt{D/k_\text{eff}}$ is the key tool: when $\ell$ is comparable to the cell dimension, gradients are significant and spatial models are necessary.

**Reaction-diffusion PDEs and Turing instability** (subsection 4.2) develops the mathematical framework for continuum spatial models and presents Turing's 1952 prediction of spontaneous pattern formation. The Turing mechanism — diffusion-driven instability from an activator-inhibitor pair with differential diffusivity — is one of the most beautiful results in mathematical biology.

**Numerical methods for PDEs** (subsection 4.3) covers the computational tools needed to simulate spatial models in biological geometries: finite differences, finite elements, and the method of lines. Practical guidance on stiffness, boundary conditions, and software tools is included.

**Particle-based spatial stochastic simulation** (subsection 4.4) addresses the most detailed spatial modeling regime: individual molecules tracked as particles in continuous space. Tools including Smoldyn, MCell, and ReaDDy are covered, with emphasis on when this level of detail is necessary and what it costs computationally.

The key conceptual skill in this section is not simulation proficiency but judgment: knowing when to add spatial detail, what kind of spatial model to use, and when the simpler well-mixed approximation is good enough. Spatial modeling is expensive; spatial detail should be added only when the biology demands it.
