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


pub async fn update_by_column_entry() -> Result<(), anyhow::Error> {
    
    dotenv::from_filename(".env").ok();
    let client = Postgrest::new("SUPABASE_URI")
    .insert_header("apikey", dotenv::var("SUPABASE_KEY").unwrap());

    let mut id_input = String::new();
    println!("Enter an ID to modify: ");
    io::stdin().read_line(&mut id_input);

    let mut column = String::new();
    println!("Now enter your column:");
    io::stdin().read_line(&mut column)
    .expect("Failed to read input");

    let col_num: u8 = column.trim().parse::<u8>()
        .expect("wrong value");

    if col_num == 1 {
        let mut new_value = String::new();
        println!("Enter an value to replace the current: ");
        io::stdin().read_line(&mut new_value);
        let new_age = json!({
            "age": &new_value.to_string()
        });
        let resp3 = client
        .from("employees")
        .eq("id", &id_input.to_string())
        .update(&new_age.to_string())
        .execute()
        .await?;

        let objects = resp3
        .text().
        await?;
        println!("{:?}", &objects);

    } else if col_num == 2 {

        let mut new_value = String::new();
        println!("Enter an value to replace the current: ");
        io::stdin().read_line(&mut new_value);
        let new_first_name = json!({
            "first_name": &new_value.to_string()
        });
        let resp3 = client
        .from("employees")
        .eq("id", &id_input.to_string())
        .update(&new_first_name.to_string())
        .execute()
        .await?;

        let objects = resp3
        .text().
        await?;
        println!("{:?}", &objects);

    } else if col_num == 3 {

        let mut new_value = String::new();
        println!("Enter an value to replace the current: ");
        io::stdin().read_line(&mut new_value);
        let new_interests = json!({
            "interests": &new_value.to_string()
        });
        let resp3 = client
        .from("employees")
        .eq("id", &id_input.to_string())
        .update(&new_interests.to_string())
        .execute()
        .await?;

        let objects = resp3
        .text().
        await?;
        println!("{:?}", &objects);

    } else if col_num == 4 {

        let mut new_value = String::new();
        println!("Enter an value to replace the current: ");
        io::stdin().read_line(&mut new_value);
        let new_city = json!({
            "city": &new_value.to_string()
        });
        let resp3 = client
        .from("employees")
        .eq("id", &id_input.to_string())
        .update(&new_city.to_string())
        .execute()
        .await?;

        let objects = resp3
        .text().
        await?;
        println!("{:?}", &objects);

    } else {
        return Ok(())
    }
    Ok(())

}



pub async fn delete_by_id() -> Result<(), anyhow::Error> {

    dotenv::from_filename(".env").ok();
    let client = Postgrest::new("SUPABASE_URI")
    .insert_header("apikey", dotenv::var("SUPABASE_KEY").unwrap());

    let mut id_input = String::new();

    println!("Enter an ID to delete: ");
    io::stdin().read_line(&mut id_input);

    let resp = client
        .from("employees")
        .delete()
        .eq("id", &id_input.to_string())
        .execute()
        .await?;

    let objects = resp
        .text().
        await?;
        println!("{:?}", &objects);
        Ok(())


}


pub async fn enter_json_fields() -> Result<(), anyhow::Error> {
    
    dotenv::from_filename(".env").ok();
    let client = Postgrest::new("SUPABASE_URI")
    .insert_header("apikey", dotenv::var("SUPABASE_KEY").unwrap());


    let mut id = String::new();
    let mut name = String::new();
    let mut age = String::new();
    let mut interests = String::new();
    let mut city = String::new();

        // id
    println!("Enter an id : ");
    io::stdin().read_line(&mut id);
        // name
    println!("Enter a first name: ");
    io::stdin().read_line(&mut name);
        // age
    println!("Enter an age: ");
    io::stdin().read_line(&mut age);
        //interests
    println!("Enter interest: ");
    io::stdin().read_line(&mut interests);
        // city
    println!("Enter a city: ");
    io::stdin().read_line(&mut city);

    // profile in json
    let json_profile = json!({
        "id": &id.to_string(),
        "first_name": &name.to_string(),
        "age": &age.to_string(),
        "interests": &interests.to_string(),
        "city": &city.to_string(),
    });

    let resp = client
        .from("employees")
        .insert(&json_profile.to_string())
        .execute()
        .await?;

    let objects = resp
        .text().
        await?;
        println!("{:?}", &objects);
        Ok(())


}



pub async fn simple_query() -> Result<(), anyhow::Error> {
    dotenv::from_filename(".env").ok();
    let client = Postgrest::new("SUPABASE_URI")
    .insert_header("apikey", dotenv::var("SUPABASE_KEY").unwrap());

    let resp = client
    .from("employees")
    .select("*")
    .execute()
    .await?;


    let objects = resp
    .text().
    await?;
    println!("{}", serde_json::to_string_pretty(&objects).unwrap());
    Ok(())

}

pub async fn add_from_json_file() -> Result<(), anyhow::Error> {
    dotenv::from_filename(".env").ok();
    let client = Postgrest::new("SUPABASE_URI")
    .insert_header("apikey", dotenv::var("SUPABASE_KEY").unwrap());

    let mut contents = {
        let file_contents = fs::read_to_string("./data/test.json")
        .expect("Should have been able to read file!");
        serde_json::from_str::<Value>(&file_contents).expect("Error serializing to JSON")
    };

    let resp = client
        .from("employees")
        .insert(&contents.to_string())
        .execute()
        .await?;

    let objects = resp
        .text().
        await?;
        println!("{:?}", &objects);
    Ok(())
}

// NOTES
