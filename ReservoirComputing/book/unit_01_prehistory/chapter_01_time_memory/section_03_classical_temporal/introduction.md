# Section 1.3: Classical Approaches to Temporal Processing

---

## Section Introduction

Before reservoir computing, before LSTMs, before even the first recurrent neural networks, engineers and scientists had developed a rich toolkit for processing temporal signals. These classical methods — filters, state-space models, Hidden Markov Models — are not simply historical curiosities. They are the shoulders on which modern temporal ML stands. And understanding their limitations precisely is what motivates the architectures that came later.

This section surveys the classical toolkit. We look at each method carefully: what it can compute, what it cannot, and where it fails. The pattern that emerges is that every classical approach involves a fundamental compromise — between memory length and parameter count, between expressiveness and tractability, between model richness and learnability from finite data. Recognizing this pattern in the classical methods makes it easier to see what reservoir computing is trying to do differently.
