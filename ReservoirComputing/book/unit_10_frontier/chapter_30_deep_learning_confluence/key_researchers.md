# Chapter 30: Key Researchers

**Albert Gu.** PhD student and postdoc at Stanford (advisor: Christopher Ré), primary author of the HiPPO framework [GuHasani2020], the S4 model [GuGoel2022], and subsequent refinements (S4D, S5). Gu's work on principled initialization of state space models from the perspective of polynomial approximation theory is the central contribution connecting reservoir computing to modern deep sequence models.

**Tri Dao.** Co-author of Mamba [GuDao2023] and FlashAttention. Contributed the selective state space mechanism (input-dependent $B$, $C$, $\Delta$) that addresses S4's limitation on content-dependent filtering. Dao's work has been foundational in making state space models practical for large-scale language modeling.

**Ramin Hasani.** Primary author of Liquid Neural Networks [HasaniLechner2021] and Closed-Form Continuous-time networks [HasaniLechner2022]. Hasani developed the continuous-time ODE-RNN approach inspired by the C. elegans connectome, demonstrating that very small but carefully designed neural networks can achieve strong performance on control tasks. Currently at MIT and Liquid AI.

**Mathias Lechner.** Co-author on the Liquid Neural Networks and CfC papers with Hasani. Contributed the formal analysis of the LNN dynamics and the connection to control-theoretic models of neural computation.

**Yoshua Bengio and collaborators.** Contributed foundational work on gated recurrent networks (GRUs [ChoVaswani2014], which are the discrete-time predecessors of liquid networks) and on the vanishing gradient problem in RNNs that motivates the HiPPO initialization.

**Yi Tay, Mostafa Dehghani, and colleagues.** Authors of the Long Range Arena benchmark [TayDehghani2021], which provided the standard evaluation suite for comparing sequence models on long-range dependency tasks. The benchmark revealed the limitations of transformers on long sequences and motivated the development of S4.

**Angelos Katharopoulos and colleagues.** Authors of Linear Transformers [KatharopoulosVyas2020], which approximated self-attention as a kernel function and revealed connections between attention and recurrent models. Linear attention can be seen as a structured reservoir readout.

**Christopher Ré.** Professor at Stanford, advisor to Albert Gu, and leader of the HazyResearch group that produced much of the S4/Mamba line of work. Ré's group has been prolific in developing efficient sequence models with theoretical foundations.

**Ilya Sutskever, Sepp Hochreiter, and Jürgen Schmidhuber.** While predating the S4 line, Schmidhuber and Hochreiter's LSTM [HochreiterSchmidhuber1997] addressed the same long-range dependency problem. The LSTM's gating mechanism is the conceptual precursor to the liquid time constant. Understanding why LSTMs work better than vanilla RNNs for long sequences also motivates why HiPPO initialization (encoding the ideal long-range fading memory) helps S4.

**Younes Belkada and colleagues.** Contributors to the study of frozen LLMs as feature extractors (linear probing). Their work has quantified the quality of representations in large pretrained models and established when linear readouts suffice — directly relevant to the "LLM as reservoir" paradigm of Section 30.6.4.
