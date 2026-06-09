# Fundamental Data Structures

BWA-MEM aligns a 150 bp sequencing read to the 3-billion-base human genome in about 1 millisecond. Think about that for a moment. The text of this book is roughly 1 million characters — the human genome is 3,000 times longer. Yet a short read can be located within it in less time than it takes for a neuron to fire. This is not the result of raw computing speed. It is the result of a clever choice of data structure: the FM-index, a compressed suffix array built from the Burrows-Wheeler transform. The data structure is the algorithm.

Data structures are the containers that organize information in memory. Choosing the right data structure for a problem is often the difference between an algorithm that completes in seconds and one that takes hours. In computational biology, the choice of data structure underlies the performance of sequence aligners, genome assemblers, variant callers, and every other tool in the standard pipeline. Understanding the time and space complexity of basic operations is prerequisite knowledge.

## Arrays and Contiguous Memory

An **array** stores elements in contiguous memory locations. Random access by index is $O(1)$ — the address of element $i$ is simply base address + $i \times$ element size. This cache-friendly layout makes arrays very fast in practice.

In Python, the `list` type is a dynamic array (resizable array of pointers, not of values — important for NumPy's advantage). NumPy `ndarray` is a true typed array with contiguous memory: a `float64` array of $10^6$ elements occupies exactly $8 \times 10^6$ bytes.

**Time complexity:**
- Access by index: $O(1)$
- Insert/delete at end: $O(1)$ amortized
- Insert/delete in middle: $O(n)$ (must shift elements)
- Search (unsorted): $O(n)$

**Biological use**: Coverage arrays (one value per genomic position), score matrices for alignment (dynamic programming tables), k-mer frequency arrays indexed by hash value.

## Hash Tables

A **hash table** maps keys to values in $O(1)$ average time by computing an index from the key using a **hash function**.

**Mechanism**: `hash(key) % table_size → bucket`. Collisions (two keys hashing to the same bucket) are handled by chaining (linked list in the bucket) or open addressing (probe to next open slot). With a good hash function and load factor < 0.7, most operations are $O(1)$ average.

**Worst case**: $O(n)$ if all keys collide (degenerate hash function or adversarial input). For DNA sequences, simple hash functions on base characters work well because the alphabet (A, C, G, T) has low entropy.

```python
# Python dict is a hash table
kmer_counts = {}
seq = "ACGATCGATCG"
k = 3
for i in range(len(seq) - k + 1):
    kmer = seq[i:i+k]
    kmer_counts[kmer] = kmer_counts.get(kmer, 0) + 1
# {'ACG': 2, 'CGA': 2, 'GAT': 2, 'ATC': 2, 'TCG': 2}
```

**Biological use**: k-mer counting (genome assembly, error correction), sequence-to-ID mapping, gene annotation lookup, BLAST word lookup tables.

## Trees

**Binary search trees (BST)**: Each node has at most two children; left subtree values < node < right subtree. $O(\log n)$ average for search, insert, delete — but $O(n)$ worst case if the tree is unbalanced (degenerate to a linked list).

**Balanced BSTs (AVL, red-black trees)**: Self-balancing; guarantee $O(\log n)$ worst case. Python's `sortedcontainers.SortedList` uses a B-tree variant; most languages' standard library sorted map/set uses a red-black tree.

**Heaps (priority queues)**: Complete binary trees where the parent is always smaller (min-heap) or larger (max-heap) than children. $O(\log n)$ insert; $O(\log n)$ remove minimum/maximum; $O(1)$ peek minimum/maximum. Heapified array — not a tree object.

```python
import heapq
# Min-heap: Dijkstra's algorithm priority queue
pq = []
heapq.heappush(pq, (0, "start"))    # (priority, node)
dist, node = heapq.heappop(pq)      # O(log n)
```

**Interval trees / segment trees**: Enable efficient querying of which intervals overlap a point or range. Used in **GenomicRanges** (R) and **pybedtools** (Python) for genomic interval operations. Finding all genes overlapping a variant position in a VCF is an interval tree query.

**Suffix trees and suffix arrays**: Compressed trie-based structures for indexing all suffixes of a string. Enable $O(m)$ (pattern length) exact match queries against an $O(n)$ (text length) index, after $O(n)$ construction. Used in genome aligners (see string algorithms subsection).

## Graphs

**Adjacency matrix**: An $n \times n$ matrix where entry $(i, j) = 1$ (or edge weight) if edge $i \to j$ exists. $O(1)$ edge query; $O(n^2)$ space — only appropriate for dense graphs.

**Adjacency list**: Each node maintains a list of its neighbors. $O(1)$ amortized edge insertion; $O(V + E)$ space — appropriate for sparse graphs (most biological networks).

In Python:
```python
from collections import defaultdict

# Metabolic network as adjacency list
graph = defaultdict(list)
graph["glucose"].append("glucose-6-phosphate")
graph["glucose-6-phosphate"].append("fructose-6-phosphate")
graph["glucose-6-phosphate"].append("glucose-1-phosphate")  # branch

# BFS from a node
from collections import deque
def bfs(graph, start):
    visited = {start}
    queue = deque([start])
    while queue:
        node = queue.popleft()
        for neighbor in graph[node]:
            if neighbor not in visited:
                visited.add(neighbor)
                queue.append(neighbor)
    return visited
```

**Biological graphs**: Protein-protein interaction networks (PPI), gene regulatory networks, metabolic networks (KEGG), co-expression networks, phylogenetic trees (DAGs), de Bruijn graphs (sequence assembly).

## Stacks and Queues

**Stack (LIFO)**: `push` and `pop` from the same end. $O(1)$ operations. Used in: DFS (recursive or explicit stack), RNA secondary structure parsing (matching brackets), undo functionality.

**Queue (FIFO)**: `enqueue` at back, `dequeue` from front. $O(1)$ operations with `collections.deque`. Used in: BFS, pipeline data processing (FIFO buffer).

**Deque (double-ended queue)**: Efficient $O(1)$ operations at both ends. Python's `collections.deque` is implemented as a doubly-linked list of fixed-size blocks.

## Worked Example: Choosing a Data Structure for Variant Lookup

Problem: Given a VCF with 5 million variants (chromosome, position, ref, alt), quickly determine for any query position whether a variant exists.

- **Sorted list + binary search**: $O(\log n)$ per query; $O(n)$ space. Works if variants fit in memory.
- **Hash table keyed by (chrom, pos)**: $O(1)$ average per query; ~$80 \times 10^6$ bytes for 5M entries (with Python dict overhead); fast to build.
- **Interval tree**: Needed only if queries are ranges (overlapping intervals), not point queries.

For a point query, a hash table (Python `dict` or `set`) is clearly best. For a range query ("give me all variants in chr1:1000000-2000000"), an interval tree or sorted array with binary search is needed.

## Why This Matters for Computational Biology

The reason BWA-MEM can align a 150 bp read to the human genome in milliseconds is that it indexes the genome using a suffix array (compressed FM-index) — an $O(n)$ space structure enabling $O(m)$ lookup. The reason genome assembly is possible at all is the de Bruijn graph — a k-mer overlap graph structure that enables assembly without all-vs-all comparison. Interval trees are the core of every genomic interval operation. Hash tables make k-mer counting tractable. Understanding these structures means you can read methods sections, evaluate algorithm scalability claims, and implement efficient solutions when standard tools do not cover your specific problem.
