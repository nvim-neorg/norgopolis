use anyhow::Result;
use tokio_stream::wrappers::ReceiverStream;

use norgopolis_module::invoker_service::Service;
use norgopolis_module::module_communication::MessagePack;
use norgopolis_module::{Code, Status};

#[derive(Default)]
struct ModuleService {}

#[norgopolis_module::async_trait]
impl Service for ModuleService {
    type Stream = ReceiverStream<Result<MessagePack, Status>>;

    async fn call(
        &self,
        fn_name: String,
        args: Option<MessagePack>,
    ) -> Result<Self::Stream, Status> {
        if fn_name == "echo" {
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            tx.send(Ok(args.unwrap())).await.unwrap();

            Ok(ReceiverStream::new(rx))
        } else {
            Err(Status::new(Code::NotFound, "Function not found!"))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    norgopolis_module::Module::start(ModuleService::default()).await
}
