use pretty_assertions::{assert_eq, assert_matches};
use std::time::Duration;
use test_log::test;
use tokio::sync::watch;
use vacs_protocol::{ClientInfo, LoginFailureReason, SignalingMessage};
use vacs_server::test_utils::TestApp;
use vacs_signaling::client;
use vacs_signaling::error::SignalingError;
use vacs_signaling::test_utils::TestRig;
use vacs_signaling::transport;

#[test(tokio::test)]
async fn login() {
    let test_app = TestApp::new().await;

    let transport = transport::tokio::TokioTransport::new(test_app.addr())
        .await
        .expect("Failed to create transport");
    let (shutdown_tx, shutdown_rx) = watch::channel(());
    let mut client = client::SignalingClient::builder(transport, shutdown_rx)
        .with_login_timeout(Duration::from_millis(100))
        .build();

    let res = client.login("client1", "token1").await;
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        vec![ClientInfo {
            id: "client1".to_string(),
            display_name: "client1".to_string()
        }]
    );

    shutdown_tx.send(()).unwrap();
}

#[test(tokio::test)]
async fn login_timeout() {
    let test_app = TestApp::new().await;

    let transport = transport::tokio::TokioTransport::new(test_app.addr())
        .await
        .expect("Failed to create transport");
    let (shutdown_tx, shutdown_rx) = watch::channel(());
    let mut client = client::SignalingClient::builder(transport, shutdown_rx)
        .with_login_timeout(Duration::from_millis(100))
        .build();

    tokio::time::sleep(Duration::from_millis(100)).await;

    let res = client.login("client1", "token1").await;
    assert!(res.is_err());
    assert_matches!(
        res.unwrap_err(),
        SignalingError::LoginError(LoginFailureReason::Timeout)
    );

    shutdown_tx.send(()).unwrap();
}

#[test(tokio::test)]
async fn login_invalid_credentials() {
    let test_app = vacs_server::test_utils::TestApp::new().await;

    let transport = transport::tokio::TokioTransport::new(test_app.addr())
        .await
        .expect("Failed to create transport");
    let (shutdown_tx, shutdown_rx) = watch::channel(());
    let mut client = client::SignalingClient::builder(transport, shutdown_rx)
        .with_login_timeout(Duration::from_millis(100))
        .build();

    let res = client.login("client1", "").await;
    assert!(res.is_err());
    assert_matches!(
        res.unwrap_err(),
        SignalingError::LoginError(LoginFailureReason::InvalidCredentials)
    );

    shutdown_tx.send(()).unwrap();
}

#[test(tokio::test)]
async fn login_duplicate_id() {
    let test_rig = TestRig::new(1).await.unwrap();

    let transport = transport::tokio::TokioTransport::new(test_rig.server().addr())
        .await
        .expect("Failed to create transport");
    let (_shutdown_tx, shutdown_rx) = watch::channel(());
    let mut client = client::SignalingClient::builder(transport, shutdown_rx)
        .with_login_timeout(Duration::from_millis(100))
        .build();

    let res = client.login("client0", "token0").await;
    assert!(res.is_err());
    assert_matches!(
        res.unwrap_err(),
        SignalingError::LoginError(LoginFailureReason::DuplicateId)
    );
}

#[test(tokio::test)]
async fn logout() {
    let mut test_rig = TestRig::new(1).await.unwrap();
    let client = test_rig.client_mut(0);

    let res = client.send(SignalingMessage::Logout).await;
    assert!(res.is_ok());
}
