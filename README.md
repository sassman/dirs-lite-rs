# dirs-lite

A minimal, dependency-free crate for getting the user's config, data, and cache directories.

## Usage

```rust
use dirs_lite::{config_dir, data_dir, cache_dir};

fn main() {
    let config = config_dir().expect("config dir");
    println!("{}", config.display());
    // Linux:   /home/alice/.config
    // macOS:   /Users/Alice/Library/Application Support
    //          /Users/Alice/.config (with `favor-xdg-style` feature)
    // Windows: C:\Users\Alice\AppData\Roaming

    let data = data_dir().expect("data dir");
    println!("{}", data.display());
    // Linux:   /home/alice/.local/share
    // macOS:   /Users/Alice/Library/Application Support
    //          /Users/Alice/.local/share (with `favor-xdg-style` feature)
    // Windows: C:\Users\Alice\AppData\Local

    let cache = cache_dir().expect("cache dir");
    println!("{}", cache.display());
    // Linux:   /home/alice/.cache
    // macOS:   /Users/Alice/Library/Caches
    //          /Users/Alice/.cache (with `favor-xdg-style` feature)
    // Windows: C:\Users\Alice\AppData\Local
}
```

## Platform Behavior

### `config_dir()`

| Platform | Path |
|----------|------|
| Linux | `$XDG_CONFIG_HOME` or `$HOME/.config` |
| macOS | `$HOME/Library/Application Support` |
| Windows | `%APPDATA%` |

### `data_dir()`

| Platform | Path |
|----------|------|
| Linux | `$XDG_DATA_HOME` or `$HOME/.local/share` |
| macOS | `$HOME/Library/Application Support` |
| Windows | `%LOCALAPPDATA%` |

### `cache_dir()`

| Platform | Path |
|----------|------|
| Linux | `$XDG_CACHE_HOME` or `$HOME/.cache` |
| macOS | `$HOME/Library/Caches` |
| Windows | `%LOCALAPPDATA%` |

## Features

- **`favor-xdg-style`** - On macOS, returns XDG-style paths (`$HOME/.config`, `$HOME/.local/share`, `$HOME/.cache`) instead of Apple paths.

## Platform Conventions

- [XDG Base Directory Specification](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html) (Linux)
- [Apple File System Programming Guide](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html) (macOS)
- [Known Folder IDs](https://docs.microsoft.com/en-us/windows/win32/shell/knownfolderid) (Windows)

## Alternatives

Need more? Consider these crates:

- [`dirs`](https://crates.io/crates/dirs) - More directory types (cache, data, etc.)
- [`directories`](https://crates.io/crates/directories) - Project-specific paths with organization support
- [`etcetera`](https://crates.io/crates/etcetera) - Strategy-based (XDG, Apple, Windows)
