# Windows Fork Changelog

## 2.2.2+fork.1

[中文](#中文) | [English](#english)

### 中文

本次社区 Fork **liansishen/oxideterm** 的 Windows x64 版本升级至官方 OxideTerm 2.2.2，纳入 v2 插件市场、独立更新的 ACP／RDP／VNC／Mosh 插件、FIDO 安全密钥认证和动态语言支持，并改善 AI 对话、文件预览与连接交互。以下变更以 `2.2.1+fork.1` 为比较基线，继续保留 AnalyseDeCircuit 的上游版权声明。

#### ✨ 上游更新

##### 插件市场与安装

- 切换到 v2 插件市场，先加载摘要，再按需获取并校验各插件的版本历史；继续按主程序版本和平台选择最高兼容版本。
- 支持按名称、最近更新、最近上架排序，并记住选择。排序覆盖完整筛选结果后再分页；切换排序回到第一页，保留搜索和分类条件。
- 新增安装队列及逐插件状态，提供安装中、排队、失败重试和取消排队入口。安装期间仍可搜索、筛选、翻页和添加其他安装任务。
- 市场与已安装插件的展开详情显示许可证，并按插件声明提供许可证和第三方许可说明链接。
- 缺少、禁用或不兼容的 ACP、RDP、VNC、Mosh 插件会显示原因和插件管理入口，帮助恢复升级后的连接与代理使用。

##### ACP、远程桌面与 Mosh

- 将 Codex、Claude Code 的 ACP 适配器，以及 RDP、VNC 和 Mosh 的运行程序移入独立市场插件。应用继续提供原生聊天、远程桌面查看器、终端、连接配置与认证流程。
- ACP 代理直接通过标准 ACP 与应用通信，RDP／VNC 和 Mosh 使用各自专用协议。停用、更新或卸载插件会停止其相关进程和会话。
- 移除设置中的 ACP 页面；安装并启用插件后自动创建代理配置，工作目录、额外参数和环境变量仍可按需设置。
- 支持通过插件发现并启动本机已安装的 OpenCode、Antigravity、Cursor、Grok Build、GitHub Copilot CLI、Qwen Code 和 Kimi CLI；使用前需要安装对应官方命令行程序并完成登录。
- 接入 Cursor 提问选择与计划确认，覆盖单选、多选、批准、拒绝和取消。
- RDP／VNC 底栏与工作区状态栏对齐，SSH 活动会话的展开箭头与其他连接类型统一；Nix 打包同步移除已外置的远程桌面程序。

##### FIDO 安全密钥

- 接入 FIDO Security Key 插件，支持通过现有 `ed25519-sk`、`ecdsa-sk` 私钥文件完成 SSH 认证，提供原生 PIN 输入、触摸提示与取消操作。
- 签名程序按认证请求启动，通过私有管道通信并校验协议和签名；取消、超时或插件停用后停止并回收。PIN 和凭据句柄排除在设置与日志之外，临时缓冲区使用后清理。
- 使用前需通过支持 FIDO 的 OpenSSH 生成密钥并将公钥配置到服务器。插件的能力范围为签名，密钥生成由 OpenSSH 提供，设备私钥保留在设备中，PIN 每次按需输入。协议与软件签名路径已有自动化验证，真实设备的触摸、PIN 和 SSH 登录仍需硬件实测。

##### 动态语言与代码预览

- 语言插件可动态声明语言名称、扩展名和精确文件名，主程序据此识别文件并提供安装入口，支持后续语言扩展。
- 安装、启用、停用或更新语言插件后，已打开的编辑器及本地、SFTP 代码预览刷新语言支持。
- 插件可携带嵌入语法，为 Vue、Svelte 等混合语言文件分别高亮脚本、样式与标记内容。
- v2 市场提供 XML、DTD、INI、Kotlin、Dart、Nix、Julia、Vue、Svelte、Slint、AWK、jq、Justfile、Groovy、Clojure／ClojureScript、Erlang、OCaml、OCaml Interface 和 Typst 等语言插件；Lua、TOML、YAML 继续内置。插件独立发布，可用版本以市场为准。

##### AI、文件与交互修复

- AI 工具活动使用紧凑平面列表，聚合相关调用并展示状态、数量和耗时，可展开查看调用与结果。
- ACP 上下文用量使用代理报告的数据，减少过早出现的接近上限提示。动态运行观察位于固定提示前缀之后，保留最新目标、状态和时间信息。
- 开始页统一使用按钮提供连接、终端、导入和管理入口，保留最近连接与快捷键。
- 本地 PDF 预览上限提高至 100 MiB，本地音视频直接使用原文件播放；其他本地预览和 SFTP 预览沿用原有大小限制。
- 本地与 SFTP 路径栏按 Enter 默认进入输入路径；方向键明确选择候选后进入候选目录，Tab 仍可接受补全。
- 终端右键菜单按实际渲染尺寸定位，在窗口边缘调整位置，覆盖缩放显示与插件菜单。
- SSH 交互式认证弹窗使用不透明遮罩，等待输入期间暂停连接动画，减少光标和倒计时重绘触发的图形工作。

#### 🛠️ Fork 自有更新

- 本次以同步上游为主，继续保留本 Fork 的终端工作区恢复、标题栏合并、活动栏控制、本地终端连接后执行、字体回退、通知和自定义更新功能。
- 发布流程支持主程序已预设目标 Fork 版本的情况，创建发布标记提交并原子推送主分支与标签。

#### 📌 升级与验证范围

- 从旧版升级后，请在插件市场安装并启用所需的 ACP、RDP、VNC、Mosh 插件；已有连接配置继续使用。平台支持、外部程序要求和本机代码权限以插件详情及启用确认提示为准。
- FIDO 与新增动态语言能力需要 OxideTerm 2.2.2 或更高版本。插件市场 v1 已冻结，原索引和安装包继续保留；后续插件与版本更新发布到 v2。
- 本次提供 Windows x64 安装版与便携版，内置稳定更新使用本 Fork 的签名清单；自定义更新保留已有仓库与公钥设置，测试版通道继续关闭。
- 维护者已确认本次 Windows 图形界面验证并批准发布；交互式安装、原位替换与重启流程未在本次准备中单独复验。发布流程继续检查包内容、签名和更新清单。
- Fork 的支持、源码文档与问题反馈位于 [liansishen/oxideterm](https://github.com/liansishen/oxideterm)，上游能力保留官方历史与归属。

### English

This Windows x64 release of the **liansishen/oxideterm community fork** moves to official OxideTerm 2.2.2, inheriting the v2 marketplace, independently updated ACP/RDP/VNC/Mosh plugins, FIDO security-key authentication, dynamic languages, and improvements to AI conversations, file previews, and connection interactions. Changes below are relative to `2.2.1+fork.1`; AnalyseDeCircuit's upstream copyright remains intact.

#### ✨ Upstream changes

##### Marketplace and installation

- Switched to the v2 marketplace, loading summaries first and fetching and verifying individual version histories on demand. Releases remain selected for compatibility with the application and platform.
- Added sorting by name, recently updated, and newly listed, with the choice saved. Sorting covers the complete filtered result before pagination; changing it returns to the first page while preserving search and category filters.
- Added an installation queue with per-plugin installing, queued, retry, and cancel-queued states. Search, filters, pagination, and additional installation requests remain available during installation.
- Expanded marketplace and installed-plugin details show licenses and links to license text and third-party notices when declared by the plugin.
- Missing, disabled, or incompatible ACP, RDP, VNC, and Mosh plugins show reasons and a plugin-management entry to help restore connections and agent use after upgrading.

##### ACP, remote desktop, and Mosh

- Moved Codex and Claude Code ACP adapters, plus RDP, VNC, and Mosh executables, into independent marketplace plugins. The application retains native chat, remote desktop viewing, terminals, connection profiles, and authentication flows.
- ACP agents communicate directly with the application using standard ACP; RDP/VNC and Mosh use dedicated protocols. Disabling, updating, or uninstalling a plugin stops its associated processes and sessions.
- Removed the ACP settings page. Installing and enabling a plugin automatically creates its agent configuration; working directory, additional arguments, and environment variables remain optional settings.
- Plugins can discover and launch locally installed OpenCode, Antigravity, Cursor, Grok Build, GitHub Copilot CLI, Qwen Code, and Kimi CLI. Install and sign in to the corresponding official CLI before use.
- Integrated Cursor questions and plan confirmation, including single choice, multiple choice, approval, rejection, and cancellation.
- Aligned RDP/VNC footers with the workspace status bar and SSH session expansion arrows with other connection types. Nix packaging also removes the extracted remote desktop executables.

##### FIDO security keys

- Added FIDO Security Key plugin integration for SSH authentication using existing `ed25519-sk` and `ecdsa-sk` private-key files, with native PIN, touch, and cancellation prompts.
- Signing providers start on demand through private pipes with protocol and signature verification, and stop and are reaped on cancellation, timeout, or plugin retirement. PINs and credential handles are excluded from settings and logs, and temporary buffers are cleared after use.
- Generate keys with FIDO-capable OpenSSH and configure the public key on the server first. The plugin provides signing, OpenSSH handles key generation, device private keys remain on the device, and PINs are entered on demand. Protocol and software-signing paths have automated coverage; real-device touch, PIN, and SSH login still require hardware validation.

##### Dynamic languages and code previews

- Language plugins dynamically declare language names, extensions, and exact filenames, enabling file recognition and installation entries for future languages.
- Open editors and local or SFTP code previews refresh language support when plugins are installed, enabled, disabled, or updated.
- Plugins can include embedded grammars to highlight scripts, styles, and markup separately in mixed-language files such as Vue and Svelte.
- The v2 marketplace offers XML, DTD, INI, Kotlin, Dart, Nix, Julia, Vue, Svelte, Slint, AWK, jq, Justfile, Groovy, Clojure/ClojureScript, Erlang, OCaml, OCaml Interface, and Typst language plugins. Lua, TOML, and YAML remain built in. Plugins release independently; consult the marketplace for available versions.

##### AI, files, and interaction fixes

- AI tool activity uses compact flat lists grouping related calls with status, count, and duration, expandable to show calls and results.
- ACP context usage uses agent-reported data, reducing premature context-limit warnings. Dynamic runtime observations follow the stable prompt prefix and retain current targets, state, and timestamps.
- The start page uses buttons for connections, terminals, import, and management, while retaining recent connections and shortcuts.
- Raised local PDF previews to 100 MiB and enabled local audio and video playback from original files. Other local and SFTP previews retain their existing limits.
- Enter in local and SFTP path bars opens the typed path by default. Arrow-key selection explicitly chooses a completion, and Tab still accepts completion.
- Terminal context menus use their rendered size and adjust at window edges, including scaled displays and plugin menus.
- SSH interactive-authentication dialogs use opaque overlays and pause connection animations while awaiting input, reducing graphical work from cursor and countdown redraws.

#### 🛠️ Fork-specific changes

- This release focuses on upstream synchronization and retains the fork's terminal workspace restoration, merged title bar, activity-bar controls, local post-connect commands, font fallback, notifications, and custom updates.
- The release workflow supports an already selected fork version by creating a release marker commit and atomically pushing main and the tag.

#### 📌 Upgrade and validation scope

- After upgrading, install and enable the ACP, RDP, VNC, or Mosh plugins you need. Existing connection profiles remain usable. Consult plugin details and enable-time approval for platform support, external programs, and local-code permissions.
- FIDO integration and new dynamic-language capabilities require OxideTerm 2.2.2 or later. Marketplace v1 is frozen with its original index and packages retained; subsequent plugins and updates are published to v2.
- This release provides Windows x64 setup and portable packages. Built-in stable updates use this fork's signed manifest; custom updates preserve existing repository and public-key settings, and beta remains disabled.
- The maintainer confirmed Windows GUI validation and approved publication. Interactive installation, in-place replacement, and restart were not separately rechecked during this preparation. Publication continues to check package contents, signatures, and the updater manifest.
- Fork support, source documentation, and issue reporting are available at [liansishen/oxideterm](https://github.com/liansishen/oxideterm); upstream capabilities retain their official history and attribution.

#### 📥 Windows x64 downloads / Windows x64 下载

- [Setup / 安装版](https://github.com/liansishen/oxideterm/releases/download/v2.2.2%2Bfork.1/OxideTerm_2.2.2%2Bfork.1_windows_x64-setup.exe)
- [Portable ZIP / 便携版](https://github.com/liansishen/oxideterm/releases/download/v2.2.2%2Bfork.1/OxideTerm_2.2.2%2Bfork.1_windows_x64_portable.zip)
- [Checksums / 校验和](https://github.com/liansishen/oxideterm/releases/download/v2.2.2%2Bfork.1/sha256sums.txt)

Windows SmartScreen may require **More info → Run anyway**. 若 Windows SmartScreen 弹出提示，请选择 **更多信息 → 仍要运行**。

[Full changelog / 完整变更](https://github.com/liansishen/oxideterm/compare/v2.2.1%2Bfork.1...v2.2.2%2Bfork.1) · [Issues / 问题反馈](https://github.com/liansishen/oxideterm/issues) · [Source documentation / 源码文档](https://github.com/liansishen/oxideterm/tree/main/docs)

## 2.2.1+fork.1

[中文](#中文) | [English](#english)

### 中文

本次社区 Fork **liansishen/oxideterm** 的 Windows x64 版本升级至官方 OxideTerm 2.2.1，纳入插件市场、按需语言支持、原生文件预览与插件工作区，升级云同步合并和恢复机制，并修复 SSH 共享连接、终端输入与 Windows 控制台问题。本 Fork 同时适配混合分屏的终端工作区恢复，并将 ConPTY 文件布局与创建模式对齐上游。以下变更以 `2.2.0+fork.1` 为比较基线，继续保留 AnalyseDeCircuit 的上游版权声明。

#### ✨ 上游更新

##### 插件市场与按需语言支持

- 插件市场和已安装列表统一采用紧凑行布局，保留介绍、版本、状态与操作入口，展开详情减少重复说明和嵌套容器。分类来自市场索引，已安装插件也可以按分类筛选。
- 两个列表均支持分页，默认每页 10 项，可自定义每页数量并直接跳转页码；每页数量设置放在列表底部。启用、禁用等结果提示使用插件名称。
- 修复插件管理器搜索框及分页输入框获得焦点后无法输入文字的问题。
- 刷新插件列表或启用、禁用、卸载其他插件时，保留未发生变化的运行中插件入口和界面，修复 Toolbox 等插件报“标签页未声明”的问题。
- 插件入口集中排列在内置工具下方，以分隔线区分；修复插件侧栏面板声明底部位置却显示在上方的问题。插件标签页复用应用统一页头、原生控件和主题。
- 将 C、C++、C#、CSS、Common Lisp、Elixir、Go、HTML、Java、JavaScript、Objective-C、Perl、PHP、R、Ruby、Rust、Scala、Swift、TypeScript、TSX、Zig 共 21 种语言的语法支持移入独立插件；另增加 Nginx、Terraform/HCL 和 Protobuf 的识别与插件入口。
- 继续内置 Bash、Zsh、Fish、PowerShell、JSON、YAML、TOML、Markdown、Dockerfile、Make、CMake、Diff、Python、Lua、SQL 共 15 种语言。外置语言缺少插件时仍可编辑文件，并通过语言插件提示补装语法支持。
- 市场按当前主程序和平台选择最高兼容插件版本，区分可安装更新与需要升级主程序的新版。启动时重新检查已安装插件，覆盖应用升级和降级；不兼容时保留插件文件和设置。

##### 原生文件预览与插件工作区

- 本地文件管理器和 SFTP 的文件预览入口支持调用已安装的 PDF、SQLite、证书和二进制检查插件，无需为每种文件打开额外工作区。
- 本地代码预览也会显示缺少语言支持的安装入口；只读预览接入原生编辑器，并在安装语言插件后更新高亮。
- PDF 预览支持翻页；SQLite 使用原生表格展示表和分页数据，保持只读。远程数据库存在非空 WAL、回滚日志或下载期间发生变化时会拒绝预览，避免展示不完整的数据库副本。
- 证书预览支持浏览同一文件中的多个证书，分别展示有效期和详细字段；有效期不等同于系统信任或吊销检查。二进制检查可展示格式、架构和节区，并结合现有十六进制预览定位文件偏移。
- 关闭预览会取消相关后台任务并释放远程临时文件，继续使用共享 SSH 连接的终端和其他消费者不受影响。进程插件沿用启用时的本机代码信任确认。
- 为 Workspace Dashboard 提供工作区状态摘要和跳转能力，可聚合最近连接、活动标签页、录制状态、连接与插件问题，并返回对应应用页面；终端正文和诊断内容不包含在摘要中。
- 为 Tailscale、Ansible 等主机来源插件提供原生连接表单入口：发现的主机信息交由用户检查并配置认证，再保存或连接。
- 为 Toolbox 等文本工具提供多行输入、只读结果、复制及连续处理能力。终端右键入口在点击时捕获选区，切换标签后仍使用当时选择的内容；结果不会自动发送回终端。

##### 云同步合并与恢复

- 云同步改为汇总各设备的独立加密快照，自动合并互不冲突的修改；并发冲突保留候选值，由用户明确选择。未纳入同步的资源不会因此从云端删除。
- 新增同步应用中断后的恢复记录，协调连接、转发、设置、快捷命令、插件数据和凭据变更。上传重试复用已准备的数据，上传期间的新本地修改继续保留待同步状态。
- 桌面、命令行和 MCP 使用同一同步流程；命令行支持预览合并结果后再应用。旧云端数据可导入新机制，原对象保留，旧版 `.oxide` 文件仍可读取。
- 修复旧备份和旧云同步数据升级时丢失应用主题选择的问题。

##### 主题与工作区

- 应用界面和终端配色可以分别选择，并在设置中通过同一个工作区预览查看组合效果；透明度在接近不透明的区间提供更细调整。（PR [AnalyseDeCircuit/oxideterm#650](https://github.com/AnalyseDeCircuit/oxideterm/pull/650)）
- 终端使用当前配色响应颜色查询和明暗外观请求；SSH 主题更新通过队列发送，避免阻塞界面。（PR [AnalyseDeCircuit/oxideterm#650](https://github.com/AnalyseDeCircuit/oxideterm/pull/650)）
- 各类工作区页面均可放入分屏标签布局。IDE 文件树支持修饰键多选、连续选择和对应批量操作，底栏高度与其他工作区统一。
- 命令面板可以找到已保存的本地终端和 WSL 配置；启动失败会在工作区显示原因，列表悬停状态也能正确刷新。（PR [AnalyseDeCircuit/oxideterm#649](https://github.com/AnalyseDeCircuit/oxideterm/pull/649)）

##### 连接、传输与终端修复

- 修复 SFTP 上传文件夹时共享 SSH 传输可能被关闭、连带断开终端的问题，完善底层通道流控和断开诊断。
- 恢复已保存连接的认证选择，并仅在认证成功后持久化用户确认的凭据，确保保存目标对应实际完成认证的连接。（PR [AnalyseDeCircuit/oxideterm#647](https://github.com/AnalyseDeCircuit/oxideterm/pull/647)）
- 修复 JumpServer 等回显验证码提示下的 TOTP 自动填写，以及便携模式从错误目录查找用户 SSH 配置的问题。
- trzsz 和 ZMODEM 下载正确执行用户选择的覆盖行为。
- 修复双向文本中的宽字符网格定位、输入法候选位置与预编辑颜色、触控板滚动增量，以及括号粘贴中的换行保留。（PR [AnalyseDeCircuit/oxideterm#651](https://github.com/AnalyseDeCircuit/oxideterm/pull/651)）
- Windows 修复 ConPTY 管道就绪与空读取处理、Ctrl+J 身份以及按键释放与已发送按下事件的配对。（PR [AnalyseDeCircuit/oxideterm#648](https://github.com/AnalyseDeCircuit/oxideterm/pull/648)）
- 修复窗口宽度或时间戳栏变化时，未修改的终端行被重新标记为当前时间的问题；保留对有样式空白的实际内容变化识别。（PR [AnalyseDeCircuit/oxideterm#655](https://github.com/AnalyseDeCircuit/oxideterm/pull/655)）
- 更新原生界面依赖与平台适配；Windows 发布构建预编译模糊效果着色器，安装包校验覆盖正常安装与分阶段更新的两份 ConPTY 载荷。

#### 🛠️ Fork 自有更新

- 适配上游的工具页面与终端混合分屏：保存终端工作区时跳过没有终端的页面子树，保留可恢复终端的身份、顺序与分屏比例，避免混合页面导致整个终端布局保存失败。恢复仍会创建新终端会话，工具页面、已有终端输出与进程状态不在恢复范围内。
- 将本 Fork 的 ConPTY 运行时布局对齐上游 `resources/conpty`，使用默认伪控制台创建模式；便携版原位更新通过 `resources` 条目替换运行时，统一安装版、便携包与更新路径。
- 保留本 Fork 的标题栏合并、活动栏控制、本地终端连接后执行、字体回退、通知与自定义更新功能；同步构建流程使用 Rust 1.97.0，并纳入工具链文件变化检测。

#### 👥 贡献者

- @stabey：贡献应用与终端主题分离、本地和 WSL 配置查找、SSH 认证恢复，以及终端输入和 Windows 控制台修复。（PR [AnalyseDeCircuit/oxideterm#647](https://github.com/AnalyseDeCircuit/oxideterm/pull/647)、PR [AnalyseDeCircuit/oxideterm#648](https://github.com/AnalyseDeCircuit/oxideterm/pull/648)、PR [AnalyseDeCircuit/oxideterm#649](https://github.com/AnalyseDeCircuit/oxideterm/pull/649)、PR [AnalyseDeCircuit/oxideterm#650](https://github.com/AnalyseDeCircuit/oxideterm/pull/650)、PR [AnalyseDeCircuit/oxideterm#651](https://github.com/AnalyseDeCircuit/oxideterm/pull/651)）
- @m00nLi：修复终端行时间戳随网格宽度变化而错误更新的问题。（PR [AnalyseDeCircuit/oxideterm#655](https://github.com/AnalyseDeCircuit/oxideterm/pull/655)）

#### 📌 升级与验证范围

- 外置语言及本次新增宿主能力对应的插件需要 OxideTerm 2.2.1 或更高版本，通过插件市场按需安装；插件独立发布，具体平台支持和运行依赖以插件详情为准。
- 多设备同步请将参与设备统一升级。新版本升级旧云端数据后使用新的同步机制，不再向旧机制回写；HTTP JSON 自托管后端需要支持新的对象列表和删除接口。
- RDP、VNC 本版继续内置。本次提供 Windows x64 安装版与便携版，内置稳定更新指向本 Fork 的签名清单；自定义更新保留已有仓库与公钥设置，测试版通道继续关闭。
- 维护者已确认本次 Windows 图形界面验证并批准发布；交互式安装、原位替换与重启流程未在本次准备中单独复验。发布流程继续检查包内容、签名和更新清单。
- Fork 的支持、源码文档与问题反馈位于 [liansishen/oxideterm](https://github.com/liansishen/oxideterm)，上游能力与贡献保留官方历史和归属。

### English

This Windows x64 release of the **liansishen/oxideterm community fork** moves to official OxideTerm 2.2.1, inheriting the improved marketplace, on-demand languages, native previews and plugin workspaces, cloud-sync merging and recovery, and fixes for shared SSH connections, terminal input, and Windows consoles. The fork also adapts terminal restoration to mixed split layouts and aligns ConPTY layout and creation mode with upstream. Changes below are relative to `2.2.0+fork.1`; AnalyseDeCircuit’s upstream copyright remains intact.

#### ✨ Upstream changes

##### Plugin marketplace and on-demand languages

- The marketplace and installed-plugin pages use compact rows with descriptions, versions, status, and actions. Expanded details remove repeated descriptions and nested containers. Categories come from the catalog and also filter installed plugins.
- Both lists support pagination, defaulting to 10 entries per page, with a custom page size and direct page navigation. Page-size controls appear below the list, and enable/disable notifications use plugin names.
- Fixed focused search and pagination fields in the plugin manager not accepting text input.
- Refreshing the plugin list or enabling, disabling, or uninstalling another plugin preserves unchanged running plugins' navigation entries and views, fixing undeclared-tab errors in plugins such as Toolbox.
- Plugin navigation entries are grouped below built-in tools with a separator. Plugin sidebar panels now honor their declared bottom position. Plugin tabs reuse the application's page headers, native controls, and themes.
- Moved syntax support for 21 languages into independent plugins: C, C++, C#, CSS, Common Lisp, Elixir, Go, HTML, Java, JavaScript, Objective-C, Perl, PHP, R, Ruby, Rust, Scala, Swift, TypeScript, TSX, and Zig. Added file recognition and plugin entry points for Nginx, Terraform/HCL, and Protobuf.
- Kept 15 languages built in: Bash, Zsh, Fish, PowerShell, JSON, YAML, TOML, Markdown, Dockerfile, Make, CMake, Diff, Python, Lua, and SQL. Files remain editable when an external language plugin is missing, with a prompt to install syntax support.
- The marketplace selects the highest plugin version compatible with the host and platform, distinguishing available updates from newer releases that require an application upgrade. Installed plugins are checked again at startup after upgrades or downgrades; incompatible plugins retain their files and settings.

##### Native file previews and plugin workspaces

- Local file-manager and SFTP previews can use installed PDF, SQLite, certificate, and binary-inspection plugins through the existing preview entry point.
- Local code previews also offer installation of missing language support. Read-only previews use the native editor and update highlighting after a language plugin is installed.
- PDF previews support page navigation. SQLite previews provide read-only native tables with table selection and pagination. Remote databases with nonempty WAL or rollback journals, or changes during download, are rejected to avoid displaying an incomplete copy.
- Certificate previews browse multiple certificates in one file and show validity dates separately from trust or revocation verification. Binary inspection displays format, architecture, and sections, with offset navigation through the existing hexadecimal preview.
- Closing a preview cancels its background work and releases remote temporary files without disconnecting other consumers of the shared SSH connection. Process plugins retain the existing enable-time approval for trusted local code.
- Added workspace summaries and navigation for Workspace Dashboard, supporting recent connections, active tabs, recording states, connection and plugin issues, and links back to application pages. Summaries exclude terminal content and diagnostic text.
- Host-source plugins such as Tailscale and Ansible can open the native connection form with discovered host metadata for users to review, configure authentication, and then save or connect.
- Text tools such as Toolbox can use multiline input, read-only results, copying, and chained operations. Terminal context-menu actions capture the selection at click time, preserve it across tab changes, and never send results back to the terminal automatically.

##### Cloud-sync merging and recovery

- Cloud sync now combines independent encrypted snapshots from participating devices, automatically merging nonconflicting edits and retaining concurrent candidates for explicit resolution. Excluding a resource from sync does not delete it remotely.
- Added recovery records for interrupted application of connection, forwarding, settings, Quick Command, plugin, and credential changes. Upload retries reuse prepared data, while edits made during upload remain pending for the next sync.
- Desktop, CLI, and MCP share the synchronization flow, with CLI support for previewing merges before applying them. Legacy cloud data can be imported into the new mechanism without deleting the original objects, and older `.oxide` files remain readable.
- Fixed application-theme preservation when upgrading legacy backups and cloud-sync data.

##### Themes and workspaces

- Application and terminal themes can be selected independently and viewed together in a workspace preview. Opacity offers finer adjustment near the opaque end of the slider. (PR [AnalyseDeCircuit/oxideterm#650](https://github.com/AnalyseDeCircuit/oxideterm/pull/650))
- Terminal color and appearance queries use the active palette. SSH palette updates are queued to avoid blocking the interface. (PR [AnalyseDeCircuit/oxideterm#650](https://github.com/AnalyseDeCircuit/oxideterm/pull/650))
- Workspace pages can share split-tab layouts. The IDE file tree supports modifier-key multiselection, range selection, and related batch actions, with a bottom bar aligned to other workspaces.
- The command palette finds saved local-terminal and WSL profiles. Launch failures surface in the workspace, and hovered list rows refresh correctly. (PR [AnalyseDeCircuit/oxideterm#649](https://github.com/AnalyseDeCircuit/oxideterm/pull/649))

##### Connection, transfer, and terminal fixes

- Fixed shared SSH transport shutdown during SFTP directory uploads, which could also disconnect the terminal, with improvements to channel flow control and disconnect diagnostics.
- Restored saved authentication choices and persisted confirmed credentials only after successful authentication, against the connection that actually authenticated. (PR [AnalyseDeCircuit/oxideterm#647](https://github.com/AnalyseDeCircuit/oxideterm/pull/647))
- Fixed TOTP autofill for echoed verification prompts such as JumpServer's, and SSH configuration discovery from the wrong directory in portable mode.
- trzsz and ZMODEM downloads honor the requested overwrite behavior.
- Fixed wide-glyph placement in bidirectional rows, IME caret positioning and preedit colors, touchpad scroll accumulation, and newline preservation in bracketed paste. (PR [AnalyseDeCircuit/oxideterm#651](https://github.com/AnalyseDeCircuit/oxideterm/pull/651))
- Fixed Windows ConPTY pipe readiness, empty reads, Ctrl+J identity, and key-release pairing with delivered key presses. (PR [AnalyseDeCircuit/oxideterm#648](https://github.com/AnalyseDeCircuit/oxideterm/pull/648))
- Fixed unchanged terminal rows receiving new timestamps when the window or timestamp gutter changes width, while preserving detection of meaningful styled-blank changes. (PR [AnalyseDeCircuit/oxideterm#655](https://github.com/AnalyseDeCircuit/oxideterm/pull/655))
- Updated native UI dependencies and platform integration. Windows release builds precompile blur shaders, and installer checks cover ConPTY payloads for both normal installation and staged updates.

#### 🛠️ Fork-specific changes

- Adapted terminal workspace restoration to upstream mixed tool-page and terminal splits. Saving skips page-only subtrees while preserving restorable terminal identities, order, and split proportions, preventing tool pages from blocking the entire terminal snapshot. Restoration still creates new terminal sessions; tool pages, previous output, and process state remain outside its scope.
- Aligned this fork’s ConPTY runtime with upstream’s `resources/conpty` layout and default pseudoconsole creation mode. Portable in-place updates replace the runtime through the `resources` entry, keeping setup, portable packages, and updates consistent.
- Retained the fork’s merged title bar, activity-bar controls, local post-connect commands, font fallback, notifications, and custom updates. Build workflows use Rust 1.97.0 and detect toolchain-file changes.

#### 👥 Contributors

- @stabey contributed independent application and terminal themes, saved local and WSL profile discovery, SSH authentication restoration, terminal input fixes, and Windows console fixes. (PR [AnalyseDeCircuit/oxideterm#647](https://github.com/AnalyseDeCircuit/oxideterm/pull/647), PR [AnalyseDeCircuit/oxideterm#648](https://github.com/AnalyseDeCircuit/oxideterm/pull/648), PR [AnalyseDeCircuit/oxideterm#649](https://github.com/AnalyseDeCircuit/oxideterm/pull/649), PR [AnalyseDeCircuit/oxideterm#650](https://github.com/AnalyseDeCircuit/oxideterm/pull/650), PR [AnalyseDeCircuit/oxideterm#651](https://github.com/AnalyseDeCircuit/oxideterm/pull/651))
- @m00nLi fixed terminal row timestamps changing when the grid width changes. (PR [AnalyseDeCircuit/oxideterm#655](https://github.com/AnalyseDeCircuit/oxideterm/pull/655))

#### 📌 Upgrade and validation scope

- External language plugins and plugins using the new host capabilities require OxideTerm 2.2.1 or later and are installed separately through the marketplace. Consult each plugin's details for supported platforms and runtime dependencies.
- Upgrade all participating devices for multi-device sync. After importing legacy cloud data, the new client uses the new synchronization mechanism and does not write back to the legacy one. Self-hosted HTTP JSON backends must support the new object-listing and deletion endpoints.
- RDP and VNC remain bundled. This release provides Windows x64 setup and portable packages. Built-in stable updates use this fork’s signed manifest; custom updates preserve existing repository and public-key settings, and beta remains disabled.
- The maintainer confirmed Windows GUI validation for this version and approved publication. Interactive installation, in-place replacement, and restart were not separately rechecked during this preparation. Publication continues to check package contents, signatures, and the updater manifest.
- Fork support, source documentation, and issue reporting are available at [liansishen/oxideterm](https://github.com/liansishen/oxideterm); upstream capabilities and contributions retain their official history and attribution.

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
