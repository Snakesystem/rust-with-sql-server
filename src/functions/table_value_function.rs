use tiberius::QueryItem;
use tokio_stream::StreamExt; // digunakan untuk menambahkan method try_next()
use crate::contexts::connection::connect_pool;

// TABLE VALUE FUNCTION
pub async fn create_table_valued_function() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    let _ = client
        .simple_query(
            r#"
create function dbo.ufnGetSalesOrderWithTotalDueMoreThan(@TotalDue money)
    returns table
        as
        return
        select SalesOrderID,
               SubTotal,
               TaxAmt,
               Freight,
               TotalDue
        from dbo.SalesOrderHeader
        where
            TotalDue > @TotalDue
    "#,
        )
        .await?;

    println!("Created table valued function.");

    Ok(())
}

#[tokio::test]
async fn create_table_valued_function_in_sql_server() {
    let result = create_table_valued_function().await;
    assert_eq!(result.is_ok(), true);
}

// EXECUTE TABLE VALUE FUNCTION
pub async fn call_table_valued_function() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    let due = -1;

    // Query the Sql Server function as if it was a normal table
    // using the `select` statement
    let mut query = client
        .query(
            r#"
select SalesOrderID,
       SubTotal,
       TaxAmt,
       Freight,
       TotalDue
from dbo.ufnGetSalesOrderWithTotalDueMoreThan(@P1)
    "#,
            &[&due], // TotalDue
        )
        .await?;

    // Iterate over the result set
    while let Some(row) = query.try_next().await? {
        if let QueryItem::Row(r) = row {
            let sales_order_id: i32 = r.get("SalesOrderID").unwrap();
            let subtotal: f64 = r.get("SubTotal").unwrap();
            let tax_amt: f64 = r.get("TaxAmt").unwrap();
            let freight: f64 = r.get("Freight").unwrap();
            let total_due: f64 = r.get("TotalDue").unwrap();
            println!("Sale order with ID: {}", sales_order_id);
            println!("Subtotal: {}", subtotal);
            println!("Tax amount: {}", tax_amt);
            println!("Freight: {}", freight);
            println!("Total due: {}", total_due);
        }
    }

    Ok(())
}

#[tokio::test]
async fn call_table_valued_function_in_sql_server() {
    let result = call_table_valued_function().await;
    assert_eq!(result.is_ok(), true);
}