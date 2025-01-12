// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    PolkadotService(#[from] polkadot_service::Error),

    #[error(transparent)]
    SubstrateCli(#[from] sc_cli::Error),

    #[error(transparent)]
    SubstrateService(#[from] sc_service::Error),

    #[error(transparent)]
    SubstrateTracing(#[from] sc_tracing::logging::Error),

    #[error("Failed to resolve provided URL")]
    AddressResolutionFailure(#[from] std::io::Error),

    #[error("URL did not resolve to anything")]
    AddressResolutionMissing,

    #[error(transparent)]
    Storage(#[from] sc_storage_monitor::Error),

    #[error("Other: {0}")]
    Other(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Other(s)
    }
}
