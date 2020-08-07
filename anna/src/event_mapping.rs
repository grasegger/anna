#[derive(Debug, Clone, Hash)]
pub struct EventMapping {
    selector: String,
    event: String,
    listener: fn(),
}
