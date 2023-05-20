use std::future::Future;
use futures::{TryFutureExt, FutureExt};

use communication::{forwarder_client::ForwarderClient, Invocation, MessagePack};
use serde::Deserialize;
use tonic::{transport::Channel, Request, Status, Response};

mod communication {
    tonic::include_proto!("communication");
}

struct ConnectionHandle(ForwarderClient<Channel>);

impl ConnectionHandle {
    pub fn invoke_raw<'a>(&'a mut self, module: String, function_name: String, args: Option<MessagePack>) -> impl Future<Output = Result<Response<MessagePack>, Status>> + 'a  {
        self.0.forward(Request::new(Invocation {
            module,
            function_name,
            args,
        }))
    }

    pub fn invoke<'a, 'b, TargetStruct>(&'a mut self, module: String, function_name: String, args: Option<MessagePack>) -> impl Future<Output = Result<TargetStruct, rmp_serde::decode::Error>> + 'a 
        where TargetStruct: Deserialize<'a>,
    {
        self.invoke_raw(module, function_name, args).map(|response| {
            let slice = response.unwrap().into_inner().data.into_boxed_slice();
            rmp_serde::from_slice::<'b, TargetStruct>(slice.as_ref())
        })
    }
}

pub async fn connect(ip: String, port: String) -> anyhow::Result<ConnectionHandle> {
    // TODO: Spin up the server if it doesn't already exist
    // NOTE: Perhaps make server spinup a feature flag?
    Ok(ConnectionHandle(ForwarderClient::connect(ip + ":" + &port).await?))
}
