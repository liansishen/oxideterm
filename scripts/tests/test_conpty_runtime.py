"""Pinning and architecture boundaries for the bundled Windows runtime."""

import hashlib
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch
import zipfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "release"))
import conpty_runtime
import package_native


class ConptyRuntimeTests(unittest.TestCase):
    def test_windows_installer_and_portable_both_stage_the_runtime(self):
        for target, arch in [("x86_64-pc-windows-msvc", "x64"), ("aarch64-pc-windows-msvc", "arm64")]:
            with self.subTest(target=target), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                resources = root / "resources"
                for relative in ["agents", "icons", f"cli-bin/{target}", f"helpers/{target}"]:
                    (resources / relative).mkdir(parents=True)
                binary = root / "oxideterm-native.exe"
                helper = root / "oxideterm-update-helper.exe"
                binary.write_bytes(b"application fixture")
                helper.write_bytes(b"update helper fixture")
                host = f"{arch} host fixture".encode()

                def stage_fixture(destination, runtime_target):
                    self.assertEqual(runtime_target, target)
                    (destination / f"conpty/{arch}").mkdir(parents=True)
                    (destination / "conpty/conpty.dll").write_bytes(b"DLL fixture")
                    (destination / f"conpty/{arch}/OpenConsole.exe").write_bytes(host)

                with (
                    patch.object(package_native, "RESOURCE_DIR", resources),
                    patch.object(package_native, "DIST_DIR", root / "dist"),
                    patch.object(package_native, "stage_conpty_runtime", side_effect=stage_fixture),
                    patch.object(package_native, "find_7zip", return_value=None),
                ):
                    label = f"windows_{arch}"
                    installer = package_native.stage_windows_installer_root(binary, target, "2.1.0", label, helper)
                    self.assertEqual((installer / "resources/conpty/conpty.dll").read_bytes(), b"DLL fixture")
                    self.assertEqual((installer / f"resources/conpty/{arch}/OpenConsole.exe").read_bytes(), host)
                    package_native.create_portable_package(binary, helper, target, "2.1.0", label)
                    with zipfile.ZipFile(root / f"dist/OxideTerm_2.1.0_{label}_portable.zip") as archive:
                        prefix = f"OxideTerm_2.1.0_{label}_portable/"
                        self.assertEqual(archive.read(prefix + "resources/conpty/conpty.dll"), b"DLL fixture")
                        self.assertEqual(archive.read(prefix + f"resources/conpty/{arch}/OpenConsole.exe"), host)
                        manifest = json.loads(archive.read(prefix + "portable-update.json"))
                        self.assertEqual(
                            set(manifest["managedEntries"]) & {"resources", "conpty.dll", "OpenConsole.exe"},
                            {"resources"},
                        )

    def test_stage_selects_matching_pair_and_rejects_corruption_before_writing(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            package = root / "runtime.nupkg"
            runtimes = {}
            with zipfile.ZipFile(package, "w") as archive:
                for target, arch in [("x86_64-pc-windows-msvc", "x64"), ("aarch64-pc-windows-msvc", "arm64")]:
                    dll = f"{arch} DLL".encode()
                    host = f"{arch} host".encode()
                    archive.writestr(f"runtimes/win-{arch}/native/conpty.dll", dll)
                    archive.writestr(f"build/native/runtimes/{arch}/OpenConsole.exe", host)
                    runtimes[target] = (arch, hashlib.sha256(dll).hexdigest(), hashlib.sha256(host).hexdigest())
            with (
                patch.object(conpty_runtime, "RUNTIMES", runtimes),
                patch.object(conpty_runtime, "PACKAGE_SHA256", hashlib.sha256(package.read_bytes()).hexdigest()),
            ):
                for target, arch in [("x86_64-pc-windows-msvc", "x64"), ("aarch64-pc-windows-msvc", "arm64")]:
                    with self.subTest(target=target):
                        destination = root / arch
                        conpty_runtime.stage_runtime(destination, target, package)
                        self.assertEqual((destination / "conpty/conpty.dll").read_bytes(), f"{arch} DLL".encode())
                        self.assertEqual((destination / f"conpty/{arch}/OpenConsole.exe").read_bytes(), f"{arch} host".encode())
                        self.assertEqual(
                            {p.relative_to(destination).as_posix() for p in destination.rglob("*") if p.is_file()},
                            {"conpty/conpty.dll", f"conpty/{arch}/OpenConsole.exe"},
                        )
                runtimes["x86_64-pc-windows-msvc"] = ("x64", runtimes["x86_64-pc-windows-msvc"][1], "0" * 64)
                with self.assertRaisesRegex(RuntimeError, "OpenConsole.exe"):
                    conpty_runtime.stage_runtime(root / "rejected", "x86_64-pc-windows-msvc", package)
                self.assertFalse((root / "rejected").exists())
                package.write_bytes(b"tampered package")
                with self.assertRaisesRegex(RuntimeError, "SHA-256 mismatch"):
                    conpty_runtime.stage_runtime(root / "rejected", "aarch64-pc-windows-msvc", package)
                self.assertFalse((root / "rejected").exists())


if __name__ == "__main__":
    unittest.main()
