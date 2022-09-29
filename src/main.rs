use tonic::{transport::Server, Request, Response, Status};

use buzzer::buzzer_server::{Buzzer, BuzzerServer};
use buzzer::{OpenDoorRequest, OpenDoorReply, ListenDoorRequest, RingDoorReply};
use futures::Stream;
use std::{time::Duration, pin::Pin, sync::{Arc, Mutex}};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::codec::CompressionEncoding;

type RingDoorResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<RingDoorReply, Status>> + Send>>;

pub mod buzzer {
    tonic::include_proto!("buzzer");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50000".parse()?;
    let buzzer = MyBuzzer::default();

    Server::builder()
        .add_service(BuzzerServer::new(buzzer)
            .send_compressed(CompressionEncoding::Gzip)
            .accept_compressed(CompressionEncoding::Gzip))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct MyBuzzer {
    counter: Arc<Mutex<i64>>,
}

#[tonic::async_trait]
impl Buzzer for MyBuzzer {

    type ListenDoorStream = ResponseStream; 

    async fn listen_door(&self, request: Request<ListenDoorRequest>) -> RingDoorResult<Self::ListenDoorStream> {
        println!("Got a ListenDoorRequest: {:?}", request);

        // Create stream
        let repeat = std::iter::repeat(buzzer::RingDoorReply {
            message: "hello".into(),
        });

        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
            println!("\tclient disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::ListenDoorStream
        ))
    }
    
    async fn open_door(&self, request: Request<OpenDoorRequest>) -> Result<Response<OpenDoorReply>, Status> {
        println!("Got a OpenDoorRequest: {:?}", request);

        let mut counter = self.counter.lock().unwrap();
        *counter += 1;

        let reply = buzzer::OpenDoorReply {
            message: format!("{}", *counter).into(),
        };

        Ok(Response::new(reply))
    }
}
