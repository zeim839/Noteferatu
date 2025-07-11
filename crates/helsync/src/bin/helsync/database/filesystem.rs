use sqlx::FromRow;
use console::style;

#[derive(FromRow)]
pub struct Filesystem {
    pub path: String,
    pub drive: String,
}

impl std::fmt::Display for Filesystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "[{}: {}] [{}: {}]",
            style("Path:").bold().cyan(),
            &self.path,
            style("Drive:").bold().cyan(),
            &self.drive
        )
    }
}
