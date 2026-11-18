use crate::error::Error;
use std::path::Path;
pub trait Cores {
    async fn start(
        &mut self,
        exe_path: &Path,
        config_path: &Path,
        ctl: String,
    ) -> Result<(), Error>;
    fn stop(&self) -> Result<(), Error>;
    fn switch_node(&self, node_name: String) -> Result<(), Error>;
    fn get_nodes(&self) -> Result<(), Error>;
    fn test_delay(&self) -> Result<(), Error>;
    fn tun_on(&self) -> Result<(), Error>;
    fn load(&mut self, path: &Path) -> Result<(), Error>;
    fn save(&self, path: &Path) -> Result<(), Error>;
}
