use std::marker::PhantomData;

use prost::Message;

pub trait EventSubscriber {
    fn handle(&self, event_bytes: &Vec<u8>);
}

pub struct ProstMessageEventSubscriber<M, F> where
    M: Message,
    F: Fn(&M)
{
    handle_prost_event: F,
    _m: PhantomData<fn() -> M>,
}

impl<M, F> ProstMessageEventSubscriber<M, F> where
    M: Message,
    F: Fn(&M)
{
    pub fn new(handle: F) -> Self {
        ProstMessageEventSubscriber {
            handle_prost_event: handle,
            _m: PhantomData::default(),
        }
    }
}


impl<M, F> EventSubscriber for ProstMessageEventSubscriber<M, F> where
    M: Message + std::default::Default,
    F: Fn(&M) {
    fn handle(&self, event_bytes: &Vec<u8>) {
        let event_bytes_clone: &[u8] = &mut event_bytes.clone();
        let event = M::decode(event_bytes_clone).unwrap(); // unwrapやめる
        (self.handle_prost_event)(&event);
    }
}
