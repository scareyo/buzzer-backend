use tonic::{transport::Server, Request, Response, Status};

use buzzer::buzzer_server::{Buzzer, BuzzerServer};
use buzzer::{OpenDoorRequest, OpenDoorReply, RingDoorRequest, RingDoorReply};
use tonic::codec::CompressionEncoding;

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
pub struct MyBuzzer {}

#[tonic::async_trait]
impl Buzzer for MyBuzzer {
    async fn ring_door(&self, request: Request<RingDoorRequest>) -> Result<Response<RingDoorReply>, Status> {
        println!("Got a RingDoorRequest: {:?}", request);

        let reply = buzzer::RingDoorReply {
            message: "hello".into(),
        };

        Ok(Response::new(reply))
    }
    
    async fn open_door(&self, request: Request<OpenDoorRequest>) -> Result<Response<OpenDoorReply>, Status> {
        println!("Got a OpenDoorRequest: {:?}", request);

        let reply = buzzer::OpenDoorReply {
            message: "hello".into(),
        };

        Ok(Response::new(reply))
    }
}
