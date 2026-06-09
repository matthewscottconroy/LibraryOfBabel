# Relational Databases and SQL

There is a famous passage in *The Hitchhiker's Guide to the Galaxy* where a computer is asked to compute "the Answer to the Ultimate Question of Life, the Universe, and Everything." The computer runs for millions of years and returns: 42. The problem, it turns out, is that nobody knew the question. This is essentially what happens when you dump all your bioinformatics results into separate TSV files without a coherent data model. The answers are there. But connecting them — which differentially expressed genes are also co-regulated? which variants are rare in the general population but common in affected individuals? which metabolic enzymes are expressed at high levels under your experimental condition? — requires asking structured questions of structured data. That is what SQL does.

Relational databases are the universal language for structured data storage and retrieval. While bioinformatics has specialized formats (FASTA, VCF, BAM), the metadata layer — sample annotations, experimental conditions, QC metrics, analysis results — lives naturally in relational databases. More importantly, the major biological databases (Ensembl, UCSC Genome Browser, PharmGKB, DrugBank) expose their data through SQL or SQL-like interfaces. Understanding SQL means you can query these databases at full depth rather than using limited web interfaces.

## Relational Model: Tables, Keys, and Normalization

A **relational database** organizes data in **tables** (relations). Each table has:
- **Columns** (attributes): named, typed (INTEGER, FLOAT, VARCHAR, TEXT, DATE)
- **Rows** (tuples): individual records
- **Primary key**: one or more columns that uniquely identify each row
- **Foreign key**: a column that references the primary key of another table — enforces referential integrity

**Normalization** reduces redundancy and update anomalies:
- **1NF**: Each column holds atomic values (no lists in a cell)
- **2NF**: No partial dependency on a composite key (every non-key attribute depends on the full key)
- **3NF**: No transitive dependency (non-key attributes depend only on the key)

A well-normalized biological database schema:

```sql
-- Genes table
CREATE TABLE genes (
    gene_id   INTEGER PRIMARY KEY,
    gene_name VARCHAR(50) NOT NULL,
    chrom     VARCHAR(10),
    start_pos INTEGER,
    end_pos   INTEGER,
    strand    CHAR(1) CHECK (strand IN ('+', '-', '*'))
);

-- Samples table
CREATE TABLE samples (
    sample_id   INTEGER PRIMARY KEY,
    sample_name VARCHAR(100),
    condition   VARCHAR(50),
    batch       INTEGER
);

-- Expression table (normalized: one row per gene per sample)
CREATE TABLE expression (
    gene_id   INTEGER REFERENCES genes(gene_id),
    sample_id INTEGER REFERENCES samples(sample_id),
    raw_count INTEGER,
    tpm       FLOAT,
    PRIMARY KEY (gene_id, sample_id)
);
```

## Core SQL: SELECT, WHERE, JOIN

```sql
-- Basic SELECT
SELECT gene_name, chrom, start_pos, end_pos
FROM genes
WHERE chrom = 'chr17' AND end_pos - start_pos > 10000;

-- Aggregation: count genes per chromosome
SELECT chrom, COUNT(*) AS gene_count, AVG(end_pos - start_pos) AS mean_length
FROM genes
GROUP BY chrom
HAVING COUNT(*) > 100
ORDER BY gene_count DESC;

-- INNER JOIN: expression values with gene names
SELECT g.gene_name, s.sample_name, s.condition, e.tpm
FROM expression e
JOIN genes   g ON e.gene_id   = g.gene_id
JOIN samples s ON e.sample_id = s.sample_id
WHERE s.condition = 'treated'
  AND e.tpm > 10;

-- LEFT JOIN: all genes, with expression values where available
SELECT g.gene_name, e.tpm
FROM genes g
LEFT JOIN expression e ON g.gene_id = e.gene_id
  AND e.sample_id = 42
ORDER BY e.tpm DESC NULLS LAST;
```

**JOIN types:**
- `INNER JOIN` (= `JOIN`): returns rows where the join condition matches in both tables
- `LEFT JOIN`: all rows from the left table, matched rows from the right (NULL if no match)
- `RIGHT JOIN`: all rows from the right table (rarely used; restructure query to use LEFT JOIN instead)
- `FULL OUTER JOIN`: all rows from both tables

## Subqueries and CTEs

**Correlated subqueries** and **Common Table Expressions (CTEs)** make complex queries readable. A CTE is essentially a named intermediate result — you compute it once, give it a name, and then use it in subsequent parts of the query. This mirrors exactly how you would approach the problem analytically: first compute the mean expression per gene in treated samples; then rank those genes; then filter to the top 100:

```sql
-- CTE: most highly expressed genes in treated samples
WITH treated_means AS (
    SELECT e.gene_id, AVG(e.tpm) AS mean_tpm
    FROM expression e
    JOIN samples s ON e.sample_id = s.sample_id
    WHERE s.condition = 'treated'
    GROUP BY e.gene_id
),
ranked AS (
    SELECT g.gene_name, t.mean_tpm,
           RANK() OVER (ORDER BY t.mean_tpm DESC) AS rnk
    FROM treated_means t
    JOIN genes g ON t.gene_id = g.gene_id
)
SELECT gene_name, mean_tpm, rnk
FROM ranked
WHERE rnk <= 100;

-- Window function: running mean expression along chromosome
SELECT gene_name, chrom, start_pos, tpm,
       AVG(tpm) OVER (
           PARTITION BY chrom
           ORDER BY start_pos
           ROWS BETWEEN 4 PRECEDING AND 4 FOLLOWING
       ) AS sliding_mean_tpm
FROM expression e JOIN genes g ON e.gene_id = g.gene_id
WHERE e.sample_id = 1
ORDER BY chrom, start_pos;
```

## Indexing for Performance

Without indexes, every query scans the entire table ($O(n)$). **Indexes** create auxiliary B-tree (or hash) structures enabling $O(\log n)$ lookup:

```sql
-- Index on foreign key (always index FK columns)
CREATE INDEX idx_expr_gene    ON expression(gene_id);
CREATE INDEX idx_expr_sample  ON expression(sample_id);

-- Composite index for common query patterns
CREATE INDEX idx_expr_sample_tpm ON expression(sample_id, tpm DESC);

-- Explain query plan (PostgreSQL)
EXPLAIN ANALYZE
SELECT gene_name, tpm
FROM expression e JOIN genes g ON e.gene_id = g.gene_id
WHERE e.sample_id = 5 AND e.tpm > 50;
```

`EXPLAIN ANALYZE` shows whether the query uses an index (Index Scan) or scans the table (Seq Scan). For a table with $10^7$ rows, the difference is seconds vs. milliseconds.

## SQLite for Lightweight Bioinformatics Databases

For analysis databases that don't need multi-user concurrency, SQLite is ideal — it is a file-based database (no server needed), supports the full SQL standard, and Python has built-in support:

```python
import sqlite3
import pandas as pd

# Create and populate a SQLite database
conn = sqlite3.connect("analysis.db")

# Load a TSV directly into a table
results_df = pd.read_csv("deseq2_results.tsv", sep="\t")
results_df.to_sql("deseq2_results", conn, if_exists="replace", index=False)

# Query using SQL from Python
query = """
    SELECT gene_id, log2FoldChange, padj
    FROM deseq2_results
    WHERE padj < 0.05 AND ABS(log2FoldChange) > 1
    ORDER BY padj
    LIMIT 50
"""
top_de_genes = pd.read_sql_query(query, conn)
conn.close()
```

## Why This Matters for Computational Biology

The Ensembl database schema is one of the most sophisticated in bioinformatics — it stores all vertebrate genome annotations in a MySQL schema with dozens of normalized tables. Querying it directly via the MySQL interface (or BioMart, which generates SQL behind the scenes) is far more powerful than using the web interface. Phenotype-genotype databases (gnomAD, ClinVar, PharmGKB) are all relational at their core. When integrating RNA-seq results, variant calls, sample metadata, and pathway annotations for a complex project, a local SQLite database beats spreadsheets immediately — JOIN operations that would require complex vlookup chains in Excel complete in milliseconds. Window functions for computing sliding averages over genomic coordinates, aggregate statistics per condition per tissue type, and ranked lists of significant genes are all natural SQL queries that take one line instead of many lines of Pandas.
