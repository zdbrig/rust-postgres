use postgres::{Client, Error, NoTls};

struct StockType {
    _id: i32,
    name: String,
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/adelphid", NoTls)?;

        client.execute(
            "INSERT INTO stocktypes (namestoktype) VALUES ($1)",
            &[&String::from("Bacem")],)?;
   

    for row in client.query("SELECT id, namestoktype from stocktypes", &[])? {
        let stock = StockType {
            _id: row.get(0),
            name: row.get(1),
        };
        println!("Stock type is {} name {}", stock._id, stock.name);
    }

    Ok(())
}
