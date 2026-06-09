#!/usr/bin/env python3
"""
generate_textbook_pdf.py — Build "Basal Cognition: Intelligence Before the Brain" as a PDF.

Assembles all Markdown chapters in correct order, adjusts heading levels for the
book hierarchy (part → chapter → section → subsection), strips redundant label
prefixes that would double-number with LaTeX, and converts via pandoc + XeLaTeX.

Requirements:
    pandoc >= 2.0           https://pandoc.org/installing.html
    XeLaTeX or pdfLaTeX     from TeX Live, MacTeX, or MiKTeX

    Ubuntu/Debian:
        sudo apt install pandoc texlive-xetex texlive-latex-extra \\
                         texlive-fonts-recommended texlive-fonts-extra

    macOS (Homebrew):
        brew install pandoc mactex

    Windows:
        https://pandoc.org/installing.html  +  https://miktex.org

Usage:
    python3 generate_textbook_pdf.py                     # full book
    python3 generate_textbook_pdf.py -o my_book.pdf      # custom output path
    python3 generate_textbook_pdf.py --draft             # first 2 units only (fast test)
    python3 generate_textbook_pdf.py --markdown-only     # dump combined .md to stdout
"""

import argparse
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path


# ── Paths ──────────────────────────────────────────────────────────────────────

BOOK_ROOT   = Path(__file__).resolve().parent
DEFAULT_OUT = BOOK_ROOT / "basal-cognition-textbook.pdf"

FRONT_MATTER = ["preface.md", "introduction.md"]
BACK_MATTER  = ["epilogue.md"]

APPENDIX_FILES = [
    "appendices/appendix-a-key-researchers.md",
    "appendices/appendix-b-timeline.md",
    "appendices/appendix-c-glossary.md",
    "appendices/appendix-d-further-reading.md",
    "appendices/appendix-e-research-groups.md",
]

# Chapter-end support files, in the order they should appear
SUPPORT_ORDER = [
    "exercises.md",
    "further-reading.md",
    "key-concepts.md",
    "key-researchers.md",
]


# ── Pandoc YAML metadata ────────────────────────────────────────────────────────
#
# Pandoc 3.x treats header-includes content as markdown (and escapes LaTeX).
# Complex LaTeX preamble code must go in a separate -H file instead.
# YAML here only carries simple metadata + documentclass settings.

YAML_HEADER = """\
---
title: "Basal Cognition: Intelligence Before the Brain"
subtitle: "From Molecular Sensing to the Origins of Mind"
documentclass: book
classoption:
  - 11pt
  - openright
  - twoside
geometry: "inner=1.5in, outer=1.1in, top=1.1in, bottom=1.1in, includefoot"
linestretch: 1.2
toc: true
toc-depth: 2
number-sections: true
colorlinks: true
linkcolor: "black"
citecolor: "black"
urlcolor: "NavyBlue"
toccolor: "black"
---
"""

# ── LaTeX preamble (passed via -H, not header-includes) ────────────────────────
#
# Written to a temp file and passed to pandoc with --include-in-header.
# This avoids pandoc 3.x's markdown-escaping of header-includes content.

LATEX_PREAMBLE = r"""
\usepackage{fancyhdr}
\usepackage{xcolor}
\usepackage{titlesec}
\usepackage{microtype}
\usepackage{emptypage}
\usepackage{booktabs}

\definecolor{partcolor}{RGB}{25,60,100}
\definecolor{chapcolor}{RGB}{47,79,79}

\titleformat{\chapter}[display]
  {\normalfont\huge\bfseries\color{chapcolor}}
  {\chaptertitlename\ \thechapter}{18pt}{\Huge}
\titlespacing{\chapter}{0pt}{-10pt}{30pt}

\titleformat{\part}[display]
  {\normalfont\Huge\bfseries\color{partcolor}\centering}
  {\partname\ \thepart}{20pt}{\Huge\centering}

\pagestyle{fancy}
\fancyhf{}
\fancyhead[LE]{\small\itshape\leftmark}
\fancyhead[RO]{\small\itshape\rightmark}
\fancyfoot[C]{\small\thepage}
\renewcommand{\headrulewidth}{0.3pt}
\setlength{\headheight}{14pt}
\fancypagestyle{plain}{%
  \fancyhf{}%
  \fancyfoot[C]{\small\thepage}%
  \renewcommand{\headrulewidth}{0pt}%
}
"""


# ── Markdown utilities ─────────────────────────────────────────────────────────

def read_file(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError:
        return ""


def strip_front_matter(text: str) -> str:
    """Remove a YAML front-matter block (--- … ---) if present."""
    if not text.startswith("---"):
        return text
    end = text.find("\n---", 3)
    if end == -1:
        return text
    return text[end + 4:].lstrip("\n")


def shift_headings(text: str, by: int) -> str:
    """
    Increase all ATX heading levels by `by`.

    shift_headings("# Title", 1)  →  "## Title"
    Headings are capped at level 6.
    """
    if by == 0:
        return text

    def _replace(m: re.Match) -> str:
        new_level = min(len(m.group(1)) + by, 6)
        return "#" * new_level + m.group(2)

    return re.sub(r"^(#{1,6})([ \t].*)$", _replace, text, flags=re.MULTILINE)


# Prefixes on the FIRST heading of a file that are redundant once LaTeX adds its
# own chapter/section labels.
_FIRST_HEADING_PREFIXES = re.compile(
    r"""
    ^(?:
        Chapter   \s+ \d+     (?:[\s:–\-]+)  |  # "Chapter 4:"  "Chapter 4 Exercises:"
        Section   \s+ [\d.]+  (?:[:\s]+)     |  # "Section 4.1:" "Section 4.1 "
        Unit      \s+ \S+     (?:[:\s]+)     |  # "Unit XI:"
        Appendix  \s+ [A-Z]   (?:[:\s]+)        # "Appendix A:"
    )
    """,
    re.VERBOSE,
)


def clean_first_heading(text: str) -> str:
    """
    Strip a redundant label prefix (e.g. "Chapter 4:", "Section 2.1:") from
    the first ATX heading in `text`.  Leaves all other headings untouched.
    """
    def _replace(m: re.Match) -> str:
        hashes = m.group(1)
        title  = _FIRST_HEADING_PREFIXES.sub("", m.group(2).lstrip()).strip()
        return f"{hashes} {title}"

    return re.sub(
        r"^(#{1,6})\s+(.+)$",
        _replace,
        text,
        count=1,
        flags=re.MULTILINE,
    )


def clean_subsection_numbers(text: str) -> str:
    """
    Strip bare numeric section prefixes (e.g. "1.1.1", "4.2") from ALL headings.

    Source files use manual numbering like "## 1.1.1 Folk Psychology" so that
    they read clearly as standalone files.  In the assembled PDF pandoc + LaTeX
    adds its own numbers, making "1.1.1" appear twice.  This function removes
    the manual prefix from every heading in the text.

    A prefix is only stripped when it matches N, N.N, N.N.N etc. immediately
    after the heading hashes — not when digits appear mid-title.
    """
    return re.sub(
        r"^(#{1,6})\s+\d+(?:\.\d+)+\s+(.*)$",
        r"\1 \2",
        text,
        flags=re.MULTILINE,
    )


def make_unnumbered(text: str) -> str:
    """
    Append {.unnumbered .unlisted} to the first heading so pandoc generates
    \\chapter*{} / \\section*{} — used for preface, introduction, epilogue.
    """
    def _replace(m: re.Match) -> str:
        line = m.group(0).rstrip()
        if "{.unnumbered" not in line:
            line += " {.unnumbered .unlisted}"
        return line

    return re.sub(r"^#{1,6}[ \t].+$", _replace, text, count=1, flags=re.MULTILINE)


def raw_latex(code: str) -> str:
    """Wrap a LaTeX snippet in a pandoc raw-block."""
    return f"\n\n```{{=latex}}\n{code}\n```\n\n"


def escape_latex(text: str) -> str:
    """Escape characters that are special in LaTeX argument position."""
    return (
        text
        .replace("\\", r"\textbackslash{}")
        .replace("&",  r"\&")
        .replace("%",  r"\%")
        .replace("$",  r"\$")
        .replace("#",  r"\#")
        .replace("_",  r"\_")
        .replace("{",  r"\{")
        .replace("}",  r"\}")
        .replace("~",  r"\textasciitilde{}")
        .replace("^",  r"\textasciicircum{}")
    )


# ── Directory traversal ────────────────────────────────────────────────────────

def _leading_int(name: str) -> int:
    m = re.search(r"\d+", name)
    return int(m.group()) if m else 999


def unit_dirs() -> list[Path]:
    return sorted(
        (d for d in BOOK_ROOT.iterdir()
         if d.is_dir() and re.match(r"unit-\d+", d.name)),
        key=lambda d: _leading_int(d.name),
    )


def chapter_dirs(unit: Path) -> list[Path]:
    return sorted(
        (d for d in unit.iterdir()
         if d.is_dir() and re.match(r"chapter-\d+", d.name)),
        key=lambda d: _leading_int(d.name),
    )


def section_files(chapter: Path) -> list[Path]:
    """
    Returns section content files in numerical order, followed by the
    four chapter-end support files (exercises, further-reading, etc.).
    """
    sections = sorted(
        (f for f in chapter.iterdir()
         if f.suffix == ".md"
         and f.name != "README.md"
         and re.match(r"section-\d+", f.name)),
        key=lambda f: _leading_int(f.name),
    )
    support = [chapter / s for s in SUPPORT_ORDER if (chapter / s).exists()]
    return sections + support


# ── Book assembly ──────────────────────────────────────────────────────────────

def assemble(draft: bool = False) -> str:
    """
    Return the full book as a single Markdown string ready for pandoc.

    Heading-level mapping (with --top-level-division=chapter):
        #   →  \\chapter{}   (chapter README titles)
        ##  →  \\section{}   (section file titles, after +1 shift)
        ### →  \\subsection{} (subsections within sections)

    Parts (units) are injected as raw LaTeX \\part{} commands so they do not
    interfere with pandoc's heading-level processing.
    """
    parts: list[str] = [YAML_HEADER]

    # ── Front matter ──────────────────────────────────────────────────────────
    for fname in FRONT_MATTER:
        path = BOOK_ROOT / fname
        if path.exists():
            body = strip_front_matter(read_file(path))
            body = clean_first_heading(body)
            body = make_unnumbered(body)
            parts.append(body + "\n\n")

    # ── Body: units → chapters → sections ─────────────────────────────────────
    units = unit_dirs()
    if draft:
        units = units[:2]

    for unit in units:
        unit_readme_raw = strip_front_matter(read_file(unit / "README.md"))

        # Extract the unit's descriptive title (strip "Unit XI:" prefix)
        m = re.search(r"^#{1,3}\s+(?:Unit\s+\S+:\s+)?(.*?)$",
                      unit_readme_raw, re.MULTILINE)
        part_title = escape_latex(m.group(1).strip()) if m else escape_latex(unit.name)

        # Inject a LaTeX \part{} — creates a full-page part divider
        parts.append(raw_latex(r"\cleardoublepage"))
        parts.append(raw_latex(rf"\part{{{part_title}}}"))

        # Unit intro body (the text below the first heading serves as a
        # "part preamble" that appears before the first chapter of the unit)
        after_title = re.sub(r"^#{1,3}[^\n]*\n", "", unit_readme_raw, count=1)
        if after_title.strip():
            parts.append(after_title.strip() + "\n\n")

        for chap in chapter_dirs(unit):
            # Chapter README: # heading → \chapter{} (no level shift needed)
            chap_body = strip_front_matter(read_file(chap / "README.md"))
            chap_body = clean_first_heading(chap_body)   # strip "Chapter N:"
            parts.append(chap_body + "\n\n")

            # Section files: shift # → ## so they become \section{}
            for sec in section_files(chap):
                body = strip_front_matter(read_file(sec))
                body = clean_first_heading(body)          # strip "Section N.N:"
                body = clean_subsection_numbers(body)     # strip "1.1.1" from all headings
                body = shift_headings(body, 1)
                parts.append(body + "\n\n")

    # ── Back matter ───────────────────────────────────────────────────────────
    for fname in BACK_MATTER:
        path = BOOK_ROOT / fname
        if path.exists():
            body = strip_front_matter(read_file(path))
            body = clean_first_heading(body)
            body = make_unnumbered(body)
            parts.append(raw_latex(r"\cleardoublepage") + body + "\n\n")

    # ── Appendices ────────────────────────────────────────────────────────────
    parts.append(raw_latex(
        r"\appendix" + "\n"
        r"\renewcommand{\chaptername}{Appendix}"
    ))
    for rel in APPENDIX_FILES:
        path = BOOK_ROOT / rel
        if path.exists():
            body = strip_front_matter(read_file(path))
            body = clean_first_heading(body)   # strip "Appendix A:"
            parts.append(body + "\n\n")

    return "".join(parts)


# ── Dependencies ───────────────────────────────────────────────────────────────

def find_latex_engine() -> str:
    for engine in ("xelatex", "lualatex", "pdflatex"):
        try:
            subprocess.run([engine, "--version"], capture_output=True, check=True)
            return engine
        except (FileNotFoundError, subprocess.CalledProcessError):
            continue
    return ""


def check_pandoc() -> str:
    try:
        r = subprocess.run(["pandoc", "--version"], capture_output=True, text=True, check=True)
        return r.stdout.splitlines()[0]
    except (FileNotFoundError, subprocess.CalledProcessError):
        return ""


# ── PDF generation ─────────────────────────────────────────────────────────────

def build_pdf(output: Path, draft: bool) -> None:
    pandoc_ver = check_pandoc()
    if not pandoc_ver:
        sys.exit(
            "pandoc not found.\n"
            "Install: https://pandoc.org/installing.html\n"
            "  Ubuntu/Debian: sudo apt install pandoc"
        )

    engine = find_latex_engine()
    if not engine:
        sys.exit(
            "No LaTeX engine found (tried xelatex, lualatex, pdflatex).\n"
            "  Ubuntu/Debian: sudo apt install texlive-xetex texlive-latex-extra\n"
            "                 texlive-fonts-recommended\n"
            "  macOS:         brew install mactex"
        )

    print(f"  pandoc : {pandoc_ver}")
    print(f"  engine : {engine}")
    print(f"  units  : {len(unit_dirs())} total"
          + (" (draft: first 2 only)" if draft else ""))
    print()

    print("Assembling book content...")
    combined = assemble(draft=draft)
    print(f"  combined markdown: {len(combined):,} characters\n")

    # Write combined markdown and LaTeX preamble to temp files
    tmp_md  = tempfile.NamedTemporaryFile(
        mode="w", suffix=".md",  encoding="utf-8",
        delete=False, dir=BOOK_ROOT, prefix="_combined_"
    )
    tmp_tex = tempfile.NamedTemporaryFile(
        mode="w", suffix=".tex", encoding="utf-8",
        delete=False, dir=BOOK_ROOT, prefix="_preamble_"
    )
    try:
        tmp_md.write(combined);  tmp_md.close()
        tmp_tex.write(LATEX_PREAMBLE); tmp_tex.close()
        tmp_path  = Path(tmp_md.name)
        preamble_path = Path(tmp_tex.name)

        print(f"Running pandoc → {output.name} ...")
        cmd = [
            "pandoc", str(tmp_path),
            "--from",  "markdown+raw_tex+smart",
            "--to",    "pdf",
            "--pdf-engine", engine,
            "--include-in-header", str(preamble_path),
            "--output", str(output),
            "--top-level-division=chapter",
            "--highlight-style=tango",
            "-V", "lang=en-US",
        ]

        result = subprocess.run(cmd, capture_output=True, text=True, cwd=BOOK_ROOT)

        if result.returncode != 0:
            # Show the last portion of stderr — that's where the real error is
            log = (result.stdout + "\n" + result.stderr).strip()
            print("\n── LaTeX/pandoc output (last 5000 chars) ──────────────────")
            print(log[-5000:])
            print("────────────────────────────────────────────────────────────")
            sys.exit(f"\nBuild failed (exit code {result.returncode}).")

    finally:
        tmp_path.unlink(missing_ok=True)
        preamble_path.unlink(missing_ok=True)

    size_mb = output.stat().st_size / 1_048_576
    print(f"\n✓  PDF written to: {output}")
    print(f"   Size: {size_mb:.1f} MB")


# ── CLI ────────────────────────────────────────────────────────────────────────

def main() -> None:
    parser = argparse.ArgumentParser(
        description="Build the Basal Cognition textbook as a PDF.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__,
    )
    parser.add_argument(
        "--output", "-o", metavar="FILE", type=Path, default=DEFAULT_OUT,
        help="Output PDF path (default: basal-cognition-textbook.pdf in book root)",
    )
    parser.add_argument(
        "--draft", action="store_true",
        help="Include only the first 2 units — fast test build",
    )
    parser.add_argument(
        "--markdown-only", action="store_true",
        help="Print the assembled Markdown to stdout instead of building a PDF",
    )
    args = parser.parse_args()

    if args.markdown_only:
        print(assemble(draft=args.draft))
        return

    print("Building Basal Cognition textbook PDF")
    print("=" * 40)
    build_pdf(args.output, draft=args.draft)


if __name__ == "__main__":
    main()
