# Stack

|                | Tauri       | Electron                |
| -------------- | ----------- | ----------------------- |
| Backend        | Rust        | Node.js                 |
| Frontend       | HTML/CSS/JS | HTML/CSS/JS             |
| Bundler        | Vite        | Vite/Webpack            |
| Cross-platform | Yes         | Yes (requires forge)    |
| ESM support    | Yes         | Partial / context aware |

# Features

|                      | Tauri           | Electron |
| -------------------- | --------------- | -------- |
| Communicates over    | IPC             | IPC      |
| Custom commands      | Yes             | No       |
| Async commands       | Yes             | No       |
| Error handling utils | Yes             | No       |
| Input / output types | JSON (serde)    | N/A      |
| Custom / branded CLI | Yes             | No       |
| Event system         | MPMC (channels) | IPC      |
| Context menus        | Yes             | Yes      |
| Multiple windows     | Yes             | Yes      |
| System tray          | Yes             | Yes      |
| Accessibility        | No              | Yes      |
| Crash reports        | No              | Yes      |
| Deep links           | Yes             | Yes      |

# APIs

|                    | Tauri             | Electron      |
| ------------------ | ----------------- | ------------- |
| clipboard          | Yes (basic)       | Yes           |
| dialog             | Yes               | Yes           |
| event              | Yes               | Yes (via IPC) |
| fs                 | Yes (whitelisted) | No            |
| http/fetch         | Yes (whitelisted) | Yes           |
| keyboard shortcuts | Yes               | Yes           |
| notifications      | Yes               | Yes           |
| processes          | Yes               | Yes           |
| protocols          | No                | Yes           |
| paths              | Yes               | No            |

# Development

|                                    | Tauri | Electron |
| ---------------------------------- | ----- | -------- |
| Developer tools                    | Yes   | Yes      |
| Can inspect elements (via context) | Yes   | No       |
| Can debug using gdb/lldb           | Yes   | No       |
| Hot module reloading               | Yes   | Yes      |

# Testing

|                                  | Tauri       | Electron |
| -------------------------------- | ----------- | -------- |
| Testing framework                | Vitest      | N/A      |
| Can mock Tauri/Electron commands | Yes         | Sort of  |
| WebDriver support                | Yes (alpha) | Yes      |

# Building

|                              | Tauri                        | Electron             |
| ---------------------------- | ---------------------------- | -------------------- |
| Windows supported installers | .msi, .exe                   | .msi, .exe, .appx    |
| Windows requirement          | >= v10, v7 (with hacks)      |                      |
| macOS supported installers   | .app, .dmg                   | .dmg, .pkg           |
| macOS requirement            | >= v10.13                    |                      |
| Linux supported installers   | .deb, .AppImage              | .deb, .flatpak, .rpm |
| Linux requirement            | Ubuntu 18 (for lowest glibc) |                      |
| CI                           | Yes: tauri-apps/tauri-action | Yes (via forge)      |
| CD                           | No                           | Yes (via forge)      |
| Can embed external binaries  | Yes                          | No                   |
| Self-updater for installers  | .msi, .exe, .app, .AppImage  | Yes (via GH release) |
| Code signing                 | Yes                          | Yes                  |

# Tauri

## Pros

- Small file size.
- Rust on the backend, which is faster and more secure.
- Because Rust is being used, if we ever want to run moon/proto/oxc/rolldown/etc directly without a child process, we can execute it natively in Rust.
- Clear separation of frontend and backend code.
- Has an isolated mode (https://tauri.app/v1/references/architecture/inter-process-communication/isolation) that will be required if we want to support 3rd party tools.
- Plugins: https://github.com/tauri-apps/plugins-workspace

## Cons

- Relatively new compared to Electron. May have more bugs, or missing functionality.

# Electron

## Pros

- JavaScript on front and back, which is useful if we want to share code.

## Cons

- Massive file size.
- Complicated local development/build system. Lot's of layers.
- Separation between frontend and backend code is not clear/can be very muddy.
