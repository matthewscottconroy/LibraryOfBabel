# Section 4: Software Engineering

When Florian Markowetz wrote "You Are Not Working For Yourself" in *PLOS Biology* in 2015, he was making a point about scientific code: every analysis you write will eventually need to be run again by someone — a collaborator, a reviewer, your future self trying to reproduce a figure for a revision that arrives three years after submission. Code that worked once, on your machine, in the right directory, with the right version of Python, is not a scientific contribution. Reproducible, documented, tested, version-controlled code is.

The sobering backdrop to this section is the ongoing crisis in computational reproducibility. A significant fraction of published bioinformatics analyses cannot be reproduced by independent groups. The causes are depressingly mundane: undocumented parameters, untracked code changes, missing software version information, hardcoded file paths that only exist on one machine. These are not deep scientific failures — they are engineering failures, and they have engineering solutions.

This section covers the software engineering practices that separate reproducible computational biology from code that happens to have worked once.

**Code quality** establishes the foundation: PEP 8 style standards, single-responsibility functions, the DRY principle, type hints, and code organization into testable modules. These are not aesthetic choices — they are the practices that make code reviewable and correctable.

**Testing** with pytest provides the infrastructure for confidence. Unit tests catch bugs in individual functions before they propagate through an analysis pipeline. Regression tests ensure that fixed bugs stay fixed. Property-based testing with Hypothesis finds edge cases that hand-written tests miss. Numerical testing with `pytest.approx` handles the floating-point tolerance issues that arise constantly in ODE solutions and statistical computations.

**Documentation** addresses the communication problem. Docstrings in NumPy or Google style, Sphinx-generated HTML documentation, well-written README files, and Jupyter notebooks as interactive analysis narratives — together these transform code from a black box into a tool that others can evaluate, reproduce, and build on.

**Reproducible research** with workflow managers (Snakemake and Nextflow), environment management (conda with pinned versions), and containerization (Docker and Singularity) ensures that the entire analysis pipeline — not just the code, but the software environment — can be reconstructed. A Snakemake workflow with pinned conda environments and a tagged git commit is the gold standard: anyone with the raw data can reproduce every figure from scratch.

**High-performance computing** covers SLURM job submission, array jobs for embarrassingly parallel workloads, memory estimation, and I/O optimization on parallel file systems. Most serious bioinformatics eventually outgrows a laptop, and the transition to HPC requires understanding job schedulers, resource requests, and the structure of parallelizable analyses.

**Code performance** closes the loop: profiling with cProfile and line_profiler to find bottlenecks, vectorization with NumPy to eliminate Python loops, Numba for JIT-compiling sequential algorithms like dynamic programming, multiprocessing and Dask for parallel computation, and GPU acceleration with CuPy and JAX for extreme-scale array operations.
