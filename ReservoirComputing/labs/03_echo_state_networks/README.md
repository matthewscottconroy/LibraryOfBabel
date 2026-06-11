**Corresponds to:** Chapter 5 — Echo State Networks: The First Reservoir Computer (Unit 2)

**Prerequisites:** Chapter 3 (RNNs and the vanishing gradient), Chapter 4 (the reservoir paradigm), NumPy, basic regression

**Learning Objectives:**
- Implement the complete ESN training pipeline from scratch: data preparation, reservoir drive, washout, ridge regression, and prediction
- Visualise the reservoir state matrix and appreciate the N-dimensional representation of input history
- Understand the effect of regularisation: choosing the ridge parameter α controls generalisation and prevents overfitting
- Develop intuition for what good ESN performance looks like by comparing predicted vs. true traces on NARMA-10
- Connect the washout period to the echo state property: removing initial-condition artefacts before using states as features
