#!/usr/bin/env node
'use strict';
const fs = require('fs');
const path = require('path');

const BASE = path.join(__dirname, 'questions');

const META = {
  11:{part:2,dir:'part-ii-dynamical-systems',ch:'ch11-chaos-theory'},
  12:{part:2,dir:'part-ii-dynamical-systems',ch:'ch12-symbolic-dynamics'},
  13:{part:2,dir:'part-ii-dynamical-systems',ch:'ch13-complex-dynamics'},
  14:{part:2,dir:'part-ii-dynamical-systems',ch:'ch14-hamiltonian-systems'},
  15:{part:2,dir:'part-ii-dynamical-systems',ch:'ch15-infinite-dimensional-dynamics'},
  16:{part:3,dir:'part-iii-information-theory',ch:'ch16-classical-information-theory'},
  17:{part:3,dir:'part-iii-information-theory',ch:'ch17-entropy-generalizations'},
  18:{part:3,dir:'part-iii-information-theory',ch:'ch18-algorithmic-information-theory'},
  19:{part:3,dir:'part-iii-information-theory',ch:'ch19-network-information-theory'},
  20:{part:3,dir:'part-iii-information-theory',ch:'ch20-information-geometry'},
};

const EXTRA = {
  11: [
    {
      q: "The Kaplan-Yorke dimension of an attractor with Lyapunov exponents λ₁≥λ₂≥...≥λₙ is defined as j + (λ₁+...+λⱼ)/|λⱼ₊₁| where j is chosen so that the sum of the first j exponents is nonneg. For the Lorenz attractor with λ₁≈0.906, λ₂=0, λ₃≈−14.572, which value is closest to the Kaplan-Yorke dimension?",
      opts:["1.062","2.062","2.906","3.062"],
      ans:1,
      exp:"j=2 since λ₁+λ₂≈0.906>0 but λ₁+λ₂+λ₃<0. D_KY = 2 + (0.906+0)/14.572 ≈ 2.062."
    },
    {
      q: "Young's formula for the Hausdorff dimension of a conformal repeller states dim_H(J) is the unique zero of which pressure function?",
      opts:["t↦P(tφ) where φ=−log|f'|","t↦P(tφ) where φ=log|f'|","t↦h_top(f)−t·χ","t↦P(−tφ) where φ=log|det Df|"],
      ans:0,
      exp:"By Bowen's equation, the Hausdorff dimension of a conformal repeller is the zero of t↦P(t·(−log|f'|)), i.e., P(tφ) with φ=−log|f'|."
    },
    {
      q: "The Pesin entropy formula h_μ(f)=∫∑λᵢ⁺ dμ (sum of positive Lyapunov exponents) holds for invariant measures that are:",
      opts:["Ergodic only","SRB (physical) measures","All invariant Borel measures","Measures of maximal entropy"],
      ans:1,
      exp:"Pesin's entropy formula holds for SRB measures (those with absolutely continuous conditional measures on unstable manifolds). For general invariant measures, only the inequality h_μ≤∫∑λᵢ⁺ dμ holds (Ruelle's inequality)."
    },
    {
      q: "The Benedicks-Carleson theorem establishes positive-measure parameters for the quadratic family fₐ(x)=1−ax² with strange attractors. The key technical condition on the critical orbit is:",
      opts:["The critical point is periodic","|fₐⁿ(0)|≥e^{−αn} (slow recurrence) with exponential growth of |Dfₐⁿ(fₐ(0))|","The entropy exceeds log 2","The map is conjugate to the full tent map"],
      ans:1,
      exp:"Benedicks-Carleson impose a slow-recurrence condition |fₐⁿ(0)|≥e^{−αn} for small α, plus exponential growth of derivatives along the critical orbit, to construct a positive-measure set of parameters with genuine strange attractors."
    },
    {
      q: "For an Axiom A attractor, the Sinai-Ruelle-Bowen measure can be characterized as the unique invariant measure satisfying:",
      opts:["It maximizes entropy among all invariant measures","It equals the Liouville measure restricted to the attractor","It is the weak* limit of (1/n)∑_{k=0}^{n-1} f^k_* Leb for Lebesgue-almost every initial condition","It is supported on periodic orbits"],
      ans:2,
      exp:"The SRB measure is the physical measure: for a positive Lebesgue measure set of initial conditions, the time averages of observables converge to integrals against the SRB measure. It is the weak* limit of forward pushforwards of Lebesgue measure."
    },
    {
      q: "The Newhouse phenomenon in surface diffeomorphisms involves the coexistence of infinitely many periodic sinks near a homoclinic tangency. The key mechanism is:",
      opts:["Smale horseshoes of infinite topological entropy","Thickness τ(Λ)·τ(Λ')>1 for the stable and unstable Cantor sets forcing persistent tangencies","Existence of an SRB measure","KAM tori preventing tangency unfolding"],
      ans:1,
      exp:"Newhouse showed that if the stable Cantor set and unstable Cantor set at a homoclinic tangency have thickness product τ(Λ_s)·τ(Λ_u)>1, the tangency is persistent under perturbation, leading to residual sets of diffeomorphisms with infinitely many attracting periodic orbits."
    },
    {
      q: "The Collet-Eckmann condition for an interval map f with critical point c requires:",
      opts:["|Dfⁿ(f(c))|≥Cλⁿ for some C,λ>1","f has an absolutely continuous invariant measure","The Lyapunov exponent at c is zero","f is topologically conjugate to the tent map"],
      ans:0,
      exp:"The Collet-Eckmann condition is exponential growth of derivatives along the critical orbit: |Dfⁿ(f(c))|≥Cλⁿ for constants C>0, λ>1. This ensures the existence of an ACIM and positive Lyapunov exponent for the SRB measure."
    },
    {
      q: "Jakobson's theorem (1981) states that for the quadratic family fₐ(x)=1−ax², the set of parameters a∈[0,2] for which fₐ admits an absolutely continuous invariant measure has:",
      opts:["Measure zero but is uncountable","Positive Lebesgue measure","Full measure","Only countably many elements"],
      ans:1,
      exp:"Jakobson's theorem establishes that the set of parameters for which the quadratic map has an ACIM has positive Lebesgue measure, though it has measure strictly less than that of the full parameter interval."
    }
  ],
  12: [
    {
      q: "The zeta function of a subshift of finite type with transition matrix A is ζ(z)=exp(∑_{n≥1} |Fix(σⁿ)|z^n/n). For an irreducible SFT, this equals:",
      opts:["det(I−zA)","1/det(I−zA)","det(I+zA)","exp(z·tr(A))"],
      ans:1,
      exp:"For an SFT with transition matrix A, |Fix(σⁿ)|=tr(Aⁿ). Then ζ(z)=exp(∑tr(Aⁿ)z^n/n)=exp(−log det(I−zA))=1/det(I−zA)."
    },
    {
      q: "A sofic shift can be defined as the image of an SFT under a factor map. Equivalently, it is a shift space whose language is recognized by a:",
      opts:["Pushdown automaton","Finite automaton (right-resolving labeled graph)","Turing machine","Context-free grammar"],
      ans:1,
      exp:"Sofic shifts are precisely the shift spaces whose language of allowed words is recognized by a finite automaton, or equivalently, that are factors of SFTs via sliding block codes (Weiss, 1973)."
    },
    {
      q: "The entropy of the golden mean shift (forbidding 11) over the binary alphabet is:",
      opts:["log 2","log φ where φ=(1+√5)/2","log(√5)","(log 5)/2"],
      ans:1,
      exp:"The golden mean shift has transition matrix A=[[1,1],[1,0]] with spectral radius φ=(1+√5)/2. By the Perron-Frobenius theorem, entropy = log(spectral radius) = log φ."
    },
    {
      q: "Two-sided SFTs S_A and S_B are conjugate (topologically) if and only if their adjacency matrices A and B are:",
      opts:["Similar over ℤ (same characteristic polynomial)","Strong shift equivalent (connected by a chain of elementary equivalences)","Have the same trace","Both irreducible with the same entropy"],
      ans:1,
      exp:"Williams' theorem (1973, with correction by Kim-Roush): two SFTs are conjugate iff their transition matrices are strong shift equivalent. Strong shift equivalence is finer than shift equivalence (which determines the zeta function)."
    },
    {
      q: "The follower set of a word w in a shift space X is F(w)={u∈L(X): wu∈L(X)}. For a sofic shift, the number of distinct follower sets is:",
      opts:["Always infinite","Finite","Equal to the number of forbidden words","Equal to exp(h(X))"],
      ans:1,
      exp:"A shift space is sofic if and only if it has only finitely many distinct follower sets. This is the Krieger characterization of sofic shifts."
    },
    {
      q: "The Bowen-Franks group of an SFT with transition matrix A is the cokernel of (I−A) over ℤ, i.e., BF(A)=ℤⁿ/(I−A)ℤⁿ. This group is an invariant of:",
      opts:["Topological conjugacy","Measure-theoretic isomorphism","Topological entropy","Flow equivalence"],
      ans:0,
      exp:"The Bowen-Franks group (and its ordered version, the dimension group) is a complete invariant for strong shift equivalence of irreducible SFTs up to a shift, hence an invariant of topological conjugacy."
    },
    {
      q: "For a primitive substitution (where some power maps each letter to a word containing all letters), the symbolic dynamical system is:",
      opts:["Always an SFT","Always minimal and uniquely ergodic","Always mixing","Conjugate to a Bernoulli shift"],
      ans:1,
      exp:"A primitive substitution generates a minimal (every orbit is dense) and uniquely ergodic (unique invariant probability measure) subshift. The Perron-Frobenius theorem gives the invariant measure weights from the leading eigenvector."
    },
    {
      q: "The Krieger embedding theorem states that any ergodic measure-preserving system (X,μ,T) with entropy h<log N can be measurably embedded into:",
      opts:["The full N-shift","The golden mean shift","Any SFT with entropy exceeding h","The Bernoulli shift on 2 symbols"],
      ans:0,
      exp:"Krieger's embedding theorem: an ergodic system with entropy h<log N embeds measure-theoretically into the full N-shift (Σ_N,μ_N,σ). Combined with the generator theorem, this gives tight entropy bounds for Bernoulli embeddings."
    }
  ],
  13: [
    {
      q: "The Mandelbrot set M consists of parameters c∈ℂ for which the Julia set J_c is connected. By Douady-Hubbard, this is equivalent to:",
      opts:["The critical point 0 has bounded orbit under f_c(z)=z²+c","The map f_c has an attracting fixed point","The Julia set has positive measure","The polynomial f_c is hyperbolic"],
      ans:0,
      exp:"By the Douady-Hubbard theorem, c∈M iff the orbit of the critical point 0 under f_c(z)=z²+c is bounded, iff J_c is connected. If 0 escapes to ∞, then J_c is a Cantor set."
    },
    {
      q: "Sullivan's dictionary between Kleinian groups and complex dynamics pairs parabolic fixed points with:",
      opts:["Repelling fixed points","Attracting fixed points","Siegel disks","Rationally indifferent (parabolic) periodic points"],
      ans:3,
      exp:"Sullivan's dictionary: loxodromic↔attracting/repelling, parabolic↔rationally indifferent (parabolic), hyperbolic group↔hyperbolic rational map, Ahlfors finiteness↔no wandering domains."
    },
    {
      q: "The Böttcher coordinate φ_c near ∞ for a polynomial P of degree d satisfies φ_c(P(z))=φ_c(z)^d. The Green's function of the basin of ∞ is:",
      opts:["G_c(z)=log|φ_c(z)|","G_c(z)=|φ_c(z)|","G_c(z)=Re(φ_c(z))","G_c(z)=log|z|"],
      ans:0,
      exp:"G_c(z)=log|φ_c(z)| is the Green's function of the basin of attraction of ∞, harmonic on ℂ∖K_c (filled Julia set), equal to 0 on K_c, and satisfying G_c(P(z))=d·G_c(z)."
    },
    {
      q: "Shishikura's theorem (1998) shows that the boundary of the Mandelbrot set has Hausdorff dimension:",
      opts:["1","2 (and has positive area)","2 (but has area zero)","log 2/log φ"],
      ans:1,
      exp:"Shishikura proved dim_H(∂M)=2. It is conjectured (but unproven) that ∂M has area zero. The full dimension=2 result is established, but the area question remains open (related to MLC)."
    },
    {
      q: "A rational map f is called geometrically finite if every critical point in the Julia set is:",
      opts:["Periodic","Preperiodic (lands on a cycle after finitely many iterations)","In the Fatou set","A fixed point"],
      ans:1,
      exp:"A rational map is geometrically finite iff every critical point in the Julia set is preperiodic (eventually periodic). This class includes all hyperbolic maps (no critical points in J) and sub-hyperbolic maps."
    },
    {
      q: "The Fatou-Shishikura inequality states that for a rational map of degree d≥2, the number of non-repelling cycles satisfies:",
      opts:["#(non-repelling cycles) ≤ 2d−2","#(non-repelling cycles) ≤ d−1","#(non-repelling cycles) ≤ d","#(non-repelling cycles) ≤ 2d"],
      ans:0,
      exp:"Fatou-Shishikura: the number of attracting and indifferent (parabolic+Siegel+Cremer) cycles is at most 2d−2, the number of critical points (counted with multiplicity) for a degree-d rational map."
    },
    {
      q: "In the theory of polynomial-like mappings (Douady-Hubbard), a degree-d polynomial-like map f: U'→U is a proper map of degree d between topological disks. Its filled Julia set K_f is defined as:",
      opts:["The Julia set of the straightened polynomial","⋂_{n≥0} f^{-n}(U')","The closure of the grand orbit of critical points","The preimage of the unit disk"],
      ans:1,
      exp:"K_f=⋂_{n≥0} f^{-n}(U') is the set of points whose entire forward orbit stays in U'. The straightening theorem says f is hybrid equivalent to a polynomial of the same degree."
    },
    {
      q: "Thurston's topological characterization of rational maps among Thurston maps (postcritically finite branched coverings of S²) is that a Thurston map f is equivalent to a rational map if and only if:",
      opts:["Its Julia set is connected","It has no Thurston obstruction (no multicurve Γ with eigenvalue ≥1 for the Thurston matrix)","It is orientation-preserving","Its postcritical set has at most 4 points"],
      ans:1,
      exp:"Thurston's theorem: a postcritically finite branched covering f:S²→S² (not Lattès) is equivalent to a rational map iff it has no Thurston obstruction, i.e., no f-invariant multicurve with Thurston matrix spectral radius ≥1."
    }
  ],
  14: [
    {
      q: "The Arnold diffusion problem concerns Hamiltonian systems with n≥3 degrees of freedom. Arnold's 1964 example showed that despite KAM tori, diffusion can occur through the Arnold web. The mechanism involves:",
      opts:["Overlap of resonance zones","Heteroclinic connections between hyperbolic tori along resonance channels","Destruction of all invariant tori","Chaotic seas filling phase space"],
      ans:1,
      exp:"Arnold's original example uses heteroclinic connections between whiskered (hyperbolic) invariant tori along a resonance layer. The diffusion is slow (polynomial-time in 1/ε) and uses the gaps in KAM barriers."
    },
    {
      q: "The Maupertuis principle states that trajectories of a natural Hamiltonian H=T+V on an energy surface H=E are geodesics of the Jacobi metric:",
      opts:["g_J=(E−V)·g_Riem","g_J=(E−V)²·g_Riem","g_J=2(E−V)·T_kinetic","g_J=√(E−V)·g_Riem"],
      ans:0,
      exp:"The Jacobi metric on the region {V≤E} is g_J=2(E−V)·g_Riem (times the kinetic energy metric for general g_Riem), so trajectories with energy E correspond to geodesics of g_J. Written as conformal rescaling: g_J=(E−V)·g_Riem for unit mass on ℝⁿ."
    },
    {
      q: "Gromov's non-squeezing theorem states that a symplectomorphism cannot map a ball B²ⁿ(r) into a cylinder B²(R)×ℝ²ⁿ⁻² if:",
      opts:["r>R","r<R","r=R","r≥√n·R"],
      ans:0,
      exp:"Gromov's non-squeezing: a ball of radius r cannot be symplectically embedded into the cylinder B²(R)×ℝ²ⁿ⁻² when r>R. This defines the symplectic capacity c(B²ⁿ(r))=πr²."
    },
    {
      q: "The Liouville-Arnold theorem guarantees that a completely integrable Hamiltonian system (n degrees of freedom, n independent Poisson-commuting integrals) has compact level sets that are:",
      opts:["Spheres Sⁿ","Tori 𝕋ⁿ","Ellipsoids","Projective spaces ℝPⁿ"],
      ans:1,
      exp:"Liouville-Arnold: connected compact regular level sets of n commuting integrals are tori 𝕋ⁿ on which the flow is linear (quasi-periodic). Action-angle coordinates (I,θ)∈U×𝕋ⁿ are defined in a neighborhood."
    },
    {
      q: "For a convex Hamiltonian H(p,q) on T*M, the Mather set consists of invariant measures that minimize the action functional. Which statement is true?",
      opts:["Mather sets are always periodic orbits","Mather sets project injectively to the configuration space M","Mather sets always have full Hausdorff dimension","Mather sets are subsets of KAM tori"],
      ans:1,
      exp:"Mather's theorem: the support of any minimizing measure (Mather set) projects injectively (as a Lipschitz graph) onto M. This is the graph property, generalizing KAM tori to non-smooth settings."
    },
    {
      q: "The symplectic capacity of the polydisk P(a,b)=B²(a)×B²(b) in ℝ⁴ is:",
      opts:["π·min(a,b)","π·max(a,b)","π·ab","π·(a+b)/2"],
      ans:0,
      exp:"The Gromov width (smallest symplectic capacity) of the polydisk P(a,b)=B²(a)×B²(b) is π·min(a,b), the radius of the smallest ball that fits symplectically. The cylindrical capacity is π·max(a,b)."
    },
    {
      q: "Floer homology for a Hamiltonian H on a closed symplectic manifold (M,ω) is defined as the homology of a chain complex generated by:",
      opts:["Critical points of H","1-periodic orbits of the Hamiltonian vector field X_H","Fixed points of the time-1 flow","Lagrangian submanifolds"],
      ans:1,
      exp:"Floer homology HF*(H) is the homology of a chain complex generated by 1-periodic orbits of X_H, with boundary map counting pseudo-holomorphic cylinders (gradient trajectories of the action functional)."
    },
    {
      q: "In the Birkhoff-Hopf theorem for twist maps of the annulus, a Birkhoff orbit of type (p,q) (p rotations in q steps) exists whenever the rotation number ρ satisfies:",
      opts:["ρ=p/q and the map is area-preserving","ρ is irrational","The map is integrable","The map has a KAM invariant curve"],
      ans:0,
      exp:"The Poincaré-Birkhoff (last geometric) theorem guarantees that for an area-preserving twist map with rotation numbers ρ_min<p/q<ρ_max on inner and outer boundaries, there are at least 2 periodic orbits of type (p,q)."
    }
  ],
  15: [
    {
      q: "The Ladyzhenskaya squeezing property (or cone property) for a semigroup S(t) on a Hilbert space says that for large t, any two trajectories either approach each other or eventually satisfy a cone condition. This property implies:",
      opts:["The attractor has infinite fractal dimension","The global attractor has finite Hausdorff dimension bounded by the number of positive Lyapunov exponents","The flow is reversible","All trajectories are periodic"],
      ans:1,
      exp:"The squeezing/cone property implies that the global attractor A has finite fractal dimension, bounded by the number of 'instabilities' (positive Lyapunov-like exponents). This is the foundation of the Babin-Vishik and Eden-Foias-Nicolaenko-Temam estimates."
    },
    {
      q: "For the 2D Navier-Stokes equations on a bounded domain with forcing, Foias-Prodi proved that the long-time dynamics are determined by finitely many Fourier modes (determining modes). The number of determining modes N satisfies:",
      opts:["N∼(L/l_d)^{4/3} where l_d is the Kolmogorov dissipation scale","N∼Re","N∼Re^{1/2}","N∼(L/l_d)^2"],
      ans:0,
      exp:"The classical estimate for determining modes of 2D Navier-Stokes is N∼(L/l_d)^{4/3} where l_d=(ν³/ε)^{1/4} is the Kolmogorov scale. This matches the estimate for the dimension of the global attractor."
    },
    {
      q: "An inertial manifold M for an evolution equation is a finite-dimensional positively invariant Lipschitz manifold that attracts all trajectories exponentially. Its existence for 2D Navier-Stokes:",
      opts:["Has been proven for all domains","Has been proven for periodic boundary conditions on thin domains","Remains open in general","Follows from the existence of a global attractor"],
      ans:3,
      exp:"The existence of an inertial manifold for 2D Navier-Stokes on general bounded domains remains open. It has been proven for some thin domains and for related model equations (Kuramoto-Sivashinsky, some parabolic PDE). Existence of a global attractor does not imply an inertial manifold."
    },
    {
      q: "The Chafee-Infante problem u_t = u_xx + λu − u³ on [0,π] with Dirichlet boundary conditions has a global attractor whose structure is completely known. As λ increases through n²:",
      opts:["The origin loses stability and a new pair of equilibria branches off","The attractor collapses to a point","A Hopf bifurcation creates periodic orbits","The system becomes chaotic"],
      ans:0,
      exp:"At λ=n², the equilibrium 0 loses stability (n-th mode) and a pair of stable nontrivial equilibria branches off via pitchfork bifurcation. The attractor is the union of equilibria and their unstable manifolds, with completely understood connection graph."
    },
    {
      q: "For semilinear parabolic equations u_t = Au + f(u) on a Hilbert space H, the existence of a global attractor requires (beyond well-posedness) primarily:",
      opts:["Gradient structure (Lyapunov function)","Point dissipativity: existence of a bounded absorbing set","Reversibility of the flow","Finite-dimensional phase space"],
      ans:1,
      exp:"Point dissipativity (existence of a bounded absorbing set that all trajectories eventually enter) plus asymptotic compactness of the semigroup S(t) is sufficient (and essentially necessary) for the existence of a compact global attractor."
    },
    {
      q: "The Mañé-Baldi theorem on the dimension of global attractors uses the concept of fractal dimension (box-counting dimension). For the 2D Navier-Stokes equations, the best upper bound on dim_F(A) is proportional to:",
      opts:["Re²","Re^{4/3}","Re","Re^{3/2}"],
      ans:1,
      exp:"The Lieb-Thirring approach (Constantin-Foias-Temam) gives dim_F(A)≤c·Re^{4/3} for 2D Navier-Stokes, matching the Kolmogorov theory prediction for the number of active degrees of freedom."
    },
    {
      q: "For the Kuramoto-Sivashinsky equation u_t + u_xxxx + u_xx + uu_x = 0, which statement about the long-time dynamics is established?",
      opts:["The equation has a finite-dimensional global attractor and an inertial manifold","The equation is completely integrable","All solutions converge to equilibria","The equation has infinite-dimensional attractors"],
      ans:0,
      exp:"The Kuramoto-Sivashinsky equation on a periodic domain has a finite-dimensional global attractor (proven by Foias-Nicolaenko-Sell-Temam) and an inertial manifold (unlike 2D NS). The attractor dimension grows like L (domain length)."
    },
    {
      q: "Caraballo-Langa-Valero proved the existence of pullback attractors for non-autonomous dynamical systems. A pullback attractor {A(t)}_{t∈ℝ} is characterized by:",
      opts:["Uniform attraction: sup_t dist(S(t,t-s)B, A(t))→0 as s→∞ for bounded B","Pullback attraction: dist(S(t,t-s)B, A(t))→0 as s→∞ for each fixed t and bounded B","Forward attraction only","Being a fixed point of the two-parameter semigroup"],
      ans:1,
      exp:"Pullback attraction: fix the current time t and pull the initial time t-s→-∞. The attractor A(t) captures the statistics of the system started far in the past and observed at time t. This differs from forward (uniform) attraction."
    }
  ],
  16: [
    {
      q: "The channel capacity of a discrete memoryless channel P(y|x) is C=max_{p(x)} I(X;Y). For a binary symmetric channel with crossover probability p, the capacity-achieving distribution is:",
      opts:["Uniform on {0,1} giving C=1−H(p)","Concentrated on 0 giving C=0","Biased with weight p giving C=H(p)","Non-unique, any distribution achieves C"],
      ans:0,
      exp:"For the BSC with crossover p, the uniform input distribution maximizes mutual information, giving C=1−H_b(p)=1−(−p log p−(1−p)log(1−p)) bits. By symmetry, the uniform distribution is unique optimal."
    },
    {
      q: "The strong converse to the channel coding theorem states that for any rate R>C, the error probability for any length-n code satisfies:",
      opts:["P_e^{(n)}→0","P_e^{(n)}→1 exponentially fast","P_e^{(n)}→1/2","P_e^{(n)} is bounded away from 0 but not necessarily approaching 1"],
      ans:1,
      exp:"The strong converse (Wolfowitz, 1957) states that for R>C, not only does P_e^{(n)} not go to 0, it goes to 1 exponentially fast (at rate given by the sphere-packing exponent). This is stronger than the weak converse P_e^{(n)}≥ε>0."
    },
    {
      q: "The rate-distortion function R(D) for a Gaussian source X∼N(0,σ²) under squared error distortion is:",
      opts:["R(D)=max(0, (1/2)log(σ²/D))","R(D)=(1/2)log(σ²/D) for all D","R(D)=log(σ/D)","R(D)=max(0,log(σ/√D))"],
      ans:0,
      exp:"For a Gaussian source with variance σ², R(D)=(1/2)log(σ²/D) for D≤σ² and R(D)=0 for D>σ². The achievability uses Gaussian codebooks; the converse uses the entropy power inequality."
    },
    {
      q: "Gallager's error exponent E(R) for reliable transmission over a DMC satisfies E(R)>0 for R<C and E(R)=0 for R>C. The random coding exponent E_r(R) for R above the critical rate R_crit is:",
      opts:["E_r(R)=E_sp(R) (sphere packing bound)","E_r(R)=(C-R)²/(2σ²)","E_r(R) equals the expurgated exponent","E_r(R) is the exponent from i.i.d. random codes"],
      ans:3,
      exp:"The random coding exponent E_r(R) is achieved by i.i.d. random codes and equals the sphere-packing bound E_sp(R) for R≥R_crit (critical rate). Below R_crit, the expurgated exponent E_ex(R)>E_r(R) is achievable by removing bad codewords."
    },
    {
      q: "The source-channel separation theorem (Shannon 1948) states that for a stationary ergodic source and a DMC, reliable communication is achievable iff:",
      opts:["The source entropy rate H<C","The Kolmogorov complexity of the source is less than C","The source has finite memory","The source is i.i.d."],
      ans:0,
      exp:"The separation theorem: independent optimization of source coding (compress to rate H) and channel coding (transmit at rate R<C) achieves the optimal performance. End-to-end reliable communication is possible iff H≤C."
    },
    {
      q: "The Plotkin bound for binary codes of minimum distance d and length n with M codewords states:",
      opts:["M≤2d/(2d-n) when 2d>n","M≤2^{n·(1-H(d/n))}","M≤n²/d","M≤2^n/(n·d)"],
      ans:0,
      exp:"The Plotkin bound: for a binary code with minimum distance d>n/2, the number of codewords M≤2d/(2d-n). For d=n/2 (or d≥n/2 with equality), M≤2n. This is derived by double-counting distances."
    },
    {
      q: "The Elias-Bassalygo bound improves on the Singleton bound for large alphabets. For a code over alphabet of size q with minimum distance d, block length n, and M codewords, the MRRW (McEliece-Rodemich-Rumsey-Welch) linear programming bound gives:",
      opts:["The best known asymptotic upper bound on code size via LP duality","A lower bound matching the Gilbert-Varshamov bound","An upper bound equivalent to the Hamming bound","A construction via random cosets"],
      ans:0,
      exp:"The MRRW bound uses linear programming (Delsarte's method/association schemes) to obtain the best known asymptotic upper bounds on code size, outperforming the Plotkin and Hamming bounds for intermediate distances."
    },
    {
      q: "In network coding for a single-source multicast network with min-cut C between source and each receiver, the max-flow min-cut theorem for networks implies the capacity is achievable using:",
      opts:["Routing alone","Linear network coding over a sufficiently large finite field","Nonlinear coding only","Time-sharing between unicast sessions"],
      ans:1,
      exp:"Ahlswede-Cai-Li-Yeung (2000): the multicast capacity equals the min-cut C, achievable via linear network coding over a finite field of size ≥number of receivers. Routing alone is insufficient in general."
    }
  ],
  17: [
    {
      q: "The Donsker-Varadhan large deviation rate functional for the empirical measure L_n=(1/n)∑δ_{X_i} of i.i.d. variables with law P is I(Q)=D_KL(Q||P). For continuous distributions, which is correct?",
      opts:["I(Q)=D_KL(P||Q)","I(Q)=D_KL(Q||P)=∫log(dQ/dP)dQ","I(Q)=H(Q)−H(P)","I(Q)=|H(Q)−H(P)|"],
      ans:1,
      exp:"By Sanov's theorem, the rate function for large deviations of the empirical measure is I(Q)=D_KL(Q||P)=∫log(dQ/dP)dQ (relative entropy of Q with respect to P), valid for all Q absolutely continuous wrt P."
    },
    {
      q: "The maximum entropy principle (Jaynes) selects the probability distribution that maximizes H(X) subject to constraints E[f_i(X)]=c_i. The resulting distribution always belongs to the:",
      opts:["Gaussian family","Exponential family with sufficient statistics f_i","Cauchy family","Uniform family"],
      ans:1,
      exp:"The maximum entropy distribution subject to linear moment constraints E[f_i]=c_i is always in the exponential family: p(x)=exp(∑λ_i f_i(x)−A(λ)) where λ_i are Lagrange multipliers and A(λ) is the log-partition function."
    },
    {
      q: "The conditional entropy satisfies H(X|Y)=H(X,Y)−H(Y). The conditional entropy H(X|Y) equals zero if and only if:",
      opts:["X and Y are independent","X is a function of Y","Y is a function of X","H(X)=H(Y)"],
      ans:1,
      exp:"H(X|Y)=0 iff X is a function of Y (there exists a deterministic map f with X=f(Y) a.s.). This means knowing Y completely determines X, leaving no residual uncertainty."
    },
    {
      q: "The Csiszár-Kullback-Pinsker inequality relates total variation distance to KL divergence. For distributions P,Q on a finite set:",
      opts:["|P−Q|_TV ≤ √(D_KL(P||Q)/2)","D_KL(P||Q) ≥ 2|P−Q|²_TV","D_KL(P||Q) ≥ |P−Q|²_TV/2","Both A and B"],
      ans:1,
      exp:"Pinsker's inequality: D_KL(P||Q)≥2|P−Q|²_TV (with TV norm |P−Q|_TV=(1/2)∑|p_i−q_i|). This is tight (up to constant). Combined: |P−Q|_TV≤√(D_KL(P||Q)/2)."
    },
    {
      q: "The entropy power inequality states that for independent random vectors X,Y in ℝⁿ: 2^{2H(X+Y)/n} ≥ 2^{2H(X)/n} + 2^{2H(Y)/n}. This is used to prove:",
      opts:["The data processing inequality","Shannon's capacity theorem for Gaussian channels","The channel coding theorem for BSC","Sanov's theorem"],
      ans:1,
      exp:"The EPI (Shannon 1948, proved rigorously by Stam 1959) is used to establish that Gaussian channels have Gaussian-optimal inputs and to prove the Shannon-Hartley theorem C=(1/2)log(1+SNR) for the AWGN channel."
    },
    {
      q: "For a continuous random variable X with density f, the differential entropy h(X)=−∫f log f dx can be negative. The maximum differential entropy under a variance constraint σ² is achieved by X∼N(0,σ²) and equals:",
      opts:["(1/2)log(2πeσ²)","log(σ√(2π))","(1/2)log(σ²)","log(2πeσ²)"],
      ans:0,
      exp:"The Gaussian maximizes differential entropy subject to variance σ²: h(N(0,σ²))=(1/2)log(2πeσ²). This is the maximum possible differential entropy for any distribution with variance ≤σ²."
    },
    {
      q: "The α-Rényi entropy H_α(X)=(1/(1-α))log∑_x p(x)^α satisfies the following monotonicity in α:",
      opts:["H_α is increasing in α","H_α is decreasing in α","H_α is non-monotone in α","H_α is constant in α"],
      ans:1,
      exp:"H_α(X) is non-increasing in α (for fixed distribution p). As α→1, H_α→H_1=H (Shannon). As α→∞, H_α→H_∞=−log max_x p(x) (min-entropy). As α→0, H_α→log|supp(X)|."
    },
    {
      q: "The mutual information I(X;Y) can be decomposed as I(X;Y)=H(X)+H(Y)−H(X,Y). The chain rule for mutual information gives:",
      opts:["I(X;Y,Z)=I(X;Y)+I(X;Z|Y)","I(X;Y,Z)=I(X;Y)·I(X;Z|Y)","I(X;Y,Z)=I(X;Y)+I(X;Z)","I(X;Y,Z)=I(X;Y,Z|W)+I(X;W)"],
      ans:0,
      exp:"The chain rule: I(X;Y,Z)=I(X;Y)+I(X;Z|Y). This follows from H(X,Y,Z)=H(X)+H(Y,Z|X) and the definition of conditional mutual information I(X;Z|Y)=H(Z|Y)−H(Z|X,Y)."
    }
  ],
  18: [
    {
      q: "The time-bounded Kolmogorov complexity K^t(x) is the length of the shortest program that outputs x in at most t steps. The time-bounded analog of the coding theorem (Li-Vitányi) states that for polynomial t(n):",
      opts:["K^t(x)≈K(x) for all x","K^t(x) can exceed K(x) by at most O(log n) for most x","K^t(x)≤K(x)+O(log t(n)) but can exceed K(x) by Ω(n) for some x","K^t(x)=K(x) whenever x is computable"],
      ans:2,
      exp:"K^t(x)≤K(x)+O(log t) since we can simulate any program in t steps with log t overhead. But for some strings, K^t(x)≫K(x): the shortest program may require super-polynomial time, so time-bounding can dramatically increase description length."
    },
    {
      q: "Solomonoff induction defines the universal prior M(x)=∑_{U(p)=x} 2^{-|p|} (sum over programs outputting x). This prior is dominated by all computable measures in the sense that for any computable μ:",
      opts:["M(x)=μ(x) for all x","M(x)≥c_μ·μ(x) for some constant c_μ>0 and all x","M(x)≤c_μ·μ(x) for some constant c_μ>0","M(x)/μ(x)→1 almost surely"],
      ans:1,
      exp:"By the universality of U, M(x)≥2^{-K_U(μ)}·μ(x)=c_μ·μ(x) for all x where K_U(μ) is the length of a self-delimiting program for μ. M dominates all computable priors up to a multiplicative constant."
    },
    {
      q: "The Algorithmic Mutual Information I(x:y)=K(x)+K(y)−K(x,y) measures shared information between strings. For strings x,y of similar complexity, I(x:y)≈0 implies:",
      opts:["x and y are identical","x and y share no algorithmic information (are 'algorithmically independent')","K(xy)=K(x)·K(y)","x is a compression of y"],
      ans:1,
      exp:"I(x:y)≈0 (up to O(log n) additive terms) means x and y are algorithmically independent: knowing x gives no compression advantage for y beyond their joint description. This is the algorithmic analog of statistical independence."
    },
    {
      q: "Martin-Löf randomness for a sequence ω can be characterized via gambling (martingales). A sequence ω is Martin-Löf random iff no lower semicomputable martingale M (starting at 1) satisfies:",
      opts:["M(ω₁...ωₙ)→∞","lim sup M(ω₁...ωₙ)=∞","M(ω₁...ωₙ)→0","M(ω₁...ωₙ) stays bounded"],
      ans:1,
      exp:"Schnorr's theorem: ω is Martin-Löf random iff no lower semicomputable martingale (effective supermartingale) succeeds on ω, where success means lim sup M(ω₁...ωₙ)=∞. This is equivalent to the ML-test definition."
    },
    {
      q: "The Kolmogorov structure function h_x(k)=min{log|S|: S is a computable set, x∈S, K(S)≤k} measures the complexity of the 'best model' of size k for x. The typical set size for a 'good' model is:",
      opts:["2^{K(x)}","2^{n-k} where n=|x|","2^{H·n} for entropy rate H","2^{K(x|S)}"],
      ans:1,
      exp:"In the two-part code, a model S with K(S)≤k accounts for h_x(k)=log|S| bits of the total complexity n=K(x). The 'best' models have |S|≈2^{n-k}: they use k bits of structure to reduce the random part from n bits to n-k bits."
    },
    {
      q: "Levin's coding theorem relates K(x) (plain complexity) to K_m(x) (monotone complexity). For a string x of length n, the monotone complexity K_m(x) satisfies:",
      opts:["K_m(x)=K(x)+O(1)","K_m(x)=K(x)+O(log n)","K_m(x)≤K(x)≤K_m(x)+2log K(x)+O(1)","K_m(x)≥K(x) always"],
      ans:2,
      exp:"K(x)≤K_m(x)+O(1) (monotone programs are valid programs) and K_m(x)≤K(x)+2log K(x)+O(1) (self-delimiting trick). So K_m and K differ by at most O(log n), making them equivalent for most purposes."
    },
    {
      q: "Bennett's logical depth of a string x is related to the time required by the 'almost shortest' program to compute x. A string x has low depth if it is:",
      opts:["Compressible (K(x)≪n)","Either random (K(x)≈n) or highly compressible (quickly computed from short program)","Produced by a long computation","Has K(x)=n/2"],
      ans:1,
      exp:"Logical depth measures the computational effort in x's generation. Random strings (max K) are shallow (trivially 'output themselves'). Highly regular strings (min K) are also shallow (short fast programs). Deep strings have intermediate complexity but require long computation — like life or mathematical proofs."
    },
    {
      q: "Chaitin's incompleteness theorem states that there is a constant L such that no formal system F can prove statements of the form 'K(x)>L'. This constant L is approximately:",
      opts:["The Kolmogorov complexity of F itself plus a constant","log|F|","The number of axioms of F","The halting probability Ω of the reference UTM"],
      ans:0,
      exp:"L≈K(F)+c where K(F) is the complexity of encoding F's axioms. A proof of 'K(x)>L' would allow an algorithm to enumerate proofs and find x with K(x)>L in time bounded by L+O(1), contradiction. The exact bound depends on the formalization."
    }
  ],
  19: [
    {
      q: "The capacity of the Gaussian broadcast channel (one sender, two receivers with noise σ₁²<σ₂²) is achieved by superposition coding. The capacity region boundary is parameterized by power split α∈[0,1] as:",
      opts:["R₁=(1/2)log(1+αP/σ₁²), R₂=(1/2)log(1+(1−α)P/(αP+σ₂²))","R₁=(1/2)log(1+αP/σ₁²), R₂=(1/2)log(1+(1−α)P/σ₂²)","R₁+R₂=(1/2)log(1+P/σ₁²)","R₁=R₂=(1/2)log(1+P/(σ₁²+σ₂²))"],
      ans:0,
      exp:"For the degraded Gaussian BC, receiver 1 (better, σ₁²<σ₂²) decodes its message treating receiver 2's signal as noise. Receiver 2 decodes its signal (gets power (1−α)P) with the worse channel. The capacity region is achieved by Gaussian superposition."
    },
    {
      q: "The Han-Kobayashi inner bound for the interference channel splits each message into common and private parts. For the Gaussian interference channel at high SNR, the degrees-of-freedom (DoF) region for two users is:",
      opts:["Each user gets 1/2 DoF (total 1 DoF)","Each user gets 1 DoF (total 2 DoF)","Each user gets 2/3 DoF (total 4/3 DoF)","DoF depends on channel coefficients"],
      ans:0,
      exp:"The DoF region of the 2-user Gaussian interference channel is (d₁,d₂) with d₁+d₂≤1, i.e., each user gets at most 1/2 DoF in the symmetric case (Etkin-Tse-Wang). This is the interference-limited regime."
    },
    {
      q: "The min-cut max-flow theorem for wireline networks has a wireless analog in the concept of:",
      opts:["Network coding over ℤ","Approximate capacity via the deterministic channel model (Avestimehr-Diggavi-Tse)","TDMA achieving exact capacity","Treating interference as noise"],
      ans:1,
      exp:"The linear deterministic model (Avestimehr-Diggavi-Tse 2011) approximates Gaussian networks and shows a min-cut/max-flow result holds in an approximate sense (within constant bits). This gives capacity to within O(1) bits for many wireless network topologies."
    },
    {
      q: "For the wiretap channel (Wyner 1975), with main channel P(y|x) and eavesdropper channel P(z|x) where z is a degraded version of y, the secrecy capacity is:",
      opts:["C_s=max_{p(x)}[I(X;Y)−I(X;Z)]","C_s=max_{p(x)}I(X;Y)","C_s=C_main−C_wiretap","C_s=I(Y;Z)"],
      ans:0,
      exp:"Wyner's secrecy capacity for the degraded wiretap channel is C_s=max_{p(x)}[I(X;Y)−I(X;Z)], where the maximization is over input distributions. The gap I(X;Y)−I(X;Z) measures the advantage of the legitimate receiver over the eavesdropper."
    },
    {
      q: "The relay channel capacity remains open in general. The best known lower bound is the Cover-El Gamal (1979) compress-and-forward bound. Decode-and-forward achieves capacity when:",
      opts:["The relay is full-duplex","The relay channel is degraded (Y₁ is a degraded version of Y₀ given X)","The relay has more power than the source","The channel is additive Gaussian"],
      ans:1,
      exp:"Decode-and-forward achieves the relay channel capacity when the channel is physically degraded: P(y₁,y₀|x₁,x₀)=P(y₁|x₁,x₀)·P(y₀|y₁,x₀). In this case, DnF matches the cut-set upper bound."
    },
    {
      q: "Interference alignment (Cadambe-Jafar 2008) shows that K users in a K-user interference channel each achieve:",
      opts:["1/K degrees of freedom","K/2 total degrees of freedom (1/2 per user)","K total degrees of freedom","1 degree of freedom each"],
      ans:1,
      exp:"Cadambe-Jafar interference alignment: K users each achieve 1/2 DoF, total K/2 DoF, asymptotically. This is achieved by aligning interference signals in a lower-dimensional subspace at each receiver, allowing signal subspace separation."
    },
    {
      q: "The capacity of the Gaussian multiple access channel (MAC) with K users each with power P and noise σ² is characterized by a rate region. The sum-rate capacity is:",
      opts:["K·(1/2)log(1+P/σ²)","(1/2)log(1+KP/σ²)","(K/2)log(1+P/(K·σ²))","(1/2)log(1+P/σ²)"],
      ans:1,
      exp:"The Gaussian MAC sum-rate capacity is (1/2)log(1+KP/σ²): when all users transmit simultaneously, the total received power is KP (users interfere constructively). This requires joint ML decoding (successive cancellation achieves it)."
    },
    {
      q: "The broadcast channel degraded message sets problem (Körner-Marton 1977) asks when two correlated sources (X,Y) can be compressed for a broadcast channel with degraded outputs (V is a degraded version of U). The key result is:",
      opts:["Separation holds: compress then code","Joint source-channel coding strictly outperforms separation when sources are correlated","The capacity region equals the product of individual capacities","Only independent sources can be transmitted"],
      ans:1,
      exp:"Körner-Marton showed that for broadcasting correlated sources over broadcast channels, joint source-channel coding can strictly outperform the separate approach, breaking the source-channel separation principle that holds for single-user systems."
    }
  ],
  20: [
    {
      q: "In information geometry, the Fisher information metric on a statistical manifold M={p_θ} is g_{ij}(θ)=E_θ[∂_i log p_θ · ∂_j log p_θ]. The Cramér-Rao bound states:",
      opts:["Var(T)≥(∂_θ E[T])²/I(θ) for any unbiased estimator T","Var(T)≤1/I(θ)","E[T]≥1/I(θ)","I(θ)≥1/Var(T)"],
      ans:0,
      exp:"The Cramér-Rao bound: Var(T)≥(∂_θ E_θ[T])²/I(θ) for any estimator T (whether biased or not). For unbiased estimators (∂_θ E_θ[T]=1), this gives Var(T)≥1/I(θ)."
    },
    {
      q: "The α-divergence D_α(p||q)=(4/(1-α²))(1−∫p^{(1-α)/2}·q^{(1+α)/2}) interpolates between forward and reverse KL. At α=1:",
      opts:["D_α→D_KL(p||q)","D_α→D_KL(q||p)","D_α→0","D_α→Hellinger distance"],
      ans:0,
      exp:"As α→1, D_α(p||q)→D_KL(p||q) (forward KL). As α→−1, D_α(p||q)→D_KL(q||p) (reverse KL). At α=0, D_α is proportional to the squared Hellinger distance."
    },
    {
      q: "The exponential family p(x;θ)=exp(θᵀT(x)−A(θ))·h(x) has Fisher information matrix I(θ)=∇²A(θ). The dual parameters (mean parameters) η=∇A(θ)=E_θ[T(X)] satisfy:",
      opts:["η parameterizes the e-flat (exponential) geodesics","θ parameterizes the m-flat (mixture) geodesics","The mapping θ↦η is the Legendre transform","Both A and C"],
      ans:3,
      exp:"In exponential families, θ (natural parameters) are e-affine coordinates and η=E[T(X)]=∇A(θ) (mean parameters) are m-affine coordinates. The relation θ↦η is the Legendre transform A*(η)+A(θ)=θ·η."
    },
    {
      q: "The Pythagorean theorem in information geometry states: for an e-flat submanifold E and m-flat submanifold M meeting at point p*, and any point q∈E, r∈M:",
      opts:["D_KL(r||q)=D_KL(r||p*)+D_KL(p*||q)","D_KL(q||r)=D_KL(q||p*)+D_KL(p*||r)","D_KL(r||q)=D_KL(r||p*)·D_KL(p*||q)","D_KL(r||q)+D_KL(q||r)=2D(r||p*)"],
      ans:0,
      exp:"The information-geometric Pythagorean theorem: D_KL(r||q)=D_KL(r||p*)+D_KL(p*||q) when p* is the e-projection of r onto M (or m-projection of q onto E). This is the non-commutative generalization of the Pythagorean theorem."
    },
    {
      q: "Natural gradient descent replaces the Euclidean gradient ∇L with the natural gradient I(θ)^{-1}∇L, where I(θ) is the Fisher information matrix. The key property is that natural gradient descent is:",
      opts:["Slower but more stable than SGD","Invariant under reparameterization of the statistical model","Equivalent to Newton's method","Guaranteed to converge in one step"],
      ans:1,
      exp:"The natural gradient (Amari 1998) is covariant: the update I(θ)^{-1}∇L is independent of the coordinate system used for the statistical model. Standard gradient descent is not invariant under reparameterization, making the learning rate geometry-dependent."
    },
    {
      q: "The geodesic distance on the statistical manifold of Gaussian distributions N(μ,σ²) (under the Fisher metric) between N(0,σ₁²) and N(0,σ₂²) is:",
      opts:["|log σ₁/σ₂|","√2·|log σ₁/σ₂|","(σ₁-σ₂)²/(σ₁σ₂)","|log σ₁²/σ₂²|/√2"],
      ans:1,
      exp:"For the 1D Gaussian manifold with Fisher metric g_{σσ}=2/σ², the geodesic distance between N(0,σ₁²) and N(0,σ₂²) is ∫_{σ₁}^{σ₂}√(2/σ²)dσ=√2·|log(σ₁/σ₂)|."
    },
    {
      q: "Bregman divergences D_F(p||q)=F(p)−F(q)−⟨∇F(q),p−q⟩ for strictly convex F generalize KL divergence (F=∑p_i log p_i). They satisfy:",
      opts:["D_F(p||q)=D_F(q||p) for all convex F","The three-point property: D_F(p||q)=D_F(p||r)+D_F(r||q) for projection r","The generalized Pythagorean theorem: D_F(p||q)+D_F(q||r)=D_F(p||r) when q is the Bregman projection of p onto a Bregman-flat set containing r","Non-negativity and D_F(p||q)=0 iff p=q"],
      ans:3,
      exp:"Every Bregman divergence D_F satisfies: (1) D_F(p||q)≥0, (2) D_F(p||q)=0 iff p=q (strict convexity of F), (3) Bregman Pythagorean theorem for projections. But D_F is generally asymmetric and does not satisfy the triangle inequality."
    },
    {
      q: "The information geometric structure of the space of quantum density matrices is given by the quantum Fisher information. For the SLD (symmetric logarithmic derivative) Fisher metric, the quantum Cramér-Rao bound gives:",
      opts:["Cov(T)≥F_Q(θ)^{-1} where F_Q is the quantum Fisher information","Cov(T)≥I_classical(θ)^{-1}","The bound is always tight for projective measurements","F_Q=I_classical for pure states only"],
      ans:0,
      exp:"The quantum Cramér-Rao bound: Cov(T)≥F_Q(θ)^{-1} where F_Q is the SLD quantum Fisher information matrix, which is always ≥ the classical Fisher information for any fixed measurement. The bound is tight for commuting observables."
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
