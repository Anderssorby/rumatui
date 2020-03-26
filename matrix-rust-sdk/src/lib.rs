// Copyright 2020 Damir Jelić
// Copyright 2020 The Matrix.org Foundation C.I.C.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This crate implements a [Matrix](https://matrix.org/) client library.

pub use crate::{error::Error, session::Session};
pub use reqwest::header::InvalidHeaderValue;
pub use ruma_client_api as api;
pub use ruma_events as events;
pub use ruma_identifiers as identifiers;

mod async_client;
mod base_client;
mod error;
mod session;

#[cfg(feature = "encryption")]
mod crypto;

pub use async_client::{AsyncClient, AsyncClientConfig, SyncSettings};
pub use base_client::{Client, Room};

pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
