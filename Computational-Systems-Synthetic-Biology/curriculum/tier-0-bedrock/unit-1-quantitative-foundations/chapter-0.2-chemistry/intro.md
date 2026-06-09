# Chapter 0.2: Chemistry

Here is a fact that should shape how you think about biology: there are about $10^{14}$ chemical reactions happening inside a single human cell every second. Each one involves specific molecules colliding with just the right orientation, overcoming a precisely calibrated energy barrier, and producing products that become the substrates for the next reaction in the chain. The cell does not do this randomly. It does it with a thermodynamic logic that has been refined over four billion years of evolution, using molecules with specific functional groups, reaction mechanisms, and stereochemical preferences that are, at bottom, just chemistry.

This is the central claim of this chapter: that life is not beyond chemistry, but is the most sophisticated expression of it. The reason this matters for computational biology is direct and practical. Every rate law you write is a claim about chemical kinetics. Every binding affinity in your model is a claim about thermodynamic equilibrium. Every simulation of a molecular system is computing trajectories in the space of chemical conformations. If you do not understand the chemistry, your models will have correct syntax and incorrect semantics — the equations will run, but the assumptions embedded in them will be wrong in ways you cannot detect.

The story of how chemistry explains life begins in the late nineteenth century, when thermodynamics was being developed to understand steam engines, and Josiah Willard Gibbs derived the free energy function that would eventually allow biochemists to predict which metabolic reactions are spontaneous and which require energy input. It accelerates in the 1920s and 1930s, when quantum mechanics explained the nature of chemical bonds and allowed chemists to understand functional group reactivity from first principles. It reaches its current form in the era of structural biology, when X-ray crystallography, NMR, and cryo-EM revealed the three-dimensional architecture of the molecular machines that carry out cellular chemistry.

You are entering this story with the full benefit of hindsight. The principles in this chapter have been tested in thousands of laboratories over a century and a half; they are as reliable as science gets. Your job is not to memorize them but to internalize them — to develop the chemical intuition that allows you to look at a molecule and immediately see which parts are reactive, which parts are charged, which parts determine biological activity.

## What This Chapter Covers

This chapter is organized into five sections that build from physical principles to biological application.

**Section 1: General Chemistry** develops the thermodynamic and kinetic foundations. You will work through Gibbs free energy and why it governs whether biochemical reactions proceed; equilibrium constants and their exponential relationship to free energies; the Henderson-Hasselbalch equation and why pH is so tightly controlled in cells; and the Arrhenius equation and how kinetics controls cellular response time. These four topics — thermodynamics, equilibrium, acid-base chemistry, and kinetics — are the physical laws layer that no biological system can violate.

**Section 2: Organic Chemistry** moves from physics to molecular structure. You will learn the functional groups that define biomolecular reactivity — carboxylates, amines, thiols, carbonyls, phosphates — and the reaction mechanisms by which they interact: nucleophilic substitution, addition to carbonyls, elimination, and oxidation-reduction. The section closes with stereochemistry, which explains why biology is chiral and why this matters for molecular recognition, drug design, and molecular simulation.

**Section 3: Biochemistry** is the largest section and covers the molecular cast of characters that populate biological systems: the twenty amino acids and the hierarchy of protein structure; enzyme kinetics and the Michaelis-Menten and Hill equations; the chemistry of nucleic acids and the thermodynamics of base pairing; lipid bilayers and membrane potential; carbohydrates and glycan chemistry; and finally the coenzymes — ATP, NAD$^+$/NADH, NADPH, CoA, SAM — that serve as the currency linking metabolic reactions into an integrated network.

**Section 4: Physical Chemistry** introduces the experimental and theoretical tools that connect molecular-scale chemistry to observable quantities: spectroscopy (Beer-Lambert law, FRET, NMR) for measurement; diffusion and Fick's laws for understanding spatial organization; and statistical thermodynamics for deriving binding equilibria and gene regulation from first principles using the Boltzmann distribution.

**Section 5: Key Connections to Biology** synthesizes the chapter by mapping each chemical concept to its computational application — from thermodynamic constraints in flux balance analysis to the nearest-neighbor model underlying RNA secondary structure prediction. This section is the explicit bridge from chemistry to computational biology.

## What You Will Be Able to Do

After completing this chapter, you should be able to:

- Compute actual Gibbs free energies under cellular conditions and assess whether a reaction has thermodynamic driving force
- Predict the charge state of any ionizable amino acid residue at any pH using the Henderson-Hasselbalch equation
- Derive and apply the Michaelis-Menten equation; fit enzyme kinetic data computationally; distinguish competitive, uncompetitive, and noncompetitive inhibition
- Estimate the melting temperature of a DNA duplex from its sequence and use this to design PCR primers and CRISPR guide RNAs
- Calculate morphogen gradient decay lengths from diffusion coefficients and degradation rates
- Recognize the mechanistic basis of common enzyme reactions and use that knowledge to predict what inhibitors would be effective
- Explain why NADH and NADPH serve different metabolic roles despite nearly identical structures, and use coenzyme stoichiometries as constraints in metabolic models
- Understand the statistical thermodynamic basis of the Hill function and apply it to models of cooperative gene expression

The chemistry in this chapter is not decoration. It is the operating system on which all of biological computation runs.
