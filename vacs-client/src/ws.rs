use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use vacs_core::signaling;

const SERVER: &str = "ws://127.0.0.1:3000/ws";
const CLIENT_ID: &str = "client1"; // Unique ID for the client
const TOKEN: &str = "token1"; // Authentication token
const PEER_ID: &str = "client2"; // Target peer ID for the CallOffer

#[tokio::main]
async fn main() {
    // Connect to the WebSocket server
    let (mut ws_stream, _) = match connect_async(SERVER).await {
        Ok((stream, response)) => {
            println!("Connected to server!");
            println!("Server response: {:?}", response);
            (stream, response)
        }
        Err(e) => {
            println!("Failed to connect to server: {:?}", e);
            return;
        }
    };

    // Perform the login flow
    let login_message = signaling::Message::Login {
        id: CLIENT_ID.to_string(),
        token: TOKEN.to_string(),
    };

    println!("Sending login message");
    if let Err(e) = send_message(&mut ws_stream, &login_message).await {
        println!("Failed to send login message: {:?}", e);
        return;
    }
    println!("Login message sent successfully.");

    // Wait for a response from the server
    if let Some(Ok(msg)) = ws_stream.next().await {
        match msg {
            Message::Text(text) => {
                println!("Received message: {}", text);

                // Check if the server responded with a LoginFailure
                if let Ok(signaling::Message::LoginFailure { reason }) =
                    signaling::Message::deserialize(&text)
                {
                    println!("Login failed: {:?}", reason);
                    return;
                }

                // If login succeeded, send a CallOffer
                let call_offer_message = signaling::Message::CallOffer {
                    sdp: "sample_sdp".to_string(),
                    peer_id: PEER_ID.to_string(),
                };
                if let Err(e) = send_message(&mut ws_stream, &call_offer_message).await {
                    println!("Failed to send CallOffer: {:?}", e);
                    return;
                }
                println!("CallOffer sent successfully.");
            }
            Message::Close(_) => {
                println!("Server closed the connection.");
                return;
            }
            _ => {
                println!("Unexpected message: {:?}", msg);
            }
        }
    } else {
        println!("Did not receive a response from the server.");
    }

    if let Some(Ok(msg)) = ws_stream.next().await {
        match msg {
            Message::Text(text) => {
                println!("Received message: {}", text);
            }
            Message::Close(_) => {
                println!("Server closed the connection.");
                return;
            }
            _ => {
                println!("Unexpected message: {:?}", msg);
            }
        }
    }

    // Cleanly close the connection
    if let Err(e) = ws_stream.close(None).await {
        println!("Failed to close the WebSocket connection: {:?}", e);
    }
    println!("Connection closed.");
}

/// Helper function to send a message through the WebSocket connection
async fn send_message(
    ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    message: &signaling::Message,
) -> anyhow::Result<()> {
    let serialized_message = signaling::Message::serialize(message)?;
    ws_stream.send(Message::from(serialized_message)).await?;
    Ok(())
}
