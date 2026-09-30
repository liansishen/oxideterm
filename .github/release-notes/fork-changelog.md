# Windows Fork Changelog

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
