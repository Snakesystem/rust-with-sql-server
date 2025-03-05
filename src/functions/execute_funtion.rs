use tiberius::QueryItem;
use tokio_stream::StreamExt; // digunakan untuk menambahkan method try_next()
use crate::contexts::connection::connect_pool;

// EXECUTE FUNCTION IN SQL
pub async fn call_scalar_function() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    let mut query = client
        .query(
            r#"
SELECT SalesOrderID, Status, dbo.ufnGetSalesOrderStatusText(Status) AS StatusDescription
FROM dbo.SalesOrderHeader
WHERE SalesOrderID = @P1
    "#,
            &[&2], // SalesOrderID
        )
        .await?;

    // Read the rows returned by the SQL Server function
    while let Some(row) = query.try_next().await? {
        if let QueryItem::Row(r) = row {
            let sales_order_id: i32 = r.get("SalesOrderID").unwrap();
            let status: u8 = r.get("Status").unwrap();
            let status_description: &str = r.get("StatusDescription").unwrap();
            println!("Sale order created with ID: {}", sales_order_id);
            println!("Status: {}", status);
            println!("Status description: {}", status_description);
        }
    }

    Ok(())
}

#[tokio::test]
async fn call_scalar_function_in_sql_server() {
    let result = call_scalar_function().await;
    assert_eq!(result.is_ok(), true);
}