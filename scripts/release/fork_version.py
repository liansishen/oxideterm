#!/usr/bin/env python3
"""Plan safe fork release versions from the upstream baseline and existing tags."""
from __future__ import annotations

import argparse
import json
import re
import subprocess
from dataclasses import asdict, dataclass

BASE_RE = re.compile(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$")
VERSION_RE = re.compile(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)\+fork\.([1-9]\d*)$")
TAG_RE = re.compile(r"^v(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)\+fork\.([1-9]\d*)$")
WINDOWS_COMPONENT_MAX = 65535


@dataclass(frozen=True)
class ReleasePlan:
    version: str
    reuse_existing: bool


def _validate_base(base: str) -> None:
    if not BASE_RE.fullmatch(base):
        raise ValueError(f"invalid upstream version: {base}")
    if any(int(part) > WINDOWS_COMPONENT_MAX for part in base.split(".")):
        raise ValueError("upstream version component exceeds the Windows resource version limit")


def next_fork_version(base: str, tags: list[str]) -> str:
    _validate_base(base)
    prefix = f"v{base}+fork."
    revisions = []
    for tag in tags:
        match = TAG_RE.fullmatch(tag)
        if match and tag.startswith(prefix):
            revisions.append(int(match.group(4)))
    next_revision = max(revisions, default=0) + 1
    if next_revision > WINDOWS_COMPONENT_MAX:
        raise ValueError("fork revision exceeds the Windows resource version limit")
    return f"{base}+fork.{next_revision}"


def plan_release(
    base: str,
    current_version: str,
    tags: list[str],
    *,
    current_tag_commit: str | None,
    head_commit: str,
    release_state: str,
) -> ReleasePlan:
    """Reuse only this exact HEAD's unpublished/currently absent release tag."""
    _validate_base(base)
    current_match = VERSION_RE.fullmatch(current_version)
    if current_match and int(current_match.group(4)) > WINDOWS_COMPONENT_MAX:
        raise ValueError("fork revision exceeds the Windows resource version limit")
    current_tag = f"v{current_version}"
    current_tag_exists = current_tag in tags
    can_reuse = (
        current_match is not None
        and ".".join(current_match.group(i) for i in range(1, 4)) == base
        and current_tag_exists
        and current_tag_commit == head_commit
        and release_state in {"draft", "missing"}
    )
    if can_reuse:
        return ReleasePlan(current_version, True)
    return ReleasePlan(next_fork_version(base, tags), False)


def verify_upstream_ancestor(base: str) -> None:
    _validate_base(base)
    upstream_ref = f"refs/tags/upstream-v{base}"
    subprocess.run(["git", "rev-parse", "--verify", upstream_ref], check=True, stdout=subprocess.DEVNULL)
    subprocess.run(["git", "merge-base", "--is-ancestor", upstream_ref, "HEAD"], check=True)


def _main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("base")
    parser.add_argument("--current-version", required=True)
    parser.add_argument("--head", required=True)
    parser.add_argument("--current-tag-commit", default="")
    parser.add_argument("--release-state", choices=("draft", "published", "missing"), required=True)
    args = parser.parse_args()
    _validate_base(args.base)
    verify_upstream_ancestor(args.base)
    tags = subprocess.check_output(["git", "tag", "--list", f"v{args.base}+fork.*"], text=True).splitlines()
    plan = plan_release(
        args.base,
        args.current_version,
        tags,
        current_tag_commit=args.current_tag_commit or None,
        head_commit=args.head,
        release_state=args.release_state,
    )
    print(json.dumps(asdict(plan)))
    return 0


if __name__ == "__main__":
    raise SystemExit(_main())
