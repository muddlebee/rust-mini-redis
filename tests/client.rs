use mini_redis::{clients::Client, server};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

/// A PING PONG test without message provided.
/// It should return "PONG".
#[tokio::test]
async fn ping_pong_without_message() {
    let (addr, _) = start_server().await;
    let mut client = Client::connect(addr).await.unwrap();

    let pong = client.ping(None).await.unwrap();
    assert_eq!(b"PONG", &pong[..]);
}

/// A PING PONG test with message provided.
/// It should return the message.
#[tokio::test]
async fn ping_pong_with_message() {
    let (addr, _) = start_server().await;
    let mut client = Client::connect(addr).await.unwrap();

    let pong = client.ping(Some("你好世界".into())).await.unwrap();
    assert_eq!("你好世界".as_bytes(), &pong[..]);
}

/// A basic "hello world" style test. A server instance is started in a
/// background task. A client instance is then established and set and get
/// commands are sent to the server. The response is then evaluated
#[tokio::test]
async fn key_value_get_set() {
    let (addr, _) = start_server().await;

    let mut client = Client::connect(addr).await.unwrap();
    client.set("hello", "world".into()).await.unwrap();

    let value = client.get("hello").await.unwrap().unwrap();
    assert_eq!(b"world", &value[..])
}

/// similar to the "hello world" style test, But this time
/// a single channel subscription will be tested instead
#[tokio::test]
async fn receive_message_subscribed_channel() {
    let (addr, _) = start_server().await;

    let client = Client::connect(addr).await.unwrap();
    let mut subscriber = client.subscribe(vec!["hello".into()]).await.unwrap();

    tokio::spawn(async move {
        let mut client = Client::connect(addr).await.unwrap();
        client.publish("hello", "world".into()).await.unwrap()
    });

    let message = subscriber.next_message().await.unwrap().unwrap();
    assert_eq!("hello", &message.channel);
    assert_eq!(b"world", &message.content[..])
}

/// test that a client gets messages from multiple subscribed channels
#[tokio::test]
async fn receive_message_multiple_subscribed_channels() {
    let (addr, _) = start_server().await;

    let client = Client::connect(addr).await.unwrap();
    let mut subscriber = client
        .subscribe(vec!["hello".into(), "world".into()])
        .await
        .unwrap();

    tokio::spawn(async move {
        let mut client = Client::connect(addr).await.unwrap();
        client.publish("hello", "world".into()).await.unwrap()
    });

    let message1 = subscriber.next_message().await.unwrap().unwrap();
    assert_eq!("hello", &message1.channel);
    assert_eq!(b"world", &message1.content[..]);

    tokio::spawn(async move {
        let mut client = Client::connect(addr).await.unwrap();
        client.publish("world", "howdy?".into()).await.unwrap()
    });

    let message2 = subscriber.next_message().await.unwrap().unwrap();
    assert_eq!("world", &message2.channel);
    assert_eq!(b"howdy?", &message2.content[..])
}

/// test that a client accurately removes its own subscribed chanel list
/// when unsubscribing to all subscribed channels by submitting an empty vec
#[tokio::test]
async fn unsubscribes_from_channels() {
    let (addr, _) = start_server().await;

    let client = Client::connect(addr).await.unwrap();
    let mut subscriber = client
        .subscribe(vec!["hello".into(), "world".into()])
        .await
        .unwrap();

    subscriber.unsubscribe(&[]).await.unwrap();
    assert_eq!(subscriber.get_subscribed().len(), 0);
}

/// test when hset command is sent to the server
/// the server will store the key, field and value
/// and return "OK" to the client
#[tokio::test]
async fn hset_command() {
    let (addr, _) = start_server().await;

    let mut client = Client::connect(addr).await.unwrap();
    client
        .hset(&"hello".to_string(), &"world".to_string(), "你好世界".into()).await.unwrap();

    let value = client.hget(&"hello".to_string(), &"world".to_string()).await.unwrap().unwrap();
    println!("value {:?}", value);
    assert_eq!("你好世界".as_bytes(), &value[..])
}

/*/// test for hgetall command
/// the server will return all the key-value pairs
#[tokio::test]
async fn hgetall_c() {
    let (addr, _) = start_server().await;

    let mut client = Client::connect(addr).await.unwrap();
    client
        .hset(&"hello".to_string(), &"world".to_string(), "你好世界".into()).await.unwrap();

    //return all the key-value pairs
    let value = client.hgetall(&"hello".to_string()).await.unwrap().unwrap();
    println!("value {:?}", value);

    //assert the key-value pairs
    assert_eq!(value.get(&"world".to_string()).unwrap().as_ref(), "你好世界".as_bytes());


}*/

async fn start_server() -> (SocketAddr, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let handle = tokio::spawn(async move { server::run(listener, tokio::signal::ctrl_c()).await });

    (addr, handle)
}
