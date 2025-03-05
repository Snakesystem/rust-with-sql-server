use tiberius::Query;

use crate::contexts::connection::connect_pool;

// QUERY TO CREATE TABLE
pub async fn create_table_fn() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    let statement = Query::new(
        r#"
create table dbo.SalesOrderHeader
(
    SalesOrderID           int identity
        constraint PK_SalesOrderHeader_SalesOrderID
            primary key,
    RevisionNumber         tinyint
        constraint DF_SalesOrderHeader_RevisionNumber default 0       not null,
    OrderDate              datetime
        constraint DF_SalesOrderHeader_OrderDate default getdate()    not null,
    DueDate                datetime                                   not null,
    ShipDate               datetime,
    Status                 tinyint
        constraint DF_SalesOrderHeader_Status default 1               not null
        constraint CK_SalesOrderHeader_Status
            check ([Status] >= 0 AND [Status] <= 8),
    SalesOrderNumber       as isnull(N'SO' + CONVERT([nvarchar](23), [SalesOrderID]), N'*** ERROR ***'),
    CreditCardApprovalCode varchar(15),
    SubTotal               money
        constraint DF_SalesOrderHeader_SubTotal default 0.00          not null
        constraint CK_SalesOrderHeader_SubTotal
            check ([SubTotal] >= 0.00),
    TaxAmt                 money
        constraint DF_SalesOrderHeader_TaxAmt default 0.00            not null
        constraint CK_SalesOrderHeader_TaxAmt
            check ([TaxAmt] >= 0.00),
    Freight                money
        constraint DF_SalesOrderHeader_Freight default 0.00           not null
        constraint CK_SalesOrderHeader_Freight
            check ([Freight] >= 0.00),
    TotalDue               as isnull([SubTotal] + [TaxAmt] + [Freight], 0),
    Comment                nvarchar(128),
    rowguid                uniqueidentifier default newid()        not null,
    ModifiedDate           datetime
)
    "#,
    );

    let _ = statement.execute(&mut client).await?;
    println!("Created table");

    let _ = client.close().await?;

    Ok(())
}

#[tokio::test]
async fn create_table_in_sql_server() {
    let result = create_table_fn().await;
    assert_eq!(result.is_ok(), true);
}