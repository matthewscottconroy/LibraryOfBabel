# Zenodo and Figshare: Research Data Archiving

When a paper is published, its supplementary materials typically live on the journal's servers. Those servers are run by commercial publishers. Publishers get acquired, go out of business, change their hosting infrastructure, or simply retire old content after years pass. Even when the data technically survives, it is often behind a paywall, not indexed by data search engines, not assigned a stable identifier, and not discoverable in a way that enables programmatic access. Supplementary files in journal PDFs are not archiving. They are temporary storage managed by someone whose primary business is not data preservation.

**Zenodo** and **figshare** are open-access research data repositories that assign **DOIs (Digital Object Identifiers)** — permanent, citable identifiers — to uploaded datasets, software, and preprints. A DOI is a persistent URL: when the host changes, the DOI redirects, ensuring that citations remain valid indefinitely. Depositing data and code in a DOI-indexed repository is now required or strongly recommended by most journals, funding agencies (NIH, NSF, Wellcome Trust), and is essential for reproducibility. Without archival of raw data and analysis code, a computational biology result is not reproducible — it is simply a claim.

## Why Archive Data?

Supplementary files in journal PDFs are not a substitute for proper data archiving:
- Supplementary files are often behind paywalls
- They may not include all intermediate data or code
- They are stored on publisher servers with no long-term preservation guarantee
- They are not indexed in data repositories and are not easily discoverable

Zenodo and figshare solve these problems:
- **Free** for public datasets (Zenodo: 50 GB/record; figshare: 20 GB/file)
- All deposits receive a **DOI** immediately
- Files are preserved indefinitely
- Indexed by Google Dataset Search, DataCite, and ORCID
- Support versioning (multiple versions, each with their own DOI)

## Zenodo

**Zenodo**, operated by CERN, is the gold standard for open science archiving:

```python
import requests
import json
import os

def deposit_to_zenodo(
    files: list[str],
    title: str,
    description: str,
    creators: list[dict],
    access_token: str,
    sandbox: bool = True   # use sandbox for testing!
):
    """
    Programmatically deposit files to Zenodo.
    
    Parameters
    ----------
    files : list of file paths to upload
    creators : list of dicts with 'name' and optionally 'orcid', 'affiliation'
    access_token : Zenodo personal access token
    sandbox : if True, use sandbox.zenodo.org (testing)
    
    Returns
    -------
    dict : deposit metadata including DOI
    """
    base_url = "https://sandbox.zenodo.org" if sandbox else "https://zenodo.org"
    headers = {"Content-Type": "application/json"}
    params = {"access_token": access_token}

    # Step 1: Create empty deposition
    response = requests.post(
        f"{base_url}/api/deposit/depositions",
        params=params,
        headers=headers,
        data=json.dumps({})
    )
    response.raise_for_status()
    deposition = response.json()
    deposition_id = deposition["id"]
    bucket_url = deposition["links"]["bucket"]

    print(f"Created deposition ID: {deposition_id}")
    print(f"Reserved DOI: {deposition['metadata'].get('prereserve_doi', {}).get('doi', 'pending')}")

    # Step 2: Upload files
    for file_path in files:
        file_name = os.path.basename(file_path)
        with open(file_path, "rb") as f:
            r = requests.put(
                f"{bucket_url}/{file_name}",
                params=params,
                data=f
            )
            r.raise_for_status()
        print(f"  Uploaded: {file_name} ({os.path.getsize(file_path) / 1e6:.1f} MB)")

    # Step 3: Add metadata
    metadata = {
        "title": title,
        "description": description,
        "upload_type": "dataset",
        "creators": creators,
        "access_right": "open",
        "license": "cc-by-4.0",
        "keywords": ["systems biology", "computational biology"],
        "related_identifiers": [],  # link to paper DOI if available
    }

    r = requests.put(
        f"{base_url}/api/deposit/depositions/{deposition_id}",
        params=params,
        headers=headers,
        data=json.dumps({"metadata": metadata})
    )
    r.raise_for_status()

    # Step 4: Publish (makes it public and finalizes the DOI)
    r = requests.post(
        f"{base_url}/api/deposit/depositions/{deposition_id}/actions/publish",
        params=params
    )
    r.raise_for_status()
    published = r.json()
    doi = published["doi"]

    print(f"\nPublished successfully!")
    print(f"  DOI: {doi}")
    print(f"  URL: https://doi.org/{doi}")
    return published


# Example usage (with sandbox for testing)
creators = [
    {
        "name": "Smith, Jane",
        "affiliation": "University of Example",
        "orcid": "0000-0000-0000-0000"
    }
]

# deposit_to_zenodo(
#     files=["data/raw/rnaseq_counts.h5", "code/analysis_pipeline.zip"],
#     title="RNA-seq data from synthetic biology circuit characterization",
#     description="Count matrices and analysis code for EGFR circuit study.",
#     creators=creators,
#     access_token=os.environ["ZENODO_TOKEN"],
#     sandbox=True  # Remove for real publication
# )
```

## GitHub-Zenodo Integration

The most common workflow for software is the automatic GitHub release integration:

```bash
# 1. Go to https://zenodo.org/account/settings/github/
# 2. Toggle on the repository you want to archive
# 3. Create a GitHub release with a version tag:

git tag -a v1.0.0 -m "Release v1.0.0 for publication"
git push origin v1.0.0

# GitHub release → Zenodo automatically creates a deposit + DOI
# Result: your software has a DOI like 10.5281/zenodo.12345678
```

```json
// .zenodo.json — customize Zenodo metadata for GitHub integration
{
    "title": "bioanalysis: Computational tools for systems biology",
    "description": "A Python package for RNA-seq, network analysis, and ODE simulation in synthetic biology research.",
    "upload_type": "software",
    "license": "MIT",
    "creators": [
        {
            "name": "Smith, Jane",
            "affiliation": "Department of Systems Biology, University of Example",
            "orcid": "0000-0000-0000-0000"
        }
    ],
    "keywords": ["systems biology", "RNA-seq", "synthetic biology"],
    "related_identifiers": [
        {
            "relation": "isSupplementTo",
            "identifier": "10.1016/j.cell.2024.00000",
            "resource_type": "publication-article"
        }
    ]
}
```

## figshare

**figshare** is widely used for figures, datasets, and preprints, with institutional integrations at many universities:

```python
import requests

def upload_to_figshare(
    file_path: str,
    title: str,
    description: str,
    access_token: str
) -> dict:
    """
    Upload a dataset to figshare.
    
    Returns
    -------
    dict with 'doi' and 'id' keys after publishing
    """
    headers = {"Authorization": f"token {access_token}"}
    base_url = "https://api.figshare.com/v2"

    # Create article
    article_data = {
        "title": title,
        "description": description,
        "type": "dataset",
        "license": 1,   # CC BY 4.0
        "tags": ["systems biology", "computational biology"]
    }
    r = requests.post(f"{base_url}/account/articles",
                      headers=headers, json=article_data)
    r.raise_for_status()
    article_id = r.json()["id"]
    print(f"Created figshare article: {article_id}")

    # Get upload location
    file_name = os.path.basename(file_path)
    r = requests.post(
        f"{base_url}/account/articles/{article_id}/files",
        headers=headers,
        json={"name": file_name}
    )
    r.raise_for_status()
    file_info = r.json()

    # Upload file
    upload_url = file_info["upload_url"]
    with open(file_path, "rb") as f:
        requests.put(upload_url, data=f)
    print(f"  Uploaded: {file_name}")

    # Complete upload
    requests.post(f"{base_url}/account/articles/{article_id}/files/{file_info['id']}",
                  headers=headers)

    # Publish
    r = requests.post(f"{base_url}/account/articles/{article_id}/publish",
                      headers=headers)
    r.raise_for_status()
    print(f"Published: https://doi.org/{r.json()['doi']}")
    return r.json()
```

## Citing Data and Software in Papers

```python
# Citation snippet for README.md or paper methods section:

citation_template = """
If you use this software, please cite:

Smith J, Jones B (2024). bioanalysis v1.0.0 [software].
Zenodo. https://doi.org/10.5281/zenodo.12345678

The RNA-seq dataset is available at:
Smith J, Jones B (2024). Synthetic EGFR circuit RNA-seq data [dataset].
Zenodo. https://doi.org/10.5281/zenodo.87654321
"""
```

## Data Management Plan Template

Most funding agencies require a data management plan (DMP). Key elements:

| Element | Standard Practice |
|---|---|
| Data types | Count matrices (HDF5), processed data (CSV), code (GitHub/Zenodo) |
| Storage during project | Institutional HPC scratch + daily backups |
| Long-term archiving | Zenodo for data and software; GEO/SRA for sequencing data |
| Access policy | Open access under CC BY 4.0 |
| Metadata standards | Dublin Core + domain-specific (MIAME for expression data) |
| Identifiers | DOI per dataset version; ORCID for researchers |

## Why This Matters

The moment a paper is accepted for publication is the wrong time to think about data deposition — it should happen during the analysis, with DOIs being reserved (Zenodo allows pre-reserving DOIs before upload) and included in the manuscript draft. Data that is not formally archived is effectively private: it may exist on a lab server that goes offline, in a lab member's personal storage when they leave, or in a format only the original researcher understands. The combination of Zenodo (for software and datasets) and dedicated databases (GEO/SRA for genomics, PDB for structures, BioModels for ODE models) ensures that every artifact needed to reproduce a computational analysis is permanently accessible, citable, and discoverable — transforming individual research contributions into community infrastructure.
