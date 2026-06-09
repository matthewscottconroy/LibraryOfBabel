# PDB File Format

Before you can analyze a protein structure computationally, you need to read the file. This sounds trivial, but it turns out to be a surprisingly rich problem — the Protein Data Bank file format has accumulated forty-five years of legacy decisions, workarounds, and quirks that have a direct impact on correctness in structural analysis. More than a few studies have been compromised by silent mishandling of alternate conformations, missing residues, or insertion codes. Understanding the file format is not bureaucratic busywork; it is the difference between analyzing the structure you think you're analyzing and analyzing something else entirely.

The **Protein Data Bank (PDB)** stores three-dimensional macromolecular structures in text file formats that encode atomic coordinates, chemical information, experimental data, and sequence. Two formats exist: the legacy PDB format, which has been in use since 1976, and the modern mmCIF format, which became the required deposition standard in 2014. You will encounter both in practice, and you need to be fluent in both.

## The Legacy PDB Format

The original **PDB format** (introduced in 1976) is a fixed-column text format. Each line is 80 characters wide with columns having specific meanings. Despite being superseded by mmCIF, the PDB format remains widely used and must be understood.

Why fixed columns at 80 characters? Because that was the width of an IBM punch card. The PDB format was designed for punch card input and has carried that constraint for nearly half a century, long after punch cards disappeared from scientific computing. This is the kind of historical accident that shapes science in ways its originators never intended.

**The ATOM Record**: The most important record type, representing one atom in the standard amino acid or nucleotide:

```
ATOM   1234  CA  ALA A  47      12.345  23.456  34.567  1.00 23.40           C
```

Columns (1-indexed):
- 1–6: Record type (`ATOM` for standard residues)
- 7–11: Serial number
- 13–16: Atom name (`CA` = alpha carbon; `CB` = beta carbon; `N`, `O`, `C` = backbone)
- 17: Alternate location indicator (blank, A, B for alternate conformations)
- 18–20: Residue name (`ALA`, `GLY`, `HIS`, etc.)
- 22: Chain ID (A, B, C, etc.)
- 23–26: Residue sequence number
- 31–38, 39–46, 47–54: X, Y, Z coordinates in Angstroms (right-aligned, 3 decimal places)
- 55–60: **Occupancy** (1.00 = atom always present; 0.50 = two alternate conformations)
- 61–66: **B-factor** (temperature factor, isotropic atomic displacement parameter in Å²)
- 77–78: Element symbol

**HETATM records** are identical in format to ATOM but are used for non-standard residues: ligands, cofactors, water molecules (HOH), modified amino acids. This distinction matters for analysis pipelines that want to process only protein atoms, or conversely, only ligands.

**SEQRES records** list the full protein sequence as read from the crystal, including residues that may be disordered (not visible in the electron density and therefore absent from ATOM records). Comparing SEQRES to the ATOM records immediately tells you which residues are missing from the model — a check you should always perform.

**CONECT records** explicitly define covalent bonds for ligands and other HETATM entities, compensating for the absence of connectivity implied by standard residue definitions.

## The mmCIF Format

**mmCIF** (macromolecular Crystallographic Information File) is the current standard for PDB deposition and is required for large structures (>62 chains, >99,999 atoms, or 100,000+ residues — all beyond the column limits of PDB format). mmCIF is a key-value pair dictionary format:

```
_atom_site.group_PDB        ATOM
_atom_site.id               1234
_atom_site.type_symbol      C
_atom_site.label_atom_id    CA
_atom_site.label_comp_id    ALA
_atom_site.label_asym_id    A
_atom_site.label_seq_id     47
_atom_site.Cartn_x          12.345
_atom_site.Cartn_y          23.456
_atom_site.Cartn_z          34.567
_atom_site.occupancy        1.00
_atom_site.B_iso_or_equiv   23.40
```

Unlike the PDB format, mmCIF has no column-width limitations, can represent arbitrarily large assemblies, and stores experimental metadata in a structured, machine-readable dictionary. The PDB now distributes all structures in mmCIF format (`.cif` files); PDB format is maintained for backward compatibility. If you are writing new structural analysis code, use mmCIF.

## Parsing with Biopython

**Biopython** provides two parsers, and the interface is admirably consistent between them:

```python
from Bio.PDB import PDBParser, MMCIFParser

# For PDB format:
parser = PDBParser(QUIET=True)
structure = parser.get_structure("1ABC", "1abc.pdb")

# For mmCIF format:
parser = MMCIFParser(QUIET=True)
structure = parser.get_structure("1ABC", "1abc.cif")

# Biopython SMCRA hierarchy: Structure → Model → Chain → Residue → Atom
model = structure[0]      # First model (index 0)
chain_A = model['A']      # Chain A
residue_47 = chain_A[47]  # Residue 47
ca_atom = residue_47['CA']  # Alpha carbon atom
print(ca_atom.get_vector())  # x, y, z coordinates as Vector
```

The SMCRA hierarchy (Structure → Model → Chain → Residue → Atom) is Biopython's data model for organizing structural information. For X-ray structures, there is only one model (index 0). For NMR ensembles, model index selects which conformer from the ensemble. This distinction matters every time you analyze an NMR structure — processing only model 0 means you are using only one conformer of the ensemble, not the full picture of conformational diversity that NMR captured.

For mmCIF dictionary access:
```python
from Bio.PDB.MMCIF2Dict import MMCIF2Dict
mmcif_dict = MMCIF2Dict('1abc.cif')
print(mmcif_dict['_cell.length_a'])  # Unit cell a dimension
```

## Common Pitfalls

These are not edge cases. They appear in a significant fraction of PDB structures, and silently ignoring them produces incorrect results.

**Alternate conformations**: When an atom/residue has multiple conformations (due to thermal motion or crystal disorder), both are recorded with alternate location identifiers (A and B) and occupancies summing to 1.0. PDBParser selects only the first alternate conformation by default; use `DisorderBuilder` for explicit handling. For drug discovery applications, alternate conformations of active site residues matter enormously — a side chain that samples two rotamers might accommodate a ligand in one conformation and clash with it in another.

**Missing residues**: Disordered loops or termini that are not visible in the electron density are absent from ATOM records but present in SEQRES. This creates gaps in residue numbering that affect structural analysis (the residue numbered 47 may follow residue 40 if residues 41–46 are missing). Always check for gaps before computing residue-based quantities.

**Non-standard residues**: Selenomethionine (MSE), phosphoserine (SEP), and other modified residues appear as HETATM but are part of the protein chain. They require special handling in analysis pipelines. Selenomethionine is particularly common — it is introduced deliberately during protein production to enable MAD/SAD phasing, and the vast majority of modern crystal structures were solved using selenomethionine-containing protein. If your pipeline strips HETATM records to "get only the protein," you will silently lose these residues.

**Insertion codes**: To maintain compatibility with existing literature numbering while inserting a residue, a residue can have an insertion code (e.g., 47A, 47B). This is common in antibody structures, where canonical Kabat numbering has been maintained for decades across thousands of antibodies, and insertions must be accommodated within that numbering scheme.

**Multiple models**: NMR structures contain multiple models (conformers from the ensemble). The ATOM records for model $i$ are enclosed between `MODEL i` and `ENDMDL` records. Processing an NMR structure as if it were a single-model X-ray structure — common in beginner analysis pipelines — will either fail or silently concatenate all conformers, producing bizarre results.

## Why This Matters

PDB file parsing is the entry point for virtually every computational structural biology task — from calculating distances between active site residues, to preparing input for molecular dynamics, to analyzing protein-ligand contacts — making fluency with PDB and mmCIF formats a fundamental skill for any structural bioinformatician. The pitfalls described here are not hypothetical; they appear in published literature and in software bugs in widely-used tools. Understanding the format deeply enough to anticipate these issues is what separates reliable structural analysis from analysis that merely appears to run without errors.
