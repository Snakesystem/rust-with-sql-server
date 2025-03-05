use crate::contexts::connection::connect_pool;

// QUERY TO INSERT ROW
pub async fn insert_row() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    let result = client
        .execute(
            r#"INSERT INTO dbo.SalesOrderHeader (
        RevisionNumber, OrderDate, DueDate, ShipDate, Status, CreditCardApprovalCode, 
        SubTotal, TaxAmt, Freight, Comment, rowguid, ModifiedDate
        ) 
        VALUES 
        (
        @P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9, @P10, @P11, @P12
        );"#,
            // These are the values for the parameters in the
            // INSERT statement
            &[
                &8i32,                                   // RevisionNumber
                &"2024-07-30",                           // OrderDate
                &"2024-08-12",                           // DueDate
                &"2024-07-07",                           // ShipDate
                &5i32,                                   // Status
                &"105041Vi84182",                        // CreditCardApprovalCode
                &20565.6206f64,                          // SubTotal
                &1971.5149f64,                           // TaxAmt
                &616.0984f64,                            // Freight
                &None::<&str>,                           // Comment
                &"79B65321-39CA-4115-9CBA-8FE0903E12E6", // rowguid
                &"2024-07-07",                           // ModifiedDate
            ],
        )
        .await?;

    println!("Rows affected: {}", result.total());

    client.close().await?;

    Ok(())
}

#[tokio::test]
async fn insert_row_in_sql_server(){
    let result = insert_row().await;
    assert_eq!(result.is_ok(), true);
}