mod dboperations;

use dboperations as db;

fn main() {
    let conn=db::connect_db();
    //db::create_table(&conn);

    //db::insert(&conn,"Test",45000.00);
    db::get_all(&conn);
    db::update(&conn,2, "OnePlus16",55000.00);
    db::delete(&conn,3);
    db::get_all(&conn);
    
}
