use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt as _;

// CONNECTION WITH HOST, PORT, USER NAME AND PASSWORD 
pub async fn connect_with_host_port_username_password() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();

    // Use SQL Server Authentication (user name and password)
    config.authentication(AuthMethod::sql_server("s21+", "diehards21+"));

    config.host("172.24.25.60\\SQL17");
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    let client = Client::connect(config, tcp.compat_write()).await?;
    println!("Connected to SQL Server");
    let _ = client.close().await?;

    Ok(())
}

#[tokio::test]
async fn connect_to_sql_server_using_host_port_username_password() {
    let result = connect_with_host_port_username_password().await;
    assert_eq!(result.is_ok(), true);
}

// CONNECTION WITH ADO.NET CONNECTION STRING
pub async fn connect_with_ado_sql_browser() -> Result<(), Box<dyn std::error::Error>> {
    // It uses an ADO.NET connection string to connect to SQL Server.
    // Replace with your actual connection string
    let config = Config::from_ado_string(
        &"Server=tcp:172.24.25.60\\SQL17;User=s21+;Password=diehards21+;TrustServerCertificate=true;Database=S21Notes;",
    )?;

    // Jika ingin mebambahkan database
    // Server=tcp:172.24.25.60\\SQL17;User=s21+;Password=diehards21+;TrustServerCertificate=true;Database=S21Notes;".replace("{}", database)

    let tcp = <TcpStream as tiberius::SqlBrowser>::connect_named(&config).await?;
    let _ = tcp.set_nodelay(true);

    let client = Client::connect(config, tcp.compat_write()).await?;
    println!("Connected to SQL Server");
    let _ = client.close().await;

    Ok(())
}

#[tokio::test]
async fn connect_to_sql_server_using_ado_sql_browser() {
    let result = connect_with_ado_sql_browser().await;
    assert_eq!(result.is_ok(), true);
}
