#!/usr/bin/env node
'use strict';
const fs = require('fs');
const path = require('path');

const BASE = path.join(__dirname, 'questions');

const META = {
  21:{part:3,dir:'part-iii-information-theory',ch:'ch21-quantum-information-theory'},
  22:{part:4,dir:'part-iv-bridges',ch:'ch22-entropy-in-dynamical-systems'},
  23:{part:4,dir:'part-iv-bridges',ch:'ch23-ergodic-information-theory'},
  24:{part:4,dir:'part-iv-bridges',ch:'ch24-symbolic-dynamics-and-information'},
  25:{part:4,dir:'part-iv-bridges',ch:'ch25-chaos-randomness-computation'},
  26:{part:4,dir:'part-iv-bridges',ch:'ch26-information-methods-in-cs'},
  27:{part:5,dir:'part-v-foundations',ch:'ch27-computability-and-dynamics'},
  28:{part:5,dir:'part-v-foundations',ch:'ch28-category-theory-and-dynamics'},
  29:{part:5,dir:'part-v-foundations',ch:'ch29-thermodynamics-and-information'},
  30:{part:5,dir:'part-v-foundations',ch:'ch30-optimal-transport'},
  31:{part:5,dir:'part-v-foundations',ch:'ch31-ergodic-number-theory'},
  32:{part:5,dir:'part-v-foundations',ch:'ch32-descriptive-set-theory'},
};

const EXTRA = {
  21: [
    {
      q: "The quantum capacity Q of a degraded quantum channel N is given by the LSD (Lloyd-Shor-Devetak) theorem as Q=max_ρ I_c(ρ,N) where I_c is the coherent information. The coherent information is defined as:",
      opts:["I_c(ρ,N)=S(N(ρ))−S(N⊗id(Φ_ρ))","I_c(ρ,N)=S(N(ρ))−S(ρ)","I_c(ρ,N)=I(A;B)_ρ","I_c(ρ,N)=S(B)−S(AB) in the state (id⊗N)(|Φ⟩⟨Φ|)"],
      ans:3,
      exp:"The coherent information I_c(ρ,N)=S(B)−S(E) in the isometric extension, where E is the environment. For a degraded channel (degradable), Q=max_ρ I_c(ρ,N) is additive and gives the quantum capacity."
    },
    {
      q: "The no-cloning theorem follows from the linearity of quantum mechanics. The proof: if U|ψ⟩|0⟩=|ψ⟩|ψ⟩ for all states |ψ⟩, then applying to |ψ⟩=|0⟩+|1⟩/√2 yields a contradiction because:",
      opts:["Unitarity is violated","U(|0⟩+|1⟩)|0⟩/√2=(|00⟩+|11⟩)/√2 ≠ (|0⟩+|1⟩)²/2=(|00⟩+|01⟩+|10⟩+|11⟩)/2","The Hilbert space dimension is wrong","Normalization fails"],
      ans:1,
      exp:"By linearity, U(|0⟩+|1⟩)|0⟩/√2=(U|0⟩|0⟩+U|1⟩|0⟩)/√2=(|00⟩+|11⟩)/√2. But cloning requires (|0⟩+|1⟩)⊗(|0⟩+|1⟩)/2=(|00⟩+|01⟩+|10⟩+|11⟩)/2. These are different states."
    },
    {
      q: "Quantum teleportation transmits a qubit |ψ⟩ from Alice to Bob using one ebit (Bell pair) and 2 classical bits. The classical communication is necessary because without it Bob's state is:",
      opts:["The maximally mixed state ρ=I/2","The state |ψ⟩ with probability 1/4","An entangled state","Orthogonal to |ψ⟩"],
      ans:0,
      exp:"Without the 2-bit classical message, Bob holds the reduced state of his half of a Bell pair, which is the maximally mixed state I/2 regardless of |ψ⟩. The classical bits tell Bob which Pauli correction to apply."
    },
    {
      q: "The quantum error correction conditions (Knill-Laflamme) state that a code space C corrects errors {E_a} iff for all |ψ⟩,|φ⟩∈C:",
      opts:["⟨ψ|E_a†E_b|φ⟩=C_{ab}⟨ψ|φ⟩ for some Hermitian matrix C","⟨ψ|E_a|φ⟩=0 for all a≠0","E_a|ψ⟩=0 for all |ψ⟩∈C","⟨ψ|E_a†E_b|ψ⟩=δ_{ab}"],
      ans:0,
      exp:"Knill-Laflamme conditions: ⟨ψ|E_a†E_b|φ⟩=C_{ab}⟨ψ|φ⟩ where C is error-independent Hermitian matrix. The 'non-degenerate' case has C diagonal. These are necessary and sufficient for the code to correct the error set {E_a}."
    },
    {
      q: "The Stinespring dilation theorem states that every quantum channel N:B(H)→B(K) has a representation N(ρ)=tr_E[V ρ V†] where V:H→K⊗E is an isometry. The dimension of E can be taken to be:",
      opts:["dim(H)·dim(K)","dim(H)²","dim(K)²","dim(H)·dim(K)² (Kraus rank squared)"],
      ans:0,
      exp:"By the Stinespring theorem, the environment dimension satisfies dim(E)≤dim(H)·dim(K). The Kraus rank (minimal number of Kraus operators) equals the minimal dim(E), which is at most dim(H)·dim(K)."
    },
    {
      q: "Quantum discord Q(A|B)=I(A:B)−J(A:B) where J(A:B)=max_{Π_B} [H(A)−H(A|{Π_B^k})] is the classical correlation. For a pure bipartite state |ψ_{AB}⟩:",
      opts:["Q(A|B)=0","Q(A|B)=S(A)=S(B)=entanglement entropy","Q(A|B)=I(A:B)/2","Q(A|B)=H(A)"],
      ans:1,
      exp:"For a pure state, I(A:B)=2S(A) (twice the entanglement entropy) and J(A:B)=S(A) (classical correlations equal one copy of entanglement entropy by optimal measurement on B). So Q(A|B)=2S(A)−S(A)=S(A)."
    },
    {
      q: "The accessible information I_acc of an ensemble {(p_x,ρ_x)} is bounded above by the Holevo quantity χ=S(∑_x p_x ρ_x)−∑_x p_x S(ρ_x). This bound is:",
      opts:["Achievable by projective measurements only","Achievable asymptotically using collective measurements on many copies","Not achievable in general","Equal to I_acc for orthogonal states only"],
      ans:1,
      exp:"The Holevo bound χ is an upper bound on accessible information. Holevo-Schumacher-Westmoreland (1997) show χ is achievable as the capacity of the classical-quantum channel {(p_x,ρ_x)} using collective (entangled) measurements on multiple copies, not necessarily product measurements."
    },
    {
      q: "For the depolarizing channel N_p(ρ)=(1−p)ρ+p·I/2 on a qubit, the quantum capacity Q(N_p) is positive for p<p_* where p_* is approximately:",
      opts:["1/4","1/3","1/2","3/4"],
      ans:1,
      exp:"The depolarizing channel's quantum capacity Q>0 for p<1/3 (since the channel is antidegradable at p=1/3 making Q=0). The exact threshold p_*≈0.2523 (numerically), but Q>0 for p below approximately 1/3 and Q=0 for p≥1/3."
    }
  ],
  22: [
    {
      q: "The Pesin-Pitskel topological pressure P(f,φ) for a continuous map f on a compact metric space and potential φ is defined using (n,ε)-separated sets. The variational principle states:",
      opts:["P(f,φ)=sup_μ {h_μ(f)+∫φ dμ}","P(f,φ)=h_top(f)+sup_μ ∫φ dμ","P(f,φ)=h_top(f)+‖φ‖_∞","P(f,φ)=sup_μ h_μ(f)·∫φ dμ"],
      ans:0,
      exp:"The variational principle for topological pressure: P(f,φ)=sup_{μ invariant} {h_μ(f)+∫φ dμ}. The supremum is achieved at equilibrium states. Setting φ=0 recovers h_top(f)=sup_μ h_μ(f) (Goodman-Dinaburg-Goodwyn)."
    },
    {
      q: "For an Anosov diffeomorphism, the measure of maximal entropy μ_MME satisfies h_{μ_MME}(f)=h_top(f). The MME for the hyperbolic toral automorphism induced by A∈SL(2,ℤ) is:",
      opts:["The Lebesgue measure (uniform)","The Parry measure (equal-weight measure on periodic orbits)","The SRB measure","A measure supported on the expanding foliation"],
      ans:0,
      exp:"For hyperbolic toral automorphisms, the Lebesgue (Haar) measure is both the SRB measure AND the measure of maximal entropy (since the Jacobian is 1 and the system is conservative). This is special to toral automorphisms."
    },
    {
      q: "The Abramov-Rokhlin formula for entropy of skew products gives, for the extension (X×Y, f̃) where f̃(x,y)=(f(x),g_x(y)): h_μ̃(f̃)=h_μ(f)+∫h_μ_x(g_x)dμ(x). This formula is used to compute entropy of:",
      opts:["Suspension flows (Abramov formula)","Group extensions","Both of the above","Direct products only"],
      ans:2,
      exp:"Abramov's formula h(suspension flow)=h(base map)/∫roof function covers suspension flows. The Abramov-Rokhlin formula for skew products covers group extensions T×_φ G. Both are special cases of the general skew product formula."
    },
    {
      q: "The entropy of a Gaussian process with spectral density S(λ) (the entropy rate of the stationary process) is given by the Kolmogorov-Szegő formula:",
      opts:["h = (1/2)∫log S(λ)dλ/(2π)","h = (1/4π)∫log(2πeS(λ))dλ","h = log‖S‖_∞","h = (1/2π)∫S(λ)dλ"],
      ans:1,
      exp:"The entropy rate of a stationary Gaussian process with spectral density S(λ) is h=(1/4π)∫_{-π}^{π}log(2πeS(λ))dλ=(1/2)log(2πe)+(1/4π)∫log S(λ)dλ. This is the Szegő-Kolmogorov formula."
    },
    {
      q: "The Lyapunov exponents of an ergodic measure-preserving map (f,μ) relate to entropy via the Margulis-Ruelle inequality h_μ(f)≤∫∑ max(0,λ_i) dμ. Equality (Pesin formula) holds when:",
      opts:["μ is ergodic","μ is absolutely continuous with respect to Lebesgue measure","μ is an SRB measure (has ACM on unstable manifolds)","f is Anosov"],
      ans:2,
      exp:"Pesin's entropy formula h_μ(f)=∫∑λ_i^+ dμ holds when μ is an SRB measure (with absolutely continuous conditional measures on unstable manifolds). For a general invariant measure, only the Ruelle inequality ≤ holds."
    },
    {
      q: "Dimensional theory of dynamical systems connects Lyapunov exponents to dimensions via the Ledrappier-Young formula. For an ergodic hyperbolic measure μ, the Hausdorff dimension of μ satisfies dim_H(μ)=∑ h_i/|λ_i| where:",
      opts:["h_i are partial entropies along the i-th unstable foliation and λ_i are the corresponding Lyapunov exponents","h_i=h_μ(f)/n for all i","h_i are the Brin-Katok local entropies","The formula only holds for conformal maps"],
      ans:0,
      exp:"Ledrappier-Young (1985): dim_H(μ)=∑_i h_i/|λ_i| where h_i are the partial entropies (entropy production per unstable direction, satisfying ∑h_i=h_μ). For conformal maps, h_i=h_μ·|λ_i|/∑|λ_j|, simplifying to Bowen's formula."
    },
    {
      q: "The topological entropy of a continuous map f:X→X can be computed via the growth rate of (n,ε)-spanning sets s(n,ε). The entropy is h_top(f)=lim_{ε→0} lim sup_{n→∞} (1/n)log s(n,ε). An alternative definition due to Misiurewicz-Szlenk for piecewise monotone interval maps gives:",
      opts:["h_top(f)=lim (1/n)log(number of monotone pieces of fⁿ)","h_top(f)=log(spectral radius of the transition matrix)","h_top(f)=∫log|f'|dμ_{MME}","All three are equivalent for piecewise monotone maps"],
      ans:3,
      exp:"For piecewise monotone interval maps, all three formulas give the same topological entropy: (1/n)log(lap number of fⁿ), log(spectral radius of kneading/transition matrix), and ∫log|f'|dμ_{MME} (Parry-Milnor-Thurston). They are equivalent by Misiurewicz-Szlenk theory."
    },
    {
      q: "Bowen's theorem on periodic orbits: for a topologically mixing Anosov map, the number of fixed points of fⁿ grows as e^{n·h_top(f)}. More precisely, |Fix(fⁿ)|/e^{n·h}→:",
      opts:["1","0","∞","A constant C depending on the SRB measure"],
      ans:0,
      exp:"For a mixing Anosov map, |Fix(fⁿ)|~e^{nh} as n→∞, i.e., |Fix(fⁿ)|/e^{nh}→1. The equidistribution of periodic points: the uniform measures on Fix(fⁿ) converge weak* to the measure of maximal entropy μ_{MME}."
    }
  ],
  23: [
    {
      q: "Shannon-McMillan-Breiman theorem: for an ergodic stationary process {X_n} with entropy rate h, (−1/n)log P(X_1,...,X_n)→h almost surely. This is the AEP (asymptotic equipartition property). The theorem implies that the size of the typical set T_n^ε satisfies:",
      opts:["|T_n^ε|≈2^{nh}","2^{n(h−ε)}≤|T_n^ε|≤2^{n(h+ε)} and P(T_n^ε)→1","2^{n(h−ε)}≤|T_n^ε|≤2^{n(h+ε)} and P(T_n^ε)→0","Both A and B"],
      ans:1,
      exp:"The AEP: the typical set T_n^ε satisfies (1) P(T_n^ε)→1, (2) |T_n^ε|≤2^{n(h+ε)}, (3) |T_n^ε|≥(1−ε)2^{n(h−ε)} for large n. Both exponential bounds hold simultaneously with high probability."
    },
    {
      q: "The Ziv-Lempel complexity of a sequence x^n, defined as the number of distinct phrases in the sequential parsing, satisfies for a stationary ergodic source with entropy rate h:",
      opts:["c(x^n)/n→h/log n (normalized complexity converges to entropy rate)","c(x^n)·log n/n→h almost surely","c(x^n)/2^{nh}→1","c(x^n)→∞ at rate 2^{nh}"],
      ans:1,
      exp:"Ziv-Lempel complexity: c(x^n)~nh/log n for a stationary ergodic source with entropy rate h. More precisely, c(x^n)·log n/n→h almost surely (Ornstein-Weiss, Ziv-Lempel). This is the basis of LZ78/LZW compression."
    },
    {
      q: "For an ergodic measure-preserving system (X,μ,T) and f,g∈L²(μ), mixing means ∫f·g∘Tⁿ dμ→∫f dμ·∫g dμ. In information-theoretic terms, mixing is equivalent to:",
      opts:["h_μ(T)>0","I(X_0;X_n)→0 where X_0,X_n are observations at time 0 and n","I(X_0;X_n)=h_μ(T) for all n","The process {X_n} having zero entropy rate"],
      ans:1,
      exp:"Mixing means the mutual information between the past and future separated by n steps goes to zero: I(X_0;X_n)→0. For ergodic systems, this is stronger than ergodicity (which only requires I(X_0;X_n)/n→0 in Cesàro sense)."
    },
    {
      q: "The prediction problem: for an ergodic stationary process, the optimal predictor of X_{n+1} from (X_1,...,X_n) achieves mean squared error approaching the conditional variance Var(X_{n+1}|past). For a Gaussian process with spectral density S(λ), the minimum prediction error is:",
      opts:["exp((1/2π)∫log S(λ)dλ)","∫S(λ)dλ","‖S‖_∞","(1/2π)∫log S(λ)dλ"],
      ans:0,
      exp:"Szegő's theorem: the minimum mean squared prediction error for a Gaussian process with spectral density S is σ²_pred=exp((1/2π)∫_{-π}^{π} log S(λ)dλ) (geometric mean of S). The process is deterministic (σ²_pred=0) iff ∫log S(λ)dλ=−∞."
    },
    {
      q: "Ornstein's isomorphism theory characterizes Bernoulli shifts up to measure-theoretic isomorphism. The key result is that two Bernoulli shifts B(p₁,...,p_k) and B(q₁,...,q_m) are isomorphic iff:",
      opts:["They have the same entropy H(p)=H(q)","They have the same alphabet size k=m","Both have positive entropy","Their Perron-Frobenius eigenvalues agree"],
      ans:0,
      exp:"Ornstein's theorem (1970): two Bernoulli shifts are measure-theoretically isomorphic iff they have the same entropy. The entropy H(p)=−∑p_i log p_i is the complete invariant for Bernoulli shifts, answering a long-standing open problem."
    },
    {
      q: "The entropy of a measure-preserving transformation T can be computed from any generating partition P (Kolmogorov-Sinai theorem): h_μ(T)=h_μ(T,P)=lim_{n→∞} (1/n)H(P∨T^{-1}P∨...∨T^{-n+1}P). A partition P is a generator for T when:",
      opts:["P has maximum entropy","The σ-algebra generated by {T^k P: k∈ℤ} equals the full σ-algebra (mod μ)","P refines every other partition","H(P)=h_μ(T)"],
      ans:1,
      exp:"P is a generator for T (or (T,P)-generating) iff ∨_{k∈ℤ} T^k P = B(X) mod μ. Krieger's generator theorem: every ergodic system with h_μ(T)<log N has a generator with N or fewer atoms."
    },
    {
      q: "The capacity of an ergodic channel (a channel with memory governed by an ergodic process) is C=max_{stationary input process} (entropy rate of output − conditional entropy rate H(Y_n|X^n,Y^{n-1}...)). For a finite-state channel (FSC), the capacity satisfies:",
      opts:["C=max_p I(X;Y) for i.i.d. inputs","C equals the capacity of the corresponding memoryless channel with same per-symbol statistics","C=lim_{n→∞}(1/n)max_{p(x^n)} I(X^n;Y^n) requiring optimization over process distributions","C=h(output)−h(noise) with Gaussian inputs"],
      ans:2,
      exp:"For finite-state channels, capacity C=lim(1/n)max I(X^n;Y^n). The maximization is over all input distributions p(x^n) (not just i.i.d.), requiring computation of the process capacity. Computing C is in general difficult (BCJR algorithm computes it for specific cases)."
    },
    {
      q: "The relative entropy (KL divergence) D(μ||ν) between two shift-invariant measures satisfies the asymptotic equipartition property: (1/n)log(dμⁿ/dνⁿ)(x)→D(μ||ν) μ-a.s. (where μⁿ,νⁿ are the n-step marginals). This is used to prove:",
      opts:["The data processing inequality","Stein's lemma: optimal type-II error exponent for hypothesis testing equals D(μ||ν)","The source coding theorem","Shannon-McMillan-Breiman theorem"],
      ans:1,
      exp:"The ergodic theorem applied to log-likelihood ratios gives Stein's lemma: in hypothesis testing μ vs ν, constraining type-I error to ≤ε, the optimal type-II error exponent is D(μ||ν). The AEP for relative entropy is the key technical step."
    }
  ],
  24: [
    {
      q: "For a sofic shift with presentation by a right-resolving labeled graph G, the topological entropy equals log λ_{PF} where λ_{PF} is the Perron-Frobenius eigenvalue of the adjacency matrix A_G. For an SFT, this coincides with:",
      opts:["The entropy of the Parry measure","log(spectral radius of transition matrix)","Both A and B","The entropy of the uniform Bernoulli measure on the alphabet"],
      ans:2,
      exp:"For an SFT, the entropy h=log λ_{PF}(A) where A is the transition matrix, and this equals the entropy of the Parry measure (the unique measure of maximal entropy, which is a Markov measure with specific transition probabilities)."
    },
    {
      q: "The follower set entropy of a word w∈L(X) in a shift space X is h_w(X)=lim_{n→∞}(1/n)log|{u: wu∈L_n(X)}|. For an irreducible sofic shift, h_w(X):",
      opts:["Equals h(X) for all w∈L(X)","Depends on w and can be strictly less than h(X)","Equals 0 for synchronizing words","Is undefined for synchronizing words"],
      ans:0,
      exp:"For an irreducible sofic shift (and more generally, for irreducible SFTs), the follower set entropy h_w(X)=h(X) for every word w in the language. This follows from mixing: long words have the same growth rate of followers as the full shift."
    },
    {
      q: "Adler-Konheim-McAndrew (1965) defined topological entropy for compact spaces. The Misiurewicz-Szlenk extension to noncompact spaces requires replacing covers by:",
      opts:["Open sets of finite diameter","Compact sets","Finite sets of orbits","Bowen's (n,ε)-balls"],
      ans:3,
      exp:"For noncompact spaces (Bowen 1973), topological entropy is defined via (n,ε)-separated and (n,ε)-spanning sets using the Bowen metric d_n(x,y)=max_{0≤k≤n-1}d(f^k(x),f^k(y)). The Adler-Konheim-McAndrew open cover definition requires compactness."
    },
    {
      q: "The shift equivalence problem for SFTs asks whether shift equivalence (A~B iff ∃R,S: AR=RB, SA=BS, RS=Aⁿ for some n) implies strong shift equivalence. Kim-Roush (1992) showed that:",
      opts:["Shift equivalence implies strong shift equivalence","Shift equivalence does not imply strong shift equivalence (counterexample over ℤ)","The two notions coincide over positive matrices","The question is still open"],
      ans:1,
      exp:"Kim-Roush (1992) proved that shift equivalence does NOT imply strong shift equivalence by constructing explicit counterexamples. This resolved a problem left open in Williams' 1973 paper and showed the NSSE (not strong shift equivalent) phenomenon."
    },
    {
      q: "The Gurevich entropy of a countable-state Markov shift with transition matrix A is h_G(A)=log(spectral radius), where the spectral radius is computed via:",
      opts:["h_G=lim_{n→∞}(1/n)log A^n_{ii} (Gurevich's definition using return probabilities)","h_G=sup_μ h_μ(σ)","h_G=log‖A‖_{ℓ²→ℓ²}","All three coincide for irreducible Markov shifts"],
      ans:3,
      exp:"For irreducible countable-state Markov shifts (Gurevich 1969): the topological entropy equals the Gurevich pressure h_G=lim(1/n)log A^n_{ii}=log r(A)=sup_μ h_μ(σ), where r(A) is the spectral radius. All three definitions coincide (unlike the transient/null recurrent case)."
    },
    {
      q: "A synchronized (word) in a sofic shift X is a word w such that for all u,v with uw,wv∈L(X), we have uwv∈L(X). Sofic shifts with a synchronizing word are called:",
      opts:["SFTs","Synchronized shifts (in the sense of Blanchard-Hansel)","Fischer covers","Mixing sofic shifts"],
      ans:1,
      exp:"Synchronized shifts (Blanchard-Hansel 1991) are sofic shifts that have at least one synchronizing word. Every SFT has a synchronizing word (any allowed word of length ≥ the memory). Synchronized shifts properly contain SFTs and are contained in sofic shifts."
    },
    {
      q: "The dimension group of an irreducible SFT (A:ℤⁿ→ℤⁿ) is the direct limit G=lim_{→}(ℤⁿ,A). This is a complete invariant for flow equivalence when equipped with:",
      opts:["Its group structure only","Its ordered group structure (G,G⁺) where G⁺ is the positive cone","Its metric structure","The image of the standard basis vectors"],
      ans:1,
      exp:"Parry-Sullivan (1975): the dimension group (G,G⁺,[1]) with its ordered group structure and distinguished order unit [1] is a complete invariant for flow equivalence of irreducible SFTs. This completes the classification up to flow equivalence."
    },
    {
      q: "For a substitution dynamical system generated by σ:A→A* (over finite alphabet A), the topological entropy h(X_σ) equals:",
      opts:["0 for all primitive substitutions","log|A| (full shift entropy)","log λ where λ is the Perron-Frobenius eigenvalue of the substitution matrix","The entropy of the substitution measure"],
      ans:0,
      exp:"Every primitive substitution system is minimal (uniquely ergodic) with zero topological entropy. The exponential growth of |σⁿ(a)| is governed by the PF eigenvalue, but this describes length growth, not complexity growth—the language complexity grows subexponentially."
    }
  ],
  25: [
    {
      q: "Wolfram's computational equivalence principle (Class IV cellular automata) posits that simple systems can achieve universal computation. The elementary CA Rule 110 was proved Turing-universal by Cook (2004). The key structure used is:",
      opts:["Glider collisions implementing logical gates","Periodic backgrounds supporting glider computation","Both A and B","The lattice symmetry of Rule 110"],
      ans:2,
      exp:"Cook's proof of Rule 110 universality uses gliders (localized propagating structures) on a periodic background. Glider collisions implement universal computation by simulating a tag system. Both the glider dynamics and the periodic background are essential."
    },
    {
      q: "Bennett and independently Zurek argued that the entropy production of a computational process is related to its logical irreversibility. Landauer's principle states:",
      opts:["Erasing one bit of information requires at least k_B T ln 2 energy dissipation","Computation is always reversible","Entropy production is proportional to the number of bits processed","Only measurement causes entropy production"],
      ans:0,
      exp:"Landauer's principle (1961): logically irreversible operations (erasing one bit of information) must dissipate at least k_B T ln 2 ≈ 3×10^{-21} J at temperature T. This connects information theory to thermodynamics and limits the energy efficiency of computation."
    },
    {
      q: "For a chaotic dynamical system with positive Lyapunov exponent λ>0, the initial condition sensitivity implies that prediction is limited to a horizon ~log(1/δ)/λ where δ is the initial measurement uncertainty. This is:",
      opts:["Polynomial in 1/δ","Logarithmic in 1/δ","Exponential in λ","Independent of λ"],
      ans:1,
      exp:"The prediction horizon T_pred~(1/λ)log(1/δ): for initial uncertainty δ, errors amplify as δ·e^{λt}≈1 after time t~(log(1/δ))/λ. The horizon grows logarithmically slow in the precision 1/δ—improving measurement precision gives diminishing returns."
    },
    {
      q: "Furstenberg's ×2×3 theorem (proved differently by Lyons, Host, Meiri) states that the only Borel probability measure on [0,1] that is invariant and ergodic for both multiplication-by-2 (mod 1) and multiplication-by-3 (mod 1) simultaneously is:",
      opts:["The Lebesgue measure and measures supported on finite orbits","Only the Lebesgue measure","Any self-similar fractal measure","The Gibbs measures for the Gauss map"],
      ans:0,
      exp:"Furstenberg conjectured (1967) that the only ergodic measures simultaneously ×2 and ×3 invariant are Lebesgue and point masses on rational numbers (which have finite orbits). This was proved by Host (1995) and Meiri for specific cases; the full conjecture (Rudolph-Johnson for positive entropy) and alternatives are well-understood."
    },
    {
      q: "The connection between pseudorandomness and one-way functions (Håstad-Impagliazzo-Levin-Luby 1999) shows: pseudorandom generators (PRGs) exist iff:",
      opts:["P≠NP","One-way functions exist","BPP≠P","The discrete logarithm is hard"],
      ans:1,
      exp:"HILL theorem (1999): PRGs (stretching a short seed to many pseudorandom bits) exist if and only if one-way functions exist. This connects two fundamental complexity-theoretic assumptions and shows pseudorandomness is equivalent to computational hardness."
    },
    {
      q: "The effective Hausdorff dimension of a sequence ω∈{0,1}^ω (Lutz 2003) is defined as dim(ω)=lim inf_{n→∞} K(ω|_n)/n where K is Kolmogorov complexity. A normal sequence (in the sense of Borel) has effective dimension:",
      opts:["0","1","1/2","Equal to the algorithmic entropy"],
      ans:1,
      exp:"A Borel-normal sequence ω has K(ω|_n)~n (near-maximal compression), so dim(ω)=lim inf K(ω|_n)/n=1. Sequences with effective dimension 1 include Martin-Löf random sequences. Sequences with dim(ω)<1 have compressible prefixes."
    },
    {
      q: "Brudno's theorem connects topological dynamics and information theory: for a topological dynamical system (X,f) and a generic point x (in the sense of some invariant measure μ), the Kolmogorov complexity K(x_0 x_1...x_{n-1}) of the orbit segment satisfies:",
      opts:["K(x_0...x_{n-1})/n→h_μ(f) almost surely","K(x_0...x_{n-1})/n→h_top(f) always","K(x_0...x_{n-1})/n→0 for periodic orbits and 1 otherwise","K(x_0...x_{n-1})/n oscillates"],
      ans:0,
      exp:"Brudno's theorem (1983): for an ergodic μ and μ-generic x, K(x|_n)/n→h_μ(f) almost surely. The Kolmogorov complexity of orbit segments converges to the measure-theoretic entropy, linking algorithmic complexity to ergodic theory."
    },
    {
      q: "The concept of ε-entropy (Kolmogorov-Tikhomirov metric entropy) for a compact metric space (M,d) counts the minimum number of ε-balls to cover M. For the space of C^k functions on [0,1] with the sup norm, the ε-entropy satisfies:",
      opts:["H_ε~ε^{-1/k}","H_ε~(1/ε)^{1/k}","H_ε~log(1/ε)","H_ε~(1/ε)^k"],
      ans:1,
      exp:"The ε-entropy of the unit ball in C^k[0,1] (k-times continuously differentiable functions with ‖f‖_{C^k}≤1) is H_ε~(1/ε)^{1/k} (Kolmogorov-Tikhomirov 1961). Smoother functions (larger k) have smaller ε-entropy — they occupy 'less' of the ambient space."
    }
  ],
  26: [
    {
      q: "The VC (Vapnik-Chervonenkis) dimension of a hypothesis class H measures its complexity. The fundamental theorem of statistical learning says H is PAC learnable iff:",
      opts:["H is finite","VCdim(H)<∞","H is convex","H has polynomial growth function"],
      ans:1,
      exp:"The fundamental theorem: a hypothesis class H is PAC learnable iff VCdim(H)<∞. The sample complexity m(ε,δ)=O((VCdim(H)+log(1/δ))/ε²) for the ERM algorithm. This is a complete characterization of binary classification learnability."
    },
    {
      q: "The information-theoretic proof of the impossibility of private information retrieval (PIR) with a single server requires:",
      opts:["Communication complexity lower bounds","Shannon's source coding theorem: the server must communicate ≥n bits (the full database)","Kolmogorov complexity arguments","Packing arguments"],
      ans:1,
      exp:"For 1-server PIR: the user wants to retrieve 1 bit x_i of an n-bit database without revealing i to the server. Shannon's theorem implies the server must communicate all n bits (otherwise information about the query index leaks), making 1-server PIR trivially expensive."
    },
    {
      q: "The communication complexity of the equality function EQ_n (Alice has x∈{0,1}^n, Bob has y∈{0,1}^n, they want to determine x=y) has deterministic complexity n but randomized (public coin) complexity:",
      opts:["O(log n)","O(1)","O(√n)","Ω(n)"],
      ans:1,
      exp:"Randomized communication complexity of EQ_n is O(1): use a random hash function H:{0,1}^n→{0,1}^k for k=O(log(1/δ)). Alice sends H(x), Bob checks H(x)=H(y). Error probability δ by collision probability analysis."
    },
    {
      q: "The expander mixing lemma states: for a d-regular graph G with adjacency matrix A having second eigenvalue λ₂≤λ, for any sets S,T⊆V:",
      opts:["|e(S,T)−d|S||T|/n|≤λ√(|S||T|)","e(S,T)≥d|S||T|/n−λ√(|S||T|)","Both A and B","e(S,T)=d|S||T|/n for Ramanujan graphs"],
      ans:2,
      exp:"The expander mixing lemma: |e(S,T)−d|S||T|/n|≤λ√(|S||T|) where e(S,T) is the number of edges between S and T, d is the degree, n=|V|, and λ is the second eigenvalue. This upper-bounds the discrepancy from expected edge count for random graphs."
    },
    {
      q: "The Lovász theta function ϑ(G) for a graph G satisfies ϑ(G)·ϑ(Ḡ)≥n (n=|V|) and gives bounds on:",
      opts:["The chromatic number χ(G)≤ϑ(Ḡ) and independence number α(G)≤ϑ(G)","The clique number ω(G)≤ϑ(G)","Both A and B","The Shannon capacity Θ(G)=ϑ(G) (Shannon capacity equals the Lovász theta function)"],
      ans:3,
      exp:"The Lovász theta function satisfies: α(G)≤ϑ(G)≤χ(Ḡ) and Θ(G)=ϑ(G) for vertex-transitive graphs (Lovász 1979). The Shannon capacity of the pentagon C₅ equals √5=ϑ(C₅), resolved by this method. In general, ϑ(G) sandwiches the Shannon capacity: α(G)≤Θ(G)≤ϑ(G)."
    },
    {
      q: "Yao's minimax principle states that the randomized complexity R(f) of a boolean function f satisfies R(f)=max_μ D_μ(f), where D_μ(f) is the deterministic complexity against the hardest distribution μ. This is a consequence of:",
      opts:["Von Neumann's minimax theorem (LP duality)","Markov's inequality","The probabilistic method","Shannon's channel coding theorem"],
      ans:0,
      exp:"Yao's minimax: R(f)=max_μ min_{det. protocol} error_{μ}(protocol) = max_μ D_μ(f). This follows from the minimax theorem (LP duality applied to the matrix game of protocols vs. inputs weighted by μ). It converts worst-case randomized to average-case deterministic."
    },
    {
      q: "The data stream model studies algorithms that process a sequence of n elements using sublinear space. The AMS (Alon-Matias-Szegedy) sketch estimates the F₂ moment (sum of squares of frequencies) using O(log n) bits with:",
      opts:["Constant relative error with constant probability","ε relative error with probability 1−δ using O(ε^{-2}log(1/δ)) bits","Exact computation in O(n log n) bits","O(√n) bits for ε=1/√n error"],
      ans:1,
      exp:"The AMS sketch: draw random ±1 hash h:domain→{±1} and maintain Z=∑_i a_i h(i). E[Z²]=F₂. Using O(ε^{-2}log(1/δ)) independent sketches with median-of-means gives ε relative error with probability 1−δ. This is optimal (lower bound Ω(ε^{-2}) bits)."
    },
    {
      q: "The channel capacity of the binary erasure channel BEC(ε) (each bit is erased with probability ε, and the receiver knows which positions were erased) is:",
      opts:["1−ε","1−H(ε)","(1−ε)log(1/ε)","H(ε)"],
      ans:0,
      exp:"The BEC(ε) capacity is C=1−ε bits per channel use. This is achieved by any linear code (e.g., Reed-Solomon over a large field or LDPC) since erasure recovery is equivalent to solving a linear system over the non-erased positions."
    }
  ],
  27: [
    {
      q: "The Weihrauch lattice classifies mathematical problems by their computational content. The problem WKL (Weak König's Lemma: every infinite binary tree has an infinite path) sits at which level?",
      opts:["Below the limit map lim","Equivalent to the limit map lim","Above ATR₀ (arithmetical transfinite recursion)","Incomparable with lim"],
      ans:0,
      exp:"In the Weihrauch lattice, WKL is strictly Weihrauch reducible to lim (the limit map, computing limits of computable sequences). WKL corresponds to LLPO (lesser limited principle of omniscience) in constructive math and is below the limit map in strength."
    },
    {
      q: "A real number x∈ℝ is computable if there exists a Turing machine that, given n, outputs an integer a_n with |x−a_n/2^n|<2^{-n}. The set of computable reals is:",
      opts:["Countable and forms a real closed field","Uncountable (one per Turing machine)","All algebraic numbers","Dense in the computable reals only"],
      ans:0,
      exp:"The computable reals form a countable set (one per Turing machine) that is a real closed field: closed under +,−,×,÷ and square roots of positives. However, the set is not closed: limits of computable sequences of computable reals need not be computable (contra-Cauchy sequences of computable reals can converge to uncomputable limits)."
    },
    {
      q: "The Mandelbrot set M is computable as a subset of ℂ in the sense of Type-2 computability (Weihrauch), meaning:",
      opts:["M is decidable (membership is computable)","Given c∉M, we can computably approximate the boundary ∂M; membership in M is co-semi-decidable","M is computably enumerable but not decidable","M is not computable in any sense"],
      ans:1,
      exp:"Hertling (2005), Braverman-Yampolsky (2009): M is computable as a compact set (its complement is computably open). However, membership in M is not decidable (the boundary ∂M contains non-computable points). The set M is 'computable' in the sense that it can be approximated to any precision."
    },
    {
      q: "Rice's theorem states that for any non-trivial property P of partial computable functions (neither always true nor always false), the set {e: φ_e has property P} is:",
      opts:["Computable","Computably enumerable but not computable","Not computably enumerable","Dependent on the property P"],
      ans:2,
      exp:"Rice's theorem: for any non-trivial extensional property P of the function computed by a TM (i.e., a property of φ_e, not the machine e itself), the set {e: φ_e satisfies P} is not computably enumerable (hence undecidable). This generalizes the undecidability of the halting problem."
    },
    {
      q: "The effective topos (Hyland 1982) is a topos built on Kleene realizability. In the effective topos, every function ℝ→ℝ is:",
      opts:["Continuous","Computable","Both continuous and computably measurable","Not defined (reals don't exist in the effective topos)"],
      ans:1,
      exp:"In the effective topos, every morphism (function) is computable in Kleene's sense: the internal logic validates 'all functions ℝ→ℝ are computable' (and hence continuous). This makes the effective topos a model of constructive mathematics where Church's thesis holds."
    },
    {
      q: "The ω-limit set of a computable dynamical system x_{n+1}=f(x_n) with computable f:ℝ→ℝ is always:",
      opts:["A computable set","A Π⁰₂ (G_δ) set in the Borel hierarchy","A computably enumerable set","A decidable set"],
      ans:1,
      exp:"The ω-limit set ω(x)=⋂_{n≥0}cl{x_k: k≥n} is always a Π⁰₂ set (countable intersection of open sets = closed = G_δ in this case, but ω-limit sets of general sequences are Π⁰₂ in the sense of taking countable intersections of closures, hence G_δ sets). For computable f, ω(x) need not be computable."
    },
    {
      q: "The BSS (Blum-Shub-Smale) model of computation over ℝ admits real number arithmetic as unit-cost operations. In this model, the Mandelbrot set membership is:",
      opts:["Decidable in BSS-polynomial time","Undecidable over ℝ (Julia sets are not BSS-decidable in general)","Decidable but BSS-NP-complete","BSS-computable but requires exponential time"],
      ans:1,
      exp:"In the BSS model, computing whether c∈M (Mandelbrot membership) is undecidable: Blum-Cucker-Smale (1997) showed that the halting problem for BSS machines reduces to Julia set membership, making it BSS-undecidable. The complement (divergence) is semi-decidable."
    },
    {
      q: "Para-computable analysis studies functions that are computable with a single oracle call. The Intermediate Value Theorem (IVT: if f(0)<0<f(1) and f is continuous, then ∃x∈[0,1] with f(x)=0) in computable analysis requires:",
      opts:["Only computability of f","Additional assumption that f is Lipschitz","An oracle for 0' (the halting set) in general","No additional assumptions — IVT is computably true"],
      ans:2,
      exp:"The IVT is not computably true: there exist computable f:[0,1]→ℝ with f(0)=-1, f(1)=1 but no computable zero. Finding a zero requires the noncomputable information in 0' (limit computability). IVT is Weihrauch equivalent to the limit map LLPO."
    }
  ],
  28: [
    {
      q: "The Eilenberg-Moore category of a monad (T,η,μ) on a category C is the category of T-algebras. The forgetful functor from T-algebras to C has a left adjoint (the free T-algebra). For the monad T on Set given by T(X)=P(X) (powerset), T-algebras are:",
      opts:["Complete lattices","Topological spaces","Groups","Rings"],
      ans:0,
      exp:"T-algebras for the powerset monad P are complete lattices (or more precisely, sup-lattices): the algebra map ξ:P(X)→X must satisfy the monad laws, forcing X to have all joins (sups). The free T-algebra on X is P(X) itself with ξ=union."
    },
    {
      q: "In topos theory, the subobject classifier Ω in the topos of sets Set is {0,1}. In the topos Sh(X) of sheaves on a topological space X, the subobject classifier is:",
      opts:["The constant sheaf {0,1}","The sheaf Ω where Ω(U)={open sets V⊆U}","The sheaf of sections of a fiber bundle","The terminal sheaf"],
      ans:1,
      exp:"In Sh(X), the subobject classifier is Ω(U)={open subsets of U}, the sheaf of open sets. A global section of Ω over U classifies open subsets of U, reflecting the intuitionistic logic of open sets in topology (where 'truth' means 'open')."
    },
    {
      q: "The Giry monad on the category Meas of measurable spaces sends a measurable space (X,Σ) to (P(X),Σ_{P(X)}) where P(X) is the set of probability measures and Σ_{P(X)} is generated by evaluation maps π_A(μ)=μ(A). The Kleisli category of the Giry monad has as morphisms:",
      opts:["Measurable functions X→Y","Markov kernels (stochastic maps) k:X→P(Y)","Probability measures on X×Y","Measure-preserving maps"],
      ans:1,
      exp:"The Kleisli category of the Giry monad has the same objects as Meas but Kleisli morphisms X→Y are Giry-Kleisli morphisms = Markov kernels k:X→P(Y). Composition is given by the Chapman-Kolmogorov equation. This is the categorical framework for probability theory."
    },
    {
      q: "A coalgebra for a functor F:C→C is a pair (X,α:X→F(X)). For the functor F(X)=A×X (on Set, for a fixed set A), F-coalgebras are:",
      opts:["Monoids","Streams (infinite sequences) and their morphisms","Deterministic automata with input alphabet A","F-coalgebras have no interesting structure for this F"],
      ans:2,
      exp:"For F(X)=A×X on Set, an F-coalgebra is α:X→A×X, i.e., a function giving an output a∈A and a next state x'∈X. This is precisely a deterministic automaton with input-free transitions (Moore machine). The final coalgebra is the set of A-valued streams A^ω."
    },
    {
      q: "The Lawvere fixed-point theorem generalizes Cantor's diagonal argument and Gödel's incompleteness. It states: in a Cartesian closed category, if there is a point-surjective morphism f:A→B^A (i.e., for every g:1→B^A, ∃a with f(a)=g), then:",
      opts:["Every morphism B→B has a fixed point","A is isomorphic to B","B has a natural number object","The category is a topos"],
      ans:0,
      exp:"Lawvere's theorem: if f:A→B^A is point-surjective (every B^A-valued point factors through f), then every endomorphism t:B→B has a fixed point. Proof: define g(a)=t(f(a)(a)) (Cantor diagonal); by surjectivity ∃a₀ with f(a₀)=g, giving f(a₀)(a₀)=t(f(a₀)(a₀))."
    },
    {
      q: "Categorical probability theory uses the Markov category framework (Fritz 2020). In a Markov category, 'copy' (comultiplication) and 'discard' (counit) morphisms allow defining:",
      opts:["Independence (joint = product) and conditional distributions","Entropy without reference to real numbers","Both A and B","Only measure-zero events"],
      ans:2,
      exp:"Markov categories (Fritz 2020) axiomatize probabilistic independence and conditional distributions via the copy (Δ: X→X⊗X) and discard (ε: X→I) maps. Entropy can be defined intrinsically within Markov categories. This framework unifies discrete probability, Gaussian probability, and measure theory."
    },
    {
      q: "In domain theory (Scottʼs model of the λ-calculus), a Scott-continuous function f:D→E between domains satisfies: f preserves directed suprema. The relationship between Scott-continuous and monotone functions is:",
      opts:["Every monotone function is Scott-continuous","Every Scott-continuous function is monotone and preserves all directed sups, but not conversely","Scott-continuity and monotonicity are equivalent","Scott-continuity is strictly weaker than monotonicity"],
      ans:1,
      exp:"Scott-continuous ⟹ monotone (trivially), but not conversely: a monotone function can fail to preserve directed suprema (limits of directed sets). Scott topology makes Scott-continuous functions exactly the continuous functions in this topology, which equals the ω-continuous functions for ω-algebraic domains."
    },
    {
      q: "The nerve of a small category C is a simplicial set N(C) with N(C)_n = functors [n]→C (chains of n composable morphisms). The geometric realization |N(C)| is the classifying space BC. For a group G (viewed as a 1-object category), BC is:",
      opts:["The group G itself","The Eilenberg-MacLane space K(G,1)","The sphere S¹ if G=ℤ","Both B and C"],
      ans:3,
      exp:"For a group G (one object, morphisms=elements of G), BC=K(G,1)=BG (the classifying space). For G=ℤ, K(ℤ,1)=S¹. For G=ℤ², K(ℤ²,1)=T² (torus). BC is the Eilenberg-MacLane space with π₁(BC)=G and all higher homotopy groups trivial."
    }
  ],
  29: [
    {
      q: "The Jarzynski equality relates free energy differences to nonequilibrium work: ⟨e^{-W/kT}⟩=e^{-ΔF/kT}. By Jensen's inequality (e^x is convex), this implies:",
      opts:["⟨W⟩=ΔF","⟨W⟩≥ΔF (second law)","⟨W⟩≤ΔF","W=ΔF always"],
      ans:1,
      exp:"Jensen's inequality applied to the Jarzynski equality: e^{-⟨W⟩/kT}≤⟨e^{-W/kT}⟩=e^{-ΔF/kT}, hence −⟨W⟩/kT≤−ΔF/kT, i.e., ⟨W⟩≥ΔF. This is the second law of thermodynamics: average work exceeds free energy change."
    },
    {
      q: "The fluctuation-dissipation theorem (Kubo 1966) relates the equilibrium fluctuations of an observable A to its response to a perturbation. For a system at temperature T, the response function χ(ω) and the spectral density S_AA(ω) are related by:",
      opts:["S_AA(ω)=2kT·Im χ(ω)/ω","Im χ(ω)=S_AA(ω)·ω/(2kT)","Both A and B","χ(ω)=S_AA(ω)/kT"],
      ans:2,
      exp:"The fluctuation-dissipation relation: S_AA(ω)=2kT·Im χ(ω)/ω (classical version). Both A and B are equivalent forms. Im χ(ω) is the dissipative part of the response; S_AA is the power spectral density. This is a fundamental result connecting noise and dissipation in linear response theory."
    },
    {
      q: "Maxwell's demon thought experiment proposes to violate the second law using information. The resolution (Szilard, Bennett, Landauer) is that the demon must erase its memory, which:",
      opts:["Is free (memory erasure costs nothing)","Costs at least k_B T ln 2 per bit erased (Landauer's principle)","Increases entropy by less than the demon gains","Is impossible for a finite-memory demon"],
      ans:1,
      exp:"Landauer's principle resolves the demon paradox: the demon's memory grows as it measures molecules. When the demon resets its memory (required to close the cycle), erasing 1 bit dissipates ≥k_B T ln 2, exactly compensating the entropy reduction from sorting."
    },
    {
      q: "The Gibbs free energy G=U−TS+PV is minimized at equilibrium for systems at constant T,P. The entropy of mixing for an ideal mixture of N components with mole fractions x_i is:",
      opts:["ΔS_mix=−R∑x_i log x_i per mole","ΔS_mix=R·log N","ΔS_mix=0 (no mixing entropy for ideal systems)","ΔS_mix=−∑x_i log(x_i/N)"],
      ans:0,
      exp:"The entropy of mixing for ideal mixtures: ΔS_mix=−R∑_i x_i ln x_i per mole of mixture (always ≥0 since x_i≤1). This is exactly the Shannon entropy formula with natural log and the gas constant R. It drives spontaneous mixing even without enthalpy changes."
    },
    {
      q: "Stochastic thermodynamics extends classical thermodynamics to mesoscopic systems. The stochastic entropy production along a trajectory x(t) from 0 to τ is σ=log[P(x(t))/P̃(x̃(t))] where P̃ is the probability of the time-reversed trajectory. The integral fluctuation theorem gives:",
      opts:["⟨e^{-σ}⟩=1","⟨σ⟩=0","σ≥0 always","⟨σ²⟩=⟨σ⟩"],
      ans:0,
      exp:"The integral fluctuation theorem (Crooks-Evans-Cohen): ⟨e^{-σ}⟩=1 exactly, valid for all driving protocols and all trajectory lengths. By Jensen: ⟨σ⟩≥0 recovers the second law. The IFT is exact even far from equilibrium."
    },
    {
      q: "Ruelle's thermodynamic formalism for dynamical systems defines the pressure P(φ)=sup_μ{h_μ+∫φ dμ}. The partition function analogy for a lattice system with potential Φ={Φ_Λ} is:",
      opts:["Z_Λ=∑_σ∈Ω_Λ exp(−βH_Λ(σ)) analogous to exp(n·P(φ))","Z_Λ=n·P(φ) directly","Z_Λ=exp(h_μ(T))","Z_Λ=∑_{|p|=n} exp(∫_p φ) (periodic orbit sum)"],
      ans:0,
      exp:"The partition function Z_Λ=∑_σ exp(−βH_Λ(σ)) is the lattice analog, with log Z_Λ/|Λ|→P(−βΦ). For Axiom A systems, the partition function is ∑_{|p|=n} exp(S_n φ(x_p)) (sum over periodic orbits) and log Z_n/n→P(φ) by the Bowen-Ruelle theorem."
    },
    {
      q: "Caratheodory's axiomatic formulation of the second law states that the second law is equivalent to: 'in every neighborhood of any state, there are states unreachable by adiabatic processes.' This is equivalent to the existence of:",
      opts:["A temperature function T","An entropy function S (Pfaffian integrability)","A Hamiltonian function H","A free energy function F"],
      ans:1,
      exp:"Carathéodory's theorem (1909): the Pfaffian form δQ (inexact differential of heat) has an integrating factor 1/T when adiabatic inaccessibility holds in every neighborhood. This gives entropy S with dS=δQ/T, an axiomatic derivation of entropy and the second law."
    },
    {
      q: "The second law of thermodynamics for quantum systems in terms of von Neumann entropy: for a unitary evolution U of an isolated system with initial state ρ, the entropy satisfies:",
      opts:["S(UρU†)≥S(ρ)","S(UρU†)=S(ρ)","S(UρU†)≤S(ρ)","S(UρU†) is unrelated to S(ρ)"],
      ans:1,
      exp:"Von Neumann entropy is invariant under unitary evolution: S(UρU†)=S(ρ). This is because the eigenvalues of ρ are unchanged by unitary conjugation. Entropy increase (second law) arises from tracing out the environment, not from the unitary dynamics itself."
    }
  ],
  30: [
    {
      q: "The Brenier theorem states that for absolutely continuous μ,ν on ℝⁿ (μ with density in L^p), the optimal transport map for the quadratic cost c(x,y)=|x−y|²/2 is:",
      opts:["The gradient of a convex function T=∇φ (unique μ-a.e.)","A linear map","Any measure-preserving bijection","A stochastic kernel"],
      ans:0,
      exp:"Brenier's theorem (1991): for μ,ν with finite second moments and μ≪Lebesgue, the unique optimal transport map for quadratic cost is T=∇φ where φ is a convex function (Brenier potential). T is the μ-a.e. unique minimizer of ∫|T(x)−x|²dμ."
    },
    {
      q: "The 2-Wasserstein distance W₂(μ,ν)=(inf_{γ∈Π(μ,ν)}∫|x−y|²dγ)^{1/2} on probability measures with finite second moments. For Gaussian measures μ=N(m₁,Σ₁) and ν=N(m₂,Σ₂), W₂² equals:",
      opts:["|m₁−m₂|²+‖Σ₁−Σ₂‖_F²","‖Σ₁^{1/2}−Σ₂^{1/2}‖_F²","|m₁−m₂|²+tr(Σ₁+Σ₂−2(Σ₁^{1/2}Σ₂Σ₁^{1/2})^{1/2})","tr(Σ₁+Σ₂)−2tr(Σ₁^{1/2}Σ₂^{1/2})"],
      ans:2,
      exp:"The Bures-Wasserstein distance between Gaussians: W₂²(N(m₁,Σ₁),N(m₂,Σ₂))=|m₁−m₂|²+B(Σ₁,Σ₂)² where B(Σ₁,Σ₂)²=tr(Σ₁)+tr(Σ₂)−2tr((Σ₁^{1/2}Σ₂Σ₁^{1/2})^{1/2}). The optimal map is affine: T(x)=m₂+A(x−m₁)."
    },
    {
      q: "The displacement interpolation μ_t=((1−t)id+tT)_#μ for 0≤t≤1 (where T is the Brenier map) gives a geodesic in (P₂(ℝⁿ),W₂). The curvature of this geodesic space is characterized by:",
      opts:["The space has non-negative sectional curvature (Alexandrov space CAT(∞))","The space has non-positive curvature (CAT(0) space)","The space is flat (Euclidean structure)","The curvature depends on the marginals μ,ν"],
      ans:0,
      exp:"(P₂(ℝⁿ),W₂) is a non-negatively curved (Alexandrov) space in the sense of geodesic convexity: the squared distance W₂² is geodesically convex. Formally, P₂(ℝⁿ) behaves like a Riemannian manifold of non-negative sectional curvature (Lott-Villani, Sturm)."
    },
    {
      q: "The Talagrand T₂ inequality states that for a measure μ satisfying the log-Sobolev inequality with constant ρ>0 (i.e., Ent_μ(f²)≤(2/ρ)∫|∇f|²dμ), we have for any probability measure ν:",
      opts:["W₂(μ,ν)≤√(2D_KL(ν||μ)/ρ)","W₂(μ,ν)²≤2D_KL(ν||μ)/ρ","W₂²(μ,ν)≤D_KL(ν||μ)","Both A and B (equivalent forms)"],
      ans:3,
      exp:"Talagrand T₂ inequality: W₂(μ,ν)²≤(2/ρ)D_KL(ν||μ). Otto-Villani proved this follows from the log-Sobolev inequality via the HWI inequality: H(ν||μ)≤W₂(ν,μ)·√(2I(ν||μ))−(ρ/2)W₂²(ν,μ). Both A (W₂≤...) and B (W₂²≤...) are equivalent."
    },
    {
      q: "Sinkhorn's algorithm for regularized optimal transport (entropic regularization with parameter ε) solves min_{γ∈Π(μ,ν)} ∫c dγ + ε·D_KL(γ||μ⊗ν). The algorithm alternates between:",
      opts:["Gradient descent steps on γ","Projection onto marginal constraints: normalize rows then columns of the transport matrix","Solving linear programs","Computing Wasserstein barycenters"],
      ans:1,
      exp:"Sinkhorn-Knopp: the regularized optimal transport is solved by alternating Bregman projections (w.r.t. KL divergence) onto the constraint sets {γ:γ1=a} and {γ:γᵀ1=b}. Each projection is a diagonal rescaling of the current coupling matrix."
    },
    {
      q: "The Lott-Sturm-Villani theory of metric measure spaces satisfying CD(K,N) (curvature-dimension condition) characterizes Riemannian manifolds with Ricci curvature ≥K and dimension ≤N via:",
      opts:["The K-convexity of entropy along W₂ geodesics","The Sobolev inequality with best constants","Bounds on eigenvalues of the Laplacian","The Harnack inequality"],
      ans:0,
      exp:"CD(K,N): a metric measure space (X,d,m) satisfies CD(K,N) iff the (N,K)-Rényi entropy S_N is K-convex along W₂ geodesics. For Riemannian manifolds, this recovers Ric≥K and dim≤N. This allows proving Brunn-Minkowski, isoperimetric, and Poincaré inequalities in general metric spaces."
    },
    {
      q: "The Schrödinger bridge problem (1931/2012 revival) between measures μ₀ and μ₁ finds the most likely diffusion process between them. It is equivalent to the entropic optimal transport problem with which cost?",
      opts:["c(x,y)=|x−y|²","c(x,y)=|x−y| (L¹ cost)","c(x,y)=(x−y)²/2T with T the time horizon, and entropic regularization ε=1","The Schrödinger bridge uses a different (dynamic) variational principle"],
      ans:3,
      exp:"The Schrödinger bridge is a dynamic problem: minimize D_KL(P||W) over path measures P with fixed endpoints marginals, where W is Wiener measure. This differs from static entropic OT; the two are related but the Schrödinger bridge is intrinsically dynamic with no direct static analog."
    },
    {
      q: "The sliced Wasserstein distance SW₂(μ,ν) is defined by averaging the 1D Wasserstein distances over all projections. For d-dimensional measures, the computational advantage over W₂ is:",
      opts:["SW₂ can be computed in O(n log n) by sorting projected samples, vs O(n³) for W₂","SW₂=W₂ for Gaussian measures","SW₂ has no statistical advantage over W₂","SW₂ is always larger than W₂"],
      ans:0,
      exp:"Sliced Wasserstein SW₂(μ,ν)=(∫₀^1 W₂²(P_θ#μ, P_θ#ν) dσ(θ))^{1/2} where σ is the uniform measure on S^{d-1}. For n samples: 1D W₂ costs O(n log n) (sorting). SW₂ approximated by Monte Carlo over random projections, each requiring O(n log n). Total: O(L·n log n) for L projections, vs O(n³) LP for W₂."
    }
  ],
  31: [
    {
      q: "The Weyl equidistribution theorem states that {nα} (fractional parts of nα) is equidistributed mod 1 iff α is irrational. Weyl's criterion gives equidistribution iff:",
      opts:["∑_{n=1}^N e^{2πiknα}/N→0 for all k≠0","∑_{n=1}^N α^n/N→0","The sequence is uniformly distributed","All three are equivalent for irrational α"],
      ans:0,
      exp:"Weyl's criterion: {x_n} is equidistributed mod 1 iff (1/N)∑_{n=1}^N e^{2πihx_n}→0 for all integers h≠0. For x_n=nα: the sum equals (e^{2πiNhα}−1)/N(e^{2πihα}−1)→0 for all h≠0 iff e^{2πihα}≠1 iff α is irrational."
    },
    {
      q: "The three-distance theorem (Steinhaus, proved by Sós) states that for any α∈[0,1]\\ℚ and n≥1, the n points {α},{2α},...,{nα} divide the circle into n+1 arcs of at most how many distinct lengths?",
      opts:["2","3","n","log n"],
      ans:1,
      exp:"The three-distance theorem: the n points {kα} (k=1,...,n) partition the circle into n+1 arcs of exactly 2 or 3 distinct lengths (3 unless n is a denominator of a convergent of α, when there are only 2). This is a fundamental property of irrational rotations."
    },
    {
      q: "Khintchine's theorem in Diophantine approximation states that for a monotone function ψ(q)→0, the set of x∈[0,1] with |x−p/q|<ψ(q) for infinitely many p/q has measure 1 if ∑ψ(q) diverges, and measure 0 if:",
      opts:["∑ψ(q) converges","ψ(q)=1/q²","The approximation is by rationals only","ψ is not monotone"],
      ans:0,
      exp:"Khintchine's theorem (1924): Lebesgue measure is full (1) or zero depending on convergence of ∑_{q=1}^∞ ψ(q). Divergence → measure 1 (almost all reals are ψ-approximable), convergence → measure 0. The monotonicity assumption was later removed (Duffin-Schaeffer conjecture/theorem, 2020)."
    },
    {
      q: "The continued fraction expansion of x=[a₀;a₁,a₂,...] has partial quotients a_n. The Gauss map T(x)=1/x−⌊1/x⌋ is the shift on continued fractions. The Gauss measure μ_G(A)=(1/log 2)∫_A dx/(1+x) is invariant for T and satisfies:",
      opts:["T is ergodic with respect to μ_G with entropy h=π²/(6 log 2)","T is mixing with entropy π²/6","T is ergodic but not mixing","T is an isometry with respect to μ_G"],
      ans:0,
      exp:"The Gauss map T is ergodic with respect to Gauss measure μ_G (Gauss, proved rigorously by Kuzmin). The metric entropy of T with respect to μ_G is h_{μ_G}(T)=π²/(6 log 2) (Rohlin 1961). T is also mixing (proven by Sinai, Ratner et al.)."
    },
    {
      q: "The Furstenberg correspondence principle relates Szemerédi-type theorems to ergodic theory. It states that for any A⊆ℕ with upper density d̄(A)=lim sup |A∩[N]|/N>0, there exists an ergodic measure-preserving system (X,μ,T) and set E with μ(E)=d̄(A) such that:",
      opts:["d̄(A∩(A-k))≥μ(E∩T^{-k}E) for all k","d̄(A∩(A-1)∩...∩(A-k))≥μ(E∩T^{-1}E∩...∩T^{-k}E) for all k (van der Waerden/AP)","μ(E)=d̄(A) and the dynamical system encodes A","Both A and B hold with equality"],
      ans:1,
      exp:"Furstenberg's correspondence: given A with d̄(A)>0, ∃ system (X,μ,T) with set E, μ(E)=d̄(A), such that d̄(A∩(A-1)∩...∩(A-k))≥μ(E∩T^{-1}E∩...∩T^{-k}E). Szemerédi's theorem follows from the ergodic theorem giving lim inf μ(⋂T^{-jk}E)>0."
    },
    {
      q: "Normal numbers in base b are defined by Borel (1909): x is normal to base b if every block of k digits appears with frequency b^{-k}. The set of normal numbers has:",
      opts:["Measure zero","Full Lebesgue measure (measure 1)","Measure equal to log b","Measure equal to 1−1/b"],
      ans:1,
      exp:"Borel's theorem (1909): almost all real numbers are normal to every base (absolutely normal). The proof uses the law of large numbers applied to the digit expansion. Despite having full measure, not a single provably absolutely normal number is explicitly known to be transcendental."
    },
    {
      q: "The Markov spectrum consists of Markov numbers m (solutions to x²+y²+z²=3xyz) and the corresponding values 1/√(9−4/m²) in the Lagrange spectrum. The Freiman theorem (Hall's ray) states that the Lagrange spectrum contains:",
      opts:["All of [1/√5,∞)","The Hall ray: all x≥√221/5 (the Freiman constant ≈2.971)","All quadratic irrationals","A Cantor set of measure zero"],
      ans:1,
      exp:"Hall's ray: the Lagrange spectrum (values of lim sup_{q→∞} 1/(q·‖qα‖)) contains all values ≥(√221)/5 (proved by Hall 1947). Freiman computed the exact constant. Below Freiman's constant, the Lagrange spectrum has gaps (e.g., the Markov values are isolated)."
    },
    {
      q: "In the ergodic theory approach to the prime number theorem, the Möbius function μ(n) (±1 on squarefree numbers, 0 otherwise) satisfies the Chowla conjecture: for any distinct a_1,...,a_k and signs ε_i∈{±1}, ∑_{n≤N} ∏μ(n+a_i)^{ε_i}=o(N). Teraväinen-Tao proved cases of this via:",
      opts:["The Riemann hypothesis","The Elliott-Halberstam conjecture","Entropy methods and the Gowers-Host-Kra theory of uniformity norms","Sieve methods alone"],
      ans:2,
      exp:"Tao (2015) proved the logarithmic form of the binary Chowla conjecture using entropy decrement arguments and the structure of multiplicative functions (Fourier-analytic methods). Full Chowla for longer correlations uses Gowers uniformity norms and ergodic-type structure theorems."
    }
  ],
  32: [
    {
      q: "A Polish space is a separable completely metrizable topological space. Which of the following is NOT a Polish space?",
      opts:["ℝ with the usual topology","ℚ with the subspace topology from ℝ","The Baire space ℕ^ℕ with the product topology","The Cantor space 2^ℕ with the product topology"],
      ans:1,
      exp:"ℚ is not a Polish space: it is separable but not completely metrizable (no compatible complete metric exists, by the Baire category theorem since ℚ=⋃{q} is a countable union of closed nowhere-dense sets). The other three are standard Polish spaces."
    },
    {
      q: "The Lusin-Sierpiński theorem in descriptive set theory states that a Borel subset B of a Polish space can be mapped by a continuous injection into:",
      opts:["ℝ","The Cantor space 2^ℕ","The Baire space ℕ^ℕ","Any other Polish space"],
      ans:1,
      exp:"The Lusin-Sierpiński theorem: every uncountable Borel set in a Polish space contains a closed copy of the Cantor space 2^ℕ. Combined with the Cantor-Bernstein-Schröder theorem for injections, every uncountable Borel Polish space is Borel isomorphic to [0,1]."
    },
    {
      q: "The Borel reducibility order ≤_B between equivalence relations E,F on Polish spaces is defined by E ≤_B F iff there is a Borel function f with x E y ⟺ f(x) F f(y). The equivalence relation E_0 (eventual agreement of binary sequences) satisfies:",
      opts:["E_0 is smooth (≤_B id_ℝ)","E_0 is ≤_B-below every non-smooth equivalence relation (E_0 is the minimal non-smooth)","E_0 is above all countable Borel equivalence relations","E_0 is Borel-complete"],
      ans:1,
      exp:"Harrington-Kechris-Louveau (1990): E_0 (on 2^ℕ by x E_0 y iff x(n)=y(n) for all large n) is the ≤_B-minimum non-smooth equivalence relation. Every non-smooth Borel equivalence relation has E_0 ≤_B E. This is a fundamental dichotomy in the Borel reducibility hierarchy."
    },
    {
      q: "A Borel equivalence relation E on a Polish space X is called hyperfinite if E=⋃_n E_n for an increasing sequence of finite Borel equivalence relations. The Dye-Krieger theorem characterizes hyperfinite ergodic equivalence relations as:",
      opts:["All orbit equivalence relations of ℤ²-actions","Orbit equivalence relations of ℤ-actions (a single measure-preserving transformation)","All countable Borel equivalence relations","Treeable equivalence relations"],
      ans:1,
      exp:"Dye's theorem (1959): any two ergodic hyperfinite equivalence relations are orbit equivalent. Ornstein-Weiss (1980): every free action of an amenable group generates a hyperfinite equivalence relation. Hyperfinite = orbit equivalence relation of a single transformation (ℤ-action)."
    },
    {
      q: "The Feldman-Moore theorem states that every countable Borel equivalence relation E on a standard Borel space arises as the orbit equivalence relation of a Borel action of:",
      opts:["A locally compact group","A countable group Γ","An amenable group","The free group F₂"],
      ans:1,
      exp:"Feldman-Moore (1977): every countable Borel equivalence relation E (where each equivalence class is countable) is the orbit equivalence relation of some Borel action of a countable group Γ. The group Γ need not be unique (in fact, every countable group acting freely generates E)."
    },
    {
      q: "The Silver dichotomy in descriptive set theory states that every Π¹₁ (co-analytic) equivalence relation E either has at most countably many classes or has a perfect set of pairwise E-inequivalent elements. This implies that Π¹₁ equivalence relations have:",
      opts:["The same cardinalities as Borel relations","Cardinality ≤ω or =2^{ℵ₀} (no intermediate cardinalities)","Possibly ℵ₁ classes under CH","Always countably many classes"],
      ans:1,
      exp:"Silver's theorem (1980): Π¹₁ equivalence relations have either ≤ℵ₀ equivalence classes or a perfect (hence uncountable, in fact |ℝ|=2^{ℵ₀} many) set of classes. No intermediate cardinalities occur, a strong form of the perfect set property for Π¹₁ sets."
    },
    {
      q: "The Cantor-Bendixson theorem states that every closed subset F of a Polish space can be written as F=P⊔C where P is perfect (closed with no isolated points) and C is:",
      opts:["Open","Countable","A Cantor set","Empty or a finite set"],
      ans:1,
      exp:"Cantor-Bendixson: F=P⊔C where P is the perfect kernel (largest perfect subset of F) and C=F\\P is countable (the scattered part — constructed by iterated removal of isolated points). The decomposition is unique."
    },
    {
      q: "Effective descriptive set theory (Moschovakis) studies the lightface Borel and projective hierarchies. The lightface Σ⁰₁ (effectively open) sets in Baire space ℕ^ℕ are precisely:",
      opts:["All open sets","Recursively open sets (unions of basic open sets along a computable sequence)","Projections of closed sets","The sets decidable by a Turing machine"],
      ans:1,
      exp:"Lightface Σ⁰₁ sets in ℕ^ℕ are the effectively open sets: unions ⋃_n N_{σ_n} of basic open (cylinder) sets where the sequence σ_n is computable. These are the domains of partial computable functions, i.e., the sets of convergent computations. The boldface Σ⁰₁ (= all open sets) is the non-effective version."
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
