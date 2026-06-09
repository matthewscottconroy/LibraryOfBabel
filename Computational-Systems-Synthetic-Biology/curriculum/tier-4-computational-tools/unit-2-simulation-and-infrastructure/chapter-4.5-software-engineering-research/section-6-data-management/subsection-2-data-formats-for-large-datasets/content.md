# Data Formats for Large Datasets

A single-cell RNA-seq experiment from 2024 might profile 500,000 cells, each measuring expression across 30,000 genes. If you stored that as a plain CSV — one row per gene, one column per cell — you would need approximately 15 terabytes of text. Reading any subset of those cells would require loading the entire file. Every analysis would start with a ten-hour data loading step. This is not a hypothetical inconvenience; it is a hard barrier to doing the science. The scale of modern genomic data has made format choice not a matter of preference but a prerequisite for tractability.

Biological datasets have grown to scales where CSV files are not viable: a single-cell RNA-seq experiment generates a 30,000-gene × 500,000-cell count matrix, which as a dense CSV would be ~15 TB. Modern large-scale biology requires **binary, compressed, self-describing, hierarchical** data formats. The two dominant choices are **HDF5** (Hierarchical Data Format 5) and **Zarr**, each with distinct strengths.

## HDF5: The Standard for Structured Scientific Data

**HDF5** is the de facto standard for scientific data storage in biology, physics, and climate science. It organizes data in a **group/dataset hierarchy** analogous to a filesystem: groups are like directories, datasets are like files containing arrays. Datasets are stored in compressed binary format and support **chunked storage** — dividing arrays into fixed-size blocks that can be read independently without loading the entire dataset.

Key properties of HDF5:
- **Self-describing**: metadata (shape, dtype, units) stored alongside data
- **Hierarchical**: complex data structures (counts + metadata + embeddings) in one file
- **Compressed**: transparent gzip/LZ4/Blosc compression
- **Chunked**: efficient partial I/O — read one slice without loading full array
- **Universally supported**: C, Python, R, MATLAB, Julia all have native HDF5 libraries

```python
import h5py
import numpy as np
import pandas as pd

def create_rnaseq_h5(
    output_file: str,
    count_matrix: np.ndarray,
    gene_names: list[str],
    sample_names: list[str],
    metadata: pd.DataFrame,
    compression: str = "gzip",
    compression_opts: int = 4,
    chunk_size: tuple = (1000, 100)
):
    """
    Store an RNA-seq count matrix in HDF5 format with metadata.
    
    Layout:
      /counts            — float32 count matrix (genes × samples)
      /gene_names        — gene identifiers (string)
      /sample_names      — sample identifiers (string)
      /metadata/         — group for sample metadata
          /condition     — treatment condition
          /batch         — sequencing batch
    """
    n_genes, n_samples = count_matrix.shape
    assert len(gene_names) == n_genes
    assert len(sample_names) == n_samples

    with h5py.File(output_file, "w") as f:
        # Root attributes: file-level metadata
        f.attrs["description"] = "RNA-seq count matrix"
        f.attrs["created"]     = pd.Timestamp.now().isoformat()
        f.attrs["n_genes"]     = n_genes
        f.attrs["n_samples"]   = n_samples

        # Count matrix with chunking and compression
        # chunk_size determines read efficiency:
        # - (n_genes, 1) → fast per-sample access
        # - (1, n_samples) → fast per-gene access
        # - (1000, 100) → balanced
        f.create_dataset(
            "counts",
            data=count_matrix.astype(np.float32),
            chunks=chunk_size,
            compression=compression,
            compression_opts=compression_opts,
            shuffle=True   # pre-shuffle bytes for better compression
        )

        # Variable-length string datasets for gene/sample names
        str_dtype = h5py.special_dtype(vlen=str)
        f.create_dataset("gene_names",   data=gene_names,   dtype=str_dtype)
        f.create_dataset("sample_names", data=sample_names, dtype=str_dtype)

        # Metadata group
        meta_group = f.create_group("metadata")
        for col in metadata.columns:
            col_data = metadata[col].values
            if col_data.dtype.kind in ("U", "O"):  # string columns
                meta_group.create_dataset(col, data=col_data.astype(str), dtype=str_dtype)
            else:
                meta_group.create_dataset(col, data=col_data)

    print(f"Saved: {output_file}")
    print(f"  Shape: {n_genes} genes × {n_samples} samples")


def read_rnaseq_h5(h5_file: str, genes: list[str] = None, samples: list[str] = None):
    """
    Read count matrix from HDF5, optionally subsetting by genes or samples.
    
    Chunked reading: only reads necessary blocks, not entire matrix.
    """
    with h5py.File(h5_file, "r") as f:
        stored_genes   = f["gene_names"][:].astype(str)
        stored_samples = f["sample_names"][:].astype(str)

        gene_idx   = (np.isin(stored_genes, genes).nonzero()[0]
                      if genes else slice(None))
        sample_idx = (np.isin(stored_samples, samples).nonzero()[0]
                      if samples else slice(None))

        # HDF5 fancy indexing: only requested chunks are decompressed
        counts = f["counts"][gene_idx, :][:, sample_idx]

        # Load metadata
        metadata_dict = {}
        if "metadata" in f:
            for key in f["metadata"]:
                metadata_dict[key] = f["metadata"][key][sample_idx]

    selected_genes   = stored_genes[gene_idx]
    selected_samples = stored_samples[sample_idx]
    metadata_df = pd.DataFrame(metadata_dict, index=selected_samples)

    print(f"Loaded: {counts.shape[0]} genes × {counts.shape[1]} samples")
    return pd.DataFrame(counts, index=selected_genes, columns=selected_samples), metadata_df


def benchmark_h5_chunking():
    """
    Demonstrate chunking strategy impact on read performance.
    """
    import time
    rng = np.random.default_rng(42)
    data = rng.negative_binomial(10, 0.1, size=(20000, 500)).astype(np.float32)
    genes = [f"GENE_{i}" for i in range(20000)]

    configs = [
        ("row_chunks",    (20000, 1)),   # optimized for column (sample) access
        ("col_chunks",    (1, 500)),     # optimized for row (gene) access
        ("balanced",      (1000, 50)),   # balanced
    ]

    for name, chunks in configs:
        fname = f"/tmp/test_{name}.h5"
        with h5py.File(fname, "w") as f:
            f.create_dataset("counts", data=data, chunks=chunks,
                             compression="gzip", compression_opts=4)

        # Benchmark: read 100 samples
        with h5py.File(fname, "r") as f:
            t0 = time.perf_counter()
            _ = f["counts"][:, :100]
            t_col = time.perf_counter() - t0

            t0 = time.perf_counter()
            _ = f["counts"][:100, :]
            t_row = time.perf_counter() - t0

        file_size_mb = os.path.getsize(fname) / 1e6
        print(f"{name:15s}  chunk={str(chunks):15s}  "
              f"read_100_cols={t_col:.3f}s  read_100_rows={t_row:.3f}s  "
              f"size={file_size_mb:.0f}MB")
```

The benchmark reveals an important lesson about chunking strategy: there is no universally optimal chunk shape. If your analysis accesses data sample-by-sample (one column at a time), you want thin tall chunks. If you access data gene-by-gene (one row at a time), you want wide flat chunks. If both patterns occur, you choose a compromise. This is a genuinely scientific decision with measurable consequences — the wrong chunking strategy can make an analysis ten times slower.

## Zarr: Cloud-Native Array Storage

**Zarr** is a newer format designed for cloud object storage. Where HDF5 stores everything in one file, Zarr stores each chunk as a separate file/object — enabling parallel writes from multiple processes and efficient partial reads from S3 or GCS without downloading the entire dataset.

```python
import zarr
import numcodecs

def create_zarr_store(
    output_path: str,
    count_matrix: np.ndarray,
    gene_names: list[str],
    sample_names: list[str],
    chunks: tuple = (1000, 100),
    use_cloud: bool = False
):
    """
    Store a count matrix in Zarr format (local or cloud).
    
    For cloud (S3), output_path = "s3://my-bucket/my-dataset.zarr"
    Requires: pip install zarr s3fs
    """
    compressor = numcodecs.Blosc(cname="lz4", clevel=5, shuffle=numcodecs.Blosc.BITSHUFFLE)

    if use_cloud and output_path.startswith("s3://"):
        import s3fs
        store = s3fs.S3Map(root=output_path, s3=s3fs.S3FileSystem())
    else:
        store = output_path

    # Create root group
    root = zarr.open_group(store, mode="w")

    # Add attributes
    root.attrs["description"] = "RNA-seq count matrix"
    root.attrs["n_genes"]     = count_matrix.shape[0]
    root.attrs["n_samples"]   = count_matrix.shape[1]

    # Store count matrix
    root.create_dataset(
        "counts",
        data=count_matrix.astype(np.float32),
        chunks=chunks,
        compressor=compressor,
        dtype="f4"
    )

    # Store string arrays
    root.create_dataset("gene_names",   data=np.array(gene_names,   dtype="U30"))
    root.create_dataset("sample_names", data=np.array(sample_names, dtype="U30"))

    print(f"Zarr store written to: {output_path}")
    print(f"  Chunks: {chunks}")
    print(zarr.tree(root))
    return root


def parallel_zarr_write(zarr_path: str, data_chunks: list):
    """
    Write data in parallel chunks — key Zarr advantage over HDF5.
    (HDF5 does not support concurrent writes safely.)
    """
    from concurrent.futures import ThreadPoolExecutor

    root = zarr.open_group(zarr_path, mode="r+")
    counts = root["counts"]

    def write_chunk(args):
        chunk_idx, data_slice, row_start = args
        counts[row_start : row_start + data_slice.shape[0], :] = data_slice

    with ThreadPoolExecutor(max_workers=4) as executor:
        executor.map(write_chunk, data_chunks)
    print("Parallel write complete")
```

## AnnData: The Single-Cell Standard

For single-cell data, **AnnData** (used by scanpy) provides a structured container on top of HDF5 (`.h5ad` files):

```python
import anndata as ad

def create_anndata(count_matrix, gene_names, cell_names, cell_metadata):
    """
    Create an AnnData object for single-cell analysis.
    
    .X         — count matrix (cells × genes)  [sparse by default]
    .obs       — cell metadata DataFrame
    .var       — gene metadata DataFrame
    .obsm      — dict of cell embeddings (PCA, UMAP, etc.)
    .obsp      — pairwise cell data (distances, neighbor graphs)
    """
    adata = ad.AnnData(
        X=count_matrix.T.astype(np.float32),   # AnnData is cells × genes
        obs=cell_metadata,
        var=pd.DataFrame(index=gene_names)
    )
    adata.var_names = gene_names
    adata.obs_names = cell_names

    # Save to HDF5-backed .h5ad file
    adata.write_h5ad("/tmp/single_cell.h5ad", compression="gzip")
    print(f"AnnData: {adata.n_obs} cells × {adata.n_vars} genes")
    return adata
```

## Format Selection Guide

| Scenario | Format | Reason |
|---|---|---|
| RNA-seq, <100 samples | CSV/TSV | Human-readable, simple |
| RNA-seq, >100 samples | HDF5 | Compression, fast I/O |
| Single-cell (cells × genes) | AnnData (.h5ad) | Standard tool support |
| Cloud-native parallel I/O | Zarr | Chunk = separate object |
| Trajectory/MD data (time × atoms × 3) | HDF5 or NetCDF4 | Chunked, compressed |
| Genome-scale matrices | Zarr | Parallel write/read |

## Why This Matters

Format choice directly determines whether large-scale analyses are feasible. A 30,000-gene × 500,000-cell matrix stored as CSV would require ~100 GB uncompressed and minutes to read; the same data as chunked HDF5 compresses to ~5 GB and enables reading any 1,000-cell subset in under a second. Zarr's architecture — each chunk as a separate file — enables writing from 100 parallel processes simultaneously and reading from cloud storage with distributed computing frameworks like Dask, turning analyses that would require a single large-memory machine into embarrassingly parallel tasks. Choosing the right format from the start of a project avoids painful data migration later; more importantly, it enables analyses that are simply not possible with naive file formats.
