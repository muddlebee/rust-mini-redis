use mini_redis::{clients::Client, DEFAULT_PORT};

use bytes::Bytes;
use clap::{Parser, Subcommand};
use std::convert::Infallible;
use std::num::ParseIntError;
use std::str;
use std::time::Duration;

#[derive(Parser, Debug)]
#[clap(
    name = "mini-redis-cli",
    version,
    author,
    about = "Issue Redis commands"
)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(name = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[clap(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[derive(Subcommand, Debug)]
enum Command {
    Ping {
        /// Message to ping
        #[clap(value_parser = bytes_from_str)]
        msg: Option<Bytes>,
    },
    /// Get the value of key.
    Get {
        /// Name of key to get
        key: String,
    },
    /// Set key to hold the string value.
    Set {
        /// Name of key to set
        key: String,

        /// Value to set.
        #[clap(value_parser = bytes_from_str)]
        value: Bytes,

        /// Expire the value after specified amount of time
        #[clap(value_parser = duration_from_ms_str)]
        expires: Option<Duration>,
    },
    ///  Publisher to send a message to a specific channel.
    Publish {
        /// Name of channel
        channel: String,

        #[clap(value_parser = bytes_from_str)]
        /// Message to publish
        message: Bytes,
    },
    /// Subscribe a client to a specific channel or channels.
    Subscribe {
        /// Specific channel or channels
        channels: Vec<String>,
    },

    /// HSET key field value [field value ...]
    /// Sets the specified fields to their respective values in the hash stored at key.
    HSet {
        /// Name of key to set
        key: String,

        /// Name of field to set
        field: String,

        /// Value to set.
        #[clap(value_parser = bytes_from_str)]
        value: Bytes,
    },

    /// HGET key field
    /// Returns the value associated with field in the hash stored at key.
    /// If the key or the field do not exist, nil is returned.
    HGet {
        /// Name of key to set
        key: String,

        /// Name of field to set
        field: String,
    },

    ///HGETALL key
    /// Returns all fields and values of the hash stored at key.
    /// In the returned value, every field name is followed by its value, so the length of the reply is twice the size of the hash.
    HGetAll {
        /// Name of key to set
        key: String,
    },
}

/// Entry point for CLI tool.
///
/// The `[tokio::main]` annotation signals that the Tokio runtime should be
/// started when the function is called. The body of the function is executed
/// within the newly spawned runtime.
///
/// `flavor = "current_thread"` is used here to avoid spawning background
/// threads. The CLI tool use case benefits more by being lighter instead of
/// multi-threaded.
#[tokio::main(flavor = "current_thread")]
async fn main() -> mini_redis::Result<()> {
    // Enable logging
    tracing_subscriber::fmt::try_init()?;

    // Parse command line arguments
    let cli = Cli::parse();

    // Get the remote address to connect to
    let addr = format!("{}:{}", cli.host, cli.port);

    // Establish a connection
    let mut client = Client::connect(&addr).await?;

    // Process the requested command
    match cli.command {
        Command::Ping { msg } => {
            let value = client.ping(msg).await?;
            if let Ok(string) = str::from_utf8(&value) {
                println!("\"{}\"", string);
            } else {
                println!("{:?}", value);
            }
        }
        Command::Get { key } => {
            if let Some(value) = client.get(&key).await? {
                if let Ok(string) = str::from_utf8(&value) {
                    println!("\"{}\"", string);
                } else {
                    println!("{:?}", value);
                }
            } else {
                println!("(nil)");
            }
        }
        Command::Set {
            key,
            value,
            expires: None,
        } => {
            client.set(&key, value).await?;
            println!("OK");
        }
        Command::Set {
            key,
            value,
            expires: Some(expires),
        } => {
            client.set_expires(&key, value, expires).await?;
            println!("OK");
        }
        Command::Publish { channel, message } => {
            client.publish(&channel, message).await?;
            println!("Publish OK");
        }
        Command::Subscribe { channels } => {
            if channels.is_empty() {
                return Err("channel(s) must be provided".into());
            }
            let mut subscriber = client.subscribe(channels).await?;

            // await messages on channels
            while let Some(msg) = subscriber.next_message().await? {
                println!(
                    "got message from the channel: {}; message = {:?}",
                    msg.channel, msg.content
                );
            }
        }
        Command::HSet { key, field, value } => {
            client.hset(&key, &field, value).await?;
            println!("HSet OK");
        }
        Command::HGet { key, field } => {
            if let Some(value) = client.hget(&key, &field).await? {
                if let Ok(string) = str::from_utf8(&value) {
                    println!("\"{}\"", string);
                } else {
                    println!("{:?}", value);
                }
            } else {
                println!("(nil)");
            }
        }
        Command::HGetAll { key } => {

            // Assuming `result` is of type `Option<HashMap<String, Bytes>>`
            if let Some(hash_map) = client.hgetall(&key).await? {
                for (key, value) in hash_map {
                    // Convert `Bytes` to a string slice for display or processing
                    if let Ok(value_str) = str::from_utf8(&value) {
                        println!("Key: {}, Value: {}", key, value_str);
                    } else {
                        println!("Key: {}, Value: [non-UTF8 data]", key);
                    }
                }
            } else {
                // Handle the case where the key does not exist or another error occurred
                return Err("key does not exist".into());
            }
        }
    }

    Ok(())
}

fn duration_from_ms_str(src: &str) -> Result<Duration, ParseIntError> {
    let ms = src.parse::<u64>()?;
    Ok(Duration::from_millis(ms))
}

fn bytes_from_str(src: &str) -> Result<Bytes, Infallible> {
    Ok(Bytes::from(src.to_string()))
}
