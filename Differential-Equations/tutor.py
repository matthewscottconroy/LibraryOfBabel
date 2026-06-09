#!/usr/bin/env python3
"""Math Tutor — adaptive mathematics problem trainer.

Usage:
    python tutor.py                      # 20-problem session
    python tutor.py --stats              # mastery dashboard
    python tutor.py --reset              # wipe progress for current profile
    python tutor.py --problems 10        # shorter session
    python tutor.py --topic odes         # focus on one topic
    python tutor.py --profile alice      # separate progress per user
"""

import sys
import os
import argparse

# Ensure venv packages are available when run directly
_venv = os.path.join(os.path.dirname(__file__), ".venv", "lib")
if os.path.isdir(_venv):
    import glob
    for _p in glob.glob(os.path.join(_venv, "python*", "site-packages")):
        if _p not in sys.path:
            sys.path.insert(0, _p)

try:
    import sympy   # noqa: F401
    import rich    # noqa: F401
except ImportError:
    print("Missing dependencies. Run:\n  pip install sympy rich\n"
          "or use the venv:\n  .venv/bin/python tutor.py")
    sys.exit(1)

from tutor.session import Session, DEFAULT_SESSION_LENGTH


def _valid_topic(s: str) -> str:
    from tutor.adaptive import PREREQUISITES
    valid = set(PREREQUISITES.keys())
    if s not in valid:
        raise argparse.ArgumentTypeError(
            f"Unknown topic '{s}'. Choose from: {', '.join(sorted(valid))}"
        )
    return s


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="tutor.py", description="Adaptive Math Problem Trainer",
        add_help=True,
    )
    parser.add_argument("--stats",    action="store_true",
                        help="Show mastery dashboard and exit")
    parser.add_argument("--reset",    action="store_true",
                        help="Wipe progress for the active profile and start fresh")
    parser.add_argument("--profile",  default="default", metavar="NAME",
                        help="Use a named progress profile (default: 'default')")
    parser.add_argument("--problems", type=int, default=DEFAULT_SESSION_LENGTH,
                        metavar="N", help="Number of problems per session")
    parser.add_argument("--topic",    default=None, type=_valid_topic,
                        metavar="TOPIC",
                        help="Focus session on one topic (e.g. odes, fourier)")
    args = parser.parse_args()

    if args.reset:
        from tutor.db import reset_db
        reset_db(args.profile)
        print(f"Progress wiped for profile '{args.profile}'. Starting fresh.\n")

    session = Session(profile=args.profile)

    if args.stats:
        session.show_stats()
    else:
        session.run(n_problems=args.problems, topic_filter=args.topic)


if __name__ == "__main__":
    main()
