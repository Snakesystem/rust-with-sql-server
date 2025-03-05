mod contexts {
    pub mod connection;
    pub mod  model;
}

mod services {
    pub mod create_table;
    pub mod insert_to_row;
    pub mod select_to_row;
    pub mod update_to_row;
    pub mod delete_to_row;
}

mod storedprocedure {
    pub mod create_sp;
    pub mod execute_sp;
    pub mod sp_with_parameter_output;
    pub mod sp_with_return_status_code;
    pub mod sp_with_return_table;
}

mod functions {
    pub mod create_function;
    pub mod execute_funtion;
    pub mod table_value_function;
}

fn main() {
    println!("Hello, world!");
}