#[cfg(feature = "backend")]
use candid::{
    types::{Serializer, Type},
    CandidType,
};

#[cfg(feature = "backend")]
use speedy::{Readable, Writable};

#[cfg(feature = "tauri")]
use surrealdb::sql::Value;

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::{
    ops::{Deref, DerefMut},
    str::FromStr,
};
use uuid::Uuid;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "backend", derive(Readable, Writable))]
pub struct Id(pub Uuid);

impl Id {
    #[cfg(feature = "backend")]
    pub async fn ic_new() -> Self {
        let call_result: Result<(Vec<u8>,), _> = ic_cdk::api::call::call(
            ic_cdk::export::Principal::management_canister(),
            "raw_rand",
            (),
        )
        .await;
        let id = match call_result {
            Ok((id,)) => id,
            Err(e) => {
                ic_cdk::trap(&format!("Failed to get id: {:#?}", e));
            }
        };
        id.into()
    }

    #[cfg(any(feature = "frontend", feature = "tauri"))]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<Uuid> for Id {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<[u8; 16]> for Id {
    fn from(value: [u8; 16]) -> Self {
        Id(Uuid::from_bytes(value))
    }
}

impl From<Vec<u8>> for Id {
    fn from(value: Vec<u8>) -> Self {
        let value: &[_] = value.get(..16).unwrap();
        let value: [u8; 16] = value.try_into().unwrap();
        Id(Uuid::from_bytes(value.into()))
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Id {
    type Error = uuid::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(Uuid::parse_str(value)?))
    }
}

impl Deref for Id {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Id {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "tauri")]
impl From<Id> for Value {
    fn from(x: Id) -> Self {
        x.0.into()
    }
}

#[cfg(feature = "tauri")]
impl TryFrom<Value> for Id {
    type Error = surrealdb::Error;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(Id(value.try_into()?))
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Id {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s).map(|f| Self(f))
    }
}

#[cfg(feature = "backend")]
impl CandidType for Id {
    fn _ty() -> Type {
        Type::Text
    }
    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_text(&self.0.to_string())
    }
}

// #[cfg(feature = "backend")]
// impl<'a_, C_: speedy::Context> speedy::Readable<'a_, C_> for Id {
//     //#[inline]
//     fn read_from<R_: speedy::Reader<'a_, C_>>(
//         _reader_: &mut R_,
//     ) -> std::result::Result<Self, C_::Error> {
//         let t0: Uuid = _reader_.read_value()?;
//         Ok(Id(t0))
//     }
//     //#[inline]
//     fn minimum_bytes_needed() -> usize {
//         {
//             let mut out = 0;
//             out += <Uuid as speedy::Readable<'a_, C_>>::minimum_bytes_needed();
//             out
//         }
//     }
//     #[inline(always)]
//     fn speedy_is_primitive() -> bool {
//         <Uuid as speedy::Readable<'a_, C_>>::speedy_is_primitive()
//             && (std::mem::size_of::<Uuid>()) == std::mem::size_of::<Self>()
//     }
//     //#[inline]
//     unsafe fn speedy_slice_from_bytes(slice: &[u8]) -> &[Self] {
//         unsafe {
//             std::slice::from_raw_parts(
//                 slice.as_ptr() as *const Self,
//                 slice.len() / std::mem::size_of::<Self>(),
//             )
//         }
//     }
//     #[inline(always)]
//     fn speedy_flip_endianness(itself: *mut Self) {
//         unsafe {
//             <Uuid as speedy::Readable<
//                 'a_,
//                 C_,
//            >>::speedy_flip_endianness(&raw mut (*itself).0);
//         }
//     }
//     #[inline(always)]
//     fn speedy_convert_slice_endianness(
//         endianness: speedy::Endianness,
//         slice: &mut [Self],
//     ) {
//         if endianness.conversion_necessary() {
//             for value in slice {
//                 <Self as speedy::Readable<'a_, C_>>::speedy_flip_endianness(value);
//             }
//         }
//     }
// }

// #[cfg(feature = "backend")]
// impl<C_: speedy::Context> speedy::Writable<C_> for Id {
//     //#[inline]
//     fn write_to<T_: ?Sized + speedy::Writer<C_>>(
//         &self,
//         _writer_: &mut T_,
//     ) -> std::result::Result<(), C_::Error> {
//         let t0 = &self.0;
//         _writer_.write_value(t0)?;
//         Ok(())
//     }
//     #[inline(always)]
//     fn speedy_is_primitive() -> bool {
//         <Uuid as speedy::Writable<C_>>::speedy_is_primitive()
//             && (std::mem::size_of::<Uuid>()) == std::mem::size_of::<Self>()
//     }
//     #[inline(always)]
//     unsafe fn speedy_slice_as_bytes(slice: &[Self]) -> &[u8]
//     where
//         Self: Sized,
//     {
//         unsafe {
//             std::slice::from_raw_parts(
//                 slice.as_ptr() as *const u8,
//                 slice.len() * std::mem::size_of::<Self>(),
//             )
//         }
//     }
// }
