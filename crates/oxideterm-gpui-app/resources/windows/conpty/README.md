# Windows ConPTY runtime

`conpty.dll` and the target-specific `OpenConsole.exe` are copied from the
`Microsoft.Windows.Console.ConPTY` package and installed next to
`oxideterm-native.exe` by `scripts/release/package_native.py`.

The loader in `crates/alacritty-terminal/src/tty/windows/conpty.rs` requires
`conpty.dll` and `OpenConsole.exe` beside the executable. It reports a startup
error if either file is missing or the library cannot be loaded; it never silently
falls back to the in-box Windows implementation.

| File | Package version | SHA-256 |
|---|---|---|
| `conpty.dll` | 1.24.260710001 | `39fba2713e2495117b1591ae8c32a3b904bea7aa66069cf7815e2844c76d75d8` |
| `OpenConsole.exe` (x64) | 1.24.260710001 | `b7fd936c2668b87b9ecf7b3366dc6568afc1c6f981874cba3e955a1c35cf8160` |

The package contains matching ARM64 `OpenConsole.exe` as well; the staging script
selects the target architecture and flattens the pair beside the executable. The
license text is `licenses/third-party/MICROSOFT-TERMINAL-LICENSE-MIT` and ships
as `MICROSOFT-TERMINAL-LICENSE-MIT`.
