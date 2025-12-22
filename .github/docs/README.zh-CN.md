<div align="center">

# 🌟 Stelliberty

[![简体中文](https://img.shields.io/badge/简体中文-red)](README.zh-CN.md)
[![English](https://img.shields.io/badge/English-blue)](../../README.md)

![正式版](https://img.shields.io/github/v/release/Kindness-Kismet/Stelliberty?style=flat-square&label=正式版)
![最新版](https://img.shields.io/github/v/tag/Kindness-Kismet/Stelliberty?style=flat-square&label=最新版&color=orange)
![Flutter](https://img.shields.io/badge/Flutter-3.38%2B-02569B?style=flat-square&logo=flutter)
![Rust](https://img.shields.io/badge/Rust-1.91%2B-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/license-Stelliberty-green?style=flat-square)

![Windows](https://img.shields.io/badge/Windows-0078D6?style=flat-square&logo=windows11&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black)
![macOS](https://img.shields.io/badge/macOS-实验性-gray?style=flat-square&logo=apple&logoColor=white)
![Android](https://img.shields.io/badge/Android-暂不支持-lightgray?style=flat-square&logo=android&logoColor=white)

基于 Flutter 和 Rust 构建的现代跨平台 Clash 客户端
采用独特的 **MD3M**（Material Design 3 Modern）视觉风格

</div>

## 📸 应用截图

<table>
  <tr>
    <td width="50%"><img src="../../.github/screenshots/home-page.jpg" alt="主页"/></td>
    <td width="50%"><img src="../../.github/screenshots/uwp-loopback-manager.jpg" alt="UWP 回环管理器"/></td>
  </tr>
  <tr>
    <td align="center"><b>主页</b></td>
    <td align="center"><b>UWP 回环管理器</b></td>
  </tr>
</table>

---

## ✨ 特性

- 🎨 **MD3M 设计系统**：独特的 Material Design 3 Modern 风格，结合 MD3 色彩管理与磨砂玻璃效果。
- 🦀 **Rust 后端**：高性能 Rust 核心驱动，Flutter UI 呈现。
- 🌐 **多语言支持**：使用 `slang` 框架的内置国际化支持。
- 🔧 **订阅管理**：完整的订阅和覆写配置支持。
- 📊 **实时监控**：连接跟踪和流量统计。
- 🪟 **原生桌面集成**：Windows 服务、系统托盘和开机自启支持。
- 🔄 **内置 UWP 回环管理器**：管理 Windows UWP 应用的回环豁免权限（仅限 Windows）。

### 🏆 实现亮点

本应用可能是细节做得最好的 Flutter 桌面应用之一：

- ✨ **系统托盘夜间模式**：Windows 托盘图标自动适配深色/浅色主题。
- 🚀 **无闪烁启动**：最大化窗口启动不产生视觉闪烁。
- 👻 **流畅窗口切换**：显示/隐藏窗口动画无闪烁。
- 🎯 **像素级完美 UI**：精心打造的 MD3M 设计系统。

---

## 📖 用户指南

### 系统要求

- **Windows**: Windows 10/11 (x64 / arm64)
- **Linux**: 主流发行版 (x64 / arm64)
- **macOS**: 实验性

> ⚠️ **平台状态**：目前已在 Windows 和 Linux 上完整测试。macOS 支持为实验性，部分功能可能不完整。

### 下载

- **稳定版本**：[Releases](https://github.com/Kindness-Kismet/stelliberty/releases)
- **测试版本**：[预发布页面](https://github.com/Kindness-Kismet/stelliberty/releases?q=prerelease%3Atrue)（体验最新特性）

### 安装

#### Windows

##### 方式一：便携版（ZIP 压缩包）
1. 从发布页面下载 `.zip` 文件。
2. 解压到任意位置（如 `D:\Stelliberty`）。
3. 直接运行解压目录中的 `stelliberty.exe`。
4. ✅ 无需安装，开箱即用。

##### 方式二：安装程序（EXE）
1. 从发布页面下载 `.exe` 安装程序。
2. 运行安装程序并按照向导完成安装。
3. 选择安装位置（参见下方限制说明）。
4. 从桌面快捷方式启动应用。
5. ✅ 包含卸载程序和桌面快捷方式。

##### 安装目录限制
为确保安全性和稳定性，安装程序对安装路径有以下限制：
- **系统盘（通常是 C: 盘）**：
  - ✅ 允许安装到：`%LOCALAPPDATA%\Programs\*`（如 `C:\Users\用户名\AppData\Local\Programs\Stelliberty`）。
  - ❌ 禁止安装到：系统盘根目录（如 `C:\`）及其他所有路径。
- **其他盘（D:、E: 等）**：
  - ✅ 完全自由，无任何限制，可安装到根目录（如 `D:\`、`E:\Stelliberty`）。

> 💡 **建议**：为获得最佳体验，建议安装到非系统盘（如 `D:\Stelliberty`），避免潜在的权限问题。默认安装路径 `%LOCALAPPDATA%\Programs\Stelliberty` 无需特殊权限，推荐大多数用户使用。

#### Linux

##### Arch Linux (AUR)
支持架构：`x86_64`、`aarch64`
- **yay**: `yay -S stelliberty-bin`
- **paru**: `paru -S stelliberty-bin`

> AUR 软件包链接：[stelliberty-bin](https://aur.archlinux.org/packages/stelliberty-bin)

##### 便携版（ZIP 压缩包）
1. 从发布页面下载适用于您架构（`amd64` 或 `arm64`）的 `.zip` 文件。
2. 解压到任意位置（如 `~/Stelliberty`）。
3. **重要：** 为应用文件夹赋予权限：`chmod 777 -R ./stelliberty`。
4. 直接运行解压目录中的 `./stelliberty`。
5. ✅ 开箱即用。

### 故障排查

#### 端口被占用（Windows）
如果遇到端口冲突，请以**管理员身份**运行命令提示符：
1. **查找进程**：`netstat -ano | findstr :端口号`
2. **结束进程**：`taskkill /F /PID <进程ID>`

#### 软件工作不正常
- **路径要求**：路径中不应包含特殊字符（空格除外）或非 ASCII 字符（如中文）。
- **安装限制**：如需安装到 EXE 安装程序不允许的位置，请使用**便携版 ZIP 压缩包**。

#### 缺少运行库（Windows）
如果应用无法启动，请安装 **Visual C++ 运行库**：[vcredist - 运行库合集](https://gitlab.com/stdout12/vcredist)。

---

## 🛠️ 开发者与贡献者指南

### 前置条件
- **Flutter SDK** (>= 3.38)
- **Rust 工具链** (>= 1.91)
- **Dart SDK** (Flutter 自带)

### 开发流程

#### 1. 安装依赖
```bash
# 安装脚本依赖
cd scripts && dart pub get && cd ..
# 安装 rinf
cargo install rinf_cli
# 安装项目依赖
flutter pub get
```

#### 2. 生成代码
首次构建或修改 Rust/Dart 接口后，需生成桥接代码和翻译文件：
```bash
# 生成 Rust-Flutter 桥接代码
rinf gen
# 生成国际化翻译文件
dart run slang
```

#### 3. 运行开发构建
```bash
# 首次运行或资源更新后，需运行预构建脚本
dart run scripts/prebuild.dart
# 启动开发
flutter run
```

### 测试
项目内置了测试框架，用于隔离测试特定功能：
```bash
# 运行覆写规则测试
flutter run --dart-define=TEST_TYPE=override
# 运行 IPC API 测试
flutter run --dart-define=TEST_TYPE=ipc-api
```
- **测试文件**：位于 `assets/test/`，请根据 `override` 或 `ipc-api` 测试需求准备相应文件。
- **注意**：测试模式仅在 Debug 构建中可用。

### 构建项目

#### 预构建
**构建项目前必须先运行预构建脚本**，它会负责编译服务、下载核心和数据文件等。
```bash
# 运行预构建
dart run scripts/prebuild.dart
# 查看帮助
dart run scripts/prebuild.dart --help
```

#### 构建命令
使用 `scripts/build.dart` 脚本进行编译和打包：
```bash
# 构建 Release 版本（默认：ZIP）
dart run scripts/build.dart
# 同时生成安装包（ZIP + EXE/DEB/RPM/AppImage）
dart run scripts/build.dart --with-installer
# 查看所有参数
dart run scripts/build.dart --help
```
- **输出位置**：构建产物位于 `build/packages/` 目录。
- **平台支持**：Windows/Linux 已完整支持，macOS 为实验性，Android 暂不支持。

### 代码规范
- ✅ `flutter analyze` 和 `cargo clippy` 零警告。
- ✅ 提交前使用 `dart format` 和 `cargo fmt` 格式化。
- ✅ Rust 代码必须使用 `Result<T, E>`，禁止 `unwrap()`。
- ✅ Dart 代码必须保持 null safety。

### 问题反馈
1. 在 **设置 → 应用行为** 中开启 **应用日志**。
2. 重现问题后，在 `data` 目录中找到日志文件。
3. **消除隐私信息**后，在 GitHub 创建 issue 并附上日志和重现步骤。

---

## 🎨 关于 MD3M 设计

**MD3M（Material Design 3 Modern）** 是一个独特的设计系统，融合了：

- 🎨 **Material Design 3**：现代色彩系统和排版
- 🪟 **磨砂玻璃效果**：半透明背景与模糊效果
- 🌈 **系统主题集成**：自动适配系统强调色
- 🌗 **深色模式支持**：无缝的明暗主题切换

这创造了一种现代、优雅的桌面应用体验，在各平台上都具有原生般的流畅感受。

---

## 📋 代码规范

- ✅ `flutter analyze` 和 `cargo clippy` 无警告
- ✅ 提交前使用 `dart format` 和 `cargo fmt` 格式化代码
- ✅ 不要修改自动生成的文件（`lib/src/bindings/`、`lib/i18n/`）
- ✅ 使用事件驱动架构，避免滥用 `setState`
- ✅ Rust 代码必须使用 `Result<T, E>`，禁止 `unwrap()`
- ✅ Dart 代码必须保持 null safety

---

## 📄 许可证

本项目采用 **Stelliberty License（星辰自由许可证）** - 详见 [LICENSE](../../LICENSE) 文件。

**简而言之**：你可以随心所欲地使用本软件，没有任何限制，无需署名。

---

<div align="center">

由 Flutter 和 Rust 驱动

</div>