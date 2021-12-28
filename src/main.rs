use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8000").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            let msg = format!("{:?} has has entered the chat\r\n", addr);
            print!("{}", msg);
            tx.send((addr, msg)).unwrap();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        match result {
                            Ok(0..=2) => {
                                let msg = format!("{:?} has left the chat\r\n", addr);
                                print!("{}", msg);
                                tx.send((addr, msg)).unwrap();
                                break;
                            }
                            Ok(_) => {
                                let msg = format!("{:?}: {}\r\n", addr, line.trim());
                                print!("{}", msg);
                                tx.send((addr, msg)).unwrap();
                            }
                            Err(error) => {
                                print!("error: {:?}", error);
                                break;
                            }
                        }
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (other_addr, msg) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
