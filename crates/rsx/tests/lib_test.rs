use rsx::config::Config;

#[test]
fn test_config_creation() {
    let config = Config::default();
    assert_eq!(config.port, 8888);
    assert_eq!(config.host, "0.0.0.0");
    assert_eq!(config.pages, "src/pages");
    assert_eq!(config.public, "public");
    assert_eq!(config.generated, "generated");
    assert_eq!(config.dist, "dist");
}

#[test]
fn test_config_root_path() {
    let config = Config::default();
    let root = config.root();
    assert!(!root.is_empty());
    println!("Root path: {root}");
}

#[test]
fn test_config_optional_fields() {
    let config = Config::default();

    // 这些字段可以是None，取决于环境变量和package.json
    println!("name: {:?}", config.name);
    println!("version: {:?}", config.version);
    println!("description: {:?}", config.description);
    println!("author: {:?}", config.author);

    // 验证root字段总是有值
    assert!(config.root.is_some());
}
