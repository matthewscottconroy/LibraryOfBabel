#!/usr/bin/env python3
r"""
build_book.py — one config-driven builder for every textbook in this repo.

Each book directory carries a ``book.toml`` manifest describing where its
content lives and how to order it.  This single script replaces the ~14 drifted
per-book ``build_pdf.py`` / ``generate_*.py`` scripts that used to live beside
each book.

Manifest schema (``<BookDir>/book.toml``)
-----------------------------------------
    title        = "Book Title"           # required
    author       = "..."                  # optional
    subtitle     = "..."                  # optional
    intro_names  = ["overview.md", ...]   # optional; ADDED to the defaults below
    outro_names  = ["exercises.md", ...]  # optional; per-directory back matter,
                                          #   sorted last in the order given
    numbersections = true                 # optional; false for books that carry
                                          #   their own numbering in headings
    toc_depth    = 2                      # optional; passed to pandoc (1-6).
                                          #   With part division LaTeX reads it
                                          #   as chapter=0/section=1/...
    header_includes = ["..."]             # optional; raw LaTeX blocks for the
                                          #   PDF preamble (via header-includes)
    mainfont     = "Noto Serif"           # optional; xelatex body font.  The
                                          #   default Latin Modern silently
                                          #   drops Greek and box-drawing glyphs
    monofont     = "Noto Sans Mono"       # optional; code/diagram font
    monofontoptions = ["Scale=0.82"]      # optional; fontspec options
    lift_intros  = false                  # optional; see "Intro lifting" below
    front_matter_shift = 0                # optional; heading shift for front
                                          #   matter (1 = emit as chapters)
    front_matter = ["preface.md", ...]    # optional; ordered, globs allowed
    back_matter  = ["epilogue.md", "appendices/*.md"]  # optional; ordered, globs
    exclude      = ["README.md", "**/scratch.md"]      # optional; globs

    [[sources]]
    root       = "book"     # directory (relative to the book dir) to walk
    recursive  = true       # default true; false = only the root's own files
    part_level = 1           # optional: directory depth that becomes a LaTeX \part

`front_matter`, `back_matter`, `exclude`, `intro_names`, and `outro_names` are
top-level keys and MUST appear before the first ``[[sources]]`` table (a TOML
requirement).

Ordering
--------
Within any directory the children are ordered as:
    1. intro-like files (README.md, intro.md, _index.md, 00-*.md, ...) first,
       in the priority order of the intro-name list, then
    2. everything else (files and sub-directories interleaved) by a natural
       numeric sort that understands "chapter-2" < "chapter-10", "1.2" < "1.10",
       then
    3. outro-like files, in the priority order of the outro-name list.

Rule 3 exists because a chapter's back matter is named for what it is, not for
where it belongs: `exercises.md` and `further_reading.md` sort before `s01_*.md`
alphabetically, which silently puts the exercises ahead of the sections they
examine.  There is no default outro list — a book opts in via `outro_names`.
Directory names may appear in `outro_names` too, which is how an `appendices/`
directory is sorted after `unit_*/` despite alphabetical order.

Intro lifting (`lift_intros = true`)
------------------------------------
By default every file's headings are shifted by its directory depth, which
treats an intro file as a sibling of the content it introduces.  With
`lift_intros`, an intro-like file is treated as its *container's* heading:

  - the intro file of a part directory loses its leading H1 (the \\part divider
    already renders that title) and its remaining headings shift by depth, so
    its subsections sit below chapter level;
  - any other intro-like file shifts by depth − 1, so a chapter directory's
    `chapter_intro.md` H1 becomes the chapter heading and sibling section
    files sit one level below it.

Modes
-----
    --check          report content .md not captured by the manifest and any
                     manifest entry that matches no files; exit non-zero on
                     problems.  Zero-byte / whitespace-only files only warn.
    --markdown OUT   write the assembled markdown (no pandoc needed).
    --html OUT       render HTML via pandoc.
    --pdf OUT        render PDF via pandoc (xelatex if available, else pdflatex).
    --chapters N-M   restrict to parts / top-level sections N..M (fast testing).

Outputs default to ``<BookDir>/output/`` (which is gitignored).
"""

from __future__ import annotations

import argparse
import fnmatch
import os
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib
from pathlib import Path

# --------------------------------------------------------------------------- #
# Defaults
# --------------------------------------------------------------------------- #

# Intro-like file names (globs).  Files matching an EARLIER entry sort before a
# LATER one, so keep the canonical names first.  Books may add to this via
# `intro_names` in their manifest.
DEFAULT_INTRO_NAMES = [
    "README.md",
    "_index.md",
    "index.md",
    "intro.md",
    "introduction.md",
    "00_introduction.md",
    "chapter_intro.md",
    "unit_intro.md",
    "chapter-intro.md",
    "unit-intro.md",
    "00-*.md",
    "00_*.md",
]

# Directory names never walked for content, wherever they appear in the tree.
DEFAULT_SKIP_DIRS = {
    ".git", ".github", ".claude", ".idea", ".vscode",
    "node_modules", "target", "__pycache__", ".ipynb_checkpoints",
    "output", "questions", "quiz-app", "tools",
}

HEADING_RE = re.compile(r"^(#{1,6})([ \t])", re.MULTILINE)
_NAT_RE = re.compile(r"(\d+)")


# --------------------------------------------------------------------------- #
# Ordering primitives
# --------------------------------------------------------------------------- #

def natural_key(name: str):
    """Natural sort key: 'chapter-2' < 'chapter-10', '1.2' < '1.10'.

    Splits on digit runs; even positions are lowercased text, odd positions are
    integers.  Both operands always share that alternating shape, so tuple
    comparison never compares an int against a str.
    """
    parts = _NAT_RE.split(name)
    return [int(p) if i % 2 else p.lower() for i, p in enumerate(parts)]


def glob_priority(name: str, globs: list[str]):
    """Return the index of the first glob that matches `name`, else None."""
    for i, pat in enumerate(globs):
        if fnmatch.fnmatch(name, pat):
            return i
    return None


def intro_priority(name: str, intro_globs: list[str]):
    """Back-compat alias: position of `name` in the intro-name list, or None."""
    return glob_priority(name, intro_globs)


def child_sort_key(name: str, intro_globs: list[str], outro_globs: list[str] = ()):
    """Sort key: intro-like first, then natural, then outro-like.

    An intro match wins over an outro match, so a name appearing in both lists
    sorts to the front rather than silently to the back.
    """
    p = glob_priority(name, intro_globs)
    if p is not None:
        return (0, p, natural_key(name))
    p = glob_priority(name, list(outro_globs))
    if p is not None:
        return (2, p, natural_key(name))
    return (1, 0, natural_key(name))


def ordered_children(
    directory: Path, intro_globs: list[str], outro_globs: list[str] = ()
) -> list[Path]:
    """Immediate children of `directory`, intro-first, natural, outro-last."""
    try:
        entries = list(directory.iterdir())
    except OSError:
        return []
    return sorted(
        entries, key=lambda p: child_sort_key(p.name, intro_globs, outro_globs)
    )


# --------------------------------------------------------------------------- #
# Manifest
# --------------------------------------------------------------------------- #

class Manifest:
    def __init__(self, book_dir: Path, data: dict):
        self.book_dir = book_dir
        self.title = data.get("title", book_dir.name)
        self.author = data.get("author")
        self.subtitle = data.get("subtitle")
        self.sources = data.get("sources", [])
        self.front_matter = data.get("front_matter", [])
        self.back_matter = data.get("back_matter", [])
        self.exclude = data.get("exclude", [])
        self.intro_globs = DEFAULT_INTRO_NAMES + list(data.get("intro_names", []))
        # No defaults: a book that does not opt in keeps the previous ordering.
        self.outro_globs = list(data.get("outro_names", []))
        self.numbersections = bool(data.get("numbersections", True))
        self.toc_depth = int(data.get("toc_depth", 2))
        self.header_includes = list(data.get("header_includes", []))
        self.mainfont = data.get("mainfont")
        self.monofont = data.get("monofont")
        self.monofontoptions = list(data.get("monofontoptions", []))
        self.lift_intros = bool(data.get("lift_intros", False))
        self.front_matter_shift = int(data.get("front_matter_shift", 0))

    @classmethod
    def load(cls, book_dir: Path) -> "Manifest":
        toml_path = book_dir / "book.toml"
        with toml_path.open("rb") as fh:
            data = tomllib.load(fh)
        return cls(book_dir, data)

    # -- glob expansion ---------------------------------------------------- #

    def _expand(self, patterns: list[str]) -> tuple[list[Path], list[str]]:
        """Expand file globs relative to the book dir.

        Returns (ordered unique existing files, patterns that matched nothing).
        """
        out: list[Path] = []
        seen: set[Path] = set()
        empty: list[str] = []
        for pat in patterns:
            matches = sorted(
                (p for p in self.book_dir.glob(pat) if p.is_file()),
                key=lambda p: natural_key(p.name),
            )
            if not matches:
                empty.append(pat)
                continue
            for m in matches:
                r = m.resolve()
                if r not in seen:
                    seen.add(r)
                    out.append(m)
        return out, empty

    def front_files(self):
        return self._expand(self.front_matter)

    def back_files(self):
        return self._expand(self.back_matter)

    def is_excluded(self, rel_posix: str) -> bool:
        for pat in self.exclude:
            if fnmatch.fnmatch(rel_posix, pat):
                return True
        return False


# --------------------------------------------------------------------------- #
# Collection
# --------------------------------------------------------------------------- #

class Entry:
    """A single emitted file plus the context needed to place it."""
    __slots__ = ("path", "depth", "part_dir", "shift", "drop_h1")

    def __init__(self, path: Path, depth: int, part_dir: Path | None,
                 shift: int | None = None, drop_h1: bool = False):
        self.path = path
        self.depth = depth
        self.part_dir = part_dir
        self.shift = depth if shift is None else shift
        self.drop_h1 = drop_h1


def _skip_dir(name: str) -> bool:
    return name in DEFAULT_SKIP_DIRS or name.startswith(".")


def collect_source(
    root: Path,
    recursive: bool,
    part_level: int | None,
    manifest: Manifest,
    handled: set[Path],
) -> list[Entry]:
    """Walk one source root, returning ordered Entry objects.

    `handled` is the set of resolved paths already emitted as front/back matter;
    such files are skipped here so they are never emitted twice.
    """
    entries: list[Entry] = []

    def walk(directory: Path, depth: int):
        first_intro_seen = False
        for child in ordered_children(
            directory, manifest.intro_globs, manifest.outro_globs
        ):
            if child.is_dir():
                if _skip_dir(child.name):
                    continue
                if recursive:
                    walk(child, depth + 1)
            elif child.is_file() and child.suffix == ".md":
                rel = child.relative_to(manifest.book_dir).as_posix()
                if manifest.is_excluded(rel):
                    continue
                if child.resolve() in handled:
                    continue
                part_dir = _part_dir_for(child, root, part_level)

                shift, drop_h1 = depth, False
                if manifest.lift_intros:
                    is_intro = (intro_priority(child.name, manifest.intro_globs)
                                is not None)
                    if is_intro and not first_intro_seen:
                        first_intro_seen = True
                        if part_dir == directory:
                            # This file's H1 is the part title; the \part
                            # divider renders it, so the body follows bare and
                            # its subsections stay below chapter level.
                            drop_h1 = True
                        else:
                            # The container's heading: one level above its
                            # sibling content.
                            shift = max(depth - 1, 0)

                entries.append(Entry(child, depth, part_dir, shift, drop_h1))

    walk(root, 0)
    return entries


def _part_dir_for(file: Path, root: Path, part_level: int | None) -> Path | None:
    """The ancestor directory of `file` at `part_level` depth below `root`."""
    if not part_level:
        return None
    rel_parts = file.relative_to(root).parts  # includes filename
    if len(rel_parts) <= part_level:
        return None
    return root.joinpath(*rel_parts[:part_level])


def all_content_md(root: Path) -> list[Path]:
    """Every .md under `root`, skipping default-excluded directories."""
    out: list[Path] = []

    def walk(directory: Path):
        try:
            children = list(directory.iterdir())
        except OSError:
            return
        for child in children:
            if child.is_dir():
                if not _skip_dir(child.name):
                    walk(child)
            elif child.is_file() and child.suffix == ".md":
                out.append(child)

    walk(root)
    return out


# --------------------------------------------------------------------------- #
# Markdown transformation
# --------------------------------------------------------------------------- #

def strip_yaml_front_matter(text: str) -> str:
    if not text.startswith("---"):
        return text
    end = text.find("\n---", 3)
    if end == -1:
        return text
    return text[end + 4:].lstrip("\n")


# A thematic break written as `---` is indistinguishable from the opening of a
# YAML metadata block, and pandoc accepts those *anywhere* in a document, not
# only at the top.  Left alone, a `---` rule pairs with the next one and pandoc
# either aborts with a YAML parse error or -- worse, when the enclosed prose
# happens to parse as YAML -- silently drops everything between the two rules.
# `***` is the same thematic break to every markdown reader and can never open
# a metadata block.
FENCE_RE = re.compile(r"^\s{0,3}(`{3,}|~{3,})")
THEMATIC_BREAK_RE = re.compile(r"^\s{0,3}-{3,}\s*$")


def normalize_thematic_breaks(text: str) -> str:
    lines = text.split("\n")
    out: list[str] = []
    fence: str | None = None
    for i, line in enumerate(lines):
        m = FENCE_RE.match(line)
        if m:
            char = m.group(1)[0]
            if fence is None:
                fence = char
            elif char == fence:
                fence = None
            out.append(line)
            continue
        # A run of dashes directly under a paragraph line is a setext H2
        # underline, not a rule -- leave those for pandoc to read as a heading.
        if (
            fence is None
            and THEMATIC_BREAK_RE.match(line)
            and (i == 0 or not lines[i - 1].strip())
        ):
            out.append("***")
            continue
        out.append(line)
    return "\n".join(out)


def shift_headings(text: str, shift: int) -> str:
    if shift <= 0:
        return text

    def _repl(m):
        return "#" * min(len(m.group(1)) + shift, 6) + m.group(2)

    return HEADING_RE.sub(_repl, text)


def strip_first_h1(text: str) -> str:
    """Remove the first top-level ATX heading line, if it opens the file."""
    return re.sub(r"^\s*#[ \t][^\n]*\n+", "", text, count=1)


def part_title(part_dir: Path, intro_globs: list[str]) -> str:
    """Human title for a \\part: the intro file's first H1, else the dir name."""
    for child in ordered_children(part_dir, intro_globs):
        if child.is_file() and intro_priority(child.name, intro_globs) is not None:
            try:
                for line in child.read_text(encoding="utf-8").splitlines():
                    m = re.match(r"^#\s+(.*)$", line)
                    if m:
                        return m.group(1).strip()
            except OSError:
                pass
            break
    name = re.sub(r"^\d+[-_.]*", "", part_dir.name)
    name = name.replace("-", " ").replace("_", " ").strip()
    return name.title() if name else part_dir.name


def read_content(path: Path) -> str | None:
    """Read a file; return None (with a warning) if empty/whitespace-only."""
    try:
        text = path.read_text(encoding="utf-8")
    except OSError as exc:
        print(f"  WARNING: cannot read {path}: {exc}", file=sys.stderr)
        return None
    if not text.strip():
        return None
    return text


# --------------------------------------------------------------------------- #
# Assembly
# --------------------------------------------------------------------------- #

def yaml_metadata(m: Manifest) -> str:
    lines = ["---", f'title: "{m.title}"']
    if m.subtitle:
        lines.append(f'subtitle: "{m.subtitle}"')
    if m.author:
        lines.append(f'author: "{m.author}"')
    lines += [
        'date: "2026"',
        "documentclass: book",
        "classoption:",
        "  - 11pt",
        "  - openany",
        "geometry:",
        "  - top=1in",
        "  - bottom=1in",
        "  - left=1.25in",
        "  - right=1in",
        "toc: true",
        f"toc-depth: {m.toc_depth}",
        f"numbersections: {'true' if m.numbersections else 'false'}",
        "colorlinks: true",
        "linkcolor: NavyBlue",
        "urlcolor: NavyBlue",
    ]
    if m.mainfont:
        lines.append(f'mainfont: "{m.mainfont}"')
    if m.monofont:
        lines.append(f'monofont: "{m.monofont}"')
    if m.monofontoptions:
        lines.append("monofontoptions:")
        for opt in m.monofontoptions:
            lines.append(f'  - "{opt}"')
    if m.header_includes:
        lines.append("header-includes:")
        for block in m.header_includes:
            # A raw-LaTeX fence so pandoc passes the block through verbatim
            # rather than reading it as markdown.
            lines.append("  - |")
            lines.append("    ```{=latex}")
            for ln in block.splitlines():
                lines.append(f"    {ln}")
            lines.append("    ```")
    lines += [
        "---",
        "",
    ]
    return "\n".join(lines)


def assemble(m: Manifest, chapter_range=None):
    """Return (combined_markdown, stats) for the whole book."""
    front, front_empty = m.front_files()
    back, back_empty = m.back_files()
    handled = {p.resolve() for p in front} | {p.resolve() for p in back}

    body_entries: list[Entry] = []
    source_entries: list[list[Entry]] = []
    for spec in m.sources:
        root = (m.book_dir / spec["root"]).resolve()
        recursive = spec.get("recursive", True)
        part_level = spec.get("part_level")
        if not root.exists():
            source_entries.append([])
            continue
        ent = collect_source(root, recursive, part_level, m, handled)
        ent = _apply_chapter_range(ent, root, part_level, chapter_range)
        source_entries.append(ent)
        body_entries.extend(ent)

    parts: list[str] = [yaml_metadata(m)]
    stats = {"files": 0, "chars": 0, "skipped_empty": 0}

    def emit(path: Path, shift: int, drop_h1: bool = False):
        text = read_content(path)
        if text is None:
            stats["skipped_empty"] += 1
            print(f"  WARNING: empty file skipped: "
                  f"{path.relative_to(m.book_dir)}", file=sys.stderr)
            return
        text = strip_yaml_front_matter(text)
        text = normalize_thematic_breaks(text)
        if drop_h1:
            text = strip_first_h1(text)
        text = shift_headings(text, shift)
        # Image links are written relative to their source file so they render
        # on GitHub; the assembled document resolves them from the source root
        # (see --resource-path), so strip the ../ prefixes.
        text = re.sub(r"\]\((?:\.\./)+figures/", "](figures/", text)
        rel = path.relative_to(m.book_dir)
        parts.append(f"\n\n<!-- === {rel} === -->\n\n")
        parts.append(text.strip())
        parts.append("\n")
        stats["files"] += 1
        stats["chars"] += len(text)

    # Front matter
    for f in front:
        emit(f, m.front_matter_shift)

    # Body, with \part dividers at part boundaries
    current_part: Path | None = None
    for ent in body_entries:
        if ent.part_dir is not None and ent.part_dir != current_part:
            current_part = ent.part_dir
            title = part_title(current_part, m.intro_globs)
            parts.append(f"\n\n# {title}\n\n")
        emit(ent.path, ent.shift, ent.drop_h1)

    # Back matter
    for f in back:
        emit(f, 0)

    stats["front_empty"] = front_empty
    stats["back_empty"] = back_empty
    stats["source_entries"] = source_entries
    return "".join(parts), stats


def _apply_chapter_range(entries, root, part_level, chapter_range):
    """Restrict entries to parts / top-level dirs N..M (1-based, inclusive)."""
    if not chapter_range:
        return entries
    lo, hi = chapter_range
    level = part_level if part_level else 1

    # Ordinal of each grouping directory, in first-appearance order.
    order: dict[Path, int] = {}

    def group_of(path: Path):
        rel = path.relative_to(root).parts
        if len(rel) <= level:
            return None
        return root.joinpath(*rel[:level])

    counter = 0
    for e in entries:
        g = group_of(e.path)
        if g is not None and g not in order:
            counter += 1
            order[g] = counter

    out = []
    for e in entries:
        g = group_of(e.path)
        if g is None:
            continue
        if lo <= order[g] <= hi:
            out.append(e)
    return out


# --------------------------------------------------------------------------- #
# --check
# --------------------------------------------------------------------------- #

def run_check(m: Manifest) -> int:
    front, front_empty = m.front_files()
    back, back_empty = m.back_files()
    handled = {p.resolve() for p in front} | {p.resolve() for p in back}

    captured: set[Path] = set(handled)
    empty_sources: list[str] = []
    empty_manifest_globs: list[str] = list(front_empty) + list(back_empty)

    for spec in m.sources:
        root = (m.book_dir / spec["root"]).resolve()
        if not root.exists():
            empty_sources.append(spec["root"] + "  (missing directory)")
            continue
        ent = collect_source(
            root, spec.get("recursive", True), spec.get("part_level"), m, handled
        )
        if not ent:
            empty_sources.append(spec["root"] + "  (no markdown captured)")
        for e in ent:
            captured.add(e.path.resolve())

    # Every content .md under any source that is neither captured nor excluded.
    uncaptured: list[Path] = []
    empty_files: list[Path] = []
    for spec in m.sources:
        root = (m.book_dir / spec["root"]).resolve()
        if not root.exists():
            continue
        for md in all_content_md(root):
            rp = md.resolve()
            rel = md.relative_to(m.book_dir).as_posix()
            if m.is_excluded(rel):
                continue
            # Empty files are always censused (warned) and never counted as a
            # coverage failure, whether or not the manifest captures them.
            try:
                if not md.read_text(encoding="utf-8").strip():
                    empty_files.append(md)
                    continue
            except OSError:
                pass
            if rp in captured:
                continue
            uncaptured.append(md)

    # ------------------------------------------------------------------ #
    print(f"=== {m.book_dir.name} : build --check ===")
    print(f"  captured files : {len(captured)}")

    problems = 0
    if uncaptured:
        problems += len(uncaptured)
        print(f"  UNCAPTURED content ({len(uncaptured)}):")
        for p in sorted(uncaptured):
            print(f"    - {p.relative_to(m.book_dir)}")
    if empty_manifest_globs:
        problems += len(empty_manifest_globs)
        print(f"  MANIFEST GLOBS MATCHING NOTHING ({len(empty_manifest_globs)}):")
        for g in empty_manifest_globs:
            print(f"    - {g}")
    if empty_sources:
        problems += len(empty_sources)
        print(f"  EMPTY / MISSING SOURCES ({len(empty_sources)}):")
        for s in empty_sources:
            print(f"    - {s}")
    if empty_files:
        print(f"  zero-byte / whitespace-only (warned, skipped) "
              f"({len(empty_files)}):")
        for p in sorted(empty_files):
            print(f"    ~ {p.relative_to(m.book_dir)}")

    if problems == 0:
        print("  OK: all content captured.")
        return 0
    print(f"  FAIL: {problems} problem(s).")
    return 1


# --------------------------------------------------------------------------- #
# pandoc back-ends
# --------------------------------------------------------------------------- #

def _pandoc_available() -> bool:
    return shutil.which("pandoc") is not None


def _latex_engine() -> str | None:
    for eng in ("xelatex", "pdflatex", "lualatex"):
        if shutil.which(eng):
            return eng
    return None


def build_markdown(combined: str, out: Path) -> bool:
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(combined, encoding="utf-8")
    kb = out.stat().st_size / 1024
    print(f"  wrote {out}  ({kb:,.0f} KB)")
    return True


def _source_roots(m: Manifest) -> list[Path]:
    """Directories images resolve against: the book dir and each source root."""
    roots = [m.book_dir]
    for spec in m.sources:
        r = (m.book_dir / spec.get("root", ".")).resolve()
        if r.exists():
            roots.append(r)
    return roots


def _run_pandoc(combined: str, out: Path, to_pdf: bool, engine: str | None,
                title: str, numbersections: bool = True,
                toc_depth: int = 2, resource_paths: list[Path] = ()) -> bool:
    out.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.NamedTemporaryFile(
        "w", suffix=".md", encoding="utf-8", delete=False
    ) as fh:
        fh.write(combined)
        tmp = fh.name
    try:
        cmd = [
            "pandoc", tmp,
            "--from", "markdown+tex_math_dollars+raw_tex+pipe_tables"
                      "+fenced_code_blocks+smart",
            "--top-level-division=part",
            "--toc", f"--toc-depth={toc_depth}",
            *(["--resource-path",
               os.pathsep.join(str(p) for p in resource_paths)]
              if resource_paths else []),
            *(["--number-sections"] if numbersections else []),
            "--highlight-style=tango",
            "--metadata", f"title={title}",
            "--output", str(out),
        ]
        if to_pdf:
            cmd += ["--to", "pdf", f"--pdf-engine={engine}"]
        else:
            # --embed-resources inlines referenced images (the SVG figures)
            # so the single output file works from anywhere.
            cmd += ["--to", "html5", "--standalone", "--embed-resources",
                    "--mathjax"]
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            sys.stderr.write("\n--- pandoc stderr (tail) ---\n")
            sys.stderr.write("\n".join(result.stderr.splitlines()[-40:]) + "\n")
            return False
    finally:
        os.unlink(tmp)
    mb = out.stat().st_size / 1_000_000
    print(f"  wrote {out}  ({mb:.1f} MB)")
    return True


# --------------------------------------------------------------------------- #
# CLI
# --------------------------------------------------------------------------- #

def parse_range(s: str):
    if "-" in s:
        lo, hi = s.split("-", 1)
        return int(lo), int(hi)
    n = int(s)
    return n, n


def find_book_dir(arg: str | None) -> Path:
    if arg:
        p = Path(arg).resolve()
        if p.is_file() and p.name == "book.toml":
            return p.parent
        return p
    return Path.cwd()


def main() -> int:
    ap = argparse.ArgumentParser(
        description="Config-driven textbook builder (reads book.toml).",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__,
    )
    ap.add_argument("book", nargs="?", default=None,
                    help="Book directory or its book.toml (default: cwd)")
    ap.add_argument("--check", action="store_true",
                    help="Report content not captured by the manifest.")
    ap.add_argument("--markdown", metavar="OUT", nargs="?", const="",
                    help="Write assembled markdown (default: output/<book>.md).")
    ap.add_argument("--html", metavar="OUT", nargs="?", const="",
                    help="Render HTML via pandoc.")
    ap.add_argument("--pdf", metavar="OUT", nargs="?", const="",
                    help="Render PDF via pandoc.")
    ap.add_argument("--chapters", metavar="N-M",
                    help="Restrict to parts/top-level sections N..M.")
    args = ap.parse_args()

    book_dir = find_book_dir(args.book)
    toml_path = book_dir / "book.toml"
    if not toml_path.exists():
        sys.exit(f"ERROR: no book.toml in {book_dir}")

    m = Manifest.load(book_dir)

    if args.check:
        return run_check(m)

    if args.markdown is None and args.html is None and args.pdf is None:
        # Default action: --check.
        return run_check(m)

    chapter_range = parse_range(args.chapters) if args.chapters else None
    combined, stats = assemble(m, chapter_range)
    print(f"  assembled {stats['files']} files, {stats['chars']:,} chars"
          + (f", {stats['skipped_empty']} empty skipped"
             if stats["skipped_empty"] else ""))

    out_dir = book_dir / "output"
    slug = book_dir.name.lower().replace(" ", "-")
    ok = True

    if args.markdown is not None:
        out = Path(args.markdown) if args.markdown else out_dir / f"{slug}.md"
        ok = build_markdown(combined, out) and ok

    if args.html is not None:
        if not _pandoc_available():
            sys.exit("ERROR: pandoc not found (needed for --html).")
        out = Path(args.html) if args.html else out_dir / f"{slug}.html"
        ok = _run_pandoc(combined, out, False, None, m.title,
                         m.numbersections, m.toc_depth,
                         _source_roots(m)) and ok

    if args.pdf is not None:
        if not _pandoc_available():
            sys.exit("ERROR: pandoc not found (needed for --pdf).")
        engine = _latex_engine()
        if not engine:
            sys.exit("ERROR: no LaTeX engine (xelatex/pdflatex) for --pdf.")
        out = Path(args.pdf) if args.pdf else out_dir / f"{slug}.pdf"
        ok = _run_pandoc(combined, out, True, engine, m.title,
                         m.numbersections, m.toc_depth,
                         _source_roots(m)) and ok

    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
