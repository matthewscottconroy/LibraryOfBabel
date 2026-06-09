# Structure Visualization Tools

When Max Perutz and John Kendrew solved the first protein structures in the late 1950s and early 1960s, they represented them with hand-built wire models — physical objects, constructed from metal rods and connectors, filling rooms. The models were beautiful and painstaking, and they made an immediate visual impact: you could see the heme group nested in the globin fold of myoglobin, the four subunits of hemoglobin arranged symmetrically. Visualization was inseparable from understanding.

Today the tools are computational rather than physical, but the insight remains: you understand a protein structure by looking at it. Equations and statistics describe structures abstractly. Images make them intuitive. The ability to clearly visualize and communicate structural information — to make a figure that shows exactly the right thing in exactly the right way — is as important for practicing structural biology as any computational skill.

Visual inspection and communication of protein structures is an essential skill for any structural biologist. Different software tools are optimized for different purposes — publication-quality figures, interactive exploration of molecular dynamics trajectories, web-based viewing, or cryo-EM density interpretation. This section surveys the major tools and provides practical guidance for creating effective structural figures.

## PyMOL

**PyMOL** (Schrödinger) is the industry standard for molecular visualization and publication-quality structural figure preparation. Its selection algebra, built-in ray tracer, and Python scripting API make it uniquely powerful. Virtually every structural biology paper published in Nature, Science, Cell, or any major journal uses PyMOL figures. Learning PyMOL is, quite simply, a prerequisite for communicating structural biology.

**Selection algebra**: Objects are selected using logical expressions combining keywords:
```
# PyMOL selection examples:
select active_site, resi 100+120+145 and chain A
select interface, chain A within 4.0 of chain B
show sticks, active_site
color red, active_site
```

**Representation styles**: `cartoon` (secondary structure ribbons), `sticks` (all covalent bonds), `surface` (solvent-accessible surface), `sphere` (van der Waals spheres), `dots` (dot surface).

**Making a publication figure**:
```python
# PyMOL Python script for a clean publication figure
fetch 4HHB  # Human hemoglobin
as cartoon
color paleblue, chain A or chain B  # Alpha chains
color lightyellow, chain C or chain D  # Beta chains
color red, organic  # Heme groups
set ray_shadows, 0
set antialias, 2
set ray_opaque_background, off
ray 2400, 2400  # High-resolution ray trace
png hemoglobin.png, dpi=300
```

**Surface coloring by electrostatics**: Calculate the molecular electrostatic potential with APBS (Adaptive Poisson-Boltzmann Solver), load the electrostatic map, and color the surface by potential (red = negative, blue = positive, white = neutral). This type of figure is invaluable for showing why a particular region of the protein surface is attractive to a positively charged ligand, or for explaining the electrostatic complementarity at a protein-protein interface.

The PyMOL Python scripting interface deserves emphasis. Once you learn it, you can automate figure generation entirely — write a script that loads a structure, applies specific selections and representations, renders, and saves a PNG. This reproducibility matters for two reasons: it means your figures are exactly recreatable (no manual clicking), and it means you can apply the same visualization scheme to dozens of structures programmatically.

## UCSF ChimeraX

**UCSF ChimeraX** is the successor to UCSF Chimera, developed with modern GPU rendering and built-in support for cryo-EM data. Key features:

- **Cryo-EM density maps**: Open `.mrc` files directly, display as volumetric isosurface or transparency surface, dock atomic models into maps.
- **Molecular surfaces**: Fast GPU-accelerated surface rendering with electrostatic coloring.
- **Video production**: Built-in animation and movie-making tools for creating publication videos showing structural transitions.
- **AlphaFold integration**: Direct access to AlphaFold Database predictions; color by pLDDT confidence.
- **Command-line scripting**: Similar to PyMOL but with a more modern interface.

ChimeraX is particularly recommended for: cryo-EM map interpretation, large complexes (ribosome, viral capsids), and any project involving density maps alongside atomic models. The ability to visualize an electron density map as a transparent cloud surrounding an atomic model, and to immediately see where the model fits well and where it diverges from the experimental density, is essential for cryo-EM structure validation. For the expanding part of the field that involves cryo-EM, ChimeraX has become as indispensable as PyMOL.

## VMD: Molecular Dynamics Trajectories

**VMD** (Visual Molecular Dynamics) is developed by the Theoretical and Computational Biophysics group at UIUC alongside the NAMD MD engine. Its primary strength is **trajectory analysis**: loading and visualizing MD simulation trajectories with hundreds of thousands of frames.

Features:
- **Tcl/Python scripting**: Full scripting API for custom analysis (calculating distances, angles, RMSD over time).
- **Biofilm/materials representations**: Beyond proteins — lipid bilayers, nanomaterials.
- **NAMD integration**: Direct pipeline from NAMD MD simulation output to VMD visualization.
- **Collective variables**: Interface to calculate order parameters and free energy surfaces.

For static protein structure visualization alone, VMD is not preferred over PyMOL; its advantage is specifically in trajectory analysis. If you are running molecular dynamics simulations and want to watch a protein move, measure how a distance fluctuates over nanoseconds, or calculate the RMSD of a binding loop over a 1-microsecond trajectory, VMD is the right tool.

## Mol* Viewer: Web-Based Default

**Mol*** (MolStar) is the web-based molecular viewer now used as the default by the PDB, UniProt, and the AlphaFold Database. It requires no installation: access any structure at rcsb.org and click "3D View." Features:
- WebGL-accelerated rendering in the browser
- Full selection and representation control
- Integration with PDB annotations (binding sites, PTMs, sequence variants)
- Embeddable in web pages as a React/Vue component

Mol* is sufficient for quick exploration and sharing structures with collaborators who don't have PyMOL. For publication figures, PyMOL or ChimeraX remains preferred. The web-based accessibility of Mol* has been genuinely transformative for the field — any researcher anywhere in the world can now examine the three-dimensional structure of any PDB entry without installing any software. This lowered barrier has made structural information accessible to a much broader scientific audience.

## NGL Viewer

**NGL Viewer** is another web-based component, primarily used as an embeddable widget in Jupyter notebooks for Python-based structural analysis workflows. Integrates with Biopython and py3Dmol:

```python
import py3Dmol
viewer = py3Dmol.view(query='pdb:1ABC')
viewer.setStyle({'cartoon': {'color': 'spectrum'}})
viewer.addSurface(py3Dmol.VDW, {'opacity': 0.7})
viewer.show()
```

The ability to display protein structures inline in a Jupyter notebook — immediately adjacent to the code that analyzed them — is extremely useful for exploratory structural analysis. You can calculate distances, identify contacts, run analyses, and visualize the results all in the same notebook environment without switching between applications.

## Practical Guide: Publication-Quality PyMOL Figures

Best practices for journal-quality figures:
1. **Use `ray` tracing** at high resolution (≥1500 × 1500 pixels, dpi=300).
2. **Avoid the default rainbow coloring** — use chemically meaningful colors (blue = N-terminus, red = C-terminus; or color by chain; or by conservation).
3. **Show relevant structural features only** — hide irrelevant loops; show surface only for the binding pocket.
4. **Include a scale bar or legend**: Provide residue numbers or domain annotations.
5. **Consistent style within a figure**: All panels at the same zoom level and lighting.
6. **For binding sites**: Show protein as surface (semi-transparent), ligand as sticks with element-based coloring (C=green, N=blue, O=red).
7. **For sequence conservation coloring**: Use the ConSurf server to calculate conservation scores and a PyMOL script to color accordingly.

Each of these guidelines reflects accumulated community experience about what makes a structural figure readable versus confusing. The rainbow coloring default, in particular, is a trap that catches almost every new PyMOL user: it produces visually striking images that are scientifically meaningless because the colors encode nothing interpretable. Meaningful colors encode something the reader can interpret — conservation, chain identity, binding affinity, experimental uncertainty.

## Why This Matters

Clear, accurate structural visualization is the primary means by which structural biology informs biology and medicine — publication figures from PyMOL and ChimeraX appear in virtually every paper reporting a new protein structure, drug complex, or mechanistic model — making visualization proficiency as important as the underlying structural analysis for communicating scientific results. The difference between a figure that makes the biological point immediately visible and one that leaves the reader confused is not just aesthetic; it determines whether your science is understood. Structural biology is one of the most visual sciences, and learning to see and show protein structures well is not a supplementary skill — it is central to doing the work.
