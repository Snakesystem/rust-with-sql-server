use tiberius::QueryItem;
use tokio_stream::StreamExt; // digunakan untuk menambahkan method try_next()
use crate::contexts::connection::connect_pool;

// CREATE PROCEDURE THAT RETURN STATUS CODE
pub async fn create_procedure_returns_status_code() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    // The `return` keyword is optional in SQL Server stored procedures,
    // if not specified, the database engine returns 0
    let _ = client
        .simple_query(
            r#"
create procedure dbo.uspUpdateOrderStatus @SalesOrderID int,
                                      @Status int
as
begin
    begin try
        update SalesOrderHeader
        set Status=@Status
        where SalesOrderID = @SalesOrderID
        return 0
    end try
    begin catch
        return -2
    end catch
end
    "#,
        )
        .await?;

    println!("Created stored procedure that returns a status code.");

    Ok(())
}

#[tokio::test]
async fn create_stored_procedure_returns_status_code_in_sql_server() {
    let result = create_procedure_returns_status_code().await;
    assert_eq!(result.is_ok(), true);
}

// EXECUTE STORED PROCEDURE THAT RETURN STATUS CODE
pub async fn call_stored_procedure_returns_status_code() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    let sales_order_id: i32 = 2;
    let status: i32 = 16;

    // It is trying to change the status of a sales order
    // valid values must be between 0 and 8
    let mut result = client
        .query(
            r#"
        declare @ReturnCode int
        exec @ReturnCode = dbo.uspUpdateOrderStatus @SalesOrderID = @P1, @Status = @P2
        select @ReturnCode as ReturnCode
        "#,
            &[
                &sales_order_id, // SalesOrderID
                &status,         // Status
            ],
        )
        .await?;

    // Check the return value
    while let Some(row) = result.try_next().await? {
        if let QueryItem::Row(r) = row {
            let return_code: i32 = r.get("ReturnCode").unwrap();

            if return_code == 0 {
                println!(
                    "Sales order with ID={} updated to {}.",
                    sales_order_id, status
                );
            } else {
                // The return code is -2, which means the stored procedure failed
                // You can fail the function as well
                println!("The status {} is not valid.", status);
            }
        }
    }

    Ok(())
}

#[tokio::test]
async fn call_stored_procedure_returns_status_code_in_sql_server() {
    let result = call_stored_procedure_returns_status_code().await;
    assert_eq!(result.is_ok(), true);
}
