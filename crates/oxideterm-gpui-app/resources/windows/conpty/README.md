# Windows ConPTY runtime

`scripts/release/conpty_runtime.py` stages the pinned
`Microsoft.Windows.Console.ConPTY` package using the upstream resource layout:

- `resources/conpty/conpty.dll`
- `resources/conpty/x64/OpenConsole.exe` for x64
- `resources/conpty/arm64/OpenConsole.exe` for ARM64

The loader in `crates/alacritty-terminal/src/tty/windows/conpty.rs` requires this
bundled runtime and creates pseudoconsoles with flags `0`. Missing files or
library loading failures are startup errors. Portable in-place updates replace
these files through the `resources` entry in `portable-update.json`.

| File | Package version | SHA-256 |
|---|---|---|
| `conpty.dll` (x64) | 1.24.260710001 | `39fba2713e2495117b1591ae8c32a3b904bea7aa66069cf7815e2844c76d75d8` |
| `OpenConsole.exe` (x64) | 1.24.260710001 | `b7fd936c2668b87b9ecf7b3366dc6568afc1c6f981874cba3e955a1c35cf8160` |

The staging script selects and verifies both files for the target architecture.
The license text is `licenses/third-party/MICROSOFT-TERMINAL-LICENSE-MIT` and ships
as `MICROSOFT-TERMINAL-LICENSE-MIT`.
