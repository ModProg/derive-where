use core::fmt::{Debug};

use serde_::{Serialize, Deserialize};

pub trait Protocol<Id> {
    type ProtocolError: ProtocolError<Id>;
}

pub trait ProtocolError<Id>: Debug + Clone + Serialize + for<'de> Deserialize<'de> {}

pub trait ChainedProtocol<Id>: 'static + Debug {
    type Protocol1: Protocol<Id>;
}

#[derive_where::derive_where(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "serde_")]
#[serde(bound(serialize = "
    <C::Protocol1 as Protocol<Id>>::ProtocolError: Serialize,
"))]
#[serde(bound(deserialize = "
    <C::Protocol1 as Protocol<Id>>::ProtocolError: for<'x> Deserialize<'x>,
"))]
pub enum ChainedProtocolError<Id, C>
where
    C: ChainedProtocol<Id>,
{
    Protocol1(<C::Protocol1 as Protocol<Id>>::ProtocolError),
}

impl<Id, C> ProtocolError<Id> for ChainedProtocolError<Id, C>
where
    C: ChainedProtocol<Id>,
{

}
