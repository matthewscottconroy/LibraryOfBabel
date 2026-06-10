# Mill's Methods of Experimental Inquiry

Suppose we want to know what caused a cholera outbreak in London in 1854. We have dozens of cases, each involving a patient who drank water from some source or another, lived in some neighbourhood or another, had some history of prior illness or another. How do we reason from this mass of particular facts to a causal conclusion? The question is not Hume's — it is not whether causation exists or what it is metaphysically — but Mill's: given that we accept causal inference, what is its logic?

John Stuart Mill's *A System of Logic* (1843) provided the most systematic empiricist attempt to answer this. His five methods — agreement, difference, joint method, concomitant variation, and residues — describe the inferential procedures by which scientists identify causal relationships from observational and experimental data. Mill was extending Hume's constant conjunction account while giving it methodological teeth. Where Hume had identified the epistemological status of causal claims (grounded in habit and observed regularity), Mill attempted to systematize their logical form.

Mill defined a cause as "the sum total of the conditions positive and negative taken together; the whole of the contingencies of every description, which being realized, the consequent invariably follows." This is broader than ordinary usage, and it anticipates the later INUS framework.

## The Five Methods

The **Method of Agreement** holds that if two or more instances in which a phenomenon occurs have only one circumstance in common, that circumstance is causally related to the phenomenon. In Snow's 1854 investigation, cases of cholera shared, above all other factors, the use of a contaminated water source. The method pointed to contaminated water as the cause, even without knowledge of germ theory.

The **Method of Difference** is the logic of the controlled experiment. If an instance in which the phenomenon occurs and an instance in which it does not occur have every circumstance in common save one, that one differing factor is the cause or effect. This is precisely the rationale behind having a control group: we eliminate all differences between the treated and untreated groups except the treatment itself. The method can be formalized:

- **P1**: In condition C₁ (treatment present), outcome O occurs.
- **P2**: In condition C₂ (treatment absent), outcome O does not occur.
- **P3**: C₁ and C₂ are identical in all other relevant respects.
- **C**: The treatment is the cause of O.

P3 is always the critical and contestable premise. The entire logic of experimental design — randomization, blinding, matched controls — is devoted to making P3 as credible as possible.

The **Joint Method** combines both: identify the factor present in all cases where the phenomenon occurs and absent in all cases where it does not. The **Method of Concomitant Variation** — the ancestor of regression analysis — holds that when a phenomenon varies in degree in step with another phenomenon, the two are causally related. The dose-response relationship in pharmacology, the relationship between price and demand in economics, between temperature and reaction rate in chemistry: all are instances of concomitant variation. The **Method of Residues** subtracts the known causes from a complex effect and assigns the residue to remaining factors. This reasoning famously led Adams and Le Verrier to predict the existence of Neptune from unexplained perturbations in Uranus's orbit.

## Known Limitations

Mill was aware of serious limitations in his methods. They presuppose that the relevant causal factors have already been identified — they cannot discover which variables to measure. If the relevant cause was absent from the investigator's initial list, the methods will fail silently.

The methods also struggle with overdetermination and multiple causation. If two independent factors X and Y each suffice for outcome P, the Method of Difference will fail: removing X while Y remains will not remove P, suggesting incorrectly that X is not causally relevant. And the methods are silent about the direction of causation — they identify correlation between antecedent and consequent but do not distinguish cause from effect, or genuine causation from common-cause correlation.

Third, Mill assumed causes operate independently, which is often false in complex systems where interaction effects are ubiquitous.

## Contemporary Significance

Contemporary causal inference methods are sophisticated descendants. Instrumental variables extend the Method of Difference to observational settings by finding an exogenous "instrument" that affects the treatment but not the outcome directly. Regression discontinuity uses sharp threshold effects to approximate experimental variation. Difference-in-differences compares changes over time in treated and control groups — a refined form of the Joint Method. Causal discovery algorithms (the PC algorithm, FCI) use patterns of conditional independence — a probabilistic refinement of Mill's methods — to infer causal structure from observational data.

Most explicitly, Judea Pearl's do-calculus provides the formal language that Mill lacked. The do(X=x) operator formalizes what it means to intervene on a variable, corresponding to Mill's ideal experimental manipulation. The rules of do-calculus specify when and how causal inferences from observational data are valid — giving Mill's intuitions a rigorous mathematical framework that he could not have imagined but would surely have recognized as continuous with his project.

Mill's methods operate within the Humean framework of regular succession, but they were not merely Humean. Mill recognized that causes are "real" in some sense that supports explanation and intervention — not merely regularity. His notion of a cause as "the whole of the conditions" anticipates Mackie's INUS analysis. And the methods implicitly distinguish genuine laws from accidental regularities by their projectibility: only genuine causal relationships support the kind of interventions the methods assume.
