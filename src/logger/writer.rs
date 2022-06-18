pub trait Writer {
    fn information(&self, message: String);
    fn warning(&self, message: String);
    fn error(&self, message: String);
}