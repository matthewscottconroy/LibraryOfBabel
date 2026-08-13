#!/usr/bin/env python3
"""Check internal cross-references in the Telecommunications book.

Every "Chapter N §N.M" reference must name a chapter that exists and a section
number that exists *within that chapter*. Chapters whose sections have not been
written yet are reported separately rather than flagged, so this is usable while
the book is still being drafted.

Also checks:
  - the chapter number and the section prefix agree ("Chapter 7 §9.2" is a slip)
  - exercise labels use one consistent scheme across the whole book
  - references to unit and appendix files resolve

Usage:
    python3 tools/checkrefs.py            # from the Telecommunications/ dir
    python3 tools/checkrefs.py --verbose  # also list unwritten chapters
"""
from __future__ import annotations

import pathlib
import re
import sys

try:
    import tomllib
except ModuleNotFoundError:  # Python < 3.11
    import tomli as tomllib  # type: ignore

HERE = pathlib.Path(__file__).resolve().parent.parent
BOOK = HERE / "book"
SUBJECT = HERE / "subject.toml"

SECTION_REF = re.compile(r"Chapter\s+(\d+)\s*\n?\s*§(\d+)\.(\d+)")
BARE_SECTION_REF = re.compile(r"(?<![\w.])§(\d+)\.(\d+)")
EXERCISE_LETTERED = re.compile(r"^\*\*([A-F])(\d+)\.\*\*", re.M)
EXERCISE_NUMBERED = re.compile(r"^\*\*(\d+)\.(\d+)\*\*", re.M)


def load_chapters() -> tuple[dict[int, int], dict[int, str], set[pathlib.Path]]:
    """Return (sections_written, names, directories_declared_in_subject_toml)."""
    subject = tomllib.loads(SUBJECT.read_text())
    written: dict[int, int] = {}
    names: dict[int, str] = {}
    declared: set[pathlib.Path] = set()
    for index, chapter in enumerate(subject["chapters"]):
        number = index + 1
        directory = (BOOK / chapter["file"]).parent
        declared.add(directory)
        written[number] = len(sorted(directory.glob("s[0-9][0-9]_*.md")))
        names[number] = chapter["name"]
    return written, names, declared


def stray_directories(declared: set[pathlib.Path]) -> list[str]:
    """Chapter directories on disk that subject.toml does not name.

    A near-miss slug (ch44_wifi vs ch44_wi_fi) produces a directory the build
    still sweeps up but the quiz config never points at, so sections written
    there look present in the PDF and absent to every other tool.
    """
    problems = []
    for directory in sorted(BOOK.glob("unit_*/ch*")):
        if not directory.is_dir() or directory in declared:
            continue
        contents = sorted(p.name for p in directory.glob("*.md"))
        problems.append(
            f"{directory.relative_to(BOOK)}: chapter directory is not named by "
            f"subject.toml ({len(contents)} files) — likely a slug mismatch"
        )
    return problems


# A cross-reference can resolve perfectly and still point at the wrong chapter.
# This table maps a topic word to the chapters that legitimately own it; a
# reference sitting next to one of these words but naming a different chapter is
# worth a human look. False positives are expected ("DNS runs over UDP
# (Chapter 36)"), so these are reported for review and never fail the run.
#
# Only topics with a single unambiguous owning chapter are listed. Cross-cutting
# words (VLAN, IPv6, DNS, NAT, TLS…) appear legitimately beside almost any
# chapter reference and produced far more noise than findings, so they are
# deliberately absent.
TOPIC_OWNERS: dict[str, set[int]] = {
    "spanning tree": {19}, "VLAN hopping": {62},
    "longest.prefix": {29}, "route leak": {32},
    "peering": {48}, "DOCSIS": {49}, "PON": {49},
    "MPLS": {50}, "submarine": {50}, "SD-WAN": {51}, "SASE": {51},
    "CDN": {52}, "DSCP": {52}, "bufferbloat": {52, 66},
    "IPAM": {53}, "runbook": {53}, "SNMP": {54}, "syslog": {54},
    "NetFlow": {54}, "sFlow": {54},
    "RADIUS": {59}, "TACACS": {59}, "zero trust": {51, 59},
    "stateful firewall": {60}, "microsegmentation": {60},
    "IPsec": {61}, "WireGuard": {61},
    "VXLAN": {67}, "GENEVE": {67}, "EVPN": {67},
    "SDN": {68}, "OpenFlow": {68}, "NETCONF": {70}, "gNMI": {54, 70},
}
ANY_CHAPTER_REF = re.compile(r"Chapter (\d+)\b")


def topic_mismatches(names: dict[int, str]) -> list[str]:
    """References whose surrounding text names a topic another chapter owns."""
    suspects = []
    for path in sorted(BOOK.rglob("*.md")):
        for number, line in enumerate(path.read_text().splitlines(), 1):
            for match in ANY_CHAPTER_REF.finditer(line):
                chapter = int(match.group(1))
                before = line[max(0, match.start() - 70):match.start()]
                for topic, owners in TOPIC_OWNERS.items():
                    if chapter in owners:
                        continue
                    if re.search(rf"\b{re.escape(topic)}\b", before, re.I):
                        suspects.append(
                            f"{path.relative_to(BOOK)}:{number}: '{topic}' next to "
                            f"Chapter {chapter} ({names.get(chapter, '?')})"
                        )
    return suspects


def chapter_of(path: pathlib.Path) -> int | None:
    """Infer the chapter number from a path like .../ch27_address_plans/..."""
    for part in path.parts:
        match = re.match(r"ch(\d+)_", part)
        if match:
            return int(match.group(1))
    return None


def main() -> int:
    verbose = "--verbose" in sys.argv
    written, names, declared = load_chapters()

    problems: list[str] = stray_directories(declared)
    checked = 0

    for path in sorted(BOOK.rglob("*.md")):
        text = path.read_text()
        relative = path.relative_to(BOOK)
        own_chapter = chapter_of(path)

        for match in SECTION_REF.finditer(text):
            checked += 1
            chapter = int(match.group(1))
            prefix = int(match.group(2))
            section = int(match.group(3))

            if chapter != prefix:
                problems.append(
                    f"{relative}: 'Chapter {chapter} §{prefix}.{section}' — "
                    f"chapter number and section prefix disagree"
                )
            elif chapter not in written:
                problems.append(
                    f"{relative}: 'Chapter {chapter} §{prefix}.{section}' — "
                    f"no such chapter (book has {len(written)})"
                )
            elif written[chapter] and section > written[chapter]:
                problems.append(
                    f"{relative}: 'Chapter {chapter} §{prefix}.{section}' — "
                    f"chapter {chapter} ({names[chapter]}) has only "
                    f"{written[chapter]} sections"
                )

        # A bare "§N.M" inside a chapter should refer to that chapter.
        if own_chapter is not None:
            for match in BARE_SECTION_REF.finditer(text):
                prefix = int(match.group(1))
                section = int(match.group(2))
                if prefix != own_chapter:
                    continue  # a cross-chapter ref; the form above covers it
                checked += 1
                if written[own_chapter] and section > written[own_chapter]:
                    problems.append(
                        f"{relative}: '§{prefix}.{section}' — this chapter has "
                        f"only {written[own_chapter]} sections"
                    )

    # Exercise labelling must be consistent book-wide.
    styles: dict[str, list[str]] = {"lettered": [], "numbered": [], "other": []}
    for path in sorted(BOOK.rglob("exercises.md")):
        text = path.read_text()
        if EXERCISE_LETTERED.search(text):
            styles["lettered"].append(str(path.relative_to(BOOK)))
        elif EXERCISE_NUMBERED.search(text):
            styles["numbered"].append(str(path.relative_to(BOOK)))
        else:
            styles["other"].append(str(path.relative_to(BOOK)))

    in_use = {name: files for name, files in styles.items() if files}
    if len(in_use) > 1:
        problems.append(
            "exercise numbering is inconsistent: "
            + ", ".join(f"{name} ({len(files)} chapters)"
                        for name, files in in_use.items())
        )
        for name, files in in_use.items():
            if len(files) <= 5:
                for f in files:
                    problems.append(f"    {name}: {f}")

    unwritten = sorted(n for n, count in written.items() if count == 0)
    suspects = topic_mismatches(names)

    print("=== Telecommunications : cross-reference check ===")
    print(f"  section references checked : {checked}")
    print(f"  chapters with sections     : {len(written) - len(unwritten)}"
          f"/{len(written)}")
    print(f"  exercise numbering         : "
          f"{', '.join(in_use) if in_use else 'none found'}")

    if verbose and unwritten:
        print(f"  not yet drafted            : "
              f"{', '.join(str(n) for n in unwritten)}")

    if suspects:
        print(f"  topic review (not failures) : {len(suspects)}")
        if verbose:
            for suspect in suspects:
                print(f"    {suspect}")
        else:
            print("    (run with --verbose to list them)")

    if problems:
        print(f"\n  {len(problems)} problem(s):\n")
        for problem in problems:
            print(f"    {problem}")
        return 1

    print("\n  OK: all cross-references resolve.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
