use homebotctl::cfg::Config;

use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_load_config() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("config.yaml");

    let config_data = r#"
    code_path: "./"
    host: "localhost"
    lan_base: "192.168.0.1"
    lan_mask: 24
    port: 22
    username: "testuser"
    password: "testpass"
    ssh_key_path: "/test/key"
    "#;

    let mut file = File::create(&file_path).unwrap();
    file.write_all(config_data.as_bytes()).unwrap();

    let config = Config::from_file(&file_path).unwrap();

    assert_eq!(config.host, "localhost");
    assert_eq!(config.lan_base, "192.168.0.1");
    assert_eq!(config.lan_mask, 24);
    assert_eq!(config.port, 22);
    assert_eq!(config.username, "testuser");
    assert_eq!(config.password, "testpass");
    assert_eq!(config.ssh_key_path, "/test/key");
}
