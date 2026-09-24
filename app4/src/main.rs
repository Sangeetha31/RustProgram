use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

mod schema;
mod models;
use models::{Product,ProductInsert};

use schema::products::dsl::*;

fn get_connection() -> SqliteConnection{
    let db_url = "store.db";
    SqliteConnection::establish(db_url)
    .expect("Failed to connect!")
}

fn create_table(){
   let mut con= get_connection();
   diesel::sql_query(
        "
        CREATE TABLE IF NOT EXISTS products(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        price REAL NOT NULL,
        category TEXT NOT NULL
   )
        "
   ).execute(&mut con)
   .expect("Failed to create table");

   println!("Products Table Created Succesfully!")
}

fn insert(){
    let prod=ProductInsert{
        name:"Dell".to_string(),
        price:5000.00,
        category:"Electronics".to_string()
    };
    let mut con=get_connection();
    diesel::insert_into(products)
    .values(prod)
    .execute(&mut con)
    .expect("Failed to Insert Data");

    println!("Record Inserted Succesfully")
}

fn getdata(){
    let mut con=get_connection();

    let products_list:Vec<Product>=products
                      .select(Product::as_select())
                      .load(&mut con)
                      .expect("Failed to Get Records");
    for pr in products_list{
        println!("Id:{}\tName:{}\tPrice:{}\tCategory:{}",pr.id,pr.name,pr.price,pr.category);
    }
}

fn update(){
    let product_id=2;
    let mut con=get_connection();

    diesel::update(products.filter(id.eq(product_id)))
                .set((
                    name.eq("Dell 3525"),
                    price.eq(750000.00),
                    category.eq("Laptops"),
                )).execute(&mut con)
                .expect("Failed to update");
            
    println!("Record Updated Succesfully!");
}

fn delete(){
    let product_id=1;
    let mut con =get_connection();
    diesel::delete(products.filter(id.eq(product_id)))
            .execute(&mut con)
            .expect("Failed to Delete!");
    println!("Record Deleted Succesfully!");
}
fn main() {
    create_table();
    insert();
    getdata();
    update();
    delete();
    getdata();
}