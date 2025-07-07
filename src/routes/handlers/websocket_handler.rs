use actix_web::{rt, web, HttpRequest, HttpResponse, Error};
use actix_ws::{Message, handle};
use futures_util::StreamExt as _;

/// WebSocket handler using modern actix-web v4 approach
pub async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, mut msg_stream) = handle(&req, stream)?;

    rt::spawn(async move {
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Text(text) => {
                    session.text(text).await.ok();
                }
                Message::Binary(bin) => {
                    session.binary(bin).await.ok();
                }
                Message::Ping(msg) => {
                    session.pong(&msg).await.ok();
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(res)
} 