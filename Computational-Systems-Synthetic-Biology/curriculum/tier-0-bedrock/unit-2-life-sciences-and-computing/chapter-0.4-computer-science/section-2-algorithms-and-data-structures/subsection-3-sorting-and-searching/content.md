# Sorting and Searching

The most basic fact about working with genomic data is that it is coordinate data. Variants have chromosomal positions. Reads align to positions. Gene annotations occupy intervals. And almost every query you want to ask — does this variant fall in an exon? which peaks overlap which promoters? how deep is the coverage at position 43,092,919 of chromosome 17? — is fundamentally a question about sorted data. The entire architecture of indexed file formats — the `.bai` index for BAM files, the tabix index for VCF files, the `.fai` index for FASTA files — is built on the simple principle that sorted data can be searched in logarithmic time, and logarithmic time is effectively instant even for billions of records.

Sorting and searching are the most common computational operations. Understanding the algorithms behind them — why they have the complexity they do, when linear-time sorts are possible — allows you to write and choose tools intelligently.

## Comparison-Based Sorting: $O(n \log n)$ Lower Bound

Any algorithm that sorts by comparing pairs of elements requires at least $\Omega(n \log n)$ comparisons in the worst case. The proof uses information theory: $n!$ possible orderings must be distinguished; each comparison provides at most 1 bit of information; therefore $\log_2(n!) \approx n \log_2 n$ comparisons are needed (Stirling's approximation).

This lower bound is not a limitation of existing algorithms — it is a theorem about what is theoretically possible. No comparison-based sort can do better than $O(n \log n)$.

### Merge Sort

Merge sort achieves $O(n \log n)$ worst case with $O(n)$ extra space:
1. Divide array into two halves
2. Recursively sort each half
3. Merge the two sorted halves in $O(n)$ time

```python
def merge_sort(arr):
    if len(arr) <= 1:
        return arr
    mid = len(arr) // 2
    left  = merge_sort(arr[:mid])
    right = merge_sort(arr[mid:])
    return merge(left, right)

def merge(left, right):
    result, i, j = [], 0, 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            result.append(left[i]); i += 1
        else:
            result.append(right[j]); j += 1
    return result + left[i:] + right[j:]
```

Merge sort is **stable** (equal elements retain original order) and **parallelizable** (the two halves are independent). SAMtools uses an external merge sort to sort BAM files that do not fit in memory: sort chunks individually, then merge.

### Quicksort

Quicksort has $O(n \log n)$ average, $O(n^2)$ worst case:
1. Choose a pivot
2. Partition: elements < pivot to left, > pivot to right
3. Recursively sort each partition

With randomized pivot selection, worst case is astronomically unlikely in practice. Quicksort has smaller constant factors than merge sort and is the basis of most `sort()` implementations (including Python's Timsort, a hybrid).

### Heapsort

$O(n \log n)$ worst case, $O(1)$ extra space (in-place). Build a max-heap from the array, then repeatedly extract the maximum. Used when in-place sorting with guaranteed $O(n \log n)$ is required.

## Linear-Time Sorts: When Comparison is Unnecessary

You might expect that $O(n \log n)$ is the best possible for sorting. It is — if you are restricted to comparing elements. But genomic data has a special structure: positions are integers in a known range. This unlocks algorithms that sort without comparisons, in $O(n)$ time. This is not a trick; it is a fundamentally different approach that exploits structure in the data.

### Counting Sort

For integer keys in range $[0, k-1]$:
1. Count occurrences of each value: $O(n + k)$
2. Compute prefix sums: $O(k)$
3. Place elements in output array using prefix sums as position indices: $O(n)$

Total: $O(n + k)$ — linear when $k = O(n)$.

**Biological application**: Sorting genomic positions on a single chromosome when the number of positions is comparable to chromosome length; sorting quality score histograms (k = 40 possible values).

### Radix Sort

Sort integers digit by digit from least significant to most significant, using counting sort at each digit. Time: $O(d \cdot (n + b))$ where $d$ is number of digits and $b$ is base.

**Biological application**: Sorting k-mers (encoded as integers) — critical in genome assembly and read alignment. Suffix array construction algorithms use radix sort to achieve $O(n \log n)$ or $O(n)$ construction.

## Binary Search: $O(\log n)$ Lookup in Sorted Data

Given a **sorted** array, binary search finds a target in $O(\log n)$ by repeatedly halving the search space:

```python
def binary_search(arr, target):
    lo, hi = 0, len(arr) - 1
    while lo <= hi:
        mid = (lo + hi) // 2
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1  # not found
```

`bisect.bisect_left(arr, x)` in Python's standard library returns the leftmost insertion point for `x` in sorted `arr` — use this rather than implementing binary search yourself.

**Genomic interval binary search**: A chromosome's variant positions are stored sorted. To find all variants overlapping region [start, end], binary search to the first position ≥ start, then scan until position > end. This is $O(\log n + k)$ where $k$ is the number of overlapping variants.

```python
import bisect

positions = sorted([100, 250, 400, 780, 1200, 1500])  # sorted variant positions
query_start, query_end = 200, 800

lo = bisect.bisect_left(positions, query_start)
hi = bisect.bisect_right(positions, query_end)
overlapping = positions[lo:hi]   # [250, 400, 780]
```

## Hash-Based Lookup: $O(1)$ Average

For exact key lookup (not range queries), hash tables provide $O(1)$ average. Python's `dict` and `set` are hash tables. Always prefer `in set_name` over searching a list when membership testing matters:

```python
# O(n) membership test — bad for repeated queries
gene_list = ["BRCA1", "TP53", "EGFR", ...]
if "TP53" in gene_list:  # scans the whole list

# O(1) membership test — correct
gene_set = {"BRCA1", "TP53", "EGFR", ...}
if "TP53" in gene_set:  # hash lookup
```

## Practical Example: Sorting Reads by Genomic Coordinate

SAMtools sort converts a name-sorted or unsorted BAM to a coordinate-sorted BAM. The algorithm:

1. Read chunks of ~768 MB into memory (configurable via `-m`)
2. Sort each chunk in memory (merge sort or radix sort on position)
3. Write sorted chunks to temporary files
4. Merge all temporary files into the final output (k-way merge using a priority queue)

The k-way merge heap: with $k$ sorted runs, maintain a min-heap of size $k$ with the smallest element from each run. Extract the minimum, emit it, and insert the next element from the same run. Each extraction is $O(\log k)$; total: $O(n \log k)$.

For the human genome with ~$6 \times 10^8$ reads at 30× coverage, with 8 chunks: $O(n \log 8) = O(3n)$ — essentially linear in the total number of reads.

## Why This Matters for Computational Biology

Genomic data is fundamentally sorted data — every indexed file format (BAM/CRAM with `.bai` index, VCF with tabix index, FASTA with `.fai` index) depends on sorted storage to enable $O(\log n)$ random access. Understanding why tabix can answer "give me all variants in chr1:1000000-2000000" in milliseconds (binary search in the index, then seek to the file offset) explains why tools require sorted input. When writing your own analysis code, knowing when to reach for `bisect`, when to build a `set`, and when to sort first and then binary-search repeatedly is the difference between an analysis that finishes and one that times out.
