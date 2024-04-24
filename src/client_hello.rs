use hello::greetings_client::GreetingsClient;
use hello::ParamRequest;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreetingsClient::connect("http://[::1]:50052").await?;

    let request = tonic::Request::new(
        ParamRequest {
            name: "Budi".to_string()
        }
    );

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}