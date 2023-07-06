tonic::include_proto!("module_communication");

use rmp_serde;
use anyhow::Result;
use serde::Serialize;

impl MessagePack {
    pub fn decode<Target>(self) -> Result<Target>
    where
        Target: serde::de::DeserializeOwned,
    {
        Ok(rmp_serde::decode::from_slice(self.data.as_slice())?)
    }

    pub fn encode<T>(target: T) -> Result<MessagePack>
        where T: Serialize
    {
        Ok(MessagePack { data: rmp_serde::encode::to_vec(&target)? })
    }
}
