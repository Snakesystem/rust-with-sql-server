use tiberius::QueryItem;
use tokio_stream::StreamExt; // digunakan untuk menambahkan method try_next()
use crate::contexts::connection::connect_pool;

// CREATE STORED PROCEDURE WITH PARAMETER OUTPUT
pub async fn create_stored_procedure_output_parameter() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    // @SalesOrderID is the output parameter of the stored procedure
    let _ = client
        .simple_query(
            r#"
create procedure dbo.uspSaveOrderHeaderGetID @DueDate datetime,
                                             @ShipDate datetime,
                                             @CreditCardApprovalCode varchar(15),
                                             @Comment nvarchar(128) = null,
                                             @ModifiedDate datetime,
                                             @SalesOrderID int output
as
begin
    insert dbo.SalesOrderHeader(DueDate, ShipDate, CreditCardApprovalCode, Comment, ModifiedDate)
    values (@DueDate, @ShipDate, @CreditCardApprovalCode, @Comment, @ModifiedDate)

    set @SalesOrderID = @@identity
end
    "#,
        )
        .await?;

    println!("Created stored procedure with output parameter.");

    Ok(())
}

#[tokio::test]
async fn create_stored_procedure_output_parameter_in_sql_server() {
    let result = create_stored_procedure_output_parameter().await;
    assert_eq!(result.is_ok(), true);
}

// EXECUTE STORED PROCEDURE WITH PARAMETER OUTPUT
pub async fn call_stored_procedure_output_parameter() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    let mut result = client
        .query(
            r#"
        declare @NewSalesOrderID int
        exec dbo.uspSaveOrderHeaderGetID
        @DueDate = @P1,
        @ShipDate = @P2,
        @CreditCardApprovalCode = @P3,
        @ModifiedDate = @P4,
        @SalesOrderID = @NewSalesOrderID output

        select @NewSalesOrderID as SalesOrderID
        "#,
            &[
                &"2024-09-12", // DueDate
                &"2024-09-22", // ShipDate
                &"10045AV521", // CreditCardApprovalCode
                &"2024-09-12", // ModifiedDate
            ],
        )
        .await?;

    // Since the output parameter (@SalesOrderID) is into a result set,
    // we need to iterate over it to get the actual output value
    while let Some(row) = result.try_next().await? {
        if let QueryItem::Row(r) = row {
            let sales_order_id: i32 = r.get("SalesOrderID").unwrap();
            println!("Sale order created with ID: {}", sales_order_id);
        }
    }

    Ok(())
}

#[tokio::test]
async fn call_stored_procedure_output_parameter_in_sql_server() {
    let result = call_stored_procedure_output_parameter().await;
    assert_eq!(result.is_ok(), true);
}