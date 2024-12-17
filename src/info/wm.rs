use super::ModuleVar;
use crate::config::file::WindowManagerConfig;
use crate::error::LazyfetchError;

pub struct WindowManager;

impl ModuleVar<WindowManagerConfig> for WindowManager {
    fn name(self) -> String {
        String::from("wm")
    }

    // NOTE: This isn't tested on all distros, so this may fail
    // Also i will refactor this to use something else, seems to fail on too much s
    #[cfg(target_os = "linux")]
    fn value(self, cfg: Option<&WindowManagerConfig>) -> Result<String, LazyfetchError> {
        let config = cfg.unwrap();

        let desktop = {
            let mut desktop = None;
            if let Ok(current_desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
                desktop = Some(current_desktop);
            }

            if let Ok(current_desktop) = std::env::var("DESKTOP_SESSION") {
                desktop = Some(current_desktop);
            }

            if let Ok(current_desktop) = std::env::var("XDG_SESSION_DESKTOP") {
                desktop = Some(current_desktop);
            }

            desktop
        };
        let Some(wm) = desktop else {
            return Err(LazyfetchError::Custom(
                "Can't detect which window manager is being used".to_owned(),
            ));
        };

        if config.show_backend.unwrap_or(true) {
            let session_type = std::env::var("XDG_SESSION_TYPE")?;

            return Ok(format!("{} ({})", wm, session_type));
        };

        Ok(wm)
    }
}
