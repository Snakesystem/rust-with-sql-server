use crate::contexts::connection::connect_pool;

// CREATE FUNCTION IN SQL
pub async fn create_scalar_function() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    let _ = client
        .simple_query(
            r#"
create function dbo.ufnGetSalesOrderStatusText(@Status tinyint)
    returns nvarchar(15)
as
-- Returns the sales order status text representation for the status value.
begin
    declare @ret [nvarchar](15)

    SET @ret =
            CASE @Status
                WHEN 1 THEN 'In process'
                WHEN 2 THEN 'Approved'
                WHEN 3 THEN 'Backordered'
                WHEN 4 THEN 'Rejected'
                WHEN 5 THEN 'Shipped'
                WHEN 6 THEN 'Cancelled'
                ELSE '** Invalid **'
                end

    return @ret
end
    "#,
        )
        .await;

    println!("Created scalar function.");

    Ok(())
}

#[tokio::test]
async fn call_create_function_in_sql_server() {
    let result = create_scalar_function().await;
    assert_eq!(result.is_ok(), true);
}