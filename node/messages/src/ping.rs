// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ping {
    pub version: u32,
    pub fork_depth: u32,
    pub node_type: NodeType,
    pub status: Status,
}

impl MessageTrait for Ping {
    /// Returns the message name.
    #[inline]
    fn name(&self) -> &str {
        "Ping"
    }

    /// Serializes the message into the buffer.
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        Ok(bincode::serialize_into(&mut *writer, &(self.version, self.fork_depth, self.node_type, self.status))?)
    }

    /// Deserializes the given buffer into a message.
    #[inline]
    fn deserialize(bytes: BytesMut) -> Result<Self> {
        let mut reader = bytes.reader();
        let (version, fork_depth, node_type, status) = bincode::deserialize_from(&mut reader)?;
        Ok(Self { version, fork_depth, node_type, status })
    }
}