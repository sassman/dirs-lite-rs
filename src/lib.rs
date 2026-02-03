use std::env;
use std::path::PathBuf;

const CONFIG_DIR: &str = ".config";

/// Returns the path to the user's config directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value from the following table, or a `None`.
///
/// |Platform | Value                                 | Example                                  |
/// | ------- | ------------------------------------- | ---------------------------------------- |
/// | Linux   | `$XDG_CONFIG_HOME` or `$HOME`/.config | /home/alice/.config                      |
/// | macOS   | `$HOME`/Library/Application Support   | /Users/Alice/Library/Application Support |
/// | Windows | `%APPDATA%`\Roaming                 | C:\Users\Alice\AppData\Roaming           |
///
/// NOTE: if the feature `favor-xdg-style` is enabled, `$HOME/.config` is favorized.
pub fn config_dir() -> Option<PathBuf> {
    if cfg!(target_os = "linux") {
        // Linux: Use $HOME/.config
        env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(std::env::home_dir)
            .map(|mut base| {
                base.push(CONFIG_DIR);
                base
            })
    } else if cfg!(target_os = "macos") {
        // macOS: Use $HOME/Library/Application Support
        //  or $HOME/.config if favor-xdg-style is enabled
        std::env::home_dir().map(|mut home| {
            if cfg!(feature = "favor-xdg-style") {
                home.push(CONFIG_DIR);
                return home;
            }
            home.push("Library");
            home.push("Application Support");
            home
        })
    } else if cfg!(target_os = "windows") {
        // Windows: Use %APPDATA%
        env::var("APPDATA").ok().map(PathBuf::from)
    } else {
        // Unsupported platform
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn set_var(key: &str, value: &str) {
        unsafe { env::set_var(key, value) };
    }

    unsafe fn remove_var(key: &str) {
        unsafe { env::remove_var(key) };
    }

    fn restore_var(key: &str, original: Option<String>) {
        // SAFETY: Tests run single-threaded with --test-threads=1
        unsafe {
            match original {
                Some(val) => set_var(key, &val),
                None => remove_var(key),
            }
        }
    }

    #[test]
    fn config_dir_returns_some() {
        let result = config_dir();
        assert!(
            result.is_some(),
            "config_dir should return Some on supported platforms"
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn linux_uses_xdg_config_home_when_set() {
        let original = env::var("XDG_CONFIG_HOME").ok();
        // SAFETY: Tests run single-threaded with --test-threads=1
        unsafe { set_var("XDG_CONFIG_HOME", "/custom/config") };

        let result = config_dir();
        assert_eq!(result, Some(PathBuf::from("/custom/config/.config")));

        restore_var("XDG_CONFIG_HOME", original);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn linux_falls_back_to_home_when_xdg_unset() {
        let original_xdg = env::var("XDG_CONFIG_HOME").ok();
        let original_home = env::var("HOME").ok();

        // SAFETY: Tests run single-threaded with --test-threads=1
        unsafe {
            remove_var("XDG_CONFIG_HOME");
            set_var("HOME", "/home/testuser");
        }

        let result = config_dir();
        assert_eq!(result, Some(PathBuf::from("/home/testuser/.config")));

        restore_var("XDG_CONFIG_HOME", original_xdg);
        restore_var("HOME", original_home);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn macos_uses_library_application_support() {
        let original = env::var("HOME").ok();
        // SAFETY: Tests run single-threaded with --test-threads=1
        unsafe { set_var("HOME", "/Users/testuser") };

        let result = config_dir();
        assert_eq!(
            result,
            Some(PathBuf::from("/Users/testuser/Library/Application Support"))
        );

        restore_var("HOME", original);
    }

    #[test]
    #[cfg(all(target_os = "macos", feature = "favor-xdg-style"))]
    fn macos_uses_library_application_support() {
        let original = env::var("HOME").ok();
        // SAFETY: Tests run single-threaded with --test-threads=1
        unsafe { set_var("HOME", "/Users/testuser") };

        let result = config_dir();
        assert_eq!(result, Some(PathBuf::from("/Users/testuser/.config")));

        restore_var("HOME", original);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn windows_uses_appdata() {
        let original = env::var("APPDATA").ok();
        // SAFETY: Tests run single-threaded with --test-threads=1
        unsafe { set_var("APPDATA", "C:\\Users\\testuser\\AppData\\Roaming") };

        let result = config_dir();
        assert_eq!(
            result,
            Some(PathBuf::from("C:\\Users\\testuser\\AppData\\Roaming"))
        );

        restore_var("APPDATA", original);
    }

    #[test]
    fn config_dir_path_is_absolute() {
        let result = config_dir();
        if let Some(path) = result {
            assert!(
                path.is_absolute(),
                "config_dir should return an absolute path"
            );
        }
    }
}
