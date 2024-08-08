use crate::helpers::{spawn_server, connect_server};
use common::pq::{send, recv, RequestMessage, RequestCode, ResponseCode};

#[tokio::test]
async fn server_echos_when_client_sends() {
    // arrange
    let addr = spawn_server().await;
    let mut client = connect_server(&addr).await.unwrap();

    // act
    let message = RequestMessage {
        code: RequestCode::Query,
        payload: "select * from dual".into(),
    };
    send(&mut client, &message).await.unwrap();
    let response = recv(&mut client).await.unwrap();

    // assert
    assert_eq!(ResponseCode::Complete, response.code);
    assert_eq!(message.payload, response.payload);

}
