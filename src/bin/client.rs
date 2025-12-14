use encrypted_chat::input::Input;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, tcp::OwnedReadHalf};

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

        let listener_handle = tokio::spawn(async move {
            read_incoming_message(reader).await;
        });

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

/// this function contains the logic used to receive and display messages
/// sent out by the server. It takes a mutable [`OwnedReadHalf`] as a parameter, which
/// should be acquired after splitting a [`TcpStream`](struct@tokio::net::TcpStream)
/// using the [`into_split`](fn@tokio::net::TcpStream::into_split) function.
async fn read_incoming_message(mut reader: OwnedReadHalf) {
    loop {
        let mut buffer = [0; 1024];

        let data = match reader.read(&mut buffer).await {
            Err(e) => {
                eprintln!("[-] Failed to receive data from server. Error {e:?}");
                continue;
            }

            Ok(0) => {
                println!("[-] Server has disconnected");
                break;
            }

            Ok(data) => data,
        };

        let message = String::from_utf8_lossy(&buffer[..data]);
        println!("received: {}", message.trim());
    }
}
