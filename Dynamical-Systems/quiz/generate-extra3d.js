#!/usr/bin/env node
'use strict';
const fs = require('fs');
const path = require('path');

const BASE = path.join(__dirname, 'questions');

const META = {
  33:{part:6,dir:'part-vi-frontiers',ch:'ch33-orbit-equivalence'},
  34:{part:6,dir:'part-vi-frontiers',ch:'ch34-sofic-entropy'},
  35:{part:6,dir:'part-vi-frontiers',ch:'ch35-isomorphism-problem'},
  36:{part:6,dir:'part-vi-frontiers',ch:'ch36-zimmer-program'},
  37:{part:6,dir:'part-vi-frontiers',ch:'ch37-complex-dynamics-frontiers'},
  38:{part:6,dir:'part-vi-frontiers',ch:'ch38-quantum-information-complexity'},
  39:{part:6,dir:'part-vi-frontiers',ch:'ch39-one-shot-information-theory'},
  40:{part:6,dir:'part-vi-frontiers',ch:'ch40-circuit-complexity-and-it'},
  41:{part:7,dir:'part-vii-connections',ch:'ch41-collatz-dynamics'},
  42:{part:7,dir:'part-vii-connections',ch:'ch42-quantum-computing-connections'},
  43:{part:7,dir:'part-vii-connections',ch:'ch43-hott-connections'},
};

const EXTRA = {
  33: [
    {
      q: "The cost of a countable Borel equivalence relation E (Levitt 1995) is c(E)=inf∑_i μ(A_i) where the infimum is over graphings (generating families {(A_i,B_i,φ_i)} of partial bijections). For the orbit equivalence relation of a free ergodic action of F_n (free group on n generators), the cost satisfies:",
      opts:["c(E_{F_n})=n","c(E_{F_n})=n−1","c(E_{F_n})=1","c(E_{F_n})=∞"],
      ans:0,
      exp:"Gaboriau's theorem: for a free ergodic action of F_n, the cost c(E)=n (the number of generators). More precisely, c(F_n)=n as an invariant of the group. For ℤ-actions, c=1. Cost is a complete invariant distinguishing free actions of free groups of different ranks."
    },
    {
      q: "Connes' embedding problem (CEP) asks whether every II₁ factor embeds into an ultrapower R^ω of the hyperfinite II₁ factor R. The connection to sofic groups is that if G is sofic, then the von Neumann algebra L(G) (the group von Neumann algebra) satisfies:",
      opts:["L(G) is hyperfinite","L(G) embeds into R^ω (Elek-Szabó)","L(G) is a type III factor","L(G) is isomorphic to L(F₂)"],
      ans:1,
      exp:"Elek-Szabó proved that if G is sofic, then L(G) embeds into an ultrapower R^ω of the hyperfinite II₁ factor. This is a positive instance of CEP. MIPRE=RE (Ji et al. 2020) implies CEP is false for general II₁ factors."
    },
    {
      q: "Popa's deformation-rigidity theory uses the concept of relative property (T). A II₁ factor M has property (T) relative to a subalgebra P⊆M if every M-P bimodule with almost invariant unit vectors has actual P-invariant vectors. This is used to prove:",
      opts:["Classification of Bernoulli actions by their base measures","Strong rigidity: the group and the action can be recovered from the equivalence relation","Existence of uncountably many non-isomorphic II₁ factors","All of the above (different applications of deformation-rigidity)"],
      ans:3,
      exp:"Popa's deformation-rigidity program yields multiple results: (1) W*-superrigidity of Bernoulli actions of property (T) groups, (2) OE-superrigidity (recovering the group and action from the equivalence relation), (3) constructing continuously many non-isomorphic II₁ factors via Bernoulli actions."
    },
    {
      q: "The equivalence relation E_t (t-equivalence, for t>0) is defined by x E_t y iff x−y∈tℤ. These are hyperfinite equivalence relations parameterized by t, and they are all:",
      opts:["Isomorphic as Borel equivalence relations regardless of t","Mutually non-isomorphic as measured equivalence relations for different t","The same as E_0 for all t","Smooth for t>0 and non-smooth for t=0"],
      ans:1,
      exp:"The E_t equivalence relations (for irrational t>0 and rational t separately) give a family of hyperfinite equivalence relations parameterized up to measure equivalence. For irrational t, E_t is orbit equivalent to the irrational rotation by t; different t give non-orbit-equivalent actions of ℤ (if the rotation numbers are in different orbits of GL(2,ℤ))."
    },
    {
      q: "The index theory of inclusions of II₁ factors (Jones 1983) defines [M:N]∈{4cos²(π/n): n≥3}∪[4,∞] for a subfactor N⊆M. The Jones index is computed as:",
      opts:["[M:N]=dim_N(L²(M))","[M:N]=dim(M)−dim(N)","[M:N]=the spectral radius of the principal graph","[M:N]=dim_{L(F_n)}(M)"],
      ans:0,
      exp:"Jones' index [M:N]=dim_{N-right}(L²(M)) is the Murray-von Neumann dimension of L²(M) as a right N-module. The restriction to values {4cos²(π/n)}∪[4,∞] is Jones' index theorem, with the gap (1,4) being excluded values."
    },
    {
      q: "Dye's theorem (1959) states that any two free ergodic probability-measure-preserving ℤ-actions are orbit equivalent. The generalization to amenable groups by Ornstein-Weiss (1980) states:",
      opts:["Any two free ergodic pmp amenable group actions are orbit equivalent","Amenable group actions have the same entropy as ℤ-actions","Amenable groups have cost 1","All free ergodic pmp actions of the same amenable group are orbit equivalent"],
      ans:0,
      exp:"Ornstein-Weiss (1980): any two free ergodic pmp actions of countably infinite amenable groups are orbit equivalent. Combined with Dye, this means orbit equivalence does not distinguish between free ergodic actions of any two amenable groups — a remarkable rigidity/flexibility dichotomy."
    },
    {
      q: "The measured equivalence (ME) of groups Γ and Λ (Gromov 1993) means they admit commuting free measure-preserving actions on a standard probability space (Ω,m) as principal torsion-free couplings. Furman proved that property (T) groups have the following ME rigidity:",
      opts:["Property (T) groups are ME-rigid: any ME with a group Λ implies Λ is virtually isomorphic to Γ","Property (T) prevents any ME","ME of two property (T) groups implies they are isomorphic","Property (T) is a ME invariant but not ME rigidity"],
      ans:0,
      exp:"Furman's ME rigidity for property (T) groups: if a property (T) group Γ is ME to Λ, then Λ is virtually isomorphic to Γ. This combines Popa's cocycle superrigidity with structural properties of the ME coupling. ME rigidity is a strong form of group rigidity."
    },
    {
      q: "The Borel cardinality of the isomorphism relation on countable groups is classified by Shelah-Thomas (2001). The isomorphism relation on countable groups is Borel reducible to which equivalence relation?",
      opts:["E_0 (tail equivalence)","The universal countable Borel equivalence relation E_∞","The graph isomorphism relation","It is not a Borel equivalence relation (analytic but not Borel)"],
      ans:1,
      exp:"Shelah-Thomas (2001): the isomorphism relation ≅_G on countable groups is a complete analytic (Σ¹₁) equivalence relation, Borel bi-reducible with the universal countable Borel equivalence relation E_∞. It is more complex than all countable Borel equivalence relations under ≤_B."
    }
  ],
  34: [
    {
      q: "Bowen's sofic entropy h_Σ(f,μ) for a free pmp action of a sofic group Γ (with sofic approximation Σ) is defined as a limit of exponential growth rates of microstate spaces. For Bernoulli actions of amenable groups, sofic entropy:",
      opts:["Equals the Shannon entropy of the base measure for any sofic approximation","Depends on the sofic approximation Σ","Equals zero for all Bernoulli actions","Equals the topological entropy of the shift"],
      ans:0,
      exp:"For Bernoulli actions of amenable groups (where sofic entropy reduces to classical Kolmogorov-Sinai entropy by the Ornstein-Weiss theorem), the sofic entropy h_Σ(f,μ)=H(μ_base) equals the Shannon entropy of the base measure, independent of the sofic approximation."
    },
    {
      q: "Hayes' theorem (2018) on the f-invariant and sofic entropy states that for a free ergodic action of F_n on (X,μ), the f-invariant f(F_n↷X,μ) and the sofic entropy h_Σ are related by:",
      opts:["h_Σ=f invariant/(n−1)","h_Σ=f invariant for all sofic approximations Σ","h_Σ≥f invariant with equality for Bernoulli actions","f invariant=(n−1)·h_Σ+(1−n)·H(μ_base) — Hayes' formula"],
      ans:3,
      exp:"Hayes' formula (2018): for a free pmp F_n-action, h_Σ=(n−1)h_Σ(Bernoulli)+(f-inv) where the f-invariant is an intrinsic invariant. More precisely, f(α)=(1−n)H(base)+h_Σ(α) for Bernoulli α, and Hayes showed the sofic entropy determines the f-invariant."
    },
    {
      q: "The sofic entropy of a Bernoulli action Γ↷(A^Γ,p^⊗Γ) is h=H(p)=−∑_a p(a)log p(a) for all sofic groups Γ with any sofic approximation. This implies:",
      opts:["Bernoulli actions of the same entropy are always isomorphic for sofic groups","Two Bernoulli actions with different base entropies are not isomorphic for free actions of sofic groups","The isomorphism problem for Bernoulli actions of sofic groups is open","Sofic entropy equals Kolmogorov complexity for Bernoulli actions"],
      ans:1,
      exp:"Since sofic entropy is an isomorphism invariant and equals H(p) for Bernoulli actions (Bowen 2010), two Bernoulli Γ-actions with different base entropies are non-isomorphic. This extends Ornstein's theorem beyond amenable groups."
    },
    {
      q: "A group Γ is sofic (Gromov 1999, Weiss 2000) if it admits a sofic approximation: a sequence of maps σ_i:Γ→Sym(V_i) with |V_i|→∞ such that the maps are asymptotically free and multiplicative. Which class of groups is known to be sofic?",
      opts:["Only amenable groups","Only residually finite groups","All amenable groups, all residually finite groups, and their extensions and limits (but no non-sofic group is known)","Only virtually free groups"],
      ans:2,
      exp:"Known sofic groups include: all amenable groups, all residually finite groups, all groups with sofic normal subgroups where the quotient is sofic (extension), direct products and free products of sofic groups, limits of sofic groups. No non-sofic group is currently known, but it is widely believed they exist."
    },
    {
      q: "The Austin-Host-Kra structure theorem for multiple recurrence says that the limits in the nonconventional ergodic averages (1/N)∑_{n=0}^{N-1} f_1∘T^n·f_2∘T^{2n}·...·f_k∘T^{kn} are characterized by the Host-Kra nilfactor structure. This implies Szemerédi's theorem because:",
      opts:["The nilfactors have positive entropy","For f_1=...=f_k=1_A with μ(A)>0, the limit is positive (by the structure theorem and polynomial Hales-Jewett)","The averages converge to ∫f_1 dμ·...·∫f_k dμ","The Furstenberg tower is finite"],
      ans:1,
      exp:"The Host-Kra structure theorem shows the L² limit of (1/N)∑f_1∘T^n...f_k∘T^{kn} equals the average over a nilfactor (nilsystem). By Bergelson-Leibman polynomial Hales-Jewett and the structure theorem, for A with μ(A)>0, lim inf(1/N)∑μ(A∩T^{-n}A∩...∩T^{-kn}A)>0, giving AP's."
    },
    {
      q: "The topological sofic entropy h_{top,Σ}(f) for a continuous action of a sofic group Γ on a compact metric space X (with sofic approximation Σ) satisfies the variational principle:",
      opts:["h_{top,Σ}(f)=sup_μ h_Σ(f,μ) over invariant measures μ","h_{top,Σ}(f)=h_top(f) (equals the classical topological entropy)","h_{top,Σ}(f)≤sup_μ h_Σ(f,μ) with possible inequality","h_{top,Σ}(f)=0 for all non-amenable groups"],
      ans:0,
      exp:"The sofic variational principle (Chung-Li 2015, Huang-Ye 2012): h_{top,Σ}(f)=sup_μ h_Σ(f,μ) for any sofic approximation Σ. This extends the classical variational principle to sofic groups, with the sofic topological entropy equaling the supremum of measure-theoretic sofic entropies."
    },
    {
      q: "For actions of a non-amenable group Γ, the sofic entropy h_Σ(f,μ) can depend on the sofic approximation Σ. This non-uniqueness was demonstrated by:",
      opts:["Bowen (2010) for free groups","Austin (2016) constructing actions where different sofic approximations give different entropies","Kerr-Li (2011) for SL(3,ℤ)","The phenomenon is only theoretical, no explicit examples exist"],
      ans:1,
      exp:"Austin (2016) constructed the first explicit examples of actions where sofic entropy depends on the sofic approximation: two sofic approximations of F₂ give different entropy values for the same action. This shows sofic entropy is not intrinsic to the action in general."
    },
    {
      q: "The Pinsker algebra Π(T) of an ergodic pmp transformation T is the largest T-invariant σ-algebra with zero entropy relative to the trivial σ-algebra. A system has completely positive entropy (CPE) iff:",
      opts:["h(T)=log 2","Π(T) is trivial (equals the trivial σ-algebra mod μ)","T is Bernoulli","T is mixing and has positive entropy"],
      ans:1,
      exp:"T has completely positive entropy (CPE) iff Π(T)={∅,X} mod μ: the Pinsker algebra is trivial, meaning no non-trivial factor has zero entropy. Every Bernoulli shift is CPE (Pinsker-Rokhlin), but CPE does not imply Bernoulli (Ornstein-Shields counterexamples)."
    }
  ],
  35: [
    {
      q: "The isomorphism problem for ergodic measure-preserving transformations asks whether two given systems are measurably isomorphic. For Bernoulli shifts, Ornstein's theorem (1970) gives a complete classification, reducing the problem to:",
      opts:["Comparing topological entropies","Comparing Kolmogorov-Sinai entropy (a complete invariant for Bernoulli shifts)","Computing the Pinsker algebra","Checking if both are weakly mixing"],
      ans:1,
      exp:"Ornstein's theorem: entropy H is a complete invariant for Bernoulli shifts. Two Bernoulli shifts are isomorphic iff they have equal entropy. The proof introduces the 'd-bar metric on processes and shows Bernoulli shifts are the d-bar closure of i.i.d. processes."
    },
    {
      q: "The Halmos-von Neumann theorem characterizes ergodic systems with pure point spectrum (discrete spectrum). An ergodic pmp system (X,μ,T) has discrete spectrum iff it is measurably isomorphic to:",
      opts:["A Bernoulli shift","An ergodic rotation on a compact abelian group","A mixing system","A K-automorphism"],
      ans:1,
      exp:"Halmos-von Neumann (1942): an ergodic pmp system has discrete spectrum (Koopman operator has an orthonormal basis of eigenfunctions) iff it is isomorphic to an ergodic rotation on a compact metric group. The group of eigenvalues is a complete invariant for discrete-spectrum systems."
    },
    {
      q: "Thouvenot's relative isomorphism theorem extends Ornstein's theory to relative (conditional) isomorphisms. It states that two Bernoulli extensions of the same base system (Z,ν,S) with the same relative entropy are:",
      opts:["Relatively isomorphic over Z","Absolutely isomorphic (as total systems)","Non-isomorphic unless the base is trivial","Isomorphic iff the total entropy matches"],
      ans:0,
      exp:"Thouvenot's theorem: Bernoulli extensions of the same base with equal relative entropy are relatively isomorphic (isomorphic as extensions, fixing the base). This extends Ornstein's theory to the conditional setting and is used in the classification of factors."
    },
    {
      q: "Ratner's measure classification theorem for unipotent flows: if U is a unipotent one-parameter subgroup of a connected Lie group G, and Γ is a lattice, then every ergodic U-invariant probability measure on G/Γ is:",
      opts:["Lebesgue measure","The unique measure of maximal entropy","Homogeneous (algebraic): the unique measure supported on a closed orbit H·x for a closed subgroup H","A Gibbs measure"],
      ans:2,
      exp:"Ratner's theorem (1991): every U-ergodic invariant measure μ on G/Γ is algebraic (homogeneous): μ=m_{Hx} for a closed subgroup H⊇U and base point x, where m_{Hx} is the H-invariant measure on the closed orbit H·x. This resolves Raghunathan's conjecture."
    },
    {
      q: "The d-bar (d̄) metric on stationary processes is defined by d̄(X,Y)=inf_{joining μ} P(X_0≠Y_0) over all joinings of the two processes. Ornstein's criterion for Bernoullicity (finitely determined processes) states T is Bernoulli iff:",
      opts:["T has zero entropy","T is isomorphic to a Markov chain","For every ε>0 and partition P, there exist N and δ such that d̄-closeness of two processes to (T,P) on N steps implies d̄-closeness of the processes to each other","T is the d̄-limit of i.i.d. processes"],
      ans:2,
      exp:"Finitely determined (FD) processes (Ornstein): T is FD iff small d̄-distance on a large block plus entropy matching implies small d̄-distance overall. FD is equivalent to Bernoulli (Ornstein's theorem). This gives a computable criterion for Bernoullicity."
    },
    {
      q: "The Pinsker factor of an ergodic system T is the largest zero-entropy factor. For a K-automorphism (Kolmogorov automorphism), the Pinsker factor is:",
      opts:["The trivial factor","The maximal factor","The Bernoulli factor","The identity transformation"],
      ans:0,
      exp:"A K-automorphism has completely positive entropy (CPE): every nontrivial factor has positive entropy. Therefore the Pinsker factor (largest zero-entropy factor) is the trivial factor {∅,X}. K-automorphisms are CPE, but CPE does not imply K in general (though for smooth systems, the implications are better understood)."
    },
    {
      q: "The entropy theory approach to the isomorphism problem breaks into: (1) systems with zero entropy (not classified by entropy), (2) systems with positive entropy. For positive entropy, the key subclass of exactly entropy-classifiable systems beyond Bernoulli are:",
      opts:["All mixing systems","Loosely Bernoulli (LB) systems (Feldman 1976)","Systems with discrete spectrum","K-automorphisms"],
      ans:1,
      exp:"Feldman's loosely Bernoulli systems (1976): a system is LB if it is the image of a Bernoulli shift under a finitary isomorphism (bounded coding). LB systems include all Bernoulli shifts, and two LB systems are isomorphic iff they have the same entropy. Non-LB zero-entropy systems require different invariants."
    },
    {
      q: "Goodman's theorem (extending Krieger's generator theorem) states that every ergodic pmp system (X,μ,T) with h_μ(T)<log N has a generating partition with N atoms. The proof uses:",
      opts:["Zorn's lemma","The Rokhlin lemma and approximation of atoms by Rokhlin towers","The spectral theorem for unitary operators","Direct construction via entropy calculations"],
      ans:1,
      exp:"Krieger's generator theorem (proved via Rokhlin-Sinai, refined by Goodman) uses the Rokhlin lemma to construct towers covering most of X, then approximates partition atoms within these towers. The bound h<log N is sharp (cannot generate with fewer atoms when entropy=log N, e.g., Bernoulli shift on N symbols)."
    }
  ],
  36: [
    {
      q: "The Zimmer conjecture (1986) asks whether lattices Γ in higher-rank simple Lie groups G (e.g., SL(n,ℝ), n≥3) can act by diffeomorphisms on compact manifolds of dimension less than the real rank of G. The Brown-Fisher-Hurtado theorem (2022) proves:",
      opts:["All Γ-actions on low-dimensional manifolds are trivial","Lattices in SL(n,ℝ) (n≥3) cannot act faithfully by C^∞ diffeomorphisms on manifolds of dimension < n−1","All Γ-actions are volume-preserving","Lattices in rank-1 groups can act on any manifold"],
      ans:1,
      exp:"BFH (2022): lattices Γ in SL(n,ℝ) for n≥3 have no faithful C^∞ (or even C^{1+α}) action on compact manifolds of dimension ≤n−2. This proves the Zimmer conjecture in the smooth category for SL(n,ℝ)-lattices. The dimension bound n−1 is sharp."
    },
    {
      q: "Property (T) of Kazhdan for a locally compact group G means every unitary representation with almost invariant vectors has actual invariant vectors. The key examples of property (T) groups include:",
      opts:["All abelian groups","SL(n,ℝ) for n≥3 and their lattices (e.g., SL(n,ℤ))","Free groups F_n for n≥2","All amenable groups"],
      ans:1,
      exp:"Kazhdan's property (T): SL(n,ℝ) for n≥3 has (T) (Kazhdan 1967). Their lattices (e.g., SL(n,ℤ)) inherit (T) as closed subgroups. Abelian groups and free groups do NOT have (T). Amenable groups with (T) are compact."
    },
    {
      q: "The Margulis arithmeticity theorem states that irreducible lattices Γ in semisimple Lie groups G of real rank ≥2 are:",
      opts:["Always free groups","Arithmetic (defined by congruence conditions over number fields)","Always property (T) groups but not necessarily arithmetic","Virtually solvable"],
      ans:1,
      exp:"Margulis arithmeticity (1974): every irreducible lattice in a semisimple Lie group of real rank ≥2 is arithmetic (commensurable to Γ=G(ℤ) for some embedding G→GL(n) over ℤ). This is a major theorem in rigidity theory, using the superrigidity theorem."
    },
    {
      q: "The Furstenberg-Poisson boundary of a random walk on a group G with step distribution μ is the Poisson boundary (B,ν). For the random walk on SL(2,ℝ) with a step distribution having finite first moment, the Poisson boundary is:",
      opts:["A point (trivial)","The projective line P¹(ℝ)=S¹ with the Poisson-hitting measure","The full flag variety","The space of maximal compact subgroups"],
      ans:1,
      exp:"For a non-degenerate probability measure μ on SL(2,ℝ) with finite first moment and not supported on an amenable subgroup, the Poisson boundary is P¹(ℝ)=S¹ equipped with the μ-stationary measure (Furstenberg, Furstenberg-Kaimanovich-Vershik). The boundary encodes the asymptotic direction of the random walk."
    },
    {
      q: "Stuck-Zimmer theorem (1994) classifies invariant random subgroups (IRS) of higher-rank simple Lie groups G. An IRS of G is a conjugation-invariant probability measure on the Chabauty space of closed subgroups Sub(G). For G=SL(n,ℝ), n≥3, every ergodic IRS is:",
      opts:["Supported on lattices or the trivial subgroup","Supported on parabolic subgroups","A point mass on {e} or on a lattice Γ≤G (hence either atomic or of full mass type)","Any closed subgroup"],
      ans:2,
      exp:"Stuck-Zimmer: for a higher-rank simple Lie group G with property (T), every ergodic IRS is either: (1) the point mass on {e}, (2) the point mass on G itself, or (3) supported on a lattice (i.e., induced from an IRS of a lattice). This is a classification of 'random lattices' in G."
    },
    {
      q: "The Ghys theorem on actions of higher-rank lattices on S¹ states that:",
      opts:["SL(n,ℤ) for n≥3 can act freely on S¹","Every action of a lattice in SL(n,ℝ) (n≥3) on S¹ by homeomorphisms factors through a finite quotient","SL(2,ℤ) has no action on S¹","Every countable group acts on S¹"],
      ans:1,
      exp:"Ghys' theorem (1999): every action of an irreducible lattice Γ in a higher-rank semisimple Lie group G on S¹ by orientation-preserving homeomorphisms has a finite orbit. Combined with Margulis normal subgroup theorem, every such action factors through a finite group."
    },
    {
      q: "The Zimmer cocycle superrigidity theorem says that measurable cocycles α:Γ×X→H (for Γ a higher-rank lattice acting on (X,μ) and H a semisimple group) are:",
      opts:["Always coboundaries","Cohomologous to algebraic (homomorphism) cocycles or compact perturbations thereof","Determined by the Lyapunov exponents","Trivial for property (T) groups"],
      ans:1,
      exp:"Zimmer's cocycle superrigidity: for an ergodic pmp action of a higher-rank simple Lie group lattice Γ, any measurable cocycle α:Γ×X→H into a semisimple Lie group H is cohomologous to a cocycle factoring through a homomorphism Γ→H (up to compact perturbation). This is a measurable version of Margulis superrigidity."
    },
    {
      q: "The Borel density theorem states that if Γ is a lattice in a connected semisimple Lie group G (with no compact factors), then Γ is Zariski dense in G. This is used to prove:",
      opts:["The existence of a Haar measure on G","Margulis' normal subgroup theorem (every normal subgroup of Γ is central or of finite index)","The uniqueness of the Poisson boundary","Zimmer's cocycle superrigidity"],
      ans:1,
      exp:"The Borel density theorem (Borel 1960): a lattice Γ in a semisimple G is Zariski dense in G. This is a key ingredient in Margulis' normal subgroup theorem: any normal N◁Γ is either finite (central) or cofinite (finite index), ruling out 'intermediate' normal subgroups."
    }
  ],
  37: [
    {
      q: "The MLC (Mandelbrot locally connected) conjecture states that M is locally connected. It is known to hold for all parameters on the real axis and for finitely renormalizable parameters (Yoccoz). The implication of MLC would be:",
      opts:["The Julia set J_c is always a Jordan curve","The hyperbolic components are dense in M (hyperbolic parameters are dense)","The Hausdorff dimension of M equals 2","All periodic windows are visible in the Mandelbrot set"],
      ans:1,
      exp:"Douady-Hubbard conjectured that MLC is equivalent to the density of hyperbolic dynamics in the family f_c(z)=z²+c (the hyperbolicity conjecture). MLC would imply a combinatorial description of M via laminations (surjection from ∂D onto ∂M)."
    },
    {
      q: "For a polynomial f of degree d, the kneading theory (Milnor-Thurston) encodes the dynamics of the critical orbit via:",
      opts:["The Böttcher coordinate","The kneading invariant: an element of ℤ[[t]]^{d-1} recording the itinerary of the critical values","The Hubbard tree (combinatorial encoding of the Fatou components)","The external angle of the critical value"],
      ans:1,
      exp:"Milnor-Thurston kneading theory: the kneading invariant ν=ν(f)∈ℤ[[t]]^{d-1} encodes the itinerary of each critical value with respect to the partition by critical points. The kneading determinant D(t)=det(I−νI) determines the topological entropy via h_top=log(1/smallest root of D(t)) (Milnor-Thurston-Bernhardt-Irwin)."
    },
    {
      q: "The Douady-Hubbard straightening theorem says every polynomial-like map f:U'→U of degree d is hybrid equivalent to a unique polynomial P_c of degree d. For degree-2 polynomial-like maps, the straightening is:",
      opts:["Any quadratic map z²+c","A unique quadratic polynomial z²+c where c∈M is determined by the filled Julia set of f","The Chebyshev polynomial","Not unique in general"],
      ans:1,
      exp:"Straightening theorem: a degree-2 polynomial-like map f is hybrid equivalent (equal up to quasiconformal conjugacy with ∂μ̄=0 on K_f) to a unique quadratic polynomial z²+c. The straightening map χ: {degree-2 PLMs}/hyb-equiv → M gives the canonical polynomial representative."
    },
    {
      q: "Near a parabolic fixed point of multiplicity m+1 (i.e., f'(z₀)=e^{2πip/q} and (f^q)'(z₀)=1 with multiplier 1), there are exactly 2mq petals (attracting and repelling alternating). The Leau-Fatou theorem gives the dynamics on each attracting petal as conjugate to:",
      opts:["z↦λz (linear contraction)","z↦z/(1+z) (parabolic normal form) or z↦z+z^{m+1} (local model)","z↦z+1 (translation)","z↦z² (squaring map)"],
      ans:1,
      exp:"In each attracting petal, the dynamics is analytically conjugate to z↦z/(1+z^m) (Écalle-Voronin normal form), or topologically to z↦z+1 on the upper half-plane. The parabolic basin consists of 2m attracting petals arranged around z₀, each biholomorphic to a half-plane."
    },
    {
      q: "The Branner-Hubbard tableaux provide a combinatorial tool for studying cubic polynomials with one escaping critical point. The tableau encodes whether each level of the critical orbit is:",
      opts:["In the filled Julia set or escaping","In a Fatou component or Julia set (B for Böttcher component, Y for Yoccoz piece)","Real or complex","Periodic or transient"],
      ans:1,
      exp:"The Branner-Hubbard tableau for a cubic f with one critical point ω₁ escaping to ∞: at each level n, the tableau records whether the image f^n(ω₂) (the other critical point) lies in the Böttcher domain (B) or in a Yoccoz puzzle piece (Y). This combinatorics determines the topology of the Julia set."
    },
    {
      q: "Lyubich's density of hyperbolicity theorem (2002) for real quadratic polynomials states that the set of real parameters c∈[−2,1/4] for which f_c(x)=x²+c is hyperbolic (has an attracting cycle) is:",
      opts:["A finite union of intervals","Dense in [−2,1/4]","Of full measure in [−2,1/4]","Nowhere dense"],
      ans:1,
      exp:"Lyubich (2002): real hyperbolic parameters are dense in [−2,1/4]. The set of non-hyperbolic (stochastic) parameters has measure zero but is topologically 'large' — this is the real version of the hyperbolicity conjecture. The positive measure set is the 'Jakobson parameters' with ACIMs."
    },
    {
      q: "The Multibrot set M_d is the analog of M for f_{c,d}(z)=z^d+c (degree d≥2). As d→∞, the Multibrot sets M_d converge (in the Hausdorff metric on compact subsets of ℂ) to:",
      opts:["The unit disk |c|≤1","The set |c|≤2","The Julia set J_c=∞","A Cantor set"],
      ans:0,
      exp:"As d→∞, M_d={c: f_{c,d}^n(0)↛∞}→{c:|c|≤1} (the closed unit disk). Intuitively, f_{c,d}(0)=c and f_{c,d}²(0)=c^d+c≈c^d for |c|<1 → 0, so the orbit stays bounded iff |c|≤1 in the limit."
    },
    {
      q: "The Fatou-Shishikura inequality (sharp form) for a rational map of degree d: # (attracting cycles) + # (parabolic cycles) + # (Siegel disks) + # (Herman rings) ≤ 2d−2. Equality holds when:",
      opts:["The map is a Lattès example","Each critical point is in the Fatou set and maps to a distinct cycle","Every critical point is periodic","The map has only repelling fixed points"],
      ans:1,
      exp:"Equality 2d−2 is achieved when each of the 2d−2 critical points (counted with multiplicity) is associated to a distinct non-repelling cycle: each attracting/parabolic cycle has exactly one critical point in its immediate basin, each Siegel disk/Herman ring has one critical point on its boundary."
    }
  ],
  38: [
    {
      q: "MIP*=RE (Ji-Natarajan-Vidick-Wright-Yuen, 2020) has the consequence that the Connes embedding problem (CEP) is false. The proof uses:",
      opts:["A direct counterexample to CEP via random matrices","The connection between quantum interactive proofs and CEP via the Kirchberg-QWEP equivalence","Classical circuit lower bounds","The Razborov-Smolensky polynomial method"],
      ans:1,
      exp:"Ji et al. (2020): MIP*=RE (multi-prover interactive proofs with quantum entanglement = recursively enumerable). Via Kirchberg's QWEP theorem, this implies the existence of a II₁ factor not embeddable in R^ω, refuting CEP. The key object is the synchronous correlation set and quantum graph isomorphism."
    },
    {
      q: "The quantum chromatic number χ_q(G) of a graph G is the minimum number of colors in a quantum coloring (using shared entanglement). It is always ≤χ(G) and can be strictly less. The graph homomorphism game shows χ_q(G)≤k iff:",
      opts:["G has a graph homomorphism to K_k","There is a quantum strategy for the graph k-coloring game (projective quantum measurements satisfying coloring constraints)","K_k is a subgraph of G","G is k-colorable by the Lovász theta function"],
      ans:1,
      exp:"χ_q(G)≤k iff there is a perfect quantum strategy for the (G,k)-coloring game: Alice and Bob (distant, sharing entanglement) can answer valid colorings consistently. This uses projective measurements {P^c_v} satisfying ∑_c P^c_v=I and P^c_u·P^c_v=0 if uv∈E(G)."
    },
    {
      q: "Quantum money (Wiesner 1969, revived by Aaronson 2009) proposes banknotes that cannot be counterfeited due to the no-cloning theorem. Public-key quantum money requires:",
      opts:["A secret encoding known only to the bank","A publicly verifiable quantum state that is computationally hard to clone","A classical serial number for each note","Quantum key distribution between bank and merchant"],
      ans:1,
      exp:"Public-key quantum money (Aaronson-Christiano 2012): a quantum state |$⟩ with a classical serial number, such that validity can be checked publicly without communicating with the bank. Security relies on computational hardness of cloning despite public verification. The no-cloning theorem alone is information-theoretic, not computational."
    },
    {
      q: "The random circuit sampling (RCS) hardness conjecture supports quantum supremacy claims. The anti-concentration property of random quantum circuits means:",
      opts:["Output probabilities are concentrated on few outcomes","The distribution of output probabilities approaches the Porter-Thomas distribution (p(x)∼Haar-random)","The circuit depth grows as O(log n)","The circuits can be efficiently simulated classically"],
      ans:1,
      exp:"Anti-concentration: for a uniformly random n-qubit circuit C of sufficient depth, the output probabilities |⟨x|C|0^n⟩|² are approximately Porter-Thomas distributed (exponential with mean 1/2^n). This means most probabilities are neither too large nor too small, making classical sampling hard."
    },
    {
      q: "The HHL (Harrow-Hassidim-Lloyd) algorithm for linear systems Ax=b claims exponential speedup. The caveat that limits practical applications is:",
      opts:["HHL requires O(n) time (same as classical)","The output is a quantum state |x⟩, and extracting classical information about x requires many measurements, erasing the speedup","The algorithm only works for unitary matrices","The algorithm requires O(n²) ancilla qubits"],
      ans:1,
      exp:"HHL prepares |x⟩=A^{-1}|b⟩/‖A^{-1}|b⟩‖ in O(polylog(n)) time (assuming sparse A and quantum access to A,b). But reading all components of x classically requires O(n) measurements, erasing the exponential speedup. HHL is useful only when a specific inner product ⟨x|M|x⟩ is needed."
    },
    {
      q: "The polynomial method in quantum query complexity (Beals-Buhrman-Cleve-Mosca-de Wolf 1998) shows that the quantum query complexity Q(f) of a Boolean function f:{0,1}^n→{0,1} satisfies Q(f)≥deg(f)/2 where deg(f) is the degree of the unique multilinear polynomial representing f. This gives:",
      opts:["Q(f)=O(1) for all constant functions","Q(f)=Θ(√n) for OR_n","Q(f)=Θ(n^{1/3}) for k-SAT","A lower bound of Ω(n^{1/2}) for sorting"],
      ans:1,
      exp:"The polynomial method gives Q(OR_n)≥deg(OR_n)/2=n/2, but Grover's algorithm achieves Q(OR_n)=O(√n). More carefully, the polynomial method gives the exact lower bound deg(OR_n)/2=n/2... wait. Actually deg(OR_n)=n but approximate degree ≈ √n. Q(OR_n)=Θ(√n) by Grover + BBBV lower bound. The polynomial method gives Q≥≈deg/2."
    },
    {
      q: "Quantum expanders are families of quantum channels {Φ_n:M_{d_n}→M_{d_n}} such that for all traceless matrices A: ‖Φ_n(A)‖ ≤ λ‖A‖ for some λ<1. The quantum analog of the expander mixing lemma shows:",
      opts:["Quantum expanders imply classical expanders","Random quantum circuits are quantum expanders with high probability","Quantum expanders are equivalent to classical expanders","Quantum expanders cannot be efficiently constructed"],
      ans:1,
      exp:"Random unitary matrices form 2-designs that are quantum expanders (with λ approaching 0 with depth). Efficient constructions of explicit quantum expanders come from quantum error-correcting codes (Hastings 2007, Ben-Aroya-Ta-Shma). The spectral gap λ<1 implies rapid mixing of the quantum random walk."
    },
    {
      q: "The complexity class QMA (quantum Merlin-Arthur) is the quantum analog of NP. The local Hamiltonian problem (k-LH: given local terms H_i and threshold a<b, is ground energy ≤a or ≥b?) is QMA-complete when k≥2. This was proved by:",
      opts:["Kitaev (1999) for k=5, then refined to k=2 by Kempe-Kitaev-Regev","Shor (1994) as a corollary of factoring","Lloyd-Terhal-Kitaev","Nielsen-Chuang"],
      ans:0,
      exp:"Kitaev (1999) proved 5-LH is QMA-complete using the circuit-to-Hamiltonian construction (Feynman-Kitaev clock construction). Kempe-Kitaev-Regev (2006) reduced to 3-LH, and Oliveira-Terhal (2005) then Hallgren-Nagaj-Narayanaswami proved 2-LH on a line is QMA-complete."
    }
  ],
  39: [
    {
      q: "One-shot entropies generalize Shannon entropy to single-copy settings. The smooth min-entropy H_min^ε(X|Y) and smooth max-entropy H_max^ε(X|Y) satisfy the duality relation:",
      opts:["H_min^ε(X|Y)_ρ + H_max^ε(X|Z)_ρ = 0 where YZ is a purification of XYZ","H_min + H_max = 2H (Shannon entropy)","H_min^ε(X|Y)=H_max^{1-ε}(X|Y)","H_min^ε+H_max^ε≥H(X)"],
      ans:0,
      exp:"Tomamichel-Colbeck-Renner duality: for a pure state |ψ⟩_{XYZ}, H_min^ε(X|Y)_ψ+H_max^ε(X|Z)_ψ=0. This is the one-shot analog of H(X|Y)+H(X|Z)=0 for pure states (where H(X|Z)=−H(X|Y) since I(X;Y)=H(X)+H(Y)−H(XY))."
    },
    {
      q: "The one-shot capacity of a quantum channel N for sending classical information with error ≤ε is characterized by the smooth max-information. The i.i.d. asymptotic limit recovers:",
      opts:["The Holevo capacity χ(N)","The quantum capacity Q(N)","The entanglement-assisted capacity C_E(N)=log d_in + S(N(ρ))","The zero-error capacity"],
      ans:0,
      exp:"By the quantum AEP (Schumacher compression) and HSW theorem, the one-shot capacity C^{1,ε}(N) satisfies C^{1,ε}(N^⊗n)/n→χ(N)=max_{p_x,ρ_x}[S(∑p_xN(ρ_x))−∑p_xS(N(ρ_x))] as n→∞, ε→0. This recovers the Holevo capacity as the asymptotic classical capacity."
    },
    {
      q: "The second-order coding rate for reliable transmission over a DMC at rate R and blocklength n is (1/2)log n·V/n (where V is the channel dispersion). This means the maximal rate with error ≤ε satisfies:",
      opts:["R*(n,ε)=C+O(1/√n)","R*(n,ε)=C−√(V/n)·Q^{-1}(ε)+O(log n/n)","R*(n,ε)=C·(1−ε)","R*(n,ε)=C−1/n"],
      ans:1,
      exp:"The second-order result (Polyanskiy-Poor-Verdú 2010, Hayashi 2009): R*(n,ε)=C−√(V/n)·Q^{-1}(ε)+O(log n/n) where V=Var_{P*}[log P(Y|X)/P(Y)] is the channel dispersion and Q^{-1} is the inverse Q-function. This gives sharp finite-blocklength corrections."
    },
    {
      q: "The resource theory of entanglement studies entanglement under LOCC (local operations and classical communication). A key result is that the distillable entanglement E_D and entanglement cost E_C satisfy:",
      opts:["E_D=E_C=E_F (entanglement of formation) for all states","E_D≤E_F≤E_C with strict inequalities for mixed states (bound entanglement possible)","E_D=E_C for all pure states and E_D<E_C for mixed states","E_D>E_C is possible"],
      ans:1,
      exp:"For pure states, E_D=E_C=E_F=S(ρ_A) (the Schmidt rank entropy). For mixed states, E_D≤E_F≤E_C in general, with strict inequalities (bound entangled states: E_D=0 but E_C>0). The Horodecki bound entanglement states are the key examples where E_D<E_C."
    },
    {
      q: "Position-based quantum cryptography (Chandran-Fehr-Gelles-Goyal-Ostrovsky 2009) attempts to verify a party's physical location. The unconditional security of most position-verification protocols is broken by:",
      opts:["The no-cloning theorem","Entanglement attacks: sharing pre-distributed EPR pairs allows attackers to simulate any position","Classical attacks using collision resistance","Quantum key distribution"],
      ans:1,
      exp:"Chandran et al. showed that any single-round quantum position-verification protocol can be broken using O(2^n) EPR pairs shared between attackers. The attack uses quantum teleportation and port-based teleportation. Unconditional secure position-based QKD remains open without computational assumptions."
    },
    {
      q: "The quantum reverse Shannon theorem (Bennett-Harrow-Morgan-Smolin 2005, later refined) states that any quantum channel N can be simulated using entanglement and classical communication, at a rate of:",
      opts:["χ(N) bits of classical communication + E_F(N) ebits","log d_in bits per channel use","S(N) bits without entanglement","C_E(N) classical bits using log d_in ebits"],
      ans:0,
      exp:"The quantum reverse Shannon theorem: N can be simulated (in the i.i.d. sense) using χ*(N) classical bits + E_F(N) ebits of entanglement, where χ*(N) is the entanglement-assisted classical capacity divided by 2. The tight statement involves the quantum channel's entanglement cost."
    },
    {
      q: "The hypothesis testing relative entropy D_H^ε(ρ||σ) (the one-shot analog of KL divergence) satisfies: in the limit of many i.i.d. copies, (1/n)D_H^ε(ρ^⊗n||σ^⊗n)→",
      opts:["D_KL(ρ||σ) for ε∈(0,1)","D_KL(ρ||σ)^2 (squared KL divergence)","The quantum Rényi divergence at α=1","0 for ε>0"],
      ans:0,
      exp:"The quantum Stein's lemma (Ogawa-Nagaoka 2000): for ε∈(0,1), (1/n)D_H^ε(ρ^⊗n||σ^⊗n)→D_KL(ρ||σ)=tr[ρ(log ρ−log σ)] as n→∞. This is the one-shot-to-asymptotic bridge for quantum hypothesis testing."
    },
    {
      q: "The thermal machine resource theory restricts free operations to Gibbs-preserving maps (maps that preserve the thermal equilibrium state γ=e^{-βH}/Z). The work value W of transforming ρ to σ (both states on the same Hamiltonian) is:",
      opts:["W=F(ρ)−F(σ) where F=U−TS (free energy)","W=S(ρ)−S(σ) (entropy difference)","W=tr[H(ρ−σ)]","W=D_KL(ρ||γ)−D_KL(σ||γ) (relative entropy to thermal)"],
      ans:3,
      exp:"In the resource theory of thermodynamics (Brandao-Horodecki-Oppenheim-Renes-Spekkens 2013): work W=k_BT·[D_KL(ρ||γ)−D_KL(σ||γ)] is the amount of work extractable when converting ρ→σ using thermal operations. D_KL(·||γ)=β(F(·)−F(γ)) is proportional to free energy above thermal equilibrium."
    }
  ],
  40: [
    {
      q: "The natural proofs barrier (Razborov-Rudich 1994) shows that most known lower bound techniques cannot prove P≠NP. A proof technique is 'natural' if the property P is: (1) constructive (polynomial-time decidable), (2) large (dense in the set of all functions), and (3) useful (implies lower bounds). The barrier states:",
      opts:["Natural proof techniques can prove P≠NP","If one-way functions exist, then no natural proof technique can separate P from NP","P≠NP implies the existence of natural proofs","Natural proofs exist iff P=NP"],
      ans:1,
      exp:"Razborov-Rudich (1994): if OWFs exist (a standard cryptographic assumption), then no natural proof technique can prove super-polynomial circuit lower bounds. This explains why diagonalization, random restrictions, and algebraic methods fail: they are natural and OWFs would imply Boolean pseudo-randomness blocking them."
    },
    {
      q: "The information complexity IC(f,ε) of a function f is the minimum amount of information Alice and Bob must reveal about their inputs during any ε-error protocol. For the equality function EQ_n, IC(EQ_n,0) satisfies:",
      opts:["IC(EQ_n,0)=n (equals the input length)","IC(EQ_n,0)=O(log n)","IC(EQ_n,0)=0 (no information needed)","IC(EQ_n,0)=Θ(n) for deterministic protocols, 0 for randomized"],
      ans:0,
      exp:"The information complexity of exact equality IC(EQ_n,0)=n bits: any deterministic protocol for EQ_n that errs with probability 0 must reveal all n bits of mutual information (since the worst-case distribution makes x=y the hardest case). For randomized protocols with error, IC(EQ_n,ε)=O(log(1/ε))."
    },
    {
      q: "The Razborov monotone circuit complexity lower bound technique (approximations method) shows that the clique function CLIQUE_k requires exponential-size monotone circuits when k=n^{1/4}. The method uses:",
      opts:["Counting arguments","Approximating AND and OR gates by low-degree polynomials over GF(2)","Sunflower lemma and combinatorial designs","Replacing monotone circuits with approximating monotone boolean functions via random restriction"],
      ans:2,
      exp:"Razborov's approximation method (1985): approximate AND by a union-closed family and OR by an intersection-closed family. Tracking the error through the circuit using sunflower lemma arguments, the composition of approximators for CLIQUE requires exponential size, proving super-polynomial monotone circuit lower bounds."
    },
    {
      q: "The Khrapchenko bound for formula size is Khrap(f)=max_{A×B: f(A∩B)=0} |{(a,b)∈A×B: a,b differ in 1 bit}|²/(|A|·|B|). For PARITY on n bits, this gives formula size lower bound:",
      opts:["n","n²","2^n","Ω(n^2)"],
      ans:1,
      exp:"Khrapchenko's bound applied to PARITY_n gives L(PARITY_n)≥n²: take A={0-heavy strings}, B={1-heavy strings}, the number of adjacent pairs is n·2^{n-1}, |A|=|B|=2^{n-1}, so Khrap≥(n·2^{n-1})²/(2^{n-1})²=n². Combined with the O(n^2) formula for PARITY using linear recursion, this is tight."
    },
    {
      q: "The Williams ACC⁰ lower bound (2011) showed NEXP⊄ACC⁰ (nondeterministic exponential time cannot be computed by constant-depth circuits with modular gates). The proof uses:",
      opts:["Diagonalization alone","An accelerated circuit SAT algorithm for ACC⁰ circuits, combined with the time hierarchy theorem","The natural proofs barrier (avoided by the specific structure of ACC⁰)","Oracle separations"],
      ans:1,
      exp:"Williams' technique: if NEXP⊆ACC⁰, then there is a fast (subexponential) SAT algorithm for ACC⁰ circuits (using the Barrington theorem and Nisan-Wigderson PRGs). But the time hierarchy theorem says SAT cannot be solved in subexponential nondeterministic time. Contradiction gives NEXP⊄ACC⁰."
    },
    {
      q: "The log-rank conjecture (Lovász-Saks 1988) states that the deterministic communication complexity D(f) of a Boolean function f satisfies D(f)=O(log^c(rank(M_f))) where M_f is the communication matrix. The best current upper bound (without the conjecture) is:",
      opts:["D(f)≤rank(M_f)+1 (trivially)","D(f)=O(√(rank(M_f))·log(rank(M_f))) (Lovett 2016)","D(f)=O(log(rank(M_f))^2)","D(f)=rank(M_f) (conjecture is proven)"],
      ans:1,
      exp:"The best unconditional upper bound is Lovett (2016): D(f)≤O(√(rank(M_f))·log(rank(M_f))), proving D(f)≤Õ(√r) where r=rank(M_f). The log-rank conjecture would give D(f)=O(polylog(r)), a major open problem separating the two sides by O(polylog) vs √."
    },
    {
      q: "The XOR lemma (Yao-Levin-Nisan) states that if a function f is (1/2+δ)-hard for randomized circuits of size S, then the XOR of k independent copies f^{⊕k} is (1/2+2^{-Ω(k)})-hard for circuits of size:",
      opts:["S^k","S/polylog(k)","S·poly(k,1/δ)","Ω(S·k)"],
      ans:1,
      exp:"The XOR lemma for computational hardness amplification: hard function ↦ harder XOR. If f is (1/2+δ)-hard for size S, then f^{⊕k} is (1/2+2^{-Ω(kδ²)})-hard for size S/polylog(k) circuits. The hardness amplifies exponentially at the cost of a polynomial loss in circuit size."
    },
    {
      q: "Algebraic circuit complexity studies arithmetic circuits computing polynomials. The depth-reduction theorem (Agrawal-Vinay, Koiran, Tavenas) shows that any size-s depth-d circuit can be converted to a depth-4 circuit of size:",
      opts:["2^{O(√(d log s))}","s^{O(1)}","2^{O(d)}","s^{d/2}"],
      ans:0,
      exp:"Depth reduction (Tavenas 2013 improvement): any polynomial of degree n computed by size-s algebraic circuit can be computed by a homogeneous ΣΠΣΠ (depth-4) circuit of size s^{O(√n)}=2^{O(√n log s)}. This reduces VP vs VNP to depth-4 lower bounds; exponential depth-4 lower bounds for explicit polynomials would separate VP and VNP."
    }
  ],
  41: [
    {
      q: "The Collatz map T:ℕ→ℕ is T(n)=n/2 if n even, 3n+1 if n odd. The '3x+1 problem' asks whether all n eventually reach 1. Terras (1976) showed the stopping time σ(n) (first k with T^k(n)<n) satisfies for almost all n (in the density sense):",
      opts:["σ(n)=O(log n)","σ(n)→∞ for all n","σ(n)~c·log n for a constant c","σ(n) is unbounded"],
      ans:2,
      exp:"Terras' theorem: for almost all n (in the sense of natural density), the stopping time σ(n)~(log(4/3)/log(4/3))·log n ≈ (2log 2/log(3/2))·log n·c. More precisely, σ(n)/log n converges in distribution to a law with specific constants. This is a probabilistic analysis treating Collatz as a random walk."
    },
    {
      q: "The 2-adic analysis of Collatz: the Collatz map extends to a map T_2:ℤ_2→ℤ_2 on the 2-adic integers. The 2-adic Collatz map has the following property regarding periodic orbits in ℤ_2:",
      opts:["No periodic orbits exist in ℤ_2\\ℕ","The 2-adic integers contain uncountably many periodic orbits","There are exactly 3 periodic orbits in ℤ_2: {0}, {-1}, and {-5,-7,-10,-14,...}","The map is ergodic with respect to Haar measure on ℤ_2"],
      ans:3,
      exp:"The Collatz map extends to T:ℤ_2→ℤ_2 (2-adic integers). The map is measure-preserving and ergodic with respect to the Haar measure on ℤ_2 (Lagarias). This is consistent with the 'random' behavior of the Collatz map. The 2-adic perspective predicts the heuristics but does not resolve the problem for ℕ."
    },
    {
      q: "The probabilistic heuristic for Collatz: treating each step as independent, the expected multiplication factor per step is (1/2)·(1/2)+(1/2)·(3/2+1/2)≈3/4. Since 3/4<1, this heuristic predicts the orbit eventually decreases. The flaw in using this to prove Collatz is:",
      opts:["The factor is actually >1","Steps are not independent — correlations in the 2-adic expansion create dependencies","The expectation should be taken over a different probability space","This correctly proves all orbits are finite"],
      ans:1,
      exp:"The heuristic assumes independence of even/odd at each step, but the parity pattern of the Collatz orbit depends on the binary expansion of n in a complex correlated way. Formalizing independence requires controlling the distribution of T^k(n) mod 2, which is precisely what makes the problem hard."
    },
    {
      q: "Tao's 2019 result on the Collatz map (almost all orbits eventually reach below any function f(n)→∞) uses:",
      opts:["The Riemann hypothesis","Ergodic theory of the 2-adic integers combined with analytic number theory (exponential sums)","Pure combinatorics","Fourier analysis over finite fields"],
      ans:1,
      exp:"Tao (2019): for any f(n)→∞, almost all n (in the logarithmic density sense) have a Collatz iterate T^k(n)≤f(n). The proof uses the Fourier/2-adic analysis of the Collatz iteration combined with the circle method from analytic number theory, controlling exponential sums over Collatz orbits."
    },
    {
      q: "The Collatz graph has vertex set ℕ and directed edge n→T(n). The conjecture is equivalent to: the Collatz graph has a single connected component (rooted at 1) when we consider the graph of the inverse map. The height function h(n) (minimum k with T^k(n)=1) has been verified for n up to approximately:",
      opts:["10^9","2^68 (Oliveira e Silva 2010)","10^100","2^1000"],
      ans:1,
      exp:"Oliveira e Silva's computational verification (2010): all n≤2^68≈2.95×10^20 eventually reach 1. This is the largest verified range. The verification uses a tree-search algorithm exploiting the structure of the Collatz graph."
    },
    {
      q: "The Collatz-Syracuse function C:ℕ_odd→ℕ_odd sends an odd n to the odd part of 3n+1 (the next odd number in the Collatz orbit). The density result of Bourgain-like methods shows that the orbit of almost all n under C:",
      opts:["Is eventually periodic with period 1 (the trivial cycle)","Decreases on average: the logarithmic mean of log|C^k(n)| decreases","Increases without bound","Is equidistributed modulo any fixed m"],
      ans:1,
      exp:"By heuristic and partial rigorous analysis, the orbit of the odd Collatz function decreases on average (since log|C(n)|≈log n+log(3/4) on average in the heuristic), though proving this for specific n remains open. Tao's result makes this rigorous in an almost-all sense."
    },
    {
      q: "The connection between the Collatz conjecture and automata theory: the Collatz function on binary representations acts as a transducer. The key computational observation is:",
      opts:["Collatz is computable by a pushdown automaton","The function n↦T(n) is computed by a finite 2-state automaton operating on the binary representation of n from least significant bit","Collatz is not computable","Collatz can be simulated by a tag system (Neary-Woods)"],
      ans:3,
      exp:"Neary-Woods showed the Collatz function can be simulated by a simple tag system (a type of sequential string rewriting system shown Turing-complete by Cocke-Minsky). This means the Collatz conjecture is related to the halting problem for tag systems, explaining its undecidability-like difficulty."
    },
    {
      q: "In the dynamical systems interpretation, the Collatz map (extended to ℤ[1/6]) generates a dynamical system related to a specific affine map on ℝ/ℤ. The associated circle map is x↦{2x} (doubling map) when n is even and x↦{2x/3+1/3} when odd, giving the connection to:",
      opts:["The Gauss map for continued fractions","Expanding circle maps and their Markov properties","The doubling map's chaos explaining Collatz difficulty","The logistic map at a specific parameter"],
      ans:1,
      exp:"The Collatz map is equivalent to an affine map on ℤ_2 (2-adic view) or can be analyzed as a non-invertible expanding map on ℝ/ℤ. The connection to Markov maps with Markov partitions {[0,1/2],[1/2,1]} (even/odd) is the basis for the probabilistic heuristics — the induced map has specific Lyapunov exponents."
    }
  ],
  42: [
    {
      q: "Shor's factoring algorithm uses the quantum phase estimation subroutine to find the order r of a mod N. The hidden subgroup problem (HSP) for ℤ is solved by measuring the quantum Fourier transform of the coset state |H⟩=|{0,r,2r,...}⟩. The QFT over ℤ_M approximates the continuous Fourier transform and gives period r because:",
      opts:["The QFT diagonalizes the translation operator","The coset state |H⟩ has Fourier transform concentrated on multiples of M/r (the dual subgroup)","The measurement collapses to the period","The QFT is an efficient classical algorithm"],
      ans:1,
      exp:"The coset state |H⟩=(1/√r)∑_{k=0}^{r-1}|kr mod M⟩ has QFT_{M} that is concentrated (with high probability) on multiples of M/r (the Pontryagin dual of rℤ in ℤ_M). Measuring gives a random multiple of M/r, from which r is recovered by continued fractions."
    },
    {
      q: "The quantum threshold theorem (Aharonov-Ben-Or, Knill-Laflamme-Zurek) states that fault-tolerant quantum computation is achievable with a constant error rate below the threshold p_{th}. The threshold for the surface code is approximately:",
      opts:["p_{th}≈1%","p_{th}≈11% (for depolarizing noise)","p_{th}≈50%","p_{th}≈0.1% (only for specific error models)"],
      ans:1,
      exp:"The threshold for fault-tolerant quantum computation under the surface code with depolarizing noise is approximately p_{th}≈10.9−11% (Wang-Fowler-Hollenberg 2011, Dennisetal.). This is remarkably high compared to earlier estimates of ~1%. Below this threshold, concatenation/surface code allows reliable QC with constant physical error rate."
    },
    {
      q: "The quantum approximate optimization algorithm (QAOA) for MaxCut on a graph G with n vertices uses p layers of alternating cost and mixer unitaries. The approximation ratio for p=1 QAOA on 3-regular graphs is known to be at least:",
      opts:["1/2","0.6924... (Farhi-Goldstone-Gutmann 2014)","0.878 (Goemans-Williamson)","Provably better than all classical algorithms"],
      ans:1,
      exp:"QAOA at p=1 on 3-regular MaxCut achieves approximation ratio ≥0.6924 (FGG 2014), better than the random 0.5 but below the GW algorithm's 0.878. For large p, QAOA approaches the optimal, but the practical advantage over classical algorithms for small p remains unclear."
    },
    {
      q: "Quantum volume (Cross et al. 2019) is a metric for quantum computer performance defined as V_Q=2^n where n is the largest number of qubits for which the quantum computer can effectively run a random square circuit. The metric captures:",
      opts:["Only gate fidelity","Only qubit count","A combined figure of merit: qubit count, gate fidelity, connectivity, and compilation efficiency","Only circuit depth"],
      ans:2,
      exp:"Quantum volume integrates multiple performance factors: number of qubits n, gate error rates, qubit connectivity (limiting the effective circuit), readout errors, and compiler quality. V_Q=2^n is the largest random n-qubit circuit the device can execute with >2/3 heavy output probability. It captures the 'useful' computational power holistically."
    },
    {
      q: "The Grover lower bound (Bennett-Bernstein-Brassard-Vazirani 1997) proves that any quantum algorithm requires Ω(√N) queries to find a marked element in an unsorted database of N elements. The proof uses:",
      opts:["Simon's problem reduction","The polynomial method (degree-2 lower bound)","The hybrid method: bounding the inner product change between adjacent states","The quantum adversary method"],
      ans:2,
      exp:"BBBV used the hybrid argument: define |ψ_i^t⟩ as the state after t queries given the marked element is i. Show ‖|ψ_i^t⟩−|ψ_j^t⟩‖ is small until t=Ω(√N) queries, bounding the probability of finding the marked element below a constant."
    },
    {
      q: "Variational quantum eigensolvers (VQE) compute ground state energies using a parameterized quantum circuit |ψ(θ)⟩ and minimize E(θ)=⟨ψ(θ)|H|ψ(θ)⟩. The barren plateau problem states that for large n, the gradient ∂E/∂θ_i:",
      opts:["Grows exponentially with n","Vanishes exponentially with n (exponentially flat landscape)","Is constant independent of n","Can be computed efficiently classically"],
      ans:1,
      exp:"Barren plateaus (McClean-Boixo-Smelyanskiy-Babbush-Neven 2018): for deep parameterized circuits on n qubits, the gradient ∂E/∂θ_i has exponentially small variance (Var[∂E/∂θ_i]∈O(2^{-n})). This makes gradient-based optimization exponentially difficult for large n, a fundamental challenge for VQE."
    },
    {
      q: "The Clifford group on n qubits (generated by H, S, CNOT) is efficiently classifiable by the Gottesman-Knill theorem: circuits using only Clifford gates can be simulated classically in:",
      opts:["O(n) time","O(n²) time (tracking stabilizers via the symplectic representation)","O(2^n) time","O(n³) time"],
      ans:1,
      exp:"The Gottesman-Knill theorem: any Clifford circuit on n qubits (with Clifford gates, Pauli measurements, and stabilizer initial states) can be simulated classically in O(n²) time using the stabilizer tableau representation (each stabilizer generator tracked as a row in an n×2n binary matrix)."
    },
    {
      q: "Quantum error correction via the toric code (Kitaev 2003) encodes 2 logical qubits in a lattice of n physical qubits on a torus. The code distance (minimum weight of undetectable error) grows as:",
      opts:["O(1)","O(log n)","O(√n)","O(n)"],
      ans:2,
      exp:"The toric code on an L×L lattice (n=2L² physical qubits) encodes 2 logical qubits with distance d=L=O(√n). This means Θ(√n) physical errors are needed to cause a logical error, giving [[n,2,√n]] code parameters. The threshold ~11% applies when errors are corrected using minimum-weight matching."
    }
  ],
  43: [
    {
      q: "In homotopy type theory (HoTT), the univalence axiom (Voevodsky) states that the canonical map (A=B)→(A≃B) is an equivalence (where = is the identity type and ≃ is equivalence of types). The key consequence is:",
      opts:["All types are sets","Equivalent types are equal (can be substituted for each other in any context)","Types form a category (not a ∞-groupoid)","The law of excluded middle holds"],
      ans:1,
      exp:"Univalence: (A=B)≃(A≃B). The consequence is that isomorphic mathematical structures are literally equal as types, so all properties are invariant under equivalence. This formalizes mathematical practice (treating isomorphic objects as equal) and implies function extensionality."
    },
    {
      q: "In HoTT, the identity type Id_A(a,b) generalizes equality. Paths (elements of Id_A(a,b)) compose, and the type Id_A(a,b) itself has identity types Id_{Id_A(a,b)}(p,q) (homotopies between paths). This gives types an ∞-groupoid structure where:",
      opts:["All identity types are propositions (mere propositions)","Types are sets (all paths are equal)","Types carry higher-dimensional homotopy information (h-levels can be arbitrarily high)","Only trivial identity types are coherent"],
      ans:2,
      exp:"The ∞-groupoid interpretation of types: terms are points, elements of Id_A(a,b) are paths, elements of Id_{Id_A(a,b)}(p,q) are homotopies, etc. ad infinitum. A type is a set (h-level 2) if all paths are equal, a groupoid (h-level 3) if all 2-paths are equal, etc. General types have no bound on h-level."
    },
    {
      q: "The Brunerie number β (computed in HoTT by Brunerie's thesis 2016) is defined as the generator of π₄(S³)≅ℤ/βℤ as computed in HoTT. The value of β is:",
      opts:["0","1","2","Undefined (π₄(S³) is trivial in HoTT)"],
      ans:2,
      exp:"Brunerie computed β=2 in HoTT, confirming π₄(S³)≅ℤ/2ℤ. This is a non-trivial computation entirely within HoTT: the Brunerie element maps to the generator 2·ι of π₄(S³), and the proof that the order is 2 was completed computationally (Ljungström-Mörtberg 2023 gave the first fully formal proof)."
    },
    {
      q: "Higher inductive types (HITs) in HoTT allow defining types by both point constructors and path constructors. The circle S¹ is defined as a HIT with one point constructor base:S¹ and one path constructor loop:base=base. The fundamental group π₁(S¹) in HoTT is:",
      opts:["ℤ (proven formally by Licata-Shulman 2013)","ℤ/2ℤ","Trivial","Depends on the model"],
      ans:0,
      exp:"Licata-Shulman (2013) formally proved π₁(S¹)=ℤ in HoTT using the encode-decode method. The proof uses the covering space (universal cover) approach: define a map Encode:S¹→Type taking base to ℤ and loop to the succ equivalence, then show encode and decode are inverses."
    },
    {
      q: "Cubical type theory (Cohen-Coquand-Huber-Mörtberg 2016) provides a constructive model of HoTT. The key innovation is the inclusion of an interval type 𝕀=[0,1] in the theory with endpoints 0,1:𝕀. Paths are elements of type (x:𝕀)→A and the computational benefit is:",
      opts:["Univalence is derivable and computable (not just an axiom)","The law of excluded middle holds computationally","All types are decidable","The theory is inconsistent"],
      ans:0,
      exp:"Cubical type theory: univalence is a theorem (not an axiom) that satisfies the computation rules. The interval 𝕀 and path type (𝕀→A) allow defining composition structures that make the theory compute. This gives a computational interpretation where univalence reduces via β-reduction."
    },
    {
      q: "The Freudenthal suspension theorem in HoTT states that for an n-connected type X (πᵢ(X)=0 for i≤n), the suspension map π_k(X)→π_{k+1}(ΣX) is an isomorphism for k≤2n and surjective for k=2n+1. This is used in HoTT to prove:",
      opts:["π_n(Sⁿ)=ℤ for all n≥1","π_n(Sⁿ)=ℤ for n≥2 (using S^{n-1} is (n-2)-connected and induction)","π₁(S¹)=ℤ directly","The homotopy groups of spheres vanish above the dimension"],
      ans:1,
      exp:"By Freudenthal, if Sⁿ⁻¹ is (n-2)-connected (true since πᵢ(Sⁿ⁻¹)=0 for i<n-1), then πₙ(Sⁿ)≅πₙ₋₁(Sⁿ⁻¹) for n≥2. By induction (π₁(S¹)=ℤ from Licata-Shulman), πₙ(Sⁿ)=ℤ for all n≥1. Freudenthal gives the inductive step."
    },
    {
      q: "In HoTT, the statement LEM (law of excluded middle) ∀A:Prop.A∨¬A and function extensionality FunExt are related to each other and to univalence as follows:",
      opts:["LEM implies FunExt which implies univalence","Univalence implies FunExt; LEM is independent of univalence and consistent with HoTT","FunExt implies LEM","Univalence is inconsistent with LEM"],
      ans:1,
      exp:"In HoTT: univalence implies function extensionality (FunExt: f=g iff ∀x.f(x)=g(x)) as a theorem. LEM is independent of HoTT with univalence — both LEM and ¬LEM are consistent with HoTT (different models satisfy or violate LEM). LEM does not follow from univalence and does not contradict it."
    },
    {
      q: "The van Kampen theorem in HoTT (Seifert-van Kampen for higher inductive types) computes π₁ of a pushout A∪_C B using the fundamental groupoid. In the HoTT version (Shulman 2013), this is proved using:",
      opts:["Classical logic and path induction","The encode-decode method applied to the pushout HIT with path constructors","The Freudenthal suspension theorem","Cubical type theory exclusively"],
      ans:1,
      exp:"The HoTT van Kampen theorem: for a pushout A∪_C B (defined as a HIT with point constructors from A and B and path constructors from C), π₁(A∪_C B) is computed via the universal property of the pushout groupoid. The encode-decode method tracks which path in A or B corresponds to each path in the pushout."
    }
  ]
};

for(const [chStr, questions] of Object.entries(EXTRA)){
  const ch = parseInt(chStr);
  const {part,dir,ch:chDir} = META[ch];
  const chPath = path.join(BASE, dir, chDir);
  const existing = fs.readdirSync(chPath).filter(f=>f.endsWith('.json')).sort();
  const chTitle = JSON.parse(fs.readFileSync(path.join(chPath,existing[0]),'utf8')).chTitle;
  let idx = existing.length + 1;
  for(const {q,opts,ans,exp} of questions){
    const num = String(idx).padStart(2,'0');
    const obj = {ch, part, chTitle, q, opts, ans, exp};
    const fp = path.join(chPath, `q${num}.json`);
    fs.writeFileSync(fp, JSON.stringify(obj, null, 2));
    console.log(`wrote ${path.relative(path.join(__dirname,'questions'), fp)}`);
    idx++;
  }
}
console.log(`\nDone. ${Object.values(EXTRA).reduce((s,a)=>s+a.length,0)} questions written.`);
