use crate::contexts::connection::connect_pool;

// CREATE PROCEDURE
pub async fn create_stored_procedure() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    let _ = client
        .simple_query(
            r#"
create procedure dbo.uspSaveOrderHeader @DueDate datetime,
                                        @ShipDate datetime,
                                        @CreditCardApprovalCode varchar(15),
                                        @Comment nvarchar(128) = null,
                                        @ModifiedDate datetime
as
begin
    insert dbo.SalesOrderHeader(DueDate, ShipDate, CreditCardApprovalCode, Comment, ModifiedDate)
    values (@DueDate, @ShipDate, @CreditCardApprovalCode, @Comment, @ModifiedDate)
end

    "#,
        )
        .await?;

    println!("Created stored procedure.");

    Ok(())
}

#[tokio::test]
async fn create_stored_procedure_in_sql_server() {
    let result = create_stored_procedure().await;
    assert_eq!(result.is_ok(), true);
}