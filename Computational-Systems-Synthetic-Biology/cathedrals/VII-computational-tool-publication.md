# Cathedral VII: A Computational Tool That Others Can Use

---

## The Question

What gap exists in the computational tools available to the field — and can you fill it with well-engineered, documented, and published software?

---

## Prerequisites

All of them. This cathedral requires:
- Deep knowledge of the problem domain (whichever tier it spans)
- Solid software engineering ([Tier 4.5](../curriculum/tier-4-computational-tools/4.5-software-engineering-research.md))
- Understanding of existing tools and their limitations
- Ability to evaluate correctness and communicate to users

---

## Identifying a Gap

### How to Find a Gap

1. **Frustration-driven**: what tool do you wish existed that doesn't?
2. **Literature-driven**: what analyses are described vaguely in papers, implemented as one-off scripts, or approximated with wrong tools?
3. **Comparison-driven**: what existing tool has known limitations you can address?
4. **Interface-driven**: what analysis requires 5 different tools that could be unified?

### Examples of Publishable Tool Projects

**Systems biology:**
- A Python library for systematic bifurcation analysis of biological ODE systems (wraps XPPAUT-style analysis in modern API)
- A tool that automatically converts between Antimony/SBML and Python/Julia ODE representations
- A library for fitting ODE models to time-series data with proper uncertainty quantification

**Bioinformatics:**
- A tool integrating multiple GRN inference methods with standardized benchmarking
- A pipeline for systematic characterization of riboswitch sequences in public genomes
- A tool for visualizing and comparing single-cell trajectory analysis from different methods

**Synthetic biology:**
- A part characterization database with automatic transfer function fitting and visualization
- A web tool for designing and simulating genetic circuits with uncertainty in part parameters
- A tool predicting RBS strength across multiple host organisms

**Genomics:**
- Improved de-duplication method for certain library prep protocols
- Visualization tool for comparing variant calls across multiple callers
- Pipeline for population-level structural variant analysis

**ML/AI:**
- Benchmarking framework for sequence-fitness prediction models
- Active learning loop tool for protein engineering campaigns
- A tool for comparing AlphaFold structure confidence across a protein family

---

## The Project

### Phase 1: Define the Tool

1. Write a one-page tool specification:
   - Problem: what is currently hard or impossible?
   - Solution: what will your tool do?
   - Users: who will use it? What is their expertise?
   - Inputs: what does the user provide?
   - Outputs: what does the tool return?
   - Scope: what will it NOT do? (important to state explicitly)

2. Survey existing tools:
   - Why can't existing tools solve the problem?
   - What will you reuse vs. implement from scratch?
   - How will you benchmark against existing approaches?

3. Define correctness criteria:
   - On known inputs, what is the expected output?
   - How will you verify the tool is correct?
   - What are the edge cases?

### Phase 2: Architecture Design

4. Design the API before writing implementation:
   ```python
   # What you want users to be able to write:
   from mybiosim import RepressilatorModel, GillespieSimulator, TrajectoryAnalyzer
   
   model = RepressilatorModel(
       alpha=100, alpha0=0.001, n=2, beta=1.0
   )
   
   sim = GillespieSimulator(model, seed=42)
   trajectory = sim.run(t_max=500, n_cells=100)
   
   analysis = TrajectoryAnalyzer(trajectory)
   period = analysis.estimate_period(species='p1')
   cv = analysis.coefficient_of_variation(species='p1', t_min=100)
   ```

5. Decide on data structures:
   - What format are intermediate results stored in?
   - What format is the final output?
   - Is there a standard format you should be compatible with (SBML, AnnData, BED)?

6. Identify performance requirements:
   - What is the expected input size (number of genes, time points, cells)?
   - What performance is acceptable? (seconds? minutes? hours?)
   - Where are the computational bottlenecks likely to be?

### Phase 3: Implementation

7. Write tests first (TDD):
   - For each function: what should it do on known inputs?
   - Write the test before the function
   - Tests serve as both specification and regression guard

8. Implement incrementally:
   - Start with the core algorithm (smallest useful piece)
   - Add features one at a time, each with tests
   - Keep the API stable after early design is settled

9. Profile early:
   ```python
   import cProfile
   cProfile.run('simulate_1000_cells(model, t_max=500)')
   ```
   - Identify bottlenecks before optimizing
   - Optimize the actual bottleneck, not what you assume is slow

10. Documentation as you go:
    - Docstrings for every public function: what it does, parameters, returns, raises, example
    - README: installation, quick start, main use cases
    - Tutorial notebook: end-to-end example on real biological data

### Phase 4: Validation and Benchmarking

11. Correctness tests:
    - Reproduce results from a published paper using your tool
    - Compare to an alternative implementation on the same inputs

12. Performance benchmarks:
    - Measure runtime at different input sizes
    - Compare to existing tools if applicable
    - Report in documentation: "expect ~X seconds for Y inputs on Z hardware"

13. Stress testing:
    - Unusual inputs: empty networks, single-node systems, disconnected graphs
    - Bad inputs: wrong types, negative concentrations, NaN values
    - Large inputs: does it fail gracefully or crash?

### Phase 5: Open-Source Release

14. Choose a license:
    - MIT: permissive; allows any use including commercial; most adoption
    - Apache 2.0: like MIT but explicit patent clause
    - GPL v3: copyleft; requires derivatives to be open source
    - For academic tools: MIT is usually appropriate

15. GitHub repository structure:
    ```
    your-tool/
    ├── src/your_tool/
    ├── tests/
    ├── docs/
    │   └── tutorials/
    ├── pyproject.toml
    ├── README.md           ← install + quick start
    ├── CHANGELOG.md        ← version history
    └── LICENSE
    ```

16. Publish to PyPI:
    ```bash
    python -m build
    twine upload dist/*
    # Users can then: pip install your-tool
    ```

17. Create documentation site: Sphinx + ReadTheDocs (free for open source)

18. Register with bio.tools or SciCrunch for discoverability in biological software registries

### Phase 6: Publication

19. Choose a venue for a methods paper:
    - *Bioinformatics* (Oxford): standard for bioinformatics tools
    - *PLOS Computational Biology*: broader computational methods
    - *NAR* (Nucleic Acids Research): especially for databases and web servers
    - *Journal of Open Source Software*: lightweight review for scientific software
    - *Nature Methods*: for significant methodological advances

20. Methods paper structure:
    - Abstract: what does it do; what's novel; key result showing it works
    - Introduction: the problem; why existing tools don't solve it
    - Methods: algorithm description; implementation details; availability
    - Results: benchmarking figures; performance comparison; biological example
    - Discussion: limitations; future extensions

21. Maintenance plan:
    - Version your releases (semantic versioning: major.minor.patch)
    - Respond to GitHub issues
    - Deprecation notices before breaking changes
    - Automated CI/CD: test on every pull request (GitHub Actions)

---

## What Makes a Good Tool Paper

- **Solves a real problem**: users actually need this
- **Validated**: demonstrated to give correct answers on benchmarks
- **Usable**: users can install and run it in 10 minutes
- **Documented**: clear examples; API reference
- **Maintained**: author responds to issues; updates for dependency changes

---

## Key Tools

- pytest + coverage: testing
- GitHub Actions: CI/CD
- Sphinx + ReadTheDocs: documentation
- PyPI + twine: distribution
- pre-commit hooks: automated code style enforcement (black, ruff, isort)
