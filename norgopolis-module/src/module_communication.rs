tonic::include_proto!("module_communication");

use rmp_serde;
use anyhow::Result;

impl MessagePack {
    pub fn decode<Target>(self) -> Result<Target>
    where
        Target: serde::de::DeserializeOwned,
    {
        Ok(rmp_serde::decode::from_slice(self.data.as_slice())?)
    }
}
