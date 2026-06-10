# Applications: Chapter 8

The concepts developed in this chapter are not abstract curiosities. They are deployed in real systems, verifying real software, and powering real mathematical discoveries. Here are six concrete applications.

---

## 1. CompCert: A Verified C Compiler

**The problem:** Compilers are complex, multi-pass programs that transform source code through a dozen intermediate representations. Each transformation can introduce bugs. Compiler bugs are particularly dangerous because they silently corrupt the programs they compile — you cannot easily distinguish a bug in your code from a bug the compiler introduced.

**The solution using dependent types:** Xavier Leroy and collaborators at INRIA built CompCert — a C compiler formally verified in the Coq proof assistant. The key specification is a semantic preservation theorem:

If `CompCert.compile(p) = Some q`, then for every input the compiled program `q` has the same observable behavior as the source program `p`.

This theorem is stated as a type in the Calculus of Inductive Constructions (Coq's underlying system). The proof is a term inhabiting that type. The compiler itself is extracted from Coq to OCaml using the proof-as-program correspondence.

**The dependent types at work:** The intermediate languages (Clight, RTL, Mach) are formalized as inductive types. Semantic preservation for each compilation pass is a Π type: for all source programs, if compilation succeeds, the semantics are preserved. Each pass is verified separately; the whole-compiler theorem is derived by composing the pass-level theorems.

CompCert is used in safety-critical embedded systems — aviation, automotive — where a compiler bug could be catastrophic. It is perhaps the most significant application of dependent type theory in industrial software.

---

## 2. seL4: A Verified Operating System Kernel

**The problem:** An operating system kernel is the ultimate trusted code — it mediates all access to hardware, and a bug in the kernel compromises every program running on the system. Traditionally, kernels are verified by informal argument and extensive testing. Neither gives guarantees.

**The solution:** The seL4 microkernel, developed by NICTA and Data61, is a fully verified operating system kernel. The verification is carried out in Isabelle/HOL, a proof assistant based on Higher-Order Logic (not dependent type theory per se), but the techniques — type families for memory models, inductive types for system state, Π types for universal properties — are directly from the dependent type tradition.

The top-level theorem: the seL4 binary, running on specific ARM hardware, correctly implements its formal specification. This covers about 9000 lines of C code and 7500 lines of assembly.

**Connection to dependent types:** The memory model for seL4 uses dependent types: a pointer into memory has a type that encodes what is stored at that address. Reading memory at address p has return type Memory(p) — the type of whatever is stored there, determined by the memory map. This prevents reading an integer as a function pointer (a common security vulnerability).

seL4 has been deployed in safety-critical systems, including US military drones, where the kernel's security properties are required by certification standards.

---

## 3. Idris: Dependent Types for Systems Programming

**The problem:** Systems programming (writing OS components, drivers, network stacks) requires both performance and correctness. Most languages for systems programming (C, C++, Rust) do not have dependent types. Most languages with dependent types (Agda, Coq) are not designed for low-level systems programming.

**The solution:** Idris, developed by Edwin Brady at the University of St Andrews, is a dependently typed programming language designed for systems programming. Idris treats programs with dependent types as first-class citizens, not just proof objects. The type system can enforce:

- `append : Vec A m → Vec A n → Vec A (m + n)` (length-indexed vectors)
- `lookup : Fin n → Vec A n → A` (safe array access, index type Fin n prevents out-of-bounds)
- `printf : (fmt : Format) → interpFmt fmt → String` (a well-typed printf where the return type depends on the format string)

The printf example is remarkable: the format string `"%d %s"` has a type that encodes "this format requires an Int and a String." Calling printf with the wrong argument types is a compile-time error.

**Production use:** Idris is not as widely deployed as Rust or C, but it demonstrates that dependent types are compatible with practical systems programming. Idris 2 (released 2020) improves performance and adds quantitative type theory (tracking usage of resources), making it competitive for real-world use.

---

## 4. Agda and the Formal Verification of HoTT Theorems

**The problem:** The HoTT Book contains many theorems with complex formal proofs. These proofs need to be mechanically verified to be confident they are correct — informal mathematical proof has errors.

**The solution:** HoTT Agda, the formalization of the HoTT Book in Agda, developed by the HoTT community. The library includes:

- Formal proofs that Π(n:ℕ).2 ≤ n → ¬IsPrime(n) (composites are not prime)
- The Seifert-Van Kampen theorem, computing the fundamental group of the circle as ℤ
- The Blakers-Massey theorem (a theorem about connectivity of homotopy pushouts)
- The Freudenthal suspension theorem (a theorem in stable homotopy theory)

These are theorems of algebraic topology, proved entirely within a type-theoretic formal system, verified by Agda's type checker.

**The dependent types at work:** The circle S¹ is defined as a Higher Inductive Type — an inductive type with a path constructor. Π and Σ types encode all the algebraic topology: homotopy groups, fibrations, fiber sequences. The universe Type₀ is the ambient space in which all the topology lives. The formal proofs are terms in dependent type theory.

This demonstrates that HoTT is not just a theoretical framework — it is a practical system for mechanizing mathematical proofs about topology.

---

## 5. The Four Color Theorem and Formal Mathematical Proofs

**The problem:** The four color theorem (every planar map can be colored with four colors so that no two adjacent regions have the same color) was first "proved" in 1976 by Appel and Haken using an extensive computer search over 1,936 cases. But the proof was not fully verified — it depended on computer output that was not formally checked.

**The solution:** Georges Gonthier formalized the proof in Coq in 2005, using the Mathematical Components library. The formal proof runs in Coq and is fully mechanically verified. Gonthier later formalized the Feit-Thompson theorem (all groups of odd order are solvable) — a proof that spans hundreds of pages in the original and required 150,000 lines of Coq.

**The dependent types at work:** The proof uses inductive types for graphs, planar maps, and graph colorings. Type families encode properties of colorings (is this a proper 4-coloring of this map?). The main theorem is a Π type: for every planar map, there exists a proper 4-coloring. The Σ type in the existential carries the actual coloring.

The fact that this proof exists in Coq — and Coq is sound — gives confidence that the theorem is actually true, in a way that informal mathematical proof does not.

---

## 6. Lean 4 and Mathlib: Formalizing Modern Mathematics

**The problem:** Modern mathematics is vast, interconnected, and full of subtlety. The number of published theorems that have been rigorously verified is tiny compared to the total corpus. Errors accumulate, especially in cutting-edge research where proofs are long and arguments are complex.

**The solution:** Mathlib, the mathematical library for Lean 4, contains over a million lines of formal mathematics: real and complex analysis, algebraic topology, number theory, algebraic geometry. The library uses dependent types throughout:

- `Ring : Type u → Prop` is a type family asserting that a type is a ring
- `Algebra R A : Prop` is a type family asserting that A is an R-algebra
- `ContinuousLinearMap E F : Type` is the type of continuous linear maps between Banach spaces E and F, with E and F themselves appearing in the type

The formalization of the Local Langlands conjecture for function fields (a major theorem in number theory) has recently been completed in Lean 4. This is a result that, even for specialists, requires months of work to verify informally. In Lean 4, the verification is automatic — if it compiles, it is correct.

**Why this matters:** Mathlib represents a qualitative change in how mathematics is done. As more of the mathematical edifice is formalized, new theorems can be built on verified foundations. The prospect of automated theorem proving — finding new mathematical results by computational search — becomes realistic when the library of verified lemmas is large enough for automated systems to work with. Dependent type theory is the language in which this new mathematics is written.
