# Windows Fork Changelog

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
