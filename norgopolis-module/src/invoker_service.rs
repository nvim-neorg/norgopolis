use std::pin::Pin;

use futures::Stream;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Status, Request, Response};

use crate::module_communication::{
    invoker_server::Invoker,
    Invocation, MessagePack,
};

pub struct InvokerService {}

#[tonic::async_trait]
impl Invoker for InvokerService {
    type InvokeStream = Pin<Box<dyn Stream<Item = Result<MessagePack, Status>> + Send>>;

    async fn invoke(
        &self,
        request: Request<Invocation>,
    ) -> Result<Response<Self::InvokeStream>, Status> {
        let invocation = request.into_inner();

        if invocation.function_name == "echo".to_string() {
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            tx.send(Ok(invocation.args.unwrap())).await.unwrap();

            Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
        } else {
            Err(Status::new(tonic::Code::NotFound, "Function not found!"))
        }
    }
}
