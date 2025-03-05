use crate::contexts::connection::connect_pool;

// QUERY DELETE ROW
pub async fn delete_row() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    // Delete the sale with order ID equals to 1
    let result = client
        .execute(
            r#"DELETE FROM dbo.SalesOrderHeader
WHERE
    SalesOrderID = @P1;"#,
            // this will be the value of the parameter @P1
            &[&1i32],
        )
        .await?;

    println!("Rows affected: {}", result.total());

    client.close().await?;

    Ok(())
}

#[tokio::test]
async fn delete_row_in_sql_server() {
    let result = delete_row().await;
    assert_eq!(result.is_ok(), true);
}