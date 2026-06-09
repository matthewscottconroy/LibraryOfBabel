# Neural Network Fundamentals

In 1943, Warren McCulloch and Walter Pitts proposed a mathematical model of a neuron as a simple threshold device: it fires if its inputs sum above a threshold, and stays silent otherwise. The model was inspired by biology but was barely useful for computation. Seventy years later, the descendants of that idea — deep neural networks with millions of parameters trained by gradient descent — can predict protein structure, decode neural activity from calcium imaging, and classify cell types from microscopy images with superhuman accuracy. What changed? Not the basic architecture, but the scale, the data, and the training algorithms.

A **neural network** is a parameterized function $f_\theta: \mathbb{R}^n \to \mathbb{R}^m$ composed of alternating linear transformations and element-wise nonlinear activations. Stacked into many layers, these networks can approximate any continuous function (Universal Approximation Theorem) and — given sufficient data and compute — learn complex representations from raw biological inputs: DNA sequences, protein structures, microscopy images, or mass spectra.

## Architecture: Layers, Weights, Activations

A **feedforward neural network** with $L$ hidden layers transforms an input $\mathbf{x}$ through a sequence of layers:

$$\mathbf{h}^{(l)} = \sigma\!\left(W^{(l)} \mathbf{h}^{(l-1)} + \mathbf{b}^{(l)}\right), \quad l = 1, \ldots, L$$

where $W^{(l)}$ is the weight matrix, $\mathbf{b}^{(l)}$ the bias vector, and $\sigma$ the activation function applied element-wise.

**Activation functions:**
- **ReLU:** $\sigma(x) = \max(0, x)$ — sparse activations; no vanishing gradient for positive inputs; default for hidden layers
- **GELU:** $\sigma(x) = x \cdot \Phi(x)$ — smoother ReLU variant; used in transformers
- **Sigmoid:** $\sigma(x) = 1/(1+e^{-x})$ — squashes to $(0,1)$; used for binary output
- **Softmax:** $\sigma_k(\mathbf{x}) = e^{x_k}/\sum_j e^{x_j}$ — normalizes to probability distribution; multiclass output

## Training: Backpropagation and Gradient Descent

**Backpropagation** computes $\partial \mathcal{L}/\partial W^{(l)}$ for all layers by applying the chain rule backward through the computation graph. For a minibatch of size $B$:

$$\mathcal{L} = \frac{1}{B}\sum_{i=1}^B \ell(f_\theta(\mathbf{x}_i), y_i)$$

Loss functions:
- Binary cross-entropy: $-[y \log \hat{y} + (1-y)\log(1-\hat{y})]$
- Categorical cross-entropy: $-\sum_k y_k \log \hat{y}_k$
- Mean squared error: $\|\mathbf{y} - \hat{\mathbf{y}}\|^2$

```python
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, TensorDataset
import numpy as np

# Build a feedforward network for binary classification
# from gene expression features
class ExpressionClassifier(nn.Module):
    def __init__(self, n_genes, hidden_dims=(256, 128, 64), dropout=0.3):
        super().__init__()
        
        layers = []
        in_dim = n_genes
        for out_dim in hidden_dims:
            layers.extend([
                nn.Linear(in_dim, out_dim),
                nn.BatchNorm1d(out_dim),  # normalize activations within minibatch
                nn.ReLU(),
                nn.Dropout(p=dropout)    # randomly zero activations during training
            ])
            in_dim = out_dim
        
        layers.append(nn.Linear(in_dim, 1))  # binary output (logit)
        self.network = nn.Sequential(*layers)
    
    def forward(self, x):
        return self.network(x).squeeze(-1)   # shape: (batch,)

# Simulated gene expression: 500 samples, 2000 genes
rng = np.random.default_rng(42)
n_samples, n_genes = 500, 2000
X_np = rng.standard_normal((n_samples, n_genes)).astype(np.float32)
y_np = (X_np[:, :20].mean(1) + rng.standard_normal(n_samples) > 0).astype(np.float32)

# Split into train/val
split = int(0.8 * n_samples)
X_train = torch.from_numpy(X_np[:split])
y_train = torch.from_numpy(y_np[:split])
X_val   = torch.from_numpy(X_np[split:])
y_val   = torch.from_numpy(y_np[split:])

train_loader = DataLoader(TensorDataset(X_train, y_train), batch_size=32, shuffle=True)

# Model setup
model = ExpressionClassifier(n_genes=n_genes, hidden_dims=(256, 128, 64), dropout=0.3)
optimizer = optim.Adam(model.parameters(), lr=1e-3, weight_decay=1e-4)
criterion = nn.BCEWithLogitsLoss()
scheduler = optim.lr_scheduler.ReduceLROnPlateau(optimizer, patience=5, factor=0.5)

def train_epoch(model, loader, optimizer, criterion):
    model.train()
    total_loss = 0
    for X_batch, y_batch in loader:
        optimizer.zero_grad()
        logits = model(X_batch)
        loss = criterion(logits, y_batch)
        loss.backward()
        # Gradient clipping: prevent explosive gradients
        torch.nn.utils.clip_grad_norm_(model.parameters(), max_norm=1.0)
        optimizer.step()
        total_loss += loss.item()
    return total_loss / len(loader)

def eval_model(model, X, y):
    model.eval()
    with torch.no_grad():
        logits = model(X)
        loss = criterion(logits, y).item()
        preds = (logits > 0).float()
        acc = (preds == y).float().mean().item()
    return loss, acc

# Training loop with early stopping
best_val_loss = float('inf')
patience_counter = 0
best_state = None
n_epochs = 100

train_losses, val_losses = [], []
for epoch in range(n_epochs):
    train_loss = train_epoch(model, train_loader, optimizer, criterion)
    val_loss, val_acc = eval_model(model, X_val, y_val)
    scheduler.step(val_loss)
    
    train_losses.append(train_loss)
    val_losses.append(val_loss)
    
    if val_loss < best_val_loss:
        best_val_loss = val_loss
        best_state = {k: v.clone() for k, v in model.state_dict().items()}
        patience_counter = 0
    else:
        patience_counter += 1
    
    if epoch % 10 == 0:
        print(f"Epoch {epoch:3d}: train_loss={train_loss:.4f}, "
              f"val_loss={val_loss:.4f}, val_acc={val_acc:.3f}")
    
    if patience_counter >= 15:
        print(f"Early stopping at epoch {epoch}")
        break

# Restore best model
model.load_state_dict(best_state)
_, final_acc = eval_model(model, X_val, y_val)
print(f"Final validation accuracy: {final_acc:.3f}")
```

## Regularization Techniques

**Overfitting** occurs when training loss decreases but validation loss increases — the network memorizes training examples rather than learning general patterns. For small biological datasets, regularization is essential:

**Batch normalization** normalizes activations within each minibatch:
$$\hat{h} = \frac{h - \mu_B}{\sqrt{\sigma_B^2 + \epsilon}} \gamma + \beta$$
Benefits: stabilizes training, allows higher learning rates, reduces sensitivity to initialization.

**Dropout** randomly zeros activations with probability $p$ during training, forcing the network to learn redundant representations:

**L2 regularization** adds $\lambda \sum_i w_i^2$ to the loss (implemented as `weight_decay` in the optimizer).

**Data augmentation**: for sequence data, generate additional training examples via reverse complement, synonymous mutations, or random crop of longer sequences.

## Diagnosing Training

```python
import matplotlib.pyplot as plt

fig, axes = plt.subplots(1, 2, figsize=(10, 4))

# Loss curves: canonical diagnostic
epochs = range(len(train_losses))
axes[0].plot(epochs, train_losses, 'C0-', label='Training loss')
axes[0].plot(epochs, val_losses, 'C1-', label='Validation loss')
axes[0].set_xlabel('Epoch')
axes[0].set_ylabel('BCE Loss')
axes[0].legend()
axes[0].set_title('Learning curves')

# Patterns to recognize:
# 1. Both high: underfitting — increase model capacity or train longer
# 2. Val >> Train: overfitting — increase dropout, L2, or get more data
# 3. Both decrease, then val increases: overfit — use early stopping checkpoint
# 4. Loss oscillates: learning rate too high — reduce lr or use scheduler

# Gradient norms: another training diagnostic
gradient_norms = []
model.train()
for X_batch, y_batch in train_loader:
    optimizer.zero_grad()
    loss = criterion(model(X_batch), y_batch)
    loss.backward()
    total_norm = sum(p.grad.norm()**2 for p in model.parameters() if p.grad is not None)**0.5
    gradient_norms.append(total_norm.item())
    break  # just one batch for illustration

print(f"Gradient norm: {gradient_norms[0]:.4f}")
# If very small (<1e-6): vanishing gradients — use residual connections
# If very large (>100): exploding gradients — clip gradients
```

## Why This Matters

Neural network fundamentals — the architecture, activation functions, backpropagation, loss functions, and regularization — are the foundation for all of deep learning for biology. Understanding these mechanisms allows you to diagnose training problems, adapt architectures to specific biological data types, and critically evaluate published deep learning models. CNNs for sequence, transformers for proteins, and graph neural networks for molecules all build directly on these foundations.
