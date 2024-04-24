# Stack

|                | Tauri       | Electron    |
| -------------- | ----------- | ----------- |
| Backend        | Rust        | Node.js     |
| Frontend       | HTML/CSS/JS | HTML/CSS/JS |
| Bundler        | Vite        | Webpack     |
| Cross-platform | Yes         | Yes         |

# API

|                      | Tauri           | Electron |
| -------------------- | --------------- | -------- |
| Communicates over    | IPC             | ?        |
| Custom commands      | Yes             | ?        |
| Async commands       | Yes             | ?        |
| Error handling       | Yes             | ?        |
| Input / output types | JSON (serde)    | ?        |
| Custom / branded CLI | Yes             | ?        |
| Event system         | MPMC (channels) | ?        |
| Context menus        | Yes             | ?        |
| Multiple windows     | Yes             | ?        |
| System tray          | Yes             | ?        |

# Development

|                          | Tauri | Electron |
| ------------------------ | ----- | -------- |
| Developer tools          | Yes   | ?        |
| Can inspect elements     | Yes   | ?        |
| Can debug using gdb/lldb | Yes   | ?        |
| Hot module reloading     | Yes   | ?        |

# Testing

|                         | Tauri       | Electron |
| ----------------------- | ----------- | -------- |
| Testing framework       | Vitest      | ?        |
| Can mock Tauri commands | Yes         | ?        |
| WebDriver support       | Yes (alpha) | ?        |

# Building

|                              | Tauri                        | Electron |
| ---------------------------- | ---------------------------- | -------- |
| Windows supported installers | .msi, .exe                   | ?        |
| Windows requirement          | >= v10, v7 (with hacks)      | ?        |
| macOS supported installers   | .app, .dmg                   | ?        |
| macOS requirement            | >= v10.13                    | ?        |
| Linux supported installers   | .deb, .AppImage              | ?        |
| Linux requirement            | Ubuntu 18 (for lowest glibc) | ?        |
| Can build in CI              | Yes: tauri-apps/tauri-action | ?        |
| Can embed external binaries  | Yes                          | ?        |
| Self-updater for installers  | .msi, .exe, .app, .AppImage  | ?        |
