<div align="center">

# 🌟 Stelliberty

[![English](https://img.shields.io/badge/English-red)](README.md)
[![简体中文](https://img.shields.io/badge/简体中文-blue)](.github/docs/README.zh-CN.md)

![Stable Version](https://img.shields.io/github/v/release/Kindness-Kismet/Stelliberty?style=flat-square&label=Stable)
![Latest Version](https://img.shields.io/github/v/tag/Kindness-Kismet/Stelliberty?style=flat-square&label=Latest&color=orange)
![Flutter](https://img.shields.io/badge/Flutter-3.38%2B-02569B?style=flat-square&logo=flutter)
![Rust](https://img.shields.io/badge/Rust-1.91%2B-orange?style=flat-square&logo=rust)
![License](https://img.shields.io/badge/license-Stelliberty-green?style=flat-square)

![Windows](https://img.shields.io/badge/Windows-0078D6?style=flat-square&logo=windows11&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black) ![macOS](https://img.shields.io/badge/macOS-experimental-gray?style=flat-square&logo=apple&logoColor=white)
![Android](https://img.shields.io/badge/Android-not_supported-lightgray?style=flat-square&logo=android&logoColor=white)

A modern cross-platform Clash client built with Flutter and Rust
Featuring the unique **MD3M** (Material Design 3 Modern) visual style

</div>

## 📸 Screenshots

<table>
  <tr>
    <td width="50%"><img src=".github/screenshots/home-page.jpg" alt="Home Page"/></td>
    <td width="50%"><img src=".github/screenshots/uwp-loopback-manager.jpg" alt="UWP Loopback Manager"/></td>
  </tr>
  <tr>
    <td align="center"><b>Home Page</b></td>
    <td align="center"><b>UWP Loopback Manager</b></td>
  </tr>
</table>

---

## ✨ Features

- 🎨 **MD3M Design System**: Unique Material Design 3 Modern style combining MD3 color management with acrylic glass effects.
- 🦀 **Rust Backend**: High-performance core powered by Rust with Flutter UI.
- 🌐 **Multi-language Support**: Built-in i18n support using `slang`.
- 🔧 **Subscription Management**: Full subscription and override configuration support.
- 📊 **Real-time Monitoring**: Connection tracking and traffic statistics.
- 🪟 **Native Desktop Integration**: Windows service, system tray, and auto-start support.
- 🔄 **Built-in UWP Loopback Manager**: Manage Windows UWP app loopback exemptions (Windows only).

### 🏆 Implementation Highlights

This might be one of the most detail-oriented Flutter desktop applications:

- ✨ **System Tray Dark Mode**: Adaptive tray icons for Windows dark/light themes.
- 🚀 **Flicker-Free Launch**: Maximized window startup without visual artifacts.
- 👻 **Smooth Window Transitions**: Show/hide animations without flickering.
- 🎯 **Pixel-Perfect UI**: Carefully crafted MD3M design system.

---

## 📖 User Guide

### System Requirements

- **Windows**: Windows 10/11 (x64 / arm64)
- **Linux**: Mainstream distributions (x64 / arm64)
- **macOS**: Experimental

> ⚠️ **Platform Status**: Fully tested on Windows and Linux. macOS support is experimental and may have incomplete functionality.

### Downloads

- **Stable Version**: [Releases](https://github.com/Kindness-Kismet/stelliberty/releases)
- **Beta Version**: [Pre-releases](https://github.com/Kindness-Kismet/stelliberty/releases?q=prerelease%3Atrue) (latest features)

### Installation

#### Windows

##### Option 1: Portable Version (ZIP Archive)
1. Download the `.zip` file from the release page.
2. Extract to your desired location (e.g., `D:\Stelliberty`).
3. Run `stelliberty.exe` directly from the extracted folder.
4. ✅ No installation required, fully portable.

##### Option 2: Installer (EXE)
1. Download the `.exe` installer from the release page.
2. Run the installer and follow the setup wizard.
3. Choose an installation location (see restrictions below).
4. Launch the application from the desktop shortcut.
5. ✅ Includes uninstaller and desktop shortcut.

##### Installation Directory Restrictions
The installer enforces path restrictions for security and stability:
- **System Drive (Usually C:)**:
  - ✅ Allowed: `%LOCALAPPDATA%\Programs\*` (e.g., `C:\Users\YourName\AppData\Local\Programs\Stelliberty`).
  - ❌ Prohibited: System drive root and all other paths.
- **Other Drives (D:, E:, etc.)**:
  - ✅ No restrictions. Install anywhere, including root directories (e.g., `D:\`, `E:\Stelliberty`).

> 💡 **Recommendation**: For the best experience, install to a non-system drive (e.g., `D:\Stelliberty`) to avoid permission issues. The default path `%LOCALAPPDATA%\Programs\Stelliberty` is recommended for most users.

#### Linux

##### Arch Linux (AUR)
Supported architectures: `x86_64`, `aarch64`
- **yay**: `yay -S stelliberty-bin`
- **paru**: `paru -S stelliberty-bin`

> AUR Package: [stelliberty-bin](https://aur.archlinux.org/packages/stelliberty-bin)

##### Portable Version (ZIP Archive)
1. Download the `.zip` file for your architecture (`amd64` or `arm64`).
2. Extract it to your desired location (e.g., `~/Stelliberty`).
3. **Important:** Grant permissions: `chmod 777 -R ./stelliberty`.
4. Run `./stelliberty` from the extracted directory.
5. ✅ Ready to use.

### Troubleshooting

#### Port Already in Use (Windows)
If you encounter port conflicts, run Command Prompt as **Administrator**:
1. **Find Process**: `netstat -ano | findstr :<port_number>`
2. **Kill Process**: `taskkill /F /PID <process_id>`

#### Software Not Working Properly
- **Path Requirements**: The path should not contain special characters (except spaces) or non-ASCII characters.
- **Installation Restrictions**: Use the **portable ZIP version** if you need to install to a location not allowed by the EXE installer.

#### Missing Runtime Libraries (Windows)
If the application fails to start, install the **Visual C++ Runtimes**: [vcredist - Runtimes AIO](https://gitlab.com/stdout12/vcredist).

---

## 🛠️ For Developers & Contributors

### Prerequisites
- **Flutter SDK** (>= 3.38)
- **Rust toolchain** (>= 1.91)
- **Dart SDK** (included with Flutter)

### Development Workflow

#### 1. Install Dependencies
```bash
# Install script dependencies
cd scripts && dart pub get && cd ..
# Install rinf
cargo install rinf_cli
# Install project dependencies
flutter pub get
```

#### 2. Generate Code
Required before first build or after modifying Rust/Dart interfaces:
```bash
# Generate Rust-Flutter bridge code
rinf gen
# Generate i18n translation files
dart run slang
```

#### 3. Run Development Build
```bash
# Run prebuild script first if it's the first time or assets need an update
dart run scripts/prebuild.dart
# Start development
flutter run
```

### Testing
The project includes a test framework for isolated feature testing:
```bash
# Run override rule test
flutter run --dart-define=TEST_TYPE=override
# Run IPC API test
flutter run --dart-define=TEST_TYPE=ipc-api
```
- **Test Files**: Located in `assets/test/`. Prepare files based on `override` or `ipc-api` test requirements.
- **Note**: Test mode is only available in Debug builds.

### Building the Project

#### Pre-build
**Always run the pre-build script before building.** It handles service compilation, core/data file downloads, and more.
```bash
# Run pre-build
dart run scripts/prebuild.dart
# View help
dart run scripts/prebuild.dart --help
```

#### Build Command
Use the `scripts/build.dart` script to compile and package:
```bash
# Build Release version (default: ZIP)
dart run scripts/build.dart
# Build with installer (ZIP + EXE/DEB/RPM/AppImage)
dart run scripts/build.dart --with-installer
# View all options
dart run scripts/build.dart --help
```
- **Output**: Build artifacts are located in `build/packages/`.
- **Platform Support**: Windows/Linux are fully supported, macOS is experimental, and Android is not yet supported.

### Code Standards
- ✅ Zero warnings from `flutter analyze` and `cargo clippy`.
- ✅ Format with `dart format` and `cargo fmt` before committing.
- ✅ Rust code must use `Result<T, E>`; no `unwrap()`.
- ✅ Dart code must maintain null safety.

### Reporting Issues
1. Enable **Application Logging** in **Settings → App Behavior**.
2. Reproduce the issue and find the log file in the `data` directory.
3. After **removing sensitive information**, create a GitHub issue with the log and reproduction steps.

---

## 🎨 About MD3M Design

**MD3M (Material Design 3 Modern)** is a unique design system that combines:

- 🎨 **Material Design 3**: Modern color system and typography
- 🪟 **Acrylic Glass Effects**: Translucent backgrounds with blur effects
- 🌈 **System Theme Integration**: Automatically adapts to system accent colors
- 🌗 **Dark Mode Support**: Seamless light/dark theme switching

This creates a modern, elegant desktop application experience with native-like feel across all platforms.

---

## 📋 Code Standards

- ✅ No warnings from `flutter analyze` and `cargo clippy`
- ✅ Format code with `dart format` and `cargo fmt` before committing
- ✅ Do not modify auto-generated files (`lib/src/bindings/`, `lib/i18n/`)
- ✅ Use event-driven architecture, avoid `setState` abuse
- ✅ Rust code must use `Result<T, E>`, no `unwrap()`
- ✅ Dart code must maintain null safety

---

## 📄 License

This project is licensed under the **Stelliberty License** - see the [LICENSE](LICENSE) file for details.

**TL;DR**: Do whatever you want with this software. No restrictions, no attribution required.

---

<div align="center">

Powered by Flutter & Rust

</div>
