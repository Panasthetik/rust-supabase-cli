use postgrest::Postgrest;
use tokio;
use dotenv::dotenv;
use anyhow::anyhow;
use anyhow::Error;

use serde_json::json;
use serde_json::Value;
use serde::Serialize;

use std::fs;
use std::io;
use std::io::Write;

use clap::{Parser, Subcommand};

use modules::{
    update_by_column_entry,
    delete_by_id,
    enter_json_fields,
    simple_query,
    add_from_json_file,
};

mod modules;

#[derive(Parser)]
#[clap(about, version, author)]
struct EntryValue {
   #[clap(subcommand)]
   command: Commands,
}

#[derive(Subcommand)]
enum Commands {
   AddFile {},
   AddEntries {},
   Delete {},
   Query {},
   ColumnEntry {},
}


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {


    let value = EntryValue::parse();

    match &value.command {
        Commands::AddFile {} => {
            let json_update = add_from_json_file()
                .await?;
            println!("{:?}", &json_update);

        },
        Commands::AddEntries {} => {
            let input_profile_json = enter_json_fields()
                .await?;
            println!("{:?}", &input_profile_json);

        },
        Commands::Delete {} => {
            let delete_row = delete_by_id()
                .await?;
            println!("{:?}", &delete_row);

        },
        Commands::Query {} => {
            let result = simple_query()
            .await?;
            println!("{:?}", &result);

        },
        Commands::ColumnEntry {} => {
            let result = update_by_column_entry()
            .await?;
            println!("{:?}", &result);

        },

    }

    Ok(())


}


