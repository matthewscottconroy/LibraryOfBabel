# Chapter 1 — Key Researchers

---

## Stephen Boyd

**Affiliation:** Stanford University (Department of Electrical Engineering)

**Contribution to this chapter:** Together with Leon Chua, Boyd proved the foundational approximation theorem (Boyd & Chua, 1985) showing that any fading-memory functional can be approximated by a Volterra series. This work provided the mathematical underpinning for why driven dynamical systems — and eventually reservoirs — can serve as universal temporal processors. Boyd's later work on convex optimization and system identification also bears indirectly on reservoir computing training methods.

**Further reading:** Boyd, S.P. (1985). *Volterra Series: Engineering Fundamentals* (doctoral dissertation, UC Berkeley). Available through university archives.

---

## Leon O. Chua

**Affiliation:** University of California, Berkeley (Department of Electrical Engineering and Computer Sciences)

**Contribution to this chapter:** Co-author with Boyd of the 1985 approximation theorem. Chua is also the inventor of the memristor (theoretically, in 1971; physically realized by HP in 2008), which features in Chapter 19 as a physical reservoir substrate. His work on nonlinear circuit theory and the Chua circuit (one of the simplest systems exhibiting chaos) bridges electronic engineering and dynamical systems in ways directly relevant to physical reservoir computing.

**Further reading:** Chua, L.O. (1971). Memristor — the missing circuit element. *IEEE Transactions on Circuit Theory*, 18(5), 507–519. Chua, L.O. & Kang, S.M. (1977). Memristive devices and systems. *Proceedings of the IEEE*, 64(2), 209–223.

---

## Andrei Kolmogorov

**Affiliation:** Moscow State University

**Contribution (historical):** While not working on neural networks directly, Kolmogorov's 1957 theorem on the representation of continuous functions of several variables as superpositions of functions of fewer variables — the Kolmogorov-Arnold representation theorem — is a distant ancestor of universal approximation theory. It establishes that the "curse of dimensionality" is not an absolute barrier to function approximation, which is a theme that runs through the whole of this chapter.

**Further reading:** Kolmogorov, A.N. (1957). On the representation of continuous functions of many variables by superposition of continuous functions of one variable and addition. *Doklady Akademii Nauk SSSR*, 114, 953–956.

---

## George Cybenko

**Affiliation:** Dartmouth College (Thayer School of Engineering)

**Contribution to this chapter:** Proved the first rigorous universal approximation theorem for feedforward neural networks (Cybenko, 1989), establishing that a single hidden layer of sigmoidal units can approximate any continuous function on a compact set. This result is the baseline against which temporal approximation (our main concern) must be compared.

**Further reading:** Cybenko, G. (1989). Approximation by superpositions of a sigmoidal function. *Mathematics of Control, Signals and Systems*, 2(4), 303–314.

---

## Vito Volterra

**Affiliation:** University of Rome (historical, 1860–1940)

**Contribution (historical):** Volterra developed the mathematical theory of functionals — mappings from functions to numbers — in the early 20th century. His series expansion for nonlinear systems (the Volterra series) is the direct precursor to the approximation theory of fading-memory systems. The Volterra series remains a central tool in nonlinear systems identification.

**Further reading:** Volterra, V. (1930). *Theory of Functionals and of Integral and Integro-Differential Equations*. Blackie & Son. Schetzen, M. (1980). *The Volterra and Wiener Theories of Nonlinear Systems*. Wiley.

---

## Kurt Hornik

**Affiliation:** Vienna University of Economics and Business

**Contribution to this chapter:** Proved a general form of the universal approximation theorem for feedforward networks (Hornik, Stinchcombe & White, 1989), showing that networks with arbitrary (but fixed) nonlinear activations satisfying mild conditions are dense in the class of continuous functions. This established the theoretical capability of feedforward networks while simultaneously highlighting their temporal blindness.

**Further reading:** Hornik, K., Stinchcombe, M., & White, H. (1989). Multilayer feedforward networks are universal approximators. *Neural Networks*, 2(5), 359–366.
