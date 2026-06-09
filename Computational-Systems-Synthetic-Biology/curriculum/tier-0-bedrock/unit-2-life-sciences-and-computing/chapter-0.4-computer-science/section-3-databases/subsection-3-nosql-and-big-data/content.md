# NoSQL and Big Data Storage

Single-cell RNA sequencing gives you a matrix: roughly 50,000 to 500,000 cells on one axis, 30,000 genes on the other. That is up to $1.5 \times 10^{10}$ values — fifteen billion numbers — for a single experiment. In dense format, this is about 120 GB. In practice it is sparse (most genes are not expressed in any given cell, so most entries are zero), but even a 5% fill gives you hundreds of millions of non-zero values. You cannot fit this in a SQL table. You cannot load it into a Pandas DataFrame without running out of RAM. You need a different data model for a different kind of data.

Not all biological data is naturally tabular. Protein structure files, heterogeneous phenotype records, single-cell RNA-seq matrices, and genomic arrays are better represented in specialized data stores. NoSQL databases provide flexible schemas for heterogeneous data; HDF5 and Zarr provide efficient array storage for high-dimensional genomics data. Understanding when relational databases are insufficient — and what to use instead — is increasingly important as data scales grow.

## When Relational Databases Are Not Enough

SQL databases assume:
1. **Fixed schema**: all rows in a table have the same columns
2. **Atomic values**: cells hold a single value (1NF)
3. **Moderate scale**: queries scanning billions of rows become slow without careful partitioning
4. **Structured relationships**: data fits cleanly into entities and relationships

When these assumptions break down, NoSQL alternatives provide:
- **Flexible schema**: documents with varying fields (protein annotations have wildly different sets of go terms, literature references, cofactors)
- **Horizontal scaling**: distribute data across many nodes for very large datasets
- **Specialized access patterns**: key-value for caching, column-family for time-series or genomic position data

## Key-Value Stores: Redis

**Redis** is an in-memory key-value store with optional persistence. Its primary use in bioinformatics is **caching**:

- Cache BLAST results: expensive compute → store result by query sequence hash → next identical query returns immediately from Redis in $O(1)$
- Cache API responses (NCBI Entrez, UniProt) to avoid rate limits and repeated network calls
- Session storage for web-based genome browsers

```python
import redis
import json
import hashlib

r = redis.Redis(host='localhost', port=6379)

def blast_with_cache(sequence: str) -> dict:
    key = "blast:" + hashlib.md5(sequence.encode()).hexdigest()
    cached = r.get(key)
    if cached:
        return json.loads(cached)
    # Run BLAST (slow)
    result = run_blast(sequence)
    # Cache for 24 hours
    r.setex(key, 86400, json.dumps(result))
    return result
```

## Document Stores: MongoDB

**MongoDB** stores data as BSON documents (binary JSON), allowing each document to have different fields. This is well-suited for biological records where:
- Different genes have different numbers of GO terms, isoforms, or literature references
- Experimental records have varying metadata depending on the assay type
- Data is ingested incrementally and the schema evolves

```python
from pymongo import MongoClient
from pymongo import ASCENDING

client = MongoClient("mongodb://localhost:27017/")
db = client["genomics_db"]
genes = db["genes"]

# Insert a document — flexible schema; each document can differ
genes.insert_many([
    {
        "gene_symbol": "BRCA1",
        "ensembl_id": "ENSG00000012048",
        "go_terms": ["GO:0006281", "GO:0006974", "GO:0007049"],
        "pubmed_ids": [7545954, 7773291],
        "expression_summary": {"tissue": "breast", "tpm": 12.4}
    },
    {
        "gene_symbol": "ACTB",
        "ensembl_id": "ENSG00000075624",
        "go_terms": ["GO:0005737"],
        # No pubmed_ids field — this is fine in MongoDB
        "interactors": ["ARPC1A", "TWF1", "COBL"]
    }
])

# Query with complex filters
high_expr_dna_repair = genes.find(
    {"go_terms": "GO:0006281",                      # DNA repair GO term
     "expression_summary.tpm": {"$gt": 5.0}},       # high expression
    {"gene_symbol": 1, "go_terms": 1, "_id": 0}     # projection (select columns)
)

# Create index for performance
genes.create_index([("gene_symbol", ASCENDING)], unique=True)
genes.create_index([("go_terms", ASCENDING)])
```

## HDF5 and Zarr: Hierarchical Array Storage

For large numerical arrays — single-cell expression matrices, genomic coverage tracks, deep mutational scanning matrices — neither SQL nor document stores are appropriate. **HDF5** and **Zarr** provide hierarchical array storage optimized for high-dimensional data.

### HDF5

HDF5 (Hierarchical Data Format 5) organizes data into groups (like directories) and datasets (typed, multi-dimensional arrays):

```python
import h5py
import numpy as np

# Write a single-cell expression matrix
with h5py.File("scrna.h5", "w") as f:
    # Matrix: cells × genes (sparse storage)
    grp = f.create_group("matrix")
    grp.create_dataset("data",    data=csr_matrix.data,    compression="gzip")
    grp.create_dataset("indices", data=csr_matrix.indices, compression="gzip")
    grp.create_dataset("indptr",  data=csr_matrix.indptr,  compression="gzip")
    grp.attrs["shape"] = csr_matrix.shape
    
    # Metadata
    f.create_dataset("barcodes", data=np.array(barcodes, dtype="S18"))
    f.create_dataset("features/id",   data=np.array(gene_ids,   dtype="S15"))
    f.create_dataset("features/name", data=np.array(gene_names, dtype="S20"))

# Read selectively (no need to load entire file)
with h5py.File("scrna.h5", "r") as f:
    barcodes = f["barcodes"][:].astype(str)       # all barcodes
    gene_ids = f["features/id"][:1000].astype(str) # first 1000 genes only
```

**AnnData** (the format used by Scanpy for single-cell analysis) uses HDF5 as its on-disk format (`.h5ad` files). A typical human single-cell dataset: 100,000 cells × 30,000 genes = $3 \times 10^9$ values — far too large for memory in dense format. HDF5 with sparse storage and chunked access makes this tractable.

### Zarr

**Zarr** is a cloud-native alternative to HDF5 with native support for chunked, compressed, N-dimensional arrays. Key advantages over HDF5:
- Supports parallel writes (multiple processes writing different chunks simultaneously)
- Native cloud storage (S3, GCS) — chunks are individual objects
- Compatible with Dask for out-of-core computation

```python
import zarr
import numpy as np

# Create a zarr array (can be stored on S3)
z = zarr.open("coverage.zarr", mode='w',
              shape=(3200000000,),  # human genome length
              chunks=(1000000,),    # 1 Mb chunks
              dtype='float32',
              compressor=zarr.Blosc(cname='lz4', clevel=5))

# Write coverage in chunks
for start in range(0, len(genome), 1000000):
    z[start:start+1000000] = compute_coverage(start, start+1000000)

# Read a specific region instantly (only that chunk is decompressed)
region_coverage = z[100000000:101000000]  # chr1:100-101 Mb
```

## Column-Family Stores: HBase/Cassandra

For genomic position data accessed by chromosome and position range, **column-family stores** (Apache HBase, Apache Cassandra) provide efficient range queries:

- Rows are keyed by genomic position (chromosome:position)
- Columns hold variant annotations (allele frequencies, functional annotations)
- Range scans across positions are efficient

In practice, most bioinformatics uses tabix-indexed files (compressed, genomically indexed text files) rather than HBase for position-based range queries — tabix provides $O(\log n)$ access without database infrastructure.

## Choosing the Right Storage for Biological Data

| Data type | Recommended storage |
|---|---|
| Gene annotations, metadata | SQLite or PostgreSQL |
| Heterogeneous protein records | MongoDB |
| Sequence data (raw reads) | gzip/bgzip flat files |
| Aligned reads | BAM/CRAM (binary indexed) |
| Variant calls | VCF with tabix index |
| Single-cell expression matrices | AnnData/.h5ad (HDF5-backed) |
| Large numerical arrays | HDF5 or Zarr |
| API response caching | Redis |
| Genomic coverage tracks | BigWig (UCSC format), Zarr |
| Genome-scale k-mer databases | Jellyfish .jf format or RocksDB |

## Why This Matters for Computational Biology

Single-cell genomics datasets routinely exceed RAM — AnnData's lazy loading from HDF5 is the only practical way to work with 500,000-cell datasets. The choice between dense and sparse storage for a $10^5 \times 3 \times 10^4$ single-cell matrix (dense: 12 GB; sparse at 10% fill: 1.2 GB) is a practical decision with immediate memory consequences. Cloud-based genomics (AWS, Google Cloud) increasingly stores data in S3/GCS using Zarr or Parquet format — understanding these formats is necessary for large-scale genomics pipelines. MongoDB's flexible schema is the right choice for building a gene annotation database that must accommodate wildly varying levels of annotation detail across genes and organisms. Knowing when a tabix-indexed VCF is sufficient (single-user, single-machine analysis) vs. when a HBase cluster is necessary (real-time clinical variant lookup at population scale) prevents both over-engineering and under-engineering.
