use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ServerConfig
{
    workers: u64,
    ignore: bool,
    auth_server: Option<String>,
}

fn main() {
    let config = ServerConfig{
        workers: 100,
        ignore: false,
        auth_server: Some("auth.server.io".to_string()),
    };

    {
        println!("json:");
        let serialize = serde_json::to_string(&config).unwrap();
        println!("serialize:{}",serialize);

        let deserialize: ServerConfig = serde_json::from_str(&serialize).unwrap();
        println!("deserialize:{:#?}",deserialize);

    }


    {
        println!("yaml:");
        let serialize = serde_yaml::to_string(&config).unwrap();
        println!("serialize:{}",serialize);

        let deserialize: ServerConfig = serde_yaml::from_str(&serialize).unwrap();
        println!("deserialize:{:#?}",deserialize);

    }


    println!("Hello, world!");
}
