pub mod user {
    tonic::include_proto!("user");
}

use user::GetUserRequest;
use user::user_client::UserClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(GetUserRequest {
        id: "1".to_string(),
    });

    let response = client.get_user(request).await?;

    println!("RESPONSE={:?}", response);
    Ok(())
}
