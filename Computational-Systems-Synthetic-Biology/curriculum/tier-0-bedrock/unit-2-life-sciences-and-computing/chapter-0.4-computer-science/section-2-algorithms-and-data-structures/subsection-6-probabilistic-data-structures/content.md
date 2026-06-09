# Probabilistic Data Structures

Here is a question that reveals a deep tension in computational genomics: how do you determine, in real time, whether a specific 31-mer has been seen before in a dataset of 10 billion sequencing reads? An exact hash table would work beautifully — if you had 80 GB of RAM. A sorted list would work — if you were willing to wait. The real answer, used in production genomics tools at sequencing centers around the world, is to accept a small probability of being wrong.

This sounds scientifically unacceptable until you think about it carefully. If you can prove that your "wrong" answers are bounded — that the false positive rate is exactly 1% and false negatives never occur — then you have a tool with known, controlled error behavior. And you can make it fit in 1.2 GB instead of 80 GB. That tradeoff, in genomics, is not a compromise. It is engineering.

Genome-scale data has a fundamental tension: we want to answer set membership and frequency queries over billions of items (k-mers, reads, genomic variants) in constant time and sub-linear space. Exact data structures (hash tables) require space proportional to the number of elements — for $10^9$ distinct 31-mers in a mammalian genome, an exact hash table needs tens of gigabytes. **Probabilistic data structures** sacrifice exactness for dramatic gains in space efficiency, returning approximate answers with provable error bounds. They are not a curiosity — jellyfish, MinHash/Mash, and HyperLogLog are deployed in production bioinformatics tools at every large genomics center.

## Bloom Filters: Approximate Set Membership

A **Bloom filter** answers "is element $x$ in set $S$?" with the following guarantees:
- **No false negatives**: if $x$ is in $S$, the Bloom filter always says YES
- **Bounded false positives**: if $x$ is NOT in $S$, the filter says YES with probability $\leq \epsilon$ (false positive rate, tunable)
- **Space**: $O(n \log(1/\epsilon))$ bits — much smaller than $O(n \log n)$ for an exact set

### Structure and Operations

A Bloom filter is a **bit array** of $m$ bits, all initially 0, and $k$ independent hash functions $h_1, \ldots, h_k$ each mapping elements to $[0, m-1]$.

**Insert $x$**: Compute $h_1(x), \ldots, h_k(x)$; set those positions in the bit array to 1.

**Query $x$**: Compute $h_1(x), \ldots, h_k(x)$; if ALL those positions are 1, return YES; if any is 0, return NO (definitive: x was never inserted).

**No deletion**: Setting a bit to 0 would affect other elements that hashed to the same position.

**Optimal parameters**: For $n$ elements and desired false positive rate $\epsilon$:

$$m = -\frac{n \ln \epsilon}{(\ln 2)^2} \approx 1.44 n \log_2(1/\epsilon) \text{ bits}$$

$$k = \frac{m}{n} \ln 2 \approx 0.693 \frac{m}{n} \text{ hash functions}$$

For $n = 10^9$ k-mers and $\epsilon = 0.01$ (1% false positive rate):
$m \approx 1.44 \times 10^9 \times 6.64 \approx 9.6 \times 10^9$ bits ≈ 1.2 GB — vs. ~8 GB for an exact hash table.

```python
import hashlib
import math

class BloomFilter:
    def __init__(self, n: int, epsilon: float):
        self.m = math.ceil(-n * math.log(epsilon) / (math.log(2)**2))
        self.k = round(self.m / n * math.log(2))
        self.bits = bytearray(math.ceil(self.m / 8))
    
    def _hashes(self, item: str):
        # Simulate k hash functions via double hashing
        h1 = int(hashlib.md5(item.encode()).hexdigest(), 16)
        h2 = int(hashlib.sha1(item.encode()).hexdigest(), 16)
        for i in range(self.k):
            yield (h1 + i * h2) % self.m
    
    def add(self, item: str):
        for pos in self._hashes(item):
            self.bits[pos // 8] |= (1 << (pos % 8))
    
    def __contains__(self, item: str) -> bool:
        return all((self.bits[pos // 8] >> (pos % 8)) & 1
                   for pos in self._hashes(item))

# Usage: filter k-mers in reads against a reference k-mer set
bf = BloomFilter(n=10**8, epsilon=0.01)
for kmer in reference_kmers:
    bf.add(kmer)

# Now for each read k-mer, check membership in O(k) time, O(1) per hash
if read_kmer in bf:
    print("Likely in reference")
```

**Biological application**: **jellyfish** uses a Bloom filter as the first pass of k-mer counting — k-mers are hashed into a Bloom filter; only k-mers seen more than once are kept for full counting. **Kraken** (metagenomic classifier) uses a compact hash with Bloom filter pre-filter to classify reads against a reference database.

## MinHash: Estimating Jaccard Similarity

**Jaccard similarity** between two sets $A$ and $B$:

$$J(A, B) = \frac{|A \cap B|}{|A \cup B|}$$

Computing exact Jaccard requires knowing all elements of both sets. For two bacterial genomes (each with ~$10^6$ distinct 21-mers), exact Jaccard is feasible. For thousands of genomes pairwise, it is not.

**MinHash** estimates Jaccard similarity using compact sketches:

1. Choose $s$ independent hash functions $h_1, \ldots, h_s$ (or one hash function with $s$ seeds)
2. For each hash function $h_i$, compute $\min_{x \in A} h_i(x)$ — the minimum hash value of all elements in $A$
3. The **MinHash sketch** of $A$ is the vector $[\min_{x \in A} h_1(x), \ldots, \min_{x \in A} h_s(x)]$

**The key theorem**: The probability that the minimum hash value is the same for sets $A$ and $B$ equals their Jaccard similarity:

$$P\left[\min_{x \in A} h(x) = \min_{x \in B} h(x)\right] = J(A, B)$$

**Proof sketch**: The minimum of $h$ over $A \cup B$ is the element in $A \cup B$ with the smallest hash value. This element falls in $A \cap B$ with probability $|A \cap B|/|A \cup B|$ = $J(A, B)$.

Therefore, the fraction of MinHash positions where sketch(A) and sketch(B) agree is an unbiased estimator of $J(A, B)$:

$$\hat{J}(A, B) = \frac{|\{i : \text{sketch}(A)[i] = \text{sketch}(B)[i]\}|}{s}$$

Standard error of the estimator: $\sqrt{J(1-J)/s}$ — with $s = 1000$, the standard error is $\leq 0.016$ for any $J \in [0,1]$.

**Mash and sourmash** use MinHash on k-mer sets of genomes to rapidly estimate genomic distance for thousands of genomes:

```python
# Conceptual: compute MinHash sketch
import hashlib

def minhash_sketch(seq: str, k: int, s: int, seed: int = 42) -> list:
    """Compute MinHash sketch of s minimizers over all k-mers."""
    import heapq
    heap = []
    for i in range(len(seq) - k + 1):
        kmer = seq[i:i+k]
        h = int(hashlib.md5((str(seed) + kmer).encode()).hexdigest(), 16)
        if len(heap) < s:
            heapq.heappush(heap, -h)  # max-heap trick
        elif -h > heap[0]:
            heapq.heapreplace(heap, -h)
    return sorted(-x for x in heap)

def jaccard_estimate(sketch_a, sketch_b):
    """Estimate Jaccard from two MinHash sketches of the same size."""
    matches = sum(a == b for a, b in zip(sketch_a, sketch_b))
    return matches / len(sketch_a)
```

## Count-Min Sketch: Approximate Frequency Counting

The **Count-Min Sketch (CMS)** estimates the frequency of elements in a stream with sub-linear space. It is a 2D array of $d$ rows and $w$ columns, with $d$ independent hash functions:

**Update** element $x$ (increment its count): for each row $i$, increment `CMS[i][h_i(x)]`

**Query** element $x$: return $\min_i \text{CMS}[i][h_i(x)]$

The estimate is always $\geq$ true count (hash collisions only add to counts, never subtract). The error bound: the estimate exceeds the true count by more than $\epsilon \cdot N$ (where $N$ is total stream size) with probability $\leq \delta$, using:
- $w = e/\epsilon$ columns
- $d = \ln(1/\delta)$ rows

**Biological application**: Estimating k-mer frequencies in a read dataset without storing all k-mers exactly — used in read error correction (k-mers with frequency 1 are likely sequencing errors) and in genome size estimation from k-mer frequency histograms.

## Why This Matters for Computational Biology

Every major sequencing data tool uses probabilistic data structures. jellyfish, the standard k-mer counter, uses a Bloom filter as a pre-filter and a compact hash table for the full count. Mash/Mash2 (NCBI Sequence Read Archive genome distance) use MinHash sketching to produce an all-vs-all distance matrix for thousands of genomes that would be intractable with exact methods. Kraken2 uses Compact Hash (similar to Bloom filter) for rapid metagenomic classification of reads. Understanding these structures means you can tune tools — you know that increasing the sketch size in sourmash reduces estimation error at the cost of more memory, that Bloom filter false positive rates can be traded against memory, and that Count-Min Sketch overestimates never underestimates (important for interpreting frequency estimates). When implementing your own tools, these structures extend what is computationally tractable by orders of magnitude.
