use web_sys::Event;

pub struct EventMapping <'a> {
    selector: String,
    event: String,
    listener: &'a dyn Fn(Event),
}

impl <'a> EventMapping <'a> {
    pub async fn new<F: 'a> (selector: String, event: String, listener: F) -> EventMapping<'a>
    where F: std::future::Future<> + Fn(web_sys::Event) + 'a {
        Self{ 
            selector: selector,
            event: event,
            listener: &listener
        }
    }
}