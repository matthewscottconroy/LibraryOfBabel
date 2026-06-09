#!/usr/bin/env python3
"""
Collect all book markdown files in correct reading order and build a PDF.

Order rules:
  1. 00-master-toc.md first
  2. For each directory: _index.md before subdirs and files
  3. Subdirs and files sorted lexicographically
  4. Skip the standalone part-*.md root files (content covered by _index.md files)
"""

import os
import subprocess
import sys
from pathlib import Path

BOOK_DIR = Path(__file__).parent

def collect_files(directory: Path) -> list[Path]:
    """Recursively collect .md files with _index.md before directory children."""
    files = []
    entries = sorted(directory.iterdir())

    # _index.md first
    index = directory / "_index.md"
    if index.exists():
        files.append(index)

    for entry in entries:
        if entry.name == "_index.md":
            continue  # already added above
        if entry.is_dir():
            files.extend(collect_files(entry))
        elif entry.suffix == ".md":
            files.append(entry)

    return files


def build_file_list() -> list[Path]:
    ordered = []

    # 0. Preface (before TOC)
    preface = BOOK_DIR / "00-preface.md"
    if preface.exists():
        ordered.append(preface)

    # 1. Master TOC / Overview
    toc = BOOK_DIR / "00-master-toc.md"
    if toc.exists():
        ordered.append(toc)

    # 2. Each part directory in order (skip root part-*.md files)
    parts = sorted(
        p for p in BOOK_DIR.iterdir()
        if p.is_dir() and p.name.startswith("part-")
    )
    for part in parts:
        ordered.extend(collect_files(part))

    return ordered


def main():
    files = build_file_list()
    print(f"Collected {len(files)} markdown files.", file=sys.stderr)

    output_pdf = BOOK_DIR / "abstract-algebra.pdf"
    metadata   = BOOK_DIR / "book-metadata.yaml"

    cmd = [
        "pandoc",
        "--pdf-engine=xelatex",
        f"--metadata-file={metadata}",
        "--from=markdown+tex_math_dollars+tex_math_single_backslash+raw_tex",
        "--variable=papersize:letter",
        "--wrap=preserve",
        "-o", str(output_pdf),
    ] + [str(f) for f in files]

    print(f"Running pandoc on {len(files)} files → {output_pdf.name}", file=sys.stderr)
    print("This may take several minutes for a book this size.", file=sys.stderr)

    result = subprocess.run(cmd, cwd=BOOK_DIR, capture_output=False)
    if result.returncode == 0:
        size_mb = output_pdf.stat().st_size / 1_048_576
        print(f"\nDone. {output_pdf} ({size_mb:.1f} MB)", file=sys.stderr)
    else:
        print(f"\nPandoc exited with code {result.returncode}.", file=sys.stderr)
        sys.exit(result.returncode)


if __name__ == "__main__":
    main()
