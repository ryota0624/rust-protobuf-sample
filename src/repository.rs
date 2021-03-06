use std::fmt::Debug;
use std::marker::PhantomData;

pub trait Entity {
    type Id;
    fn entity_id(&self) -> &Self::Id;
}

#[derive(Debug)]
pub enum Error<E: Entity> {
    EntityNotFound,
    EntityAlreadyExist(),
    _UnUse(PhantomData<fn() -> E>),
}

pub trait Repository<E: Entity> {
    type S;
    fn find_by_id(&self, id: &E::Id) -> Option<&E>;
    fn store(self, t: E) -> Result<Self::S, Error<E>>;
}
