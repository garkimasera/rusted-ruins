/// Rules for game world
#[derive(Serialize, Deserialize)]
pub struct World {
    /// Restart map path
    pub restart_path: String,
    /// Script id to execute on restart
    pub restart_script: String,
}
