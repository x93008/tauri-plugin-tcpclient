use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Tcpclient;
#[cfg(mobile)]
use mobile::Tcpclient;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the tcpclient APIs.
pub trait TcpclientExt<R: Runtime> {
    fn tcpclient(&self) -> &Tcpclient<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TcpclientExt<R> for T {
    fn tcpclient(&self) -> &Tcpclient<R> {
        self.state::<Tcpclient<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("tcpclient")
        .invoke_handler(tauri::generate_handler![
            commands::connect,
            commands::connect_with_bind,
            commands::disconnect,
            commands::send,
            commands::is_connected,
            commands::get_connections,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let tcpclient = mobile::init(app, api)?;
            #[cfg(desktop)]
            let tcpclient = desktop::init(app, api)?;
            app.manage(tcpclient);
            Ok(())
        })
        .build()
}
