use tauri::{Runtime, WebviewWindowBuilder};
use crate::constants::MAIN_WINDOW_LABEL;

mod dashboard;

pub enum WindowType {
    Dashboard,
}

impl WindowType {
    pub fn label(&self) -> &str {
        match self {
            WindowType::Dashboard => MAIN_WINDOW_LABEL,
        }
    }

    pub fn configure<R: Runtime>(
        &self,
        builder: WebviewWindowBuilder<R>
    ) -> WebviewWindowBuilder<R> {
        match self {
            WindowType::Dashboard => dashboard::configure(builder),
        }
    }
}