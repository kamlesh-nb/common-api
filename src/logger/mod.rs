mod azure_app_insights;
pub use azure_app_insights::*;

pub trait Logger {
    fn information(&self, message: String);
    fn warning(&self, message: String);
    fn error(&self, message: String);
}
