use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_tcpclient);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Tcpclient<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("", "TcpclientPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_tcpclient)?;
    Ok(Tcpclient(handle))
}

/// Access to the tcpclient APIs.
pub struct Tcpclient<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Tcpclient<R> {
    /// Connect to a TCP server (mobile stub)
    pub async fn connect(&self, _id: String, _endpoint: String) -> crate::Result<()> {
        Err(crate::Error::ConnectionError(
            "TCP client not supported on mobile yet".into(),
        ))
    }

    /// Connect with bind (mobile stub)
    pub async fn connect_with_bind(
        &self,
        _id: String,
        _local_addr: String,
        _endpoint: String,
    ) -> crate::Result<()> {
        Err(crate::Error::ConnectionError(
            "TCP client not supported on mobile yet".into(),
        ))
    }

    /// Disconnect (mobile stub)
    pub async fn disconnect(&self, _id: String) -> crate::Result<()> {
        Err(crate::Error::ConnectionError(
            "TCP client not supported on mobile yet".into(),
        ))
    }

    /// Send data (mobile stub)
    pub async fn send(&self, _id: String, _data: Vec<u8>) -> crate::Result<()> {
        Err(crate::Error::ConnectionError(
            "TCP client not supported on mobile yet".into(),
        ))
    }

    /// Check if connected (mobile stub)
    pub async fn is_connected(&self, _id: String) -> bool {
        false
    }

    /// Get connections (mobile stub)
    pub async fn get_connections(&self) -> Vec<String> {
        vec![]
    }
}
