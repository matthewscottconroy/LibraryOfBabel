# Chapter 8: Further Reading

## Primary Sources

**Lukoševičius, M. (2012). A practical guide to applying echo state networks. In Montavon, G., Orr, G. B., & Müller, K.-R. (Eds.), *Neural Networks: Tricks of the Trade* (2nd ed., pp. 659–686). Springer.**
[Lukosevicius2012]

The definitive practical reference for reservoir computing hyperparameter tuning. Covers spectral radius, input scaling, regularization, and reservoir size with concrete recommendations and intuitive explanations. Importantly, the chapter also discusses what *not* to do — common mistakes and misconceptions that cause practitioners to waste time. If you read only one reference in this chapter's further reading list, this is the one.

**Bergstra, J., & Bengio, Y. (2012). Random search for hyper-parameter optimization. *Journal of Machine Learning Research*, 13, 281–305.**
[BergstraBengio2012]

The theoretical and empirical case for random search over grid search. Shows that when only a few hyperparameters are "effectively relevant" for a task (i.e., performance varies strongly along a low-dimensional subspace of the hyperparameter space), random search finds good configurations much faster than grid search. For reservoir computing, this is directly applicable: spectral radius and input scaling are nearly always relevant, while other hyperparameters (connectivity, bias scaling) often have weak effects. The paper is clearly written and the argument is elegant.

## Background and Extensions

**Lukoševičius, M., & Jaeger, H. (2009). Reservoir computing approaches to recurrent neural network training. *Computer Science Review*, 3(3), 127–149.**
[Lukosevicius2009]

The comprehensive review that bridges theoretical foundations and practical considerations. Contains analysis of hyperparameter interactions and connects the spectral radius to memory capacity in a way that makes the practical recommendations theoretically grounded.

**Jaeger, H. (2007). Echo state network. *Scholarpedia*, 2(9), 2330.**
[Jaeger2007]

A concise encyclopedia entry from Jaeger himself. Useful as a quick reference for definitions and contains practical notes on hyperparameter setting that are often cited but hard to find elsewhere.

**Bürger, J., Goudarzi, A., Stefanovic, D., & Teuscher, C. (2015). Hierarchical composition of memristive networks for synaptic plasticity and learning. *Microelectronics Journal*, 45(11), 1389–1398.**
[Burger2015]

An example of hyperparameter analysis in a physical reservoir, illustrating how the abstract parameters ($\rho$, $\sigma_{in}$, $\alpha$) map to physical quantities in a real implementation.

**Schrauwen, B., Verstraeten, D., & Van Campenhout, J. (2007). An overview of reservoir computing: theory, applications, and implementations. In *Proceedings of the 15th European Symposium on Artificial Neural Networks* (pp. 471–482).**
[Schrauwen2007]

An early overview paper that includes empirical comparisons of different hyperparameter settings. Historically important as one of the first systematic studies of what makes a good reservoir.

**Snoek, J., Larochelle, H., & Adams, R. P. (2012). Practical Bayesian optimization of machine learning algorithms. In *Advances in Neural Information Processing Systems 25*, 2951–2959.**
[Snoek2012]

The paper that brought Gaussian process-based Bayesian optimization into mainstream machine learning practice. The Spearmint software described here (or its successor, GPyOpt) is a practical tool for reservoir hyperparameter optimization and is referenced in Lab 8.2.
