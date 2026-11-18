use crate::cores::cores_trait::Cores;
use crate::error::Error;
pub struct MihomoCmd {
    pub nodes_list: Vec<String>,
}
impl Cores for MihomoCmd {
    fn start(&self) -> Result<(), Error> {
        Ok(())
    }
    fn stop(&self) -> Result<(), Error> {
        Ok(())
    }
    fn switch_node(&self, node_name: String) -> Result<(), Error> {
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
