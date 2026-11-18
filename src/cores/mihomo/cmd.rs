use crate::cores::cores_trait::Cores;
use crate::error::Error;
use std::path::Path;
use std::process::Stdio;
use tokio::net::TcpStream;
use tokio::process::{Child, Command};
pub struct MihomoCmd {
    pub nodes_list: Vec<String>,
    pub child: Option<Child>,
    pub http: reqwest::Client,
    pub is_on: bool,
}
impl Cores for MihomoCmd {
    async fn start(
        &mut self,
        exe_path: &Path,
        config_path: &Path,
        ctl: String,
    ) -> Result<(), Error> {
        let mut cmd = Command::new(exe_path);
        cmd.arg("-d")
            .arg(config_path)
            .arg("-ext-ctl")
            .arg(&ctl) // 固定 external controller 地址，别用默认值碰运气
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        let mut child = cmd
            .spawn()
            .map_err(|e| Error::new(format!("启动 mihomo 失败（{}）: {e}", exe_path.display())))?;

        loop {
            if let Some(status) = child.try_wait().map_err(|e| Error::new(e.to_string()))? {
                return Err(Error::new(format!(
                    "mihomo 启动后立即退出: {status}（见日志）"
                )));
            }
            if TcpStream::connect(&ctl).await.is_ok() {
                break;
            }
        }
        self.child = Some(child);
        self.is_on = true;
        Ok(())
    }
    fn stop(&self) -> Result<(), Error> {
        Ok(())
    }
    fn switch_node(&self, node_name: String) -> Result<(), Error> {
        Ok(())
    }
    fn get_nodes(&self) -> Result<(), Error> {
        Ok(())
    }
    fn test_delay(&self) -> Result<(), Error> {
        Ok(())
    }
    fn tun_on(&self) -> Result<(), Error> {
        Ok(())
    }
    fn load(&mut self, path: &std::path::Path) -> Result<(), Error> {
        Ok(())
    }
    fn save(&self, path: &std::path::Path) -> Result<(), Error> {
        Ok(())
    }
}
