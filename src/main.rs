use csv::table::Table;

fn main() {
    let csv = include_str!("data/test.csv");
    let table = Table::from_csv(csv, None);
    // println!("{:?}",table);
    table.select("*");
}
