// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of ink!.
//
// ink! is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ink! is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with ink!.  If not, see <http://www.gnu.org/licenses/>.

mod account;
mod accessor;
mod instance;
mod record;
mod storage;
mod typed_encoded;
mod types;

use self::{
    instance::TestEnvInstance,
    record::{
        Record,
        CallContractRecord,
        CreateContractRecord,
        EmitEventRecord,
    },
    storage::{
        Entry,
        Storage,
    },
    account::{
        AccountsDb,
    },
    typed_encoded::TypedEncoded,
    types::{
        Balance,
        AccountId,
        Moment,
        BlockNumber,
        Hash,
        Call,
    },
};

pub use self::accessor::TestEnv;
