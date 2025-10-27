// SPDX-License-Identifier: (Apache-2.0 OR MIT)

mod json;

#[cfg(feature = "yyjson")]
mod yyjson;

#[cfg(feature = "yyjson")]
pub(crate) use yyjson::deserialize;

#[cfg(not(feature = "yyjson"))]
pub(crate) use json::deserialize;

pub(crate) use json::deserialize_cbor;
