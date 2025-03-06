use crate::contexts::connection::{begin_transaction, commit_transaction, connect_pool, rollback_transaction};

pub async fn insert_row_with_transaction() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    // Mulai transaksi manual
    begin_transaction(&mut client).await?;

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
            &[
                &8i32, &"2024-07-30", &"2024-08-12", &"2024-07-07", &5i32, 
                &"105041Vi84182", &20565.6206f64, &1971.5149f64, &616.0984f64, 
                &None::<&str>, &"79B65321-39CA-4115-9CBA-8FE0903E12E6", &"2024-07-07"
            ],
        )
        .await;

    match result {
        Ok(res) => {
            println!("Rows affected: {}", res.total());
            commit_transaction(&mut client).await?;
        }
        Err(err) => {
            println!("Error: {:?}", err);
            rollback_transaction(&mut client).await?;
        }
    }
    client.close().await?;
    Ok(())
}


#[tokio::test]
async fn insert_row_with_transaction_in_sql_server() {
    let result = insert_row_with_transaction().await;
    assert_eq!(result.is_ok(), true);
}
