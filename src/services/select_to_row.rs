use tiberius::{Query, QueryItem};
use tokio_stream::StreamExt; // digunakan untuk menambahkan method try_next()
use crate::contexts::connection::connect_pool;



// QUERY SELECT KE TABLE
pub async fn select_row() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    // Get the sale with an order ID equals to 1
    let mut result = Query::new(
        r#"select SalesOrderID,
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
from dbo.SalesOrderHeader
WHERE
    SalesOrderID = @P1;"#,
    );

    // this will be the value of the parameter @P1
    result.bind(1i32);

    // get the result set from SQL Server
    let mut rows = result.query(&mut client).await?;

    // This will read all the returned rows
    while let Some(row) = rows.try_next().await? {
        match row {
            // This section contains the rows returned by the query
            QueryItem::Row(r) => {
                let salesorderid: i32 = r.get(0).unwrap();
                let revisionnumber: u8 = r.get(1).unwrap();
                let orderdate: chrono::NaiveDateTime = r.get(2).unwrap();
                let duedate: chrono::NaiveDateTime = r.get(3).unwrap();
                let shipdate: chrono::NaiveDateTime = r.get(4).unwrap();
                let status: u8 = r.get(5).unwrap();
                let salesordernumber: &str = r.get(6).unwrap();
                let creditcardapprovalcode: &str = r.get(7).unwrap();
                let subtotal: f64 = r.get(8).unwrap();
                let taxamt: f64 = r.get(9).unwrap();
                let freight: f64 = r.get(10).unwrap();
                let totaldue: f64 = r.get(11).unwrap();
                let comment: &str = r.get(12).unwrap_or_else(|| "");
                let rowguid: uuid::Uuid = r.get(13).unwrap();
                let modifieddate: chrono::NaiveDateTime = r.get(14).unwrap();

                println!("SalesOrderID: {}", salesorderid);
                println!("RevisionNumber: {}", revisionnumber);
                println!("OrderDate: {}", orderdate);
                println!("DueDate: {}", duedate);
                println!("ShipDate: {}", shipdate);
                println!("Status: {}", status);
                println!("SalesOrderNumber: {}", salesordernumber);
                println!(
                    "CreditCardApprovalCode: {}",
                    creditcardapprovalcode.to_owned()
                );
                println!("SubTotal: {}", subtotal);
                println!("TaxAmt: {}", taxamt);
                println!("Freight: {}", freight);
                println!("TotalDue: {}", totaldue);
                println!("Comment: {}", comment.to_owned());
                println!("rowguid: {}", rowguid);
                println!("ModifiedDate: {}", modifieddate);
                println!("--------------------------------------");
                println!();
            }

            // This section contains the metadata of the result set
            QueryItem::Metadata(meta) => {
                println!("Metadata: {:?}", meta);
                // The above line comes out with this:
                // ResultMetadata { columns: [
                // Column { name: "SalesOrderID", column_type: Int4 },
                // Column { name: "RevisionNumber", column_type: Int1 },
                // Column { name: "OrderDate", column_type: Datetime },
                // Column { name: "DueDate", column_type: Datetime },
                // Column { name: "ShipDate", column_type: Datetimen },
                // Column { name: "Status", column_type: Int1 },
                // Column { name: "SalesOrderNumber", column_type: NVarchar },
                // Column { name: "CreditCardApprovalCode", column_type: BigVarChar },
                // Column { name: "SubTotal", column_type: Money },
                // Column { name: "TaxAmt", column_type: Money },
                // Column { name: "Freight", column_type: Money },
                // Column { name: "TotalDue", column_type: Money },
                // Column { name: "Comment", column_type: NVarchar },
                // Column { name: "rowguid", column_type: Guid },
                // Column { name: "ModifiedDate", column_type: Datetimen }], result_index: 0 }
            }
        }
    }

    Ok(())
}

#[tokio::test]
async fn select_row_from_sql_server() {
    let result = select_row().await;
    assert_eq!(result.is_ok(), true);
}