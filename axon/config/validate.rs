use super::Config;

pub fn validate_config(config: &Config) -> Result<(), String> {
    if config.server_name.is_empty() {
        return Err("server_name cannot be empty".to_string());
    }
    if config.listen_port == 0 {
        return Err("listen_port must be greater than 0".to_string());
    }
    Ok(())
}