use async_trait::async_trait;
use ezsockets::ClientConfig;
use std::io::BufRead;

struct Client {}

#[async_trait]
impl ezsockets::ClientExt for Client {
    type Call = ();

    async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        tracing::info!("received message: {text}");
        Ok(())
    }

    async fn on_binary(&mut self, bytes: Vec<u8>) -> Result<(), ezsockets::Error> {
        tracing::info!("received bytes: {bytes:?}");
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), ezsockets::Error> {
        let () = call;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = ClientConfig::new("ws://localhost:8080/websocket");
    // 如果加上这句话则会触发 服务端的 kick_me 逻辑
    // config = config.query_parameter("kick_me", "Yes");
    // 调用 connect 方法连接 socket
    let (handle, future) = ezsockets::connect(|_client| Client {}, config).await;
    tokio::spawn(async move {
        future.await.unwrap();
    });

    // 监听标准输入，如果有输入的话则通过 websocket 发送消息
    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        let line = line.unwrap();
        tracing::info!("sending {line}");
        handle.text(line).unwrap();
    }
}
