use tiberius::{Client, ExecuteResult};
use tokio::net::TcpStream;
use tokio_util::compat::Compat;
use crate::contexts::connection::{connect_pool, BeginTransaction};

// QUERY UPDATE ROW
pub async fn update_row_with_transaction() -> Result<(), Box<dyn std::error::Error>> {
    let mut client: Client<Compat<TcpStream>> = connect_pool("S21Notes").await?;
    
    let mut transaction: BeginTransaction = BeginTransaction::new(&mut client).await?;

    // Sekarang bisa langsung pakai `transaction` seolah-olah itu `client`
    let result: ExecuteResult = transaction
        .execute(
            r#"UPDATE dbo.SalesOrderHeader
            SET
                RevisionNumber = @P1,
                OrderDate = @P2,
                DueDate = @P3,
                ShipDate = @P4,
                Status = @P5,
                CreditCardApprovalCode = @P6,
                SubTotal = @P7,
                TaxAmt = @P8,
                Freight = @P9,
                Comment = @P10,
                rowguid = @P11,
                ModifiedDate = @P12
            WHERE
        SalesOrderID = @P13;"#,
            &[
                &4i32,                                             // RevisionNumber
                &"2024-07-31",                                     // OrderDate
                &"2024-08-12",                                     // DueDate
                &"2024-07-07",                                     // ShipDate
                &5i32,                                             // Status
                &"105041Vi84182",                                  // CreditCardApprovalCode
                &20565.6206f64,                                    // SubTotal
                &1333971.5149f64,                                     // TaxAmt
                &616.0984f64,                                      // Freight
                &"I updated this row from a Rust .NET application.", // Comment
                &"6d805000-034b-421e-8489-9168b7fe3de6",           // rowguid
                &"2024-07-07",                                     // ModifiedDate
                &3i32,                                             // SalesOrderID
            ],
        )
        .await.inspect_err(|err| println!("Error: {}", err))?;

    println!("Rows affected: {}", result.total());

    transaction.commit().await?;

    Ok(())
}

#[tokio::test]
async fn update_row_in_sql_server() {
    let result = update_row_with_transaction().await;
    assert_eq!(result.is_ok(), true);
}