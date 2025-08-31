use tonic::transport::Channel;
use tonic::Request;

pub mod mastrmnd {
    tonic::include_proto!("mastrmnd");
}

use mastrmnd::echo_client::EchoClient;
use mastrmnd::RegisterRequest;

/// Connects to the Echo gRPC server and performs a Register call.
///
/// endpoint: e.g., "http://127.0.0.1:50051"
/// message: message to send in RegisterRequest
pub async fn run_client(endpoint: &str, message: String) -> Result<String, Box<dyn std::error::Error>> {
    let channel: Channel = Channel::from_shared(endpoint.to_string())?
        .connect()
        .await?;

    let mut client = EchoClient::new(channel);

    let request = Request::new(RegisterRequest { message });

    let response = client.register(request).await?;
    Ok(response.into_inner().message)
}
