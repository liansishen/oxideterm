# Windows ConPTY runtime

`conpty.dll` and `x64/OpenConsole.exe` are copied from the
`Microsoft.Windows.Console.ConPTY` package and installed next to
`oxideterm-native.exe` by `scripts/release/package_native.py`.

The vendored loader in `crates/alacritty-terminal/src/tty/windows/conpty.rs`
loads `conpty.dll` when it is found in the executable directory or on `PATH`,
and otherwise falls back to the ConPTY that ships with Windows. The fallback
drops the input-mode sequences a client writes when it re-asserts terminal
state, which breaks mouse reporting in multiplexers such as Herdr after a tab or
window switch.

| File | Package version | SHA-256 |
|---|---|---|
| `conpty.dll` | 1.24.260710001 | `39fba2713e2495117b1591ae8c32a3b904bea7aa66069cf7815e2844c76d75d8` |
| `x64/OpenConsole.exe` | 1.24.260710001 | `b7fd936c2668b87b9ecf7b3366dc6568afc1c6f981874cba3e955a1c35cf8160` |

The package also carries `arm64/OpenConsole.exe`. Add it here, plus the matching
copy in the packager, before packaging an `aarch64-pc-windows-msvc` build. The
license text is `licenses/third-party/MICROSOFT-TERMINAL-LICENSE-MIT` and ships
as `MICROSOFT-TERMINAL-LICENSE-MIT`.
