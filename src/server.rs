use tonic::{Request, Response, Status, transport::Server};
use user::user_server::UserServer;
use user::{GetUserRequest, GetUserResponse};

pub mod user {
    tonic::include_proto!("user");
}

#[derive(Default)]
pub struct UserService;

#[tonic::async_trait]
impl user::user_server::User for UserService {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = user::GetUserResponse {
            id: request.into_inner().id,
            name: "test".to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user_service = UserService::default();

    println!("User service listening on {}", addr);

    Server::builder()
        .add_service(UserServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
