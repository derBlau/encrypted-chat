use tokio::net::TcpListener;

use encrypted_chat::input::Input;

const IP: &str = "127.0.0.1";
const PORT: &str = "8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = IP.to_string() + ":" + PORT;
    let socket = TcpListener::bind(address).await?;

    loop {
        let (client, address) = socket.accept().await?;
        println!("[+]NEW CONNECTION WITH CLIENT {address}");

        let (reader, writer) = client.into_split();

        let listener_handle = tokio::spawn(async move { todo!() });
        let sender_handle = tokio::spawn(async move {
            Input::send_message(writer).await;
        });

        tokio::select! {
            _ = sender_handle => {}
            _ = listener_handle => {}
        }
    }

    Ok(())
}
