#!/usr/bin/env python3
"""
build_pdf.py — Build "Dynamical Systems: From Flows to Chaos" as a PDF.

Requirements:
    pandoc >= 2.0   https://pandoc.org/installing.html
    XeLaTeX         from TeX Live / MacTeX / MiKTeX

Usage:
    python3 build_pdf.py                      # full book
    python3 build_pdf.py -o my_book.pdf       # custom output path
    python3 build_pdf.py --draft              # first 2 parts only
    python3 build_pdf.py --markdown-only      # dump combined .md to stdout
"""

import argparse, os, re, subprocess, sys, tempfile
from pathlib import Path

BOOK_ROOT   = Path(__file__).resolve().parent
DEFAULT_OUT = BOOK_ROOT / "dynamical-systems.pdf"

YAML_HEADER = """\
---
title: "Dynamical Systems: From Flows to Chaos"
subtitle: "Nonlinear Dynamics, Bifurcations, and Complex Systems"
documentclass: book
classoption: [11pt, openright, twoside]
geometry: "inner=1.5in, outer=1.1in, top=1.1in, bottom=1.1in, includefoot"
linestretch: 1.2
toc: true
toc-depth: 2
number-sections: true
colorlinks: true
linkcolor: "black"
urlcolor: "NavyBlue"
toccolor: "black"
---
"""

LATEX_PREAMBLE = r"""
\usepackage{fancyhdr}
\usepackage{xcolor}
\usepackage{titlesec}
\usepackage{microtype}
\usepackage{emptypage}
\usepackage{amsmath}

\definecolor{chapcolor}{RGB}{60,30,90}

\titleformat{\chapter}[display]
  {\normalfont\huge\bfseries\color{chapcolor}}
  {\chaptertitlename\ \thechapter}{18pt}{\Huge}
\titlespacing{\chapter}{0pt}{-10pt}{30pt}

\pagestyle{fancy}
\fancyhf{}
\fancyhead[LE]{\small\itshape\leftmark}
\fancyhead[RO]{\small\itshape\rightmark}
\fancyfoot[C]{\small\thepage}
\renewcommand{\headrulewidth}{0.3pt}
\setlength{\headheight}{14pt}
\fancypagestyle{plain}{\fancyhf{}\fancyfoot[C]{\small\thepage}\renewcommand{\headrulewidth}{0pt}}
"""

def read_file(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except OSError:
        return ""

def strip_front_matter(text: str) -> str:
    if not text.startswith("---"):
        return text
    end = text.find("\n---", 3)
    return text[end + 4:].lstrip("\n") if end != -1 else text

def shift_headings(text: str, by: int) -> str:
    if by == 0:
        return text
    def _r(m):
        return "#" * min(len(m.group(1)) + by, 6) + m.group(2)
    return re.sub(r"^(#{1,6})([ \t].*)$", _r, text, flags=re.MULTILINE)

def clean_first_heading(text: str) -> str:
    prefix_re = re.compile(r"^(?:Part\s+\S+[:\s]+|Chapter\s+\d+[:\s]+|Section\s+[\d.]+[:\s]+)", re.VERBOSE)
    def _r(m):
        return m.group(1) + " " + prefix_re.sub("", m.group(2).lstrip()).strip()
    return re.sub(r"^(#{1,6})\s+(.+)$", _r, text, count=1, flags=re.MULTILINE)

def raw_latex(code: str) -> str:
    return f"\n\n```{{=latex}}\n{code}\n```\n\n"

def escape_latex(text: str) -> str:
    return text.replace("\\","\\textbackslash{}").replace("&","\\&").replace("%","\\%").replace("$","\\$").replace("#","\\#").replace("_","\\_").replace("{","\\{").replace("}","\\}").replace("~","\\textasciitilde{}").replace("^","\\textasciicircum{}")

def _leading_int(name: str) -> int:
    m = re.search(r"\d+", name)
    return int(m.group()) if m else 999

def part_dirs() -> list:
    return sorted(
        (d for d in BOOK_ROOT.iterdir() if d.is_dir() and re.match(r"part-", d.name)),
        key=lambda d: _leading_int(d.name),
    )

def make_unnumbered(text: str) -> str:
    def _r(m):
        line = m.group(0).rstrip()
        return line if "{.unnumbered" in line else line + " {.unnumbered .unlisted}"
    return re.sub(r"^#{1,6}[ \t].+$", _r, text, count=1, flags=re.MULTILINE)

def assemble(draft: bool = False) -> str:
    parts_list = [YAML_HEADER]

    readme = BOOK_ROOT / "README.md"
    if readme.exists():
        body = strip_front_matter(read_file(readme))
        body = make_unnumbered(body)
        parts_list.append(body + "\n\n")

    pdirs = part_dirs()
    if draft:
        pdirs = pdirs[:2]

    for pdir in pdirs:
        readme_raw = strip_front_matter(read_file(pdir / "README.md"))
        m = re.search(r"^#{1,3}\s+(?:Part\s+\S+:\s+)?(.*?)$", readme_raw, re.MULTILINE)
        part_title = escape_latex(m.group(1).strip()) if m else escape_latex(pdir.name)

        parts_list.append(raw_latex(r"\cleardoublepage"))
        parts_list.append(raw_latex(rf"\part{{{part_title}}}"))

        # Section .md files within the part dir
        sections = sorted(
            (f for f in pdir.iterdir() if f.suffix == ".md" and f.name != "README.md"),
            key=lambda f: _leading_int(f.name),
        )
        after_title = re.sub(r"^#{1,3}[^\n]*\n", "", readme_raw, count=1)
        if after_title.strip():
            parts_list.append(after_title.strip() + "\n\n")

        for sec in sections:
            body = strip_front_matter(read_file(sec))
            body = clean_first_heading(body)
            parts_list.append(body + "\n\n")

    return "".join(parts_list)

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

def build_pdf(output: Path, draft: bool) -> None:
    pandoc_ver = check_pandoc()
    if not pandoc_ver:
        sys.exit("pandoc not found. Install: https://pandoc.org/installing.html")
    engine = find_latex_engine()
    if not engine:
        sys.exit("No LaTeX engine found. Install texlive-xetex.")

    print(f"  pandoc : {pandoc_ver}")
    print(f"  engine : {engine}")
    print()

    combined = assemble(draft=draft)
    print(f"  markdown: {len(combined):,} chars")

    tmp_md  = tempfile.NamedTemporaryFile(mode="w", suffix=".md", encoding="utf-8", delete=False, dir=BOOK_ROOT, prefix="_combined_")
    tmp_tex = tempfile.NamedTemporaryFile(mode="w", suffix=".tex", encoding="utf-8", delete=False, dir=BOOK_ROOT, prefix="_preamble_")
    try:
        tmp_md.write(combined); tmp_md.close()
        tmp_tex.write(LATEX_PREAMBLE); tmp_tex.close()
        tmp_path = Path(tmp_md.name); preamble_path = Path(tmp_tex.name)

        print(f"Running pandoc → {output.name} ...")
        cmd = ["pandoc", str(tmp_path), "--from", "markdown+raw_tex+smart", "--to", "pdf",
               "--pdf-engine", engine, "--include-in-header", str(preamble_path),
               "--output", str(output), "--top-level-division=chapter", "--highlight-style=tango", "-V", "lang=en-US"]
        result = subprocess.run(cmd, capture_output=True, text=True, cwd=BOOK_ROOT)
        if result.returncode != 0:
            log = (result.stdout + "\n" + result.stderr).strip()
            print(log[-5000:])
            sys.exit(f"Build failed (exit code {result.returncode}).")
    finally:
        Path(tmp_md.name).unlink(missing_ok=True)
        Path(tmp_tex.name).unlink(missing_ok=True)

    size_mb = output.stat().st_size / 1_048_576
    print(f"\n✓  PDF written to: {output}")
    print(f"   Size: {size_mb:.1f} MB")

def main() -> None:
    parser = argparse.ArgumentParser(description="Build the Dynamical Systems textbook PDF.")
    parser.add_argument("--output", "-o", metavar="FILE", type=Path, default=DEFAULT_OUT)
    parser.add_argument("--draft", action="store_true")
    parser.add_argument("--markdown-only", action="store_true")
    args = parser.parse_args()

    if args.markdown_only:
        print(assemble(draft=args.draft))
        return

    print("Building Dynamical Systems textbook PDF")
    print("=" * 40)
    build_pdf(args.output, draft=args.draft)

if __name__ == "__main__":
    main()
