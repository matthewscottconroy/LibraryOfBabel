# Section 25.1: Reservoir Language Models

## 25.1.1 The Language Modeling Problem

A language model assigns probabilities to sequences of words. Given a vocabulary $\mathcal{V}$ of size $V$, the language model defines a distribution over sequences $\mathbf{w} = (w_1, w_2, \ldots, w_T)$:

$$P(\mathbf{w}) = \prod_{t=1}^T P(w_t | w_1, \ldots, w_{t-1}) = \prod_{t=1}^T P(w_t | \mathbf{w}_{<t})$$

by the chain rule. The core computational challenge is estimating the conditional distributions $P(w_t | \mathbf{w}_{<t})$ — the probability of each next word given its full history.

Language modeling is foundational: it underlies machine translation (as the target language prior), speech recognition (as the language prior for acoustic models), text generation, and serves as a measure of how well a model "understands" text.

**Perplexity** is the standard evaluation metric. For a test corpus of $N$ words:

$$\text{PPL} = \exp\left(-\frac{1}{N}\sum_{t=1}^N \log P(w_t | \mathbf{w}_{<t})\right) = 2^{-\frac{1}{N}\sum_t \log_2 P(w_t | \mathbf{w}_{<t})}$$

Perplexity is the geometric mean of the per-word reciprocal probability — roughly, the "surprise" per word. Lower is better. Human-level perplexity on Penn Treebank is approximately 70–90; state-of-the-art LLMs achieve < 20.

## 25.1.2 Reservoir Language Model Architecture

The reservoir language model [ForssiEtAl2007, TinoEtAl2010] uses the reservoir state as a continuous representation of the prefix history:

**Input encoding**: Each word $w_t$ is encoded as a one-hot vector $\mathbf{e}_{w_t} \in \{0,1\}^V$ or as a pre-trained word embedding $\mathbf{v}_{w_t} \in \mathbb{R}^d$. For vocabulary size $V$ up to a few thousand, one-hot encoding is used; for larger vocabularies, embeddings (from Word2Vec, GloVe, or a small trained embedding table) are necessary.

**Reservoir update**:
$$\mathbf{x}(t) = \tanh(W_{\text{res}}\mathbf{x}(t-1) + W_{\text{in}}\mathbf{e}_{w_t})$$

**Output**: The probability distribution over the next word is:

$$\hat{\mathbf{p}}(t) = \text{softmax}(W_{\text{out}}\mathbf{x}(t) + \mathbf{b}_{\text{out}})$$

where $W_{\text{out}} \in \mathbb{R}^{V \times N}$, and the predicted probability of word $w_{t+1}$ is $\hat{p}_{w_{t+1}}(t)$.

**Training**: $W_{\text{out}}$ is trained to minimize the cross-entropy loss:

$$\mathcal{L} = -\frac{1}{T}\sum_t \log \hat{p}_{w_{t+1}}(t)$$

For moderate vocabulary sizes ($V \leq 10,000$), this is equivalent to multinomial ridge regression with $V$ output classes. For larger vocabularies, the softmax normalization requires special treatment (hierarchical softmax, negative sampling, or sampled softmax).

## 25.1.3 Perplexity Benchmarks

### Penn Treebank (PTB) Benchmark

The Penn Treebank word-level language modeling benchmark [MarcusEtAl1993] uses the standard split: sections 0–20 for training ($\sim 930K$ words), sections 21–22 for validation ($\sim 74K$ words), sections 23–24 for test ($\sim 82K$ words). Vocabulary size: 10,000 words.

Representative perplexities (test set):
- KN5 (5-gram with Kneser-Ney smoothing): $\sim 141$
- LSTM (2-layer, 650 units): $\sim 77$
- ESN ($N = 1000$, no embedding): $\sim 125$–$140$
- ESN ($N = 2000$ + Word2Vec embeddings): $\sim 105$–$120$
- Transformer (small, 12 layers): $\sim 55$
- GPT-2 (117M parameters): $\sim 35$

The ESN falls between n-gram models and LSTM in perplexity. This is consistent with the expectation that the ESN provides richer history encoding than n-grams (which only look back $n-1$ words) but less flexible modeling than an LSTM (which can learn complex gated memory mechanisms).

### Text8 Benchmark

Text8 is a character-level language modeling dataset (first $10^8$ characters of Wikipedia, preprocessed). Character models avoid vocabulary size issues and test lower-level pattern learning.

| Model | Bits-per-character (BPC) |
|---|---|
| n-gram (5-gram) | 1.60 |
| ESN (N=2000, char-level) | 1.45 |
| LSTM (char-level, 1000 units) | 1.25 |
| Transformer-XL | 1.06 |

The ESN achieves competitive character-level language modeling relative to simple baselines. The char-level task is well-suited to reservoirs because: (1) the relevant dependencies are at the word level (a few dozen characters), within the reservoir's memory horizon, and (2) the pattern statistics are more stationary than word-level statistics.

## 25.1.4 Python Implementation: Reservoir Character-Level Language Model

```python
"""
Character-Level Reservoir Language Model
Trained on a text corpus. Evaluates perplexity and generates text.

Requirements: numpy, matplotlib
"""

import numpy as np
from collections import Counter


class ReservoirCharLM:
    """
    Character-level reservoir language model.
    
    Models P(c_t | c_{t-1}, ..., c_0) using a reservoir hidden state.
    
    Parameters
    ----------
    n_reservoir : int
    spectral_radius : float
    input_scaling : float
    leaking_rate : float
    ridge_alpha : float
    seed : int
    """
    
    def __init__(
        self,
        n_reservoir: int = 500,
        spectral_radius: float = 0.95,
        input_scaling: float = 0.5,
        leaking_rate: float = 0.9,
        ridge_alpha: float = 1e-5,
        seed: int = 42,
    ):
        self.N = n_reservoir
        self.rho = spectral_radius
        self.s_in = input_scaling
        self.alpha = leaking_rate
        self.ridge = ridge_alpha
        self.rng = np.random.RandomState(seed)
        
        self.char2idx = {}
        self.idx2char = {}
        self.V = 0
        self.W_res = None
        self.W_in = None
        self.bias = None
        self.W_out = None
    
    def _build_vocab(self, text: str) -> None:
        chars = sorted(set(text))
        self.char2idx = {c: i for i, c in enumerate(chars)}
        self.idx2char = {i: c for c, i in self.char2idx.items()}
        self.V = len(chars)
    
    def _init_weights(self) -> None:
        N, V = self.N, self.V
        W = self.rng.randn(N, N)
        W[self.rng.rand(N, N) > 0.1] = 0.0
        ev = np.linalg.eigvals(W)
        W *= self.rho / (np.max(np.abs(ev)) + 1e-10)
        self.W_res = W
        self.W_in = self.s_in * self.rng.randn(N, V)
        self.bias = 0.1 * self.rng.randn(N)
    
    def _one_hot(self, idx: int) -> np.ndarray:
        v = np.zeros(self.V)
        v[idx] = 1.0
        return v
    
    def _run_reservoir(
        self,
        char_indices: list,
        washout: int = 200,
    ) -> tuple:
        T = len(char_indices)
        x = np.zeros(self.N)
        states = []
        for t in range(T):
            oh = self._one_hot(char_indices[t])
            pre = self.W_res @ x + self.W_in @ oh + self.bias
            x = (1 - self.alpha) * x + self.alpha * np.tanh(pre)
            if t >= washout:
                states.append(x.copy())
        return np.array(states)
    
    def fit(
        self,
        text: str,
        washout: int = 200,
        chunk_size: int = 10000,
    ) -> 'ReservoirCharLM':
        """Train on text corpus."""
        self._build_vocab(text)
        self._init_weights()
        print(f"Vocabulary: {self.V} characters")
        
        indices = [self.char2idx[c] for c in text]
        
        # Collect states and targets in chunks (memory management)
        all_states = []
        all_targets = []
        
        for start in range(0, len(indices) - 1, chunk_size):
            end = min(start + chunk_size, len(indices))
            chunk = indices[start:end]
            
            states = self._run_reservoir(chunk, washout=washout if start == 0 else 0)
            
            target_start = start + (washout if start == 0 else 0) + 1
            target_end = target_start + len(states)
            targets = indices[target_start:target_end]
            
            if len(targets) == len(states):
                all_states.append(states)
                all_targets.extend(targets)
        
        X = np.vstack(all_states)            # (T', N)
        Y_idx = np.array(all_targets[:len(X)])
        Y = np.eye(self.V)[Y_idx]            # (T', V) one-hot targets
        
        print(f"Training on {len(X)} timesteps...")
        
        # Ridge regression: W_out = (X'X + lambda I)^{-1} X'Y
        A = X.T @ X + self.ridge * np.eye(self.N)
        b = X.T @ Y
        self.W_out = np.linalg.solve(A, b).T   # (V, N)
        
        return self
    
    def _predict_probs(self, x: np.ndarray) -> np.ndarray:
        """Compute next-character probability distribution."""
        logits = self.W_out @ x
        # Numerically stable softmax
        logits -= logits.max()
        probs = np.exp(logits)
        return probs / probs.sum()
    
    def perplexity(self, text: str, washout: int = 100) -> float:
        """Compute perplexity on held-out text."""
        indices = [self.char2idx.get(c, 0) for c in text]
        x = np.zeros(self.N)
        log_prob = 0.0
        count = 0
        
        for t in range(len(indices) - 1):
            oh = self._one_hot(indices[t])
            pre = self.W_res @ x + self.W_in @ oh + self.bias
            x = (1 - self.alpha) * x + self.alpha * np.tanh(pre)
            
            if t >= washout:
                probs = self._predict_probs(x)
                log_prob += np.log(probs[indices[t+1]] + 1e-10)
                count += 1
        
        return np.exp(-log_prob / count)
    
    def generate(
        self,
        seed_text: str,
        n_chars: int = 200,
        temperature: float = 1.0,
    ) -> str:
        """Generate text given seed."""
        x = np.zeros(self.N)
        
        # Warm up on seed
        for c in seed_text:
            idx = self.char2idx.get(c, 0)
            oh = self._one_hot(idx)
            pre = self.W_res @ x + self.W_in @ oh + self.bias
            x = (1 - self.alpha) * x + self.alpha * np.tanh(pre)
        
        # Generate
        result = list(seed_text)
        current_idx = self.char2idx.get(seed_text[-1], 0)
        
        for _ in range(n_chars):
            oh = self._one_hot(current_idx)
            pre = self.W_res @ x + self.W_in @ oh + self.bias
            x = (1 - self.alpha) * x + self.alpha * np.tanh(pre)
            
            probs = self._predict_probs(x)
            # Temperature scaling
            logits = np.log(probs + 1e-10) / temperature
            logits -= logits.max()
            probs = np.exp(logits) / np.exp(logits).sum()
            
            current_idx = self.rng.choice(self.V, p=probs)
            result.append(self.idx2char[current_idx])
        
        return ''.join(result)


def run_char_lm_demo(corpus_file: str = None) -> None:
    """Demonstrate character-level language model."""
    if corpus_file is None:
        # Use a small sample corpus
        corpus = """
        The reservoir computing framework offers a powerful approach to temporal sequence
        modeling. Unlike traditional recurrent networks that require backpropagation 
        through time, reservoir networks train only the output weights. The reservoir
        provides a rich, high-dimensional representation of the input history.
        Natural language exhibits complex temporal dependencies at multiple scales.
        """ * 50  # Repeat for enough training data
    else:
        with open(corpus_file) as f:
            corpus = f.read()
    
    n = len(corpus)
    train = corpus[:int(0.8*n)]
    test = corpus[int(0.8*n):]
    
    print(f"Corpus size: {n} chars (train: {len(train)}, test: {len(test)})")
    
    lm = ReservoirCharLM(
        n_reservoir=500,
        spectral_radius=0.95,
        leaking_rate=0.9,
    )
    lm.fit(train)
    
    ppl = lm.perplexity(test)
    print(f"\nTest perplexity: {ppl:.2f}")
    
    print("\nGenerated text (temperature=1.0):")
    print(lm.generate("The reservoir", n_chars=200, temperature=1.0))
    
    print("\nGenerated text (temperature=0.5, lower randomness):")
    print(lm.generate("The reservoir", n_chars=200, temperature=0.5))


if __name__ == "__main__":
    run_char_lm_demo()
```

## 25.1.5 Limitations and Where Reservoir LMs Struggle

**Long-range dependencies**: Many linguistic phenomena require tracking dependencies over dozens or hundreds of words — subject-verb agreement in complex sentences, coreference resolution, discourse coherence. The reservoir's fading memory has a characteristic horizon $\tau_{\text{mem}}$ beyond which input information is lost. Language has dependencies that exceed any fixed horizon.

**Rare word modeling**: The reservoir maps each input word to a fixed vector $W_{\text{in}}\mathbf{e}_w$. For rare words (appearing only a few times in training), the reservoir state after seeing a rare word may not meaningfully differ from the state after an OOV word. Pre-trained word embeddings (Section 25.1.2) partially address this.

**Vocabulary size**: The output layer $W_{\text{out}} \in \mathbb{R}^{V \times N}$ grows linearly with vocabulary size. For $V = 50,000$ (a realistic English vocabulary) and $N = 2000$, $W_{\text{out}}$ has $10^8$ parameters — not enormous by modern standards but computationally significant.

**Compositional structure**: Natural language meaning is compositional: the meaning of a phrase is determined by the meanings of its parts and their structural combination. Reservoir states do not directly represent compositional structure. This is the fundamental limitation, examined further in Section 25.2.
