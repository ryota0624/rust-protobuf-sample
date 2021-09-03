use prost::Message;

pub enum Error {}

pub trait EventPublisher {
    fn publish<E>(self, e: &E, converter: &dyn EventConverter<E>) -> Result<(), Error>;
}

pub struct ConsoleEventPublisher;

impl EventPublisher for ConsoleEventPublisher {
    fn publish<E>(self, e: &E, converter: &dyn EventConverter<E>) -> Result<(), Error> {
        let bytes = converter.to_bytes(e);
        println!("ConsoleEventPublisher.publish {:?}!", bytes);
        Ok(())
    }
}

pub struct InmemoryEventPublisher<M: Fn(&Vec<u8>)> {
    listeners: Vec<M>
}

impl<M: Fn(&Vec<u8>)>  InmemoryEventPublisher<M> {
    pub fn new() -> Self {
        InmemoryEventPublisher{
            listeners: Vec::new()
        }
    }
}

impl<M: Fn(&Vec<u8>)> InmemoryEventPublisher<M> {
    pub fn add_listener(mut self, listener: M) -> Self {
        self.listeners.push(listener);
        self
    }
}

impl<M: Fn(&Vec<u8>)> EventPublisher for InmemoryEventPublisher<M> {
    fn publish<E>(self, e: &E, converter: &dyn EventConverter<E>) -> Result<(), Error> {
        let bytes = converter.to_bytes(e);
        // println!("ConsoleEventPublisher.publish {:?}!", bytes);
        for listener in self.listeners {
            listener(&bytes);
        }
        Ok(())
    }
}

pub trait EventConverter<E> {
    fn to_bytes(&self, e: &E) -> Vec<u8>;
}

pub struct ProstEventConverter;

impl<E> EventConverter<E> for ProstEventConverter where E: Message {
    fn to_bytes(&self, e: &E) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(18);
        e.encode(&mut buf).expect("prost::Message should success encode");
        buf
    }
}
