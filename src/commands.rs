use tauri::{command, AppHandle, Runtime};

use crate::Result;
use crate::TcpclientExt;

#[command]
pub(crate) async fn connect<R: Runtime>(
    app: AppHandle<R>,
    id: String,
    endpoint: String,
) -> Result<()> {
    app.tcpclient().connect(id, endpoint).await
}

#[command]
pub(crate) async fn connect_with_bind<R: Runtime>(
    app: AppHandle<R>,
    id: String,
    local_addr: String,
    endpoint: String,
) -> Result<()> {
    app.tcpclient().connect_with_bind(id, local_addr, endpoint).await
}

#[command]
pub(crate) async fn disconnect<R: Runtime>(app: AppHandle<R>, id: String) -> Result<()> {
    app.tcpclient().disconnect(id).await
}

#[command]
pub(crate) async fn send<R: Runtime>(
    app: AppHandle<R>,
    id: String,
    data: Vec<u8>,
) -> Result<()> {
    app.tcpclient().send(id, data).await
}

#[command]
pub(crate) async fn is_connected<R: Runtime>(app: AppHandle<R>, id: String) -> bool {
    app.tcpclient().is_connected(id).await
}

#[command]
pub(crate) async fn get_connections<R: Runtime>(app: AppHandle<R>) -> Vec<String> {
    app.tcpclient().get_connections().await
}
