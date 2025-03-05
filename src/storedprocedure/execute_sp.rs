use crate::contexts::connection::connect_pool;

//EXECUTE STORED PROCEDURE
pub async fn call_stored_procedure() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = connect_pool("S21Notes").await?;

    // The @Comment parameter is not set because it defaults to NULL
    // However it's still possible to set this parameter if desired
    let _ = client
        .execute(
            r#"exec dbo.uspSaveOrderHeader
        @DueDate = @P1,
        @ShipDate = @P2,
        @CreditCardApprovalCode = @P3,
        @ModifiedDate = @P4"#,
            &[
                &"2024-08-20", // DueDate
                &"2024-08-22", // ShipDate
                &"12345",      // CreditCardApprovalCode
                &"2024-08-20", // ModifiedDate
            ],
        )
        .await?;

    println!("Sale order created.");

    Ok(())
}

#[tokio::test]
async fn call_stored_procedure_in_sql_server() {
    let result = call_stored_procedure().await;
    assert_eq!(result.is_ok(), true);
}