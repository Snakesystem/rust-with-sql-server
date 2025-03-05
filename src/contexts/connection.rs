use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub async fn connect_pool(database: &str) -> Result<Client<Compat<TcpStream>>, Box<dyn std::error::Error>> {
    // It uses an ADO.NET connection string to connect to SQL Server.
    // Replace with your actual connection string
    let config = Config::from_ado_string(
        &"Server=tcp:172.24.25.60\\SQL17;User=s21+;Password=diehards21+;TrustServerCertificate=true;Database={};".replace("{}", database),
    )?;

    let tcp = <TcpStream as tiberius::SqlBrowser>::connect_named(&config).await?;
    // let _ = tcp.set_nodelay(true);

    let client = Client::connect(config, tcp.compat_write()).await?;
    println!("Connected to SQL Server");
    // let _ = client.close().await;

    Ok(client)
}