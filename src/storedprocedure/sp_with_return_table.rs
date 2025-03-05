use tiberius::QueryItem;
use tokio_stream::StreamExt; // digunakan untuk menambahkan method try_next()
use crate::contexts::connection::connect_pool;


// CREATE PROCEDURE THAT RETURN TABLE
pub async fn create_stored_procedure_returns_table() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    let _ = client
        .simple_query(
            r#"
create procedure dbo.uspGetSaleOrderByID @SalesOrderID int
as
begin

    select SalesOrderID,
           RevisionNumber,
           OrderDate,
           DueDate,
           ShipDate,
           Status,
           SalesOrderNumber,
           CreditCardApprovalCode,
           SubTotal,
           TaxAmt,
           Freight,
           TotalDue,
           Comment,
           rowguid,
           ModifiedDate
    into #saleorder
    from dbo.SalesOrderHeader
    where
        SalesOrderID=@SalesOrderID

    -- Get sale order detail
    select SalesOrderID,
           RevisionNumber,
           OrderDate,
           DueDate,
           ShipDate,
           Status,
           SalesOrderNumber,
           CreditCardApprovalCode,
           SubTotal,
           TaxAmt,
           Freight,
           TotalDue,
           Comment,
           rowguid,
           ModifiedDate
    from #saleorder

    -- Get receipt (summary)
    select SalesOrderID,
           OrderDate,
           SubTotal,
           TaxAmt,
           Freight,
           TotalDue
    from #saleorder
end
    "#,
        )
        .await?;

    println!("Created stored procedure that returns a table.");

    Ok(())
}

#[tokio::test]
async fn call_create_stored_procedure_returns_table_in_sql_server() {
    let result = create_stored_procedure_returns_table().await;
    assert_eq!(result.is_ok(), true);
}

// EXECUTE STORE PROCEDURE THAT RETURN TABLE
pub async fn call_stored_procedure_returns_table() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    let mut result = client
        .query(
            r#"
        exec dbo.uspGetSaleOrderByID @SalesOrderID = @P1
        "#,
            &[&2i32], // SalesOrderID
        )
        .await?;

    while let Some(row) = result.try_next().await? {
        if let QueryItem::Row(r) = row {
            let number_columns = r.columns().iter().count();

            // The index identifies the result set
            // (0 for the first result set, 1 for the second, and so on)
            // If the stored procedure returns multiple result sets,
            // you can handle them accordingly
            let result_index = r.result_index();
            println!("Number of columns: {}", number_columns);
            println!("Result index: {}", result_index);

            if result_index == 0 {
                println!("Sale order details:");

                // Print each column value
                let sales_order_id: i32 = r.get("SalesOrderID").unwrap();
                let revision_number: u8 = r.get("RevisionNumber").unwrap();
                let order_date: chrono::NaiveDateTime = r.get("OrderDate").unwrap();
                let due_date: chrono::NaiveDateTime = r.get("DueDate").unwrap();
                let ship_date: chrono::NaiveDateTime = r.get("ShipDate").unwrap();
                let status: u8 = r.get("Status").unwrap();
                let sales_order_number: &str = r.get("SalesOrderNumber").unwrap();
                let credit_card_approval_code: &str = r.get("CreditCardApprovalCode").unwrap();
                let subtotal: f64 = r.get("SubTotal").unwrap();
                let tax_amt: f64 = r.get("TaxAmt").unwrap();
                let freight: f64 = r.get("Freight").unwrap();
                let total_due: f64 = r.get("TotalDue").unwrap();
                let comment: &str = r.get("Comment").unwrap_or_else(|| "");
                let rowguid: uuid::Uuid = r.get("rowguid").unwrap();
                let modified_date: chrono::NaiveDateTime = r.get("ModifiedDate").unwrap();

                println!("Sale order ID: {}", sales_order_id);
                println!("Revision number: {}", revision_number);
                println!("Order date: {}", order_date);
                println!("Due date: {}", due_date);
                println!("Ship date: {}", ship_date);
                println!("Status: {}", status);
                println!("Sales order number: {}", sales_order_number);
                println!("Credit card approval code: {}", credit_card_approval_code);
                println!("Subtotal: {}", subtotal);
                println!("Tax amount: {}", tax_amt);
                println!("Freight: {}", freight);
                println!("Total due: {}", total_due);
                println!("Comment: {}", comment);
                println!("Row GUID: {}", rowguid);
                println!("Modified date: {}", modified_date);
                println!();
            } else if result_index == 1 {
                println!("Receipt summary:");

                // Print each column value
                let sales_order_id: i32 = r.get("SalesOrderID").unwrap();
                let order_date: chrono::NaiveDateTime = r.get("OrderDate").unwrap();
                let subtotal: f64 = r.get("SubTotal").unwrap();
                let tax_amt: f64 = r.get("TaxAmt").unwrap();
                let freight: f64 = r.get("Freight").unwrap();
                let total_due: f64 = r.get("TotalDue").unwrap();

                println!("Sale order ID: {}", sales_order_id);
                println!("Order date: {}", order_date);
                println!("Subtotal: {}", subtotal);
                println!("Tax amount: {}", tax_amt);
                println!("Freight: {}", freight);
                println!("Total due: {}", total_due);
                println!();
            }
        }
    }
    Ok(())
}

#[tokio::test]
async fn call_stored_procedure_returns_table_in_sql_server() {
    let result = call_stored_procedure_returns_table().await;
    assert_eq!(result.is_ok(), true);
}