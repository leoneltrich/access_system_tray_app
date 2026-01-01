use tauri::{AppHandle, Runtime, WebviewWindowBuilder};
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

    pub fn configure<'a, R: Runtime>(
        &self,
        builder: WebviewWindowBuilder<'a, R, AppHandle<R>>
    ) -> WebviewWindowBuilder<'a, R, AppHandle<R>> {
        match self {
            WindowType::Dashboard => dashboard::configure(builder),
        }
    }
}