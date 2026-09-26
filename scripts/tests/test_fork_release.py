"""Focused tests for fork release version and workflow contracts."""
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "release"))
sys.path.insert(0, str(Path(__file__).resolve().parents[2] / ".github" / "scripts"))

import fork_version
import package_native
import bump_version

ROOT = Path(__file__).resolve().parents[2]


class ForkReleaseTests(unittest.TestCase):
    def test_bump_script_accepts_fork_semver_metadata(self):
        bump_version.validate_version("2.1.0+fork.1")

    def test_first_release_uses_revision_one(self):
        plan = fork_version.plan_release(
            "2.1.0", "2.1.0", ["v2.1.0"], current_tag_commit=None,
            head_commit="new-head", release_state="missing",
        )
        self.assertEqual(plan, fork_version.ReleasePlan("2.1.0+fork.1", False))

    def test_two_successive_releases_advance_after_first_is_published(self):
        first = fork_version.plan_release(
            "2.1.0", "2.1.0", [], current_tag_commit=None,
            head_commit="release-one", release_state="missing",
        )
        self.assertEqual(first.version, "2.1.0+fork.1")
        second = fork_version.plan_release(
            "2.1.0", first.version, ["v2.1.0+fork.1"],
            current_tag_commit="release-one", head_commit="release-one",
            release_state="published",
        )
        self.assertEqual(second, fork_version.ReleasePlan("2.1.0+fork.2", False))

    def test_published_current_release_allocates_next_revision(self):
        tags = ["v2.1.0+fork.1"]
        plan = fork_version.plan_release(
            "2.1.0", "2.1.0+fork.1", tags, current_tag_commit="head",
            head_commit="head", release_state="published",
        )
        self.assertEqual(plan, fork_version.ReleasePlan("2.1.0+fork.2", False))

    def test_failed_release_retries_only_when_tag_is_exact_head(self):
        tags = ["v2.1.0+fork.1"]
        for state in ("draft", "missing"):
            with self.subTest(state=state):
                plan = fork_version.plan_release(
                    "2.1.0", "2.1.0+fork.1", tags, current_tag_commit="head",
                    head_commit="head", release_state=state,
                )
                self.assertEqual(plan, fork_version.ReleasePlan("2.1.0+fork.1", True))

    def test_new_main_head_never_reuses_old_release_tag(self):
        plan = fork_version.plan_release(
            "2.1.0", "2.1.0+fork.1", ["v2.1.0+fork.1"], current_tag_commit="old-head",
            head_commit="new-head", release_state="draft",
        )
        self.assertEqual(plan, fork_version.ReleasePlan("2.1.0+fork.2", False))

    def test_release_state_missing_with_tag_on_different_head_allocates_next(self):
        plan = fork_version.plan_release(
            "2.1.0", "2.1.0+fork.1", ["v2.1.0+fork.1"], current_tag_commit="old-head",
            head_commit="new-head", release_state="missing",
        )
        self.assertEqual(plan.version, "2.1.0+fork.2")
        self.assertFalse(plan.reuse_existing)

    def test_rejects_non_release_versions_and_windows_overflow(self):
        with self.assertRaises(ValueError):
            fork_version.next_fork_version("2.1.0-beta.1", [])
        with self.assertRaises(ValueError):
            fork_version.next_fork_version("2.1.0", ["v2.1.0+fork.65535"])
        with self.assertRaises(ValueError):
            fork_version.next_fork_version("65536.1.0", [])

    def test_windows_numeric_version_uses_fork_revision(self):
        self.assertEqual(package_native.windows_numeric_version("2.1.0+fork.1"), "2.1.0.1")
        self.assertEqual(package_native.windows_numeric_version("2.1.0"), "2.1.0.0")
        with self.assertRaises(RuntimeError):
            package_native.windows_numeric_version("2.1.0+fork.65536")

    def test_fork_workflow_is_manual_and_reuses_native_package(self):
        workflow = (ROOT / ".github/workflows/fork-release.yml").read_text(encoding="utf-8")
        self.assertIn("workflow_dispatch:", workflow)
        self.assertIn("uses: ./.github/workflows/native-package.yml", workflow)
        self.assertIn("fork_release: true", workflow)
        self.assertIn('git push --atomic origin HEAD:main "refs/tags/${TAG}"', workflow)
        native = (ROOT / ".github/workflows/native-package.yml").read_text(encoding="utf-8")
        self.assertIn("vars.OXIDETERM_UPDATER_PUBKEY", native)
        self.assertIn("minisign -V -p", native)
        self.assertIn("draft: ${{ inputs.fork_release }}", native)
        self.assertIn("latest.version !== version", native)
        self.assertIn("OxideTerm_${version}_windows_x64_portable.zip", native)


if __name__ == "__main__":
    unittest.main()
