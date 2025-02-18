use csv::table::Table;

fn main() {
    let csv = include_str!("data/portfolios.csv");
    let table = Table::from_csv(csv, None);
    table.order_by("name").select("*");
}
