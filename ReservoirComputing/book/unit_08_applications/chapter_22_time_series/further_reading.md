# Chapter 22: Further Reading

## ECG and Cardiac Signal Processing

**Moody, G. B., & Mark, R. G. (2001).** The impact of the MIT-BIH Arrhythmia Database. *IEEE Engineering in Medicine and Biology Magazine*, 20(3), 45–50.
History and description of the MIT-BIH database. Required reading for understanding the benchmark.

**Pan, J., & Tompkins, W. J. (1985).** A real-time QRS detection algorithm. *IEEE Transactions on Biomedical Engineering*, 32(3), 230–236.
The standard Pan-Tompkins QRS detector, used as the first preprocessing step in beat-based ECG classification. The algorithm has remained remarkably competitive despite its age.

**Luz, E. J. S., Schwartz, W. R., Cámara-Chávez, G., & Menotti, D. (2016).** ECG-based heartbeat classification for arrhythmia detection: A survey. *Computer Methods and Programs in Biomedicine*, 127, 144–164.
Comprehensive survey of ECG classification methods, providing context for reservoir computing approaches.

**Kachuee, M., Fazeli, S., & Sarrafzadeh, M. (2018).** ECG heartbeat classification: A deep transferable representation. In *ICHI 2018*.
State-of-the-art deep CNN approach to MIT-BIH classification, providing the competitive upper bound for comparison with RC systems.

## EEG and Seizure Detection

**Shoeb, A., & Guttag, J. (2010).** Application of machine learning to epileptic seizure detection. In *ICML 2010*.
The patient-specific SVM seizure detector that established performance standards on the CHB-MIT database.

**Goldberger, A. L., et al. (2000).** PhysioBank, PhysioToolkit, and PhysioNet: Components of a new research resource for complex physiologic signals. *Circulation*, 101(23), e215–e220.
Description of the PhysioNet platform and its databases, including MIT-BIH and CHB-MIT.

**Ramgopal, S., et al. (2014).** Seizure detection, seizure prediction, and closed-loop warning systems in epilepsy. *Epilepsy & Behavior*, 37, 291–307.
Clinical review of seizure detection systems, providing context for the performance requirements that RC systems must meet for clinical deployment.

## Anomaly Detection

**Chalapathy, R., & Chawla, S. (2019).** Deep learning for anomaly detection: A survey. *arXiv:1901.03407*.
Comprehensive survey of deep learning approaches to anomaly detection, providing the competitive landscape for RC-based methods.

**Zimmer, M., Viéville, T., & Soudan, M. (2019).** Stability versus accuracy for echo state networks. *Neural Networks*, 120, 209–226.
Theoretical analysis of stability and prediction accuracy trade-offs in reservoir anomaly detectors.

**Schmidl, S., Wenig, P., & Papenbrock, T. (2022).** Anomaly detection in time series: A comprehensive evaluation. *Proceedings of the VLDB Endowment*, 15(9), 1779–1797.
Systematic benchmark of 26 anomaly detection methods on 976 datasets. RC prediction-based detectors are among the competitive baselines.

## Energy and Industrial Time Series

**Haben, S., Arora, S., Giasemidis, G., Voss, M., & Greetham, D. V. (2021).** Review of low voltage load forecasting: Methods, applications, and recommendations. *Applied Energy*, 304, 117798.
Comprehensive review of electricity load forecasting methods, including RC approaches.

**Gallicchio, C., & Micheli, A. (2017).** Echo state property of deep reservoir computing networks. *Cognitive Computation*, 9(3), 337–350.
The foundational deep ESN paper, with analysis directly applicable to hierarchical architectures for biomedical and load forecasting applications.
