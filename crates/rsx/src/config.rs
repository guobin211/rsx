use clap::Parser;
use std::{fs, path::Path};

#[derive(Debug, Clone, Parser)]
#[command(name = "rsx", about = "rsx framework config")]
pub struct Config {
    /// app name and process name , default is package.json.name or Cargo.toml.name
    #[arg(long)]
    pub name: Option<String>,
    /// app version, default is package.json.version or Cargo.toml.version
    #[arg(long)]
    pub version: Option<String>,
    /// app description, default is package.json.description or Cargo.toml.description
    #[arg(long)]
    pub description: Option<String>,
    /// app author, default is package.json.author or Cargo.toml.author
    #[arg(long)]
    pub author: Option<String>,
    /// The rsx pages directory
    #[arg(long, default_value = "src/pages")]
    pub pages: String,
    /// The public directory for static files
    #[arg(long, default_value = "public")]
    pub public: String,
    /// The root directory for the project
    #[arg(long)]
    pub root: Option<String>,
    /// The output directory for generated files
    #[arg(long, default_value = "generated")]
    pub generated: String,
    /// The output directory for the dist files
    #[arg(long, default_value = "dist")]
    pub dist: String,
    /// The port for the server
    #[arg(long, default_value = "8888")]
    pub port: u16,
    /// The host for the server
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,
}

impl Config {
    pub fn new() -> Self {
        let mut config = Self::try_parse().unwrap_or_default();

        // 处理环境变量（如果命令行参数没有提供）
        if config.name.is_none() {
            config.name = std::env::var("RSX_NAME").ok();
        }
        if config.version.is_none() {
            config.version = std::env::var("RSX_VERSION").ok();
        }
        if config.description.is_none() {
            config.description = std::env::var("RSX_DESCRIPTION").ok();
        }
        if config.author.is_none() {
            config.author = std::env::var("RSX_AUTHOR").ok();
        }
        if config.root.is_none() {
            config.root = std::env::var("RSX_ROOT").ok();
        }

        // 如果root仍然没有设置，使用当前目录
        if config.root.is_none() {
            config.root = Some(
                std::env::current_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
        }

        // 从package.json读取默认值（如果命令行参数和环境变量都没有提供）
        let package_json = Path::new("package.json");
        if package_json.exists() {
            let json_content = fs::read_to_string(package_json).unwrap_or_default();
            let json: serde_json::Value = serde_json::from_str(&json_content).unwrap_or_default();

            if config.name.is_none() {
                config.name = json.get("name").and_then(|v| v.as_str()).map(String::from);
            }
            if config.version.is_none() {
                config.version = json
                    .get("version")
                    .and_then(|v| v.as_str())
                    .map(String::from);
            }
            if config.description.is_none() {
                config.description = json
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(String::from);
            }
            if config.author.is_none() {
                config.author = json
                    .get("author")
                    .and_then(|v| v.as_str())
                    .map(String::from);
            }
        }

        log::debug!("config: {config:?}");
        config
    }

    /// 获取root路径，确保返回Some值
    pub fn root(&self) -> &str {
        self.root.as_deref().unwrap_or(".")
    }

    /// 从命令行参数创建配置（用于测试或手动解析）
    pub fn from_args() -> Self {
        Self::new()
    }
}

impl Default for Config {
    fn default() -> Self {
        // 创建一个默认配置，不解析命令行参数
        let mut config = Self {
            name: None,
            version: None,
            description: None,
            author: None,
            pages: "src/pages".to_string(),
            public: "public".to_string(),
            root: None,
            generated: "generated".to_string(),
            dist: "dist".to_string(),
            port: 8888,
            host: "0.0.0.0".to_string(),
        };

        // 处理环境变量
        if config.name.is_none() {
            config.name = std::env::var("RSX_NAME").ok();
        }
        if config.version.is_none() {
            config.version = std::env::var("RSX_VERSION").ok();
        }
        if config.description.is_none() {
            config.description = std::env::var("RSX_DESCRIPTION").ok();
        }
        if config.author.is_none() {
            config.author = std::env::var("RSX_AUTHOR").ok();
        }
        if config.root.is_none() {
            config.root = std::env::var("RSX_ROOT").ok();
        }

        // 如果root仍然没有设置，使用当前目录
        if config.root.is_none() {
            config.root = Some(
                std::env::current_dir()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
        }

        // 从package.json读取默认值
        let package_json = Path::new("package.json");
        if package_json.exists() {
            let json_content = fs::read_to_string(package_json).unwrap_or_default();
            let json: serde_json::Value = serde_json::from_str(&json_content).unwrap_or_default();

            if config.name.is_none() {
                config.name = json.get("name").and_then(|v| v.as_str()).map(String::from);
            }
            if config.version.is_none() {
                config.version = json
                    .get("version")
                    .and_then(|v| v.as_str())
                    .map(String::from);
            }
            if config.description.is_none() {
                config.description = json
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(String::from);
            }
            if config.author.is_none() {
                config.author = json
                    .get("author")
                    .and_then(|v| v.as_str())
                    .map(String::from);
            }
        }

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        println!("{config:?}");
        assert_eq!(config.port, 8888);
    }

    #[test]
    fn test_all_parameters_optional() {
        // 测试所有参数都有合理的默认值
        let config = Config::default();

        // 这些字段应该有默认值
        assert!(!config.pages.is_empty());
        assert!(!config.public.is_empty());
        assert!(!config.generated.is_empty());
        assert!(!config.dist.is_empty());
        assert!(!config.host.is_empty());
        assert!(config.port > 0);

        // root应该有值（当前目录或环境变量）
        assert!(config.root.is_some());
        assert!(!config.root().is_empty());

        // 其他字段可以是None（从package.json读取或保持None）
        println!("name: {:?}", config.name);
        println!("version: {:?}", config.version);
        println!("description: {:?}", config.description);
        println!("author: {:?}", config.author);
    }

    #[test]
    fn test_default_values() {
        let config = Config::default();
        assert_eq!(config.port, 8888);
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.pages, "src/pages");
        assert_eq!(config.public, "public");
        assert_eq!(config.generated, "generated");
        assert_eq!(config.dist, "dist");
    }

    #[test]
    fn test_root_method() {
        let config = Config::default();
        let root = config.root();
        assert!(!root.is_empty());
        println!("Root path: {root}");
    }

    #[test]
    fn test_config_with_env_vars() {
        // 设置环境变量
        unsafe {
            std::env::set_var("RSX_NAME", "Test App");
            std::env::set_var("RSX_PORT", "9999");
        }

        let config = Config::default();

        assert_eq!(config.name, Some("Test App".to_string()));
        // 注意：port是u16类型，有默认值，环境变量不会影响它
        assert_eq!(config.port, 8888);

        // 清理环境变量
        unsafe {
            std::env::remove_var("RSX_NAME");
            std::env::remove_var("RSX_PORT");
        }
    }
}
