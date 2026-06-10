# Appendices

The appendices collect reference material that supports the main text without interrupting its narrative flow. They are designed for two uses: quick lookup during active reading ("what was the Sherman-Morrison formula again?"), and systematic review for readers building or shoring up their mathematical foundations.

## Appendix A: Linear Algebra Reference

[→ Appendix A](appendix_a_linear_algebra/README.md)

The essential linear algebra for reservoir computing: singular value decomposition, the pseudoinverse and its role in ridge regression, the Sherman-Morrison-Woodbury formula that makes online RLS efficient, and matrix norm relationships that appear throughout the theory.

## Appendix B: Probability and Statistics

[→ Appendix B](appendix_b_probability/README.md)

Gaussian random vectors and their conditionals; Bayesian linear regression and its exact equivalence to ridge regression with a Gaussian prior; cross-validation methods including the generalized cross-validation formula; concentration inequalities (Hoeffding, Bernstein, McDiarmid) used in generalization bound proofs.

## Appendix C: Numerical Methods

[→ Appendix C](appendix_c_numerical_methods/README.md)

ODE integration for continuous-time reservoirs (Euler, RK4, adaptive methods); eigenvalue computation (power iteration, full decomposition); conjugate gradient for large linear systems; reproducibility protocol for reservoir experiments (random seed management, MLflow logging).

## Appendix D: Software Guide

[→ Appendix D](appendix_d_software/README.md)

Practical guide to the main software tools used in this book: ReservoirPy (Python, the most complete RC library), Brian2 (spiking neural networks for LSMs), PyTorch integration for hybrid architectures, Qiskit for quantum RC, and experiment tracking with MLflow.

## Appendix E: Benchmark Datasets

[→ Appendix E](appendix_e_benchmarks/README.md)

Precise mathematical definitions and Python generation code for every standard benchmark used in the book: NARMA-10/20, Mackey-Glass, Lorenz, Santa Fe laser, FSDD spoken digits, and the Jaeger-Haas channel equalization task. Includes a summary performance table for state-of-the-art reservoir results.

## Appendix F: Key Researchers

[→ Appendix F](appendix_f_researchers/README.md)

Extended profiles of 18 researchers central to reservoir computing's development: Herbert Jaeger, Wolfgang Maass, Mantas Lukoševičius, Benjamin Schrauwen, Claudio Gallicchio, Alessio Micheli, Daniel Brunner, Guy Van der Sande, Ingo Fischer, Jordi Soriano, Kohei Nakajima, David Sussillo, Surya Ganguli, Daniel Gauthier, Jaideep Pathak, Julie Grollier, Peter Tino, and David Verstraeten.

## Appendix G: Symbol Glossary

[→ Appendix G](appendix_g_glossary/README.md)

Complete tables of notation used throughout the book: matrices, vectors, scalars, functions and operators, probability distributions, subscript and superscript conventions, special sets, and a full acronym list. When in doubt about what a symbol means, look here first.

---

*The appendices are self-contained and can be read in any order. Cross-references to the main text are given in brackets.*
