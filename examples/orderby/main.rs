use csv::table::Table;

fn main() {
    let table = Table::from_csv(include_str!("data/table.csv"), Some("\t"));
    println!("not ordered:");
    table.select("*");

    println!("order by name ascending:");
    table.order_by("name").select("*");

    println!("\nTODO descending");
}
