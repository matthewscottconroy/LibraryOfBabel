# Chapter 25: Further Reading

## Foundations

**Elman, J. L. (1990).** Finding structure in time. *Cognitive Science*, 14(2), 179–211.
The foundational paper on simple recurrent networks for language processing. Directly anticipates reservoir computing's approach to NLP. Required reading.

**Linzen, T., Dupoux, E., & Goldberg, Y. (2016).** Assessing the ability of LSTMs to learn syntax-sensitive dependencies. *Transactions of the Association for Computational Linguistics*, 4, 521–535.
The agreement tracking benchmark. Defines the experimental protocol and provides the main results used as baselines in this chapter. Essential reading for Section 25.2.

## Reservoir NLP

**Tino, P., & Kotismannis, M. (2010).** Architectural bias in recurrent neural networks: Fractal analysis. *Neural Computation*, 22(7), 1673–1712.
Theoretical analysis of what languages reservoir networks can and cannot learn, connecting the reservoir's dynamical properties to its language modeling capacity.

**Tino, P., Cernansky, M., & Benuskova, L. (2004).** Markovian architectural bias of recurrent neural networks. *IEEE Transactions on Neural Networks*, 15(1), 6–15.
Analysis showing that reservoir (and simple recurrent) networks implement approximations to Markov models of text, with the reservoir state as a proxy for Markov state.

**Forssi, J., et al. (2007).** Echo state networks for spoken language acquisition. In *Proceedings of ESANN 2007*.
Early demonstration of reservoir language models on spoken text, providing baselines for the comparison in Section 25.1.3.

## Formal Language Theory

**Sipser, M. (2012).** *Introduction to the Theory of Computation* (3rd ed.). Cengage Learning.
Standard reference for formal language theory: regular, context-free, and Turing-recognizable languages; the Chomsky hierarchy. Provides the theoretical framework for Section 25.2.

**Reber, A. S. (1967).** Implicit learning of artificial grammars. *Journal of Verbal Learning and Verbal Behavior*, 6(6), 855–863.
The original Reber grammar paper, showing that humans can implicitly learn a finite-state grammar from exposure without explicit instruction — a result that motivates reservoir grammar induction experiments.

**Weiss, G., Goldberg, Y., & Yahav, E. (2018).** On the practical computational power of finite precision RNNs for language recognition. In *ACL 2018*.
Theoretical analysis of the formal language class recognizable by finite-precision RNNs, directly relevant to the limits of reservoir computation for language.

## Syntax and Semantics in Neural Models

**Manning, C. D., Clark, K., Hewitt, J., Khandelwal, U., & Levy, O. (2020).** Emergent linguistic structure in artificial neural networks trained by self-supervision. *Proceedings of the National Academy of Sciences*, 117(48), 30046–30054.
Analysis of what linguistic structures emerge in self-supervised neural networks, directly relevant to the probing classifier methodology applied to reservoirs.

**Hewitt, J., & Manning, C. D. (2019).** A structural probe for finding syntax in word representations. In *NAACL 2019*.
Introduces the structural probe — a geometric probe that tests whether syntactic distances are linearly encoded in neural representations. Applicable to reservoir states.

## Language Model Benchmarks

**Marcus, M. P., Marcinkiewicz, M. A., & Santorini, B. (1993).** Building a large annotated corpus of English: The Penn Treebank. *Computational Linguistics*, 19(2), 313–330.
Description of the Penn Treebank, the standard benchmark for language model perplexity comparison.

**Merity, S., Xiong, C., Bradbury, J., & Socher, R. (2017).** Pointer sentinel mixture models. In *ICLR 2017*.
Establishes the WikiText-103 benchmark, a larger and more realistic alternative to PTB for language model evaluation.
