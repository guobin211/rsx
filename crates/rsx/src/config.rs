pub struct Config {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,

    /// The rsx pages directory.
    pub page: String,
    /// The output directory for the built pages.
    pub dist: String,
    /// The public directory for static files.
    pub public: String,
}
