use std::env;

use authentication::auth_client::AuthClient;
use authentication::{SignInRequest, SignOutRequest, SignUpRequest};
use tokio::time::{sleep, Duration};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::authentication::{SignInResponse, SignOutResponse, SignUpResponse, StatusCode};

pub mod authentication {
    tonic::include_proto!("authentication");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Explain AUTH_SERVICE_HOST_NAME
    let auth_hostname = env::var("AUTH_SERVICE_HOST_NAME").unwrap_or("[::1]".to_owned());

    // Establish connection when auth service
    let mut client = AuthClient::connect(format!("http://{}:50051", auth_hostname)).await?;

    loop {
        // Create random username using new_v4()
        let username: String = Uuid::new_v4().to_string();
        // Create random password using new_v4()
        let password: String = Uuid::new_v4().to_string();

        // Create a new `SignUpRequest`.
        let request: Request<SignUpRequest> = tonic::Request::new(SignUpRequest {
            username: username.clone(),
            password: password.clone(),
        });

        // Make a sign up request. Propagate any errors.
        let response: Response<SignUpResponse> = client.sign_up(request).await?;

        // Log the response
        println!(
            "SIGN UP RESPONSE STATUS: {:?}",
            StatusCode::from_i32(response.into_inner().status_code)
        );

        // ---------------------------------------------

        // Create a new `SignInRequest`.
        let request: Request<SignInRequest> = tonic::Request::new(SignInRequest {
            username: username.clone(),
            password: password.clone(),
        });

        // Make a sign in request. Propagate any errors. Convert Response<SignInResponse> into SignInResponse.
        let response: SignInResponse = client.sign_in(request).await?.into_inner();

        // Log response status_code
        println!(
            "SIGN IN RESPONSE STATUS: {:?}",
            StatusCode::from_i32(response.status_code)
        );

        // ---------------------------------------------

        // Create a new `SignOutRequest`.
        let request: Request<SignOutRequest> = tonic::Request::new(SignOutRequest {
            session_token: response.session_token,
        });

        // Make a sign out request. Propagate any errors.
        let response: Response<SignOutResponse> = client.sign_out(request).await?;

        // Log response status_code
        println!(
            "SIGN OUT RESPONSE STATUS: {:?}",
            StatusCode::from_i32(response.into_inner().status_code)
        );

        println!("--------------------------------------",);

        sleep(Duration::from_secs(3)).await;
    }
}
