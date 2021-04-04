use tiberius::{Config, Client, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use bigdecimal::{BigDecimal, Zero};

#[derive(Debug)]
struct Customer {
    id: i32,
    rand_string: String,
    rand_decimal: BigDecimal,
    rand_bit: bool,
}

#[cfg(not(all(windows, feature = "sql-browser-tokio")))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::new();

    config.host("0.0.0.0");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("SA", "Password!123"));
    config.trust_cert();

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let stream = client.query("SELECT * FROM [test_db].[source].[customer]", &[]).await?;
    let rows = stream.into_first_result().await?;

    let mut customers = Vec::new();

    for row in rows {
        customers.push(Customer {
            id: row
                .try_get("id")?
                .unwrap_or_else(|| 0),
            rand_string: row
                // example of how to use the `::<_>` to specify the type we want to get out of the column.
                .try_get::<&str, _>("rand_string")?
                .unwrap_or_else(|| "unknown")
                .to_string(),
            rand_decimal: row
                .try_get("rand_decimal")?
                .unwrap_or_else(|| BigDecimal::zero()),
            rand_bit: row
                .try_get("rand_bit")?
                .unwrap_or_else(|| false),
        })
    }

    println!("{:#?}", customers);

    Ok(())
}
