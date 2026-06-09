# SBML: Systems Biology Markup Language

Imagine you spend three months carefully constructing a mathematical model of the NF-κB signaling network — twenty species, forty reactions, a dozen feedback loops, kinetic parameters painstakingly fit to experimental data. You submit your paper, and the reviewers ask whether your results are consistent with an existing model from another lab. That model was built in MATLAB. Your model lives in a Python script. Without a common interchange format, answering that question requires one of you to manually re-implement the other's model from scratch, with all the transcription errors and ambiguities that entails. This is precisely the problem that **SBML — the Systems Biology Markup Language** — was designed to eliminate.

SBML is the standard XML-based format for representing mathematical models of biological processes. It was designed to enable model exchange: so that a model built in MATLAB can be imported by Python, simulated by COPASI, and deposited in the BioModels Database without loss of information. Every published systems biology model should be deposited in SBML format.

## SBML Structure

SBML organizes model components into a hierarchical XML structure. The Level 3 Version 2 specification (the current standard) defines:

- **Compartments**: spatial volumes in which reactions occur (e.g., cytoplasm, nucleus)
- **Species**: chemical or biological entities with initial amounts or concentrations
- **Parameters**: numerical constants used in kinetic laws
- **Reactions**: transformations between species, defined by kinetic laws (rate expressions)
- **Rules**: algebraic or assignment relationships between quantities
- **Events**: discontinuous changes triggered by conditions

```xml
<?xml version="1.0" encoding="UTF-8"?>
<sbml xmlns="http://www.sbml.org/sbml/level3/version2/core"
      level="3" version="2">

  <model id="repressilator" name="Elowitz-Leibler repressilator">

    <!-- Compartments: define volumes for concentration-based kinetics -->
    <listOfCompartments>
      <compartment id="cell" name="Cell volume" size="1e-15" spatialDimensions="3"
                   units="litre" constant="true"/>
    </listOfCompartments>

    <!-- Species: mRNAs and proteins -->
    <listOfSpecies>
      <species id="m1" name="lacI mRNA"  compartment="cell"
               initialAmount="0" hasOnlySubstanceUnits="false"
               boundaryCondition="false" constant="false"/>
      <species id="m2" name="tetR mRNA"  compartment="cell"
               initialAmount="0" hasOnlySubstanceUnits="false"
               boundaryCondition="false" constant="false"/>
      <species id="m3" name="cl mRNA"    compartment="cell"
               initialAmount="0" hasOnlySubstanceUnits="false"
               boundaryCondition="false" constant="false"/>
      <species id="p1" name="LacI protein" compartment="cell"
               initialAmount="0" hasOnlySubstanceUnits="false"
               boundaryCondition="false" constant="false"/>
      <species id="p2" name="TetR protein" compartment="cell"
               initialAmount="0" hasOnlySubstanceUnits="false"
               boundaryCondition="false" constant="false"/>
      <species id="p3" name="cI protein"   compartment="cell"
               initialAmount="0" hasOnlySubstanceUnits="false"
               boundaryCondition="false" constant="false"/>
    </listOfSpecies>

    <!-- Parameters: rate constants -->
    <listOfParameters>
      <parameter id="alpha"  value="216"    constant="true" units="dimensionless"/>
      <parameter id="alpha0" value="0.216"  constant="true" units="dimensionless"/>
      <parameter id="n"      value="2"      constant="true" units="dimensionless"/>
      <parameter id="beta"   value="5"      constant="true" units="per_second"/>
      <parameter id="delta"  value="0.347"  constant="true" units="per_second"/>
    </listOfParameters>

    <!-- Reactions: transcription and translation -->
    <listOfReactions>
      <!-- Transcription of lacI (repressed by cI protein p3) -->
      <reaction id="J1" name="lacI transcription" reversible="false">
        <listOfProducts>
          <speciesReference species="m1" stoichiometry="1"/>
        </listOfProducts>
        <listOfModifiers>
          <modifierSpeciesReference species="p3"/>
        </listOfModifiers>
        <kineticLaw>
          <math xmlns="http://www.w3.org/1998/Math/MathML">
            <apply>
              <minus/>
              <apply>
                <plus/>
                <apply>
                  <divide/>
                  <ci>alpha</ci>
                  <apply>
                    <plus/>
                    <cn>1</cn>
                    <apply><power/><ci>p3</ci><ci>n</ci></apply>
                  </apply>
                </apply>
                <ci>alpha0</ci>
              </apply>
              <apply><times/><ci>delta</ci><ci>m1</ci></apply>
            </apply>
          </math>
        </kineticLaw>
      </reaction>
    </listOfReactions>

  </model>
</sbml>
```

Notice what that XML encodes: not just numbers, but the complete mathematical structure of the model — which species are involved in which reactions, what the kinetic law is, what the units are, what the initial conditions are. This is what "self-describing" means in practice. A simulator reading this file needs no external documentation to understand the model.

## Reading and Writing SBML with libSBML

```python
import libsbml

def validate_sbml(sbml_file):
    """
    Validate an SBML file for syntactic and semantic correctness.
    Returns: (document, n_errors, n_warnings)
    """
    reader = libsbml.SBMLReader()
    document = reader.readSBMLFromFile(sbml_file)

    n_errors = document.getNumErrors()
    n_warnings = 0
    errors = []

    for i in range(n_errors):
        error = document.getError(i)
        if error.getSeverity() == libsbml.LIBSBML_SEV_WARNING:
            n_warnings += 1
        else:
            errors.append(f"  [{error.getSeverityAsString()}] {error.getMessage()}")

    n_real_errors = n_errors - n_warnings
    print(f"SBML validation: {n_real_errors} errors, {n_warnings} warnings")
    for err in errors:
        print(err)

    if n_real_errors == 0:
        print("  VALID SBML file")
    return document, n_real_errors, n_warnings

def inspect_sbml_model(sbml_file):
    """Extract model summary from an SBML file."""
    reader = libsbml.SBMLReader()
    doc = reader.readSBMLFromFile(sbml_file)
    model = doc.getModel()

    if model is None:
        print("ERROR: No model found in SBML file")
        return

    print(f"Model: {model.getId()} — {model.getName()}")
    print(f"  SBML Level {doc.getLevel()} Version {doc.getVersion()}")
    print(f"  Compartments: {model.getNumCompartments()}")
    print(f"  Species:      {model.getNumSpecies()}")
    print(f"  Reactions:    {model.getNumReactions()}")
    print(f"  Parameters:   {model.getNumParameters()}")
    print(f"  Rules:        {model.getNumRules()}")
    print(f"  Events:       {model.getNumEvents()}")

    print("\nSpecies:")
    for i in range(model.getNumSpecies()):
        sp = model.getSpecies(i)
        print(f"  {sp.getId()}: initial = {sp.getInitialAmount():.3f}, "
              f"compartment = {sp.getCompartment()}")

    print("\nReactions:")
    for i in range(model.getNumReactions()):
        rxn = model.getReaction(i)
        # Get kinetic law as formula string
        kl = rxn.getKineticLaw()
        formula = libsbml.formulaToL3String(kl.getMath()) if kl else "N/A"
        print(f"  {rxn.getId()}: {formula[:60]}...")

    return model

# BioModels Database: download models programmatically
def download_biomodels_model(model_id="BIOMD0000000012", output_file=None):
    """
    Download a model from BioModels Database by accession number.
    BIOMD0000000012 = Elowitz repressilator
    """
    import requests
    url = f"https://www.ebi.ac.uk/biomodels/model/download/{model_id}?filename={model_id}_url.xml"
    response = requests.get(url)
    response.raise_for_status()

    if output_file is None:
        output_file = f"{model_id}.xml"
    with open(output_file, "wb") as f:
        f.write(response.content)
    print(f"Downloaded: {model_id} → {output_file}")
    return output_file
```

## Programmatically Creating SBML Models

```python
def create_simple_degradation_model(k_deg=0.1, initial_amount=100.0):
    """
    Programmatically create an SBML model: A → ∅ with rate k_deg * A.
    Demonstrates the libSBML API for model construction.
    """
    document = libsbml.SBMLDocument(3, 2)  # Level 3 Version 2
    model = document.createModel()
    model.setId("simple_degradation")
    model.setName("First-order degradation")

    # Compartment
    compartment = model.createCompartment()
    compartment.setId("cell")
    compartment.setConstant(True)
    compartment.setSize(1.0)
    compartment.setSpatialDimensions(3)

    # Species A
    species_A = model.createSpecies()
    species_A.setId("A")
    species_A.setCompartment("cell")
    species_A.setInitialAmount(initial_amount)
    species_A.setHasOnlySubstanceUnits(False)
    species_A.setBoundaryCondition(False)
    species_A.setConstant(False)

    # Parameter: degradation rate
    param = model.createParameter()
    param.setId("k_deg")
    param.setValue(k_deg)
    param.setConstant(True)

    # Reaction: A -> (null), rate = k_deg * A
    reaction = model.createReaction()
    reaction.setId("degradation")
    reaction.setReversible(False)

    # Substrate (species reference)
    substrate = reaction.createReactant()
    substrate.setSpecies("A")
    substrate.setStoichiometry(1.0)
    substrate.setConstant(True)

    # Kinetic law: k_deg * A
    kinetic_law = reaction.createKineticLaw()
    math_ast = libsbml.parseL3Formula("k_deg * A")
    kinetic_law.setMath(math_ast)

    # Write to file
    writer = libsbml.SBMLWriter()
    sbml_string = writer.writeSBMLToString(document)
    print(f"Created model: {model.getId()}")
    print(f"  Species: {model.getNumSpecies()}, Reactions: {model.getNumReactions()}")

    # Validate
    errors = document.getNumErrors(libsbml.LIBSBML_SEV_ERROR)
    print(f"  Validation: {errors} errors")
    return sbml_string

sbml_model = create_simple_degradation_model(k_deg=0.1, initial_amount=50.0)
```

## Simulating SBML Models with RoadRunner

**libroadrunner** is the fastest SBML simulator in Python, implementing CVODE for stiff ODE solving:

```python
import roadrunner

def simulate_sbml(sbml_file_or_string, t_end=200, n_points=1000,
                   selections=None):
    """
    Simulate an SBML model using roadrunner (CVODE solver).
    """
    # Load model (string or file path)
    if sbml_file_or_string.startswith("<?xml"):
        rr = roadrunner.RoadRunner(sbml_file_or_string)
    else:
        rr = roadrunner.RoadRunner(sbml_file_or_string)

    # Configure selections (what to track)
    if selections:
        rr.timeCourseSelections = ["time"] + selections
    else:
        rr.timeCourseSelections = ["time"] + rr.getFloatingSpeciesIds()

    # Simulate
    result = rr.simulate(0, t_end, n_points)
    import pandas as pd
    df = pd.DataFrame(result, columns=result.colnames)

    print(f"Simulation complete: {len(df)} time points, "
          f"{len(df.columns)-1} tracked variables")
    return df

# Example: simulate the repressilator
# result_df = simulate_sbml("BIOMD0000000012.xml", t_end=200, n_points=2000)
# result_df.plot(x="time", y=["[m1]", "[m2]", "[m3]"])
```

## Why This Matters

SBML is the lingua franca of systems biology models. Without a common format, a model published by one lab cannot be imported, reanalyzed, or extended by another lab — every group would need to re-implement every model from scratch. The BioModels Database curates >1,000 published SBML models, covering a vast range of signaling, metabolic, and regulatory models. When you publish an ODE model of gene regulation, depositing it as validated SBML in BioModels ensures it is immediately usable by the community — with roadrunner for simulation, COPASI for parameter estimation, libSBML for programmatic access. SBML is not bureaucracy; it is the infrastructure that transforms individual research contributions into community resources.
