# Classical Genetics

In 1866, Gregor Mendel published his pea plant experiments in the Proceedings of the Natural History Society of Brünn — and was ignored for thirty-five years. When his work was rediscovered in 1900, it became clear that he had deduced the fundamental rules of heredity by nothing more than careful counting and a willingness to apply quantitative reasoning to biology at a time when almost no one else was doing so. Mendel did not know about chromosomes, meiosis, or DNA. He simply crossed plants, counted offspring in thousands, and noticed that the ratios were not arbitrary. That discipline — using observed ratios to infer underlying molecular mechanisms — is exactly what modern computational genetics does, at genome scale, with GWAS and population genomic tools.

Classical genetics predates molecular biology by nearly a century, yet its formal logic remains indispensable. The concepts of alleles, dominance, linkage, and complementation are the vocabulary of genetic analysis — and they map directly onto the quantitative tools of population genetics and genome-wide association studies. Understanding how the rules of inheritance were deduced from crossing experiments sharpens your intuition for interpreting genetic data computationally.

## Mendelian Inheritance

Gregor Mendel (1866) deduced the rules of inheritance from pea plant crosses long before the physical substrate (DNA) was known. His two laws remain foundational:

**Law of Segregation**: Each individual carries two alleles for each gene; during gamete formation, the alleles segregate equally so each gamete carries one allele. This is a consequence of meiosis — homologous chromosomes separate into different daughter cells.

**Law of Independent Assortment**: Alleles of different genes assort independently into gametes (when on different chromosomes or far apart on the same chromosome). This is a consequence of the random orientation of bivalents at meiosis I.

### Monohybrid and Dihybrid Crosses

For a single locus with dominant allele A and recessive allele a:
- Cross Aa × Aa → 1 AA : 2 Aa : 1 aa (genotypic)
- Phenotypic ratio: **3 dominant : 1 recessive**

For two independent loci (A/a and B/b), crossing AaBb × AaBb:
- Phenotypic ratio: **9 A_B_ : 3 A_bb : 3 aaB_ : 1 aabb**

Deviations from these expected ratios are the signal for epistasis, linkage, or lethality.

### Chi-Squared Test for Genetic Ratios

When an observed ratio deviates from the expected, the **chi-squared ($\chi^2$) test** determines if the deviation is significant:

$$\chi^2 = \sum_i \frac{(O_i - E_i)^2}{E_i}$$

where $O_i$ are observed and $E_i$ are expected counts. Degrees of freedom = (number of classes - 1). A $\chi^2$ value exceeding the critical value at $p = 0.05$ rejects the null hypothesis of Mendelian ratios.

**Example**: Cross expected to give 9:3:3:1 (total N = 400). Expected: 225, 75, 75, 25. Observed: 220, 80, 70, 30.

$$\chi^2 = \frac{(220-225)^2}{225} + \frac{(80-75)^2}{75} + \frac{(70-75)^2}{75} + \frac{(30-25)^2}{25} = 0.11 + 0.33 + 0.33 + 1.00 = 1.77$$

With 3 df, critical $\chi^2_{0.05} = 7.82$. Since 1.77 < 7.82, we fail to reject 9:3:3:1 — the data are consistent with independent assortment.

## Epistasis

**Epistasis** occurs when one gene masks the phenotypic expression of another. It distorts the expected 9:3:3:1 dihybrid ratio:

| Epistasis type | Phenotypic ratio | Example |
|---|---|---|
| Dominant epistasis | 12:3:1 | Squash color |
| Recessive epistasis | 9:3:4 | Labrador coat color |
| Duplicate dominant | 15:1 | Some flavor traits |
| Duplicate recessive | 9:7 | Sweet pea flower color |

Labrador coat color: gene A determines pigment deposition (A_ = pigment; aa = albino); gene B determines pigment type (B_ = black; bb = brown/chocolate). But aa is epistatic: aa dogs are yellow regardless of B genotype.

Epistasis is not just a classical genetics concept — it has deep implications for fitness landscapes and why evolution is unpredictable. When genes interact, the effect of one mutation depends on the genetic background. This is why you cannot simply sum the effects of mutations to predict the fitness of a multi-mutant genotype, and why protein engineering is harder than it looks.

## Penetrance and Expressivity

Two modifiers qualify Mendelian rules:

- **Penetrance**: the fraction of individuals with a genotype who show the associated phenotype. BRCA1 mutations are ~70% penetrant for breast cancer by age 80. Less than 100% penetrance complicates pedigree analysis.
- **Expressivity**: the degree to which a phenotype is expressed among affected individuals. Neurofibromatosis type 1 has highly variable expressivity — patients with the same NF1 mutation range from mild café-au-lait spots to severe neurofibromas.

## Linkage and Genetic Mapping

Genes on the same chromosome are **linked** and tend to be inherited together. But during meiosis, **recombination (crossing-over)** can separate linked alleles. The frequency of recombination between two loci defines their **genetic distance** in **centimorgans (cM)**: 1 cM = 1% recombination frequency.

Morgan's mapping function (Haldane):

$$\theta = \frac{1}{2}(1 - e^{-2d})$$

where $\theta$ is the observed recombination fraction and $d$ is the map distance in Morgans. This corrects for double crossovers.

Modern genetic maps are built from millions of SNPs genotyped in pedigrees or large populations. Linkage disequilibrium (LD) — the non-random association of alleles at nearby loci — decays with recombination distance. Haplotype blocks (regions of high LD) are typically 10–100 kb in humans.

## Complementation Tests

When two recessive mutants with similar phenotypes are crossed, the F1 offspring reveal whether the mutations are in the same gene:

- **Complement** (wild-type phenotype): mutations are in different genes; each parent contributes a functional copy of the gene mutated in the other
- **Fail to complement** (mutant phenotype): mutations are in the same gene; no functional copy is present

Formally, a **complementation group** defines a gene. This was used to catalog the genes involved in phage T4 assembly (rII region; Benzer's fine-structure mapping), bacteriophage development, and human genetic diseases.

## Why This Matters for Computational Biology

Classical genetics underlies the entire logic of genome-wide association studies (GWAS): the null model is Mendelian segregation at each locus, and deviations (association with phenotype) are detected by chi-squared or logistic regression tests. Linkage analysis still drives gene mapping for rare Mendelian diseases. The concept of epistasis has a direct computational analog in the design of synthetic circuits — orthogonal (non-interacting) components are the circuits-design equivalent of unlinked, non-epistatic genes. Complementation logic is applied in two-hybrid screens, CRISPR screens, and functional genomics to assign genes to pathways. Understanding Mendelian segregation quantitatively is the starting point for all population genetic inference.
