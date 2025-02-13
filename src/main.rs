use csv::Table;

fn main() {
    let companies = Table::from_csv(include_str!("data/companies.csv"), "\t");
    let remove = Table::from_csv(include_str!("data/remove.csv"), "\t");

    // companies.pretty_print("*");
    // remove.pretty_print("*");
    let left = Table::from_csv(include_str!("data/left.csv"), "\t");
    let right = Table::from_csv(include_str!("data/right.csv"), "\t");
    // left.pretty_print("*");
    // right.pretty_print("*");
    let join1 = left.left_join(&right, "name", "name", true);
    let join2 = left.right_join(&right, "name", "name", true);
    //
    companies
        .left_join(&remove, "aisAccountID", "aisaccountid", false)
        .order_by("aisAccountID")
        .select("aisAccountID");
    // join2.pretty_print("*");
}
