use tiberius::ExecuteResult;

use crate::contexts::connection::{connect_pool, BeginTransaction};

pub async fn delete_row_with_transaction() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = connect_pool("S21Notes").await?;

    let mut transaction: BeginTransaction = BeginTransaction::new(&mut connection).await?;

    let result: ExecuteResult = transaction.execute(
        r#"DELETE FROM dbo.SalesOrderHeader
                WHERE
            SalesOrderID = @P1;"#, &[&9i32],
        ).
        await.inspect_err(|err| println!("Error: {}", err))?;
    println!("Rows affected: {}", result.total());

    transaction.commit().await?;

    Ok(())
}

#[tokio::test]
async fn delete_row_with_transaction_in_sql_server() {
    let result = delete_row_with_transaction().await;
    assert_eq!(result.is_ok(), true);
}