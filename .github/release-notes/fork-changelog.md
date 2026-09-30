# Windows Fork Changelog

## 2.2.0+fork.1

[中文](#中文) | [English](#english)

### 中文

本次社区 Fork **liansishen/oxideterm** 的 Windows x64 版本升级至官方 OxideTerm 2.2.0，纳入通知与审计、加密会话录制、TOTP 动态验证码凭据和粘贴前编辑，并新增本 Fork 的终端工作区恢复。继续保留 AnalyseDeCircuit 的上游版权声明，以及本 Fork 已发布的 Windows 字体、ConPTY、通知与自定义更新功能。以下变更以 `2.1.0+fork.2` 为比较基线。

#### ✨ 上游更新

##### 通知、审计与加密录制

- 通知中心扩展为“通知与审计”，提供操作记录、会话汇总和会话录制。审计默认关闭，开启后记录后续操作；已有记录在关闭后仍可查看。
- 支持按时间、类别、级别、来源和结果筛选，查看操作耗时、退出码、传输字节数及父子操作关系。覆盖连接与认证、终端命令、文件与编辑器操作、端口转发、配置与云同步，以及 AI、MCP、插件和命令行任务；结果依据可获得的协议、Shell 集成或生命周期信息记录。
- 操作记录默认保留 90 天、512 MiB，可调整并按筛选范围导出 JSON 或 CSV 摘要。受保护详情在本机加密保存；主动选择详情导出并确认后，会生成包含命令、路径、账号或端点的明文文件。
- 新增本地、SSH、Telnet、Mosh 和串口的终端输出录制，保存输出与尺寸变化，支持播放、暂停、时间定位和倍速。录制默认关闭，开启时单独确认；默认保留 7 天、2 GiB，内容在本机加密保存并排除云同步。
- 录制采用有容量上限的队列，存储不及时会对读取施加背压，输入、取消和关闭继续处理。不采集键盘输入或终端文件传输载荷，终端显示的敏感内容仍可能进入录制。纳入 Linux 录制恢复容量后 PTY 读取停住的上游修复。

##### TOTP 凭据与 SSH 认证

- 在“设置 → 凭据”管理可复用的动态验证码凭据，支持 Base32 密钥、`otpauth://` 链接、SHA-1／SHA-256／SHA-512、6 位或 8 位验证码和自定义周期。目标 SSH 主机与各级跳板可分别绑定凭据。
- 自动填写限定于 SSH 认证阶段，识别常见验证码提示并支持自定义正则表达式；仅有一个明确匹配的隐藏输入项时填写。同轮其他问题先收集人工回答，每次认证自动提交一次，后续挑战转为人工输入。
- 密钥保存在受保护存储，连接配置只保存引用；凭据可编辑或停用，仍被连接使用时阻止删除。加密连接文件与云同步保留绑定关系，密钥仅在选择包含敏感凭据时传输。

##### 粘贴、会话和快捷命令

- 新增“编辑后粘贴”，复用现有编辑器进行多行编辑、选择和撤销，多行粘贴确认也可进入编辑。代码围栏仅在主动选择移除时清理；未修改或撤销全部修改时保留原始内容，编辑后保留来源的 CRLF 换行约定。
- 已保存的本地配置在侧栏按配置组织运行中的终端，支持展开、折叠、聚焦和继续新建。移除会话会关闭该配置对应的终端并记住隐藏状态，重新打开配置后恢复显示。（PR [AnalyseDeCircuit/oxideterm#631](https://github.com/AnalyseDeCircuit/oxideterm/pull/631)）
- 修复 Windows 将暂时没有 PTY 输出误判为结束的问题。（PR [AnalyseDeCircuit/oxideterm#631](https://github.com/AnalyseDeCircuit/oxideterm/pull/631)）
- SSH、Telnet、Mosh 和串口会话退出后保留终端与最后输出；打开会话及退出专注模式时保留侧栏可见状态。
- 快捷命令预设分组支持删除与拖动排序，删除组内的命令移入“自定义”；排序重启后保留，保存失败时保留原状态。

##### 界面、同步与兼容性

- 修复云同步预览的借用冲突、过期终端尺寸请求、相邻滚动区共享状态、替换密码的光标位置，以及连续或跨块回车导致的额外日志空行。
- 终端菜单随焦点离开关闭，串口的手动 X/Y/ZMODEM 操作集中到“二进制传输”，传输失败提示区分超时、协议和文件错误。审计列表铺满可用宽度，笔记在无背景图时使用不透明背景。
- 上游新增终端与 IDE 自定义中文／CJK 字体名称，并将 Windows 安装脚本明确编码为 UTF-8。
- 源码包含上游 Linux Nix 打包、NixOS 集成及由 Nix 管理更新的支持，以及依赖哈希、包校验与维护文档更新；本次发行资产为 Windows x64。（PR [AnalyseDeCircuit/oxideterm#623](https://github.com/AnalyseDeCircuit/oxideterm/pull/623)、[AnalyseDeCircuit/oxideterm#632](https://github.com/AnalyseDeCircuit/oxideterm/pull/632)）

#### 🛠️ Fork 自有更新

- 默认恢复上次终端工作区，重新打开本地终端和已保存 SSH 连接，恢复标签标题、分屏布局与比例、活动标签及窗格；可在设置中关闭。恢复会创建新会话，临时 SSH 连接因缺少可恢复的认证配置而跳过，已有终端输出与进程状态不会恢复。
- 保存的本地配置参与恢复与分屏，沿用其命令行环境、工作目录及连接后执行配置，并新增可选 Windows 会话标识图标。
- 内置稳定更新使用本 Fork 的清单与签名公钥；本次不提供测试版通道。自定义更新继续使用用户明确配置的仓库与公钥，帮助页面资源链接指向本 Fork。
- 补全 Windows 会话图标的来源、商标说明和发行资产哈希，纳入许可与打包校验。继续保留此前发布的内置 CJK 字体回退修复、现代 ConPTY 运行时、终端通知、标题栏合并与活动栏控制。

#### 👥 贡献者

- 感谢 @dzzzc 改进已保存本地会话的侧栏管理，并修复 Windows PTY 输出中断。（PR [AnalyseDeCircuit/oxideterm#631](https://github.com/AnalyseDeCircuit/oxideterm/pull/631)）
- 感谢 @sgnay 提供 Linux Nix 打包、NixOS 集成、更新识别、依赖校验与维护文档。（PR [AnalyseDeCircuit/oxideterm#623](https://github.com/AnalyseDeCircuit/oxideterm/pull/623)、[AnalyseDeCircuit/oxideterm#632](https://github.com/AnalyseDeCircuit/oxideterm/pull/632)）

#### 🔒 升级与验证范围

- 提供 Windows x64 安装版与便携版，使用本 Fork 已有签名更新清单。已有设置保留自定义更新来源；继续跟随本 Fork 时使用 `liansishen/oxideterm` 及下方公钥。
- 维护者已确认本次版本的 Windows 图形界面验证并批准发布。交互式安装、原位替换与重启流程未在本次准备中单独复验；发布工作流继续执行签名、安装包内容与更新清单检查。
- 审计和录制分别主动开启并独立管理保留策略；导出受保护详情的明文文件需要妥善保管。工作区恢复不保存密码、私钥或令牌。
- Fork 的支持、源码文档与问题反馈位于 [liansishen/oxideterm](https://github.com/liansishen/oxideterm)。上游功能及贡献保留官方历史与归属。

### English

This Windows x64 release of the **liansishen/oxideterm community fork** moves to official OxideTerm 2.2.0, inheriting Notification & Audit, encrypted session recordings, reusable TOTP credentials, and editing before paste, and adds the fork's terminal workspace restoration. It preserves AnalyseDeCircuit's upstream copyright and the fork's previously released Windows font, ConPTY, notification, and custom-update capabilities. Changes below are relative to `2.1.0+fork.2`.

#### ✨ Upstream changes

##### Notifications, auditing, and encrypted recordings

- Expanded the notification center into Notification & Audit, with operation records, session summaries, and recordings. Auditing is off by default and captures subsequent operations when enabled; existing records remain readable after disabling it.
- Added filters for time, category, severity, source, and outcome, with operation duration, exit codes, transferred bytes, and parent/child relationships. Coverage includes connections and authentication, terminal commands, files and editor actions, forwarding, configuration and cloud sync, and AI, MCP, plugin, and CLI tasks. Outcomes use available protocol, shell-integration, or lifecycle evidence.
- Operation records default to 90 days and 512 MiB, with configurable retention and filtered JSON or CSV summary export. Protected details remain encrypted locally; explicitly confirming a details export creates a plaintext file containing commands, paths, accounts, or endpoints.
- Added local, SSH, Telnet, Mosh, and serial output recordings with output and resize events, playback, pause, seeking, and speed controls. Recording is off by default and requires separate confirmation. Recordings default to seven days and 2 GiB, stay encrypted locally, and are excluded from cloud sync.
- Bounded recording queues apply read backpressure when storage falls behind while continuing to service input, cancellation, and close. Keyboard input and terminal file-transfer payloads are excluded; sensitive content displayed by the terminal can still be recorded. Included the upstream Linux fix for PTY reads remaining paused after recording capacity returns.

##### TOTP credentials and SSH authentication

- Added reusable TOTP credentials under Settings → Credentials, supporting Base32 secrets, `otpauth://` URIs, SHA-1/SHA-256/SHA-512, six or eight digits, and configurable periods. Target SSH hosts and individual jump hosts can bind their own credentials.
- Limited autofill to SSH authentication, with common verification prompts and custom regular expressions. Autofill requires exactly one matching hidden input. Other answers are collected manually first, each authentication attempt submits a code automatically once, and subsequent challenges use manual input.
- Kept secrets in protected storage and references in connection metadata. Credentials can be edited or disabled; deletion is blocked while connections use them. Encrypted connection archives and cloud sync preserve bindings and include secrets only when sensitive credentials are selected.

##### Paste, sessions, and quick commands

- Added Edit Before Paste with the existing editor's multiline editing, selection, and undo, including access from multiline paste confirmation. Code fences are removed only by an explicit action. Unchanged content and fully undone edits retain the original clipboard text, while edited content preserves the source's CRLF convention.
- Grouped running local terminals under persistent saved-profile sidebar entries, with expand/collapse, focus, and additional-terminal actions. Removing a session closes that profile's terminals and remembers its hidden state; reopening the profile restores visibility. (PR [AnalyseDeCircuit/oxideterm#631](https://github.com/AnalyseDeCircuit/oxideterm/pull/631))
- Fixed Windows treating temporarily empty PTY reads as session termination. (PR [AnalyseDeCircuit/oxideterm#631](https://github.com/AnalyseDeCircuit/oxideterm/pull/631))
- SSH, Telnet, Mosh, and serial panes retain their output after exit. Opening sessions and leaving Zen mode preserve sidebar visibility.
- Allowed deletion and drag reordering of preset quick-command groups. Deleted groups move their commands into Custom, ordering survives restart, and failed saves preserve the previous state.

##### Interface, synchronization, and compatibility

- Fixed cloud-sync preview borrowing conflicts, obsolete terminal resize requests, shared scrolling state, caret positioning when replacing passwords, and extra log lines from repeated or chunk-split carriage returns.
- Dismissed terminal menus on focus loss, consolidated manual serial X/Y/ZMODEM actions under Binary Transfer, and distinguished timeout, protocol, and file errors. Audit rows fill the available width, and notes use an opaque background when no image is configured.
- Added upstream custom Chinese/CJK font names for terminal and IDE settings and UTF-8 Windows installer scripts.
- Included upstream Linux Nix packaging, NixOS integration, and Nix-managed updates, with dependency-hash, package-verification, and maintenance-documentation improvements. This release's assets are Windows x64. (PR [AnalyseDeCircuit/oxideterm#623](https://github.com/AnalyseDeCircuit/oxideterm/pull/623), PR [AnalyseDeCircuit/oxideterm#632](https://github.com/AnalyseDeCircuit/oxideterm/pull/632))

#### 🛠️ Fork-specific changes

- Restored the previous terminal workspace by default, reopening local terminals and saved SSH connections with tab titles, split layout and proportions, active tab, and active pane. This can be disabled in settings. Restoration creates new sessions, skips temporary SSH connections without restorable authentication settings, and does not restore previous output or process state.
- Integrated saved local profiles with restoration and splits, retaining their shell, working directory, and post-connect configuration, and added an optional Windows session icon.
- Routed built-in stable updates through this fork's manifest and signing public key. This release offers no beta channel. Custom updates continue to use the explicitly configured repository and key, and Help resource links point to this fork.
- Completed the Windows session icon's provenance, trademark notice, and release asset hash checks. Retained the previously released bundled CJK fallback fix, modern ConPTY runtime, terminal notifications, merged title bar, and activity-bar controls.

#### 👥 Contributors

- Thank you to @dzzzc for saved local-session sidebar management and the Windows PTY output fix. (PR [AnalyseDeCircuit/oxideterm#631](https://github.com/AnalyseDeCircuit/oxideterm/pull/631))
- Thank you to @sgnay for Linux Nix packaging, NixOS integration, update detection, dependency verification, and maintenance documentation. (PR [AnalyseDeCircuit/oxideterm#623](https://github.com/AnalyseDeCircuit/oxideterm/pull/623), PR [AnalyseDeCircuit/oxideterm#632](https://github.com/AnalyseDeCircuit/oxideterm/pull/632))

#### 🔒 Upgrade and validation scope

- Provides Windows x64 setup and portable packages with this fork's existing signed update manifest. Existing settings retain the custom update source; use `liansishen/oxideterm` and the public key below to continue following this fork.
- The maintainer confirmed Windows GUI validation for this version and approved publication. Interactive installation, in-place replacement, and restart were not separately rechecked during this preparation; the release workflow continues to validate signatures, package contents, and the updater manifest.
- Auditing and recording require separate activation and have independent retention policies. Protect plaintext files exported with protected details. Workspace restoration stores no passwords, private keys, or tokens.
- Fork support, source documentation, and issue reporting are available at [liansishen/oxideterm](https://github.com/liansishen/oxideterm). Upstream capabilities and contributions retain their official history and attribution.

## 2.1.0+fork.2

### English

This Windows x64 update to the community fork **liansishen/oxideterm** fixes bundled CJK font fallback when the primary terminal font lacks Chinese glyphs. It retains the official OxideTerm 2.1.0 baseline and the upstream copyright attribution to AnalyseDeCircuit.

#### ✨ Upstream changes

- The upstream baseline remains OxideTerm 2.1.0. This release adds no upstream changes relative to `2.1.0+fork.1`.

#### 🛠️ Fork-specific changes

- Fixed Windows font fallback lookup for the bundled Maple Mono NF CN font. Selecting it as the Chinese / CJK font now includes the application's registered font collection when resolving missing glyphs in the primary font.
- Explicitly associated each fallback mapping with the collection containing the matched family. Both bundled fonts and installed system fonts participate in the configured fallback order; system defaults remain available for uncovered characters.
- Included release maintenance since the previous tag: release assets attach to the verified tag, draft validation completes before publication, and updater manifests use permanent versioned download URLs. Failed publication can reuse a verified Windows package for the same release commit.

#### 🧪 Validation and availability

- PR #10 passed the Rust workspace checks, formatting, tests, Windows native checks, and translation completeness checks. The initial Linux run had two failing terminal integration tests; both passed individually and the failed CI job passed on rerun.
- Windows GUI rendering was verified on a physical Windows installation for the reported fallback scenario. System font choices require the corresponding family to be installed; Maple Mono NF CN is bundled with the application.
- Windows x64 setup and portable packages use this fork's existing signed update manifests. Report fork-specific issues at [liansishen/oxideterm](https://github.com/liansishen/oxideterm/issues).

### 中文

本次社区 Fork **liansishen/oxideterm** 的 Windows x64 更新修复了主字体缺少中文字形时，内置 CJK 字体无法参与回退的问题。版本继续基于官方 OxideTerm 2.1.0，并保留 AnalyseDeCircuit 的上游版权声明。

#### ✨ 上游更新

- 上游基线保持为 OxideTerm 2.1.0。相较于 `2.1.0+fork.1`，本次没有新增上游变更。

#### 🛠️ Fork 自有更新

- 修复了 Windows 对内置 Maple Mono NF CN 字体的回退查找。将其选为“中文 / CJK 字体”后，解析主字体缺失的字形时会使用应用已注册的字体集合。
- 为每项回退映射显式指定包含目标字体的集合。内置字体与已安装的系统字体按配置顺序参与回退，未覆盖的字符继续使用系统默认回退。
- 纳入上一标签以来的发布维护：发布资产关联已验证的标签，草稿校验完成后再公开，更新清单使用永久的版本下载链接；发布失败时可复用同一发布提交已经验证的 Windows 安装包。

#### 🧪 验证与使用范围

- PR #10 的 Rust 工作区检查、格式检查、测试、Windows 原生检查和语言包完整性检查均已通过。首次 Linux 检查有两个终端集成测试失败，单独复测均通过，失败的 CI 作业重跑后通过。
- 已在 Windows 实机验证本次问题涉及的字体回退场景。选择系统字体时仍需安装对应字体；Maple Mono NF CN 随应用内置。
- 提供 Windows x64 安装版与便携版，沿用本 Fork 已有的签名更新清单。Fork 相关问题请提交至 [liansishen/oxideterm](https://github.com/liansishen/oxideterm/issues)。

## 2.1.0+fork.1

### English

This is the first Windows x64 release of the community fork **liansishen/oxideterm**, based on official OxideTerm 2.1.0. The upstream application and its copyright remain attributed to AnalyseDeCircuit. This release combines the upstream baseline with the fork's Windows terminal, workspace, notification, and update improvements.

#### ✨ Upstream changes

- Inherited OxideTerm 2.1.0's mixed workspaces with up to four panes per tab. Local, SSH, and Mosh terminals can share a layout with SFTP, the editor, and port forwarding, including pages connected to different hosts. Combined workspaces can move between native windows while retaining their sessions.
- Inherited reusable local terminal profiles with shell, working directory, grouping, icon, and color settings, plus session-manager and cloud-sync integration.
- Inherited theme previews, collapsible serial and tmux controls, multiline AI message editing, sustained terminal-output processing improvements, and native window and installer fixes.

#### 🛠️ Fork-specific changes

- Added a custom GitHub update channel with a configurable repository and minisign public key. Windows users can choose **Update and restart** to download, verify, and install an update. Both setup and portable packages are supported. Cancelling a download, changing its source, or failing signature verification prevents automatic installation.
- Added upstream-based fork revisions: `2.1.0+fork.1`, `2.1.0+fork.2`, and so on. Releases include signed Windows packages and a `latest.json` updater manifest, and become public after package and manifest validation.
- Added **Post-connect command** to saved local terminal profiles. The command is applied when opening the profile, including local split creation, and participates in profile storage and synchronization.
- Added a custom **Chinese / CJK font** setting while retaining the separate primary terminal font choice.
- Added optional merged title-bar/tab-bar presentation and controls for hiding or showing the left activity bar.
- Added terminal notification protocol handling and background bell notifications through the application's notification surfaces.
- Bundled a modern Windows ConPTY runtime with both setup and portable packages, and included its files in portable in-place updates. SSH ProxyCommand processes also open without a separate console window.

#### 🔒 Updating and validation

- New installations of this fork default to its configured update source. Existing saved channel choices are preserved; to follow this fork, select **Custom** in the update settings and use the repository and public key shown below.
- This release provides **Windows x64** packages. Automated checks cover compilation, related Rust behavior, packaging helpers, translations, and workflow structure. Interactive Windows installation, replacement, and restart smoke tests remain pending.
- Report fork-specific issues in [liansishen/oxideterm](https://github.com/liansishen/oxideterm/issues).

### 中文

这是社区 Fork **liansishen/oxideterm** 的首个 Windows x64 版本，基于官方 OxideTerm 2.1.0。上游程序及其版权仍归属于 AnalyseDeCircuit。本次发布整合了上游基线，以及本 Fork 的 Windows 终端、工作区、通知和更新功能改进。

#### ✨ 上游更新

- 继承 OxideTerm 2.1.0 的混合工作区：每个标签最多包含四个窗格，本地、SSH、Mosh 终端可与 SFTP、编辑器、端口转发组合，并支持不同主机的页面。组合工作区可在原生窗口之间移动并保留会话。
- 继承可重复使用的本地终端配置，包含命令行环境、工作目录、分组、图标和颜色设置，并接入会话管理器和云同步。
- 继承主题预览、可折叠的串口与 tmux 控制栏、AI 历史消息多行编辑、持续终端输出处理改进，以及原生窗口和安装器修复。

#### 🛠️ Fork 自有更新

- 新增自定义 GitHub 更新通道，可配置仓库地址和 minisign 公钥。Windows 用户点击“更新并重启”后，可自动下载、校验和安装更新，支持安装版与便携版。取消下载、切换来源或签名校验失败时，会停止自动安装。
- 新增跟随上游基线的 Fork 子版本号，例如 `2.1.0+fork.1`、`2.1.0+fork.2`。发布内容包含已签名的 Windows 安装包和 `latest.json` 更新清单，通过打包和清单检查后公开。
- 为可保存的本地终端配置新增“连接后执行”。打开配置或创建本地分屏时应用启动命令，并接入配置保存与同步。
- 为“中文 / CJK 字体”新增自定义字体设置，同时保留独立的终端主字体选择。
- 新增可选的标题栏与标签栏合并显示，以及左侧活动栏的隐藏和显示控制。
- 新增终端通知协议处理和后台响铃通知，通过程序已有通知界面呈现。
- 安装版与便携版均附带新版 Windows ConPTY 运行时，便携版原位更新也会替换这些运行时文件。SSH ProxyCommand 子进程启动时可隐藏额外的控制台窗口。

#### 🔒 更新方式与验证范围

- 全新安装默认使用本 Fork 配置的更新源；已有设置会保留原更新通道。希望跟随本 Fork 时，请在更新设置中选择“自定义”，并填写下方仓库地址与公钥。
- 本次提供 **Windows x64** 安装包。自动检查覆盖编译、相关 Rust 行为、打包脚本、语言包及工作流结构。Windows 实机安装、文件替换与重启操作仍待验证。
- Fork 相关问题请提交至 [liansishen/oxideterm](https://github.com/liansishen/oxideterm/issues)。
