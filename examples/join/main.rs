use csv::table::Table;

// fn main() {
//     let left = Table::from_csv(include_str!("data/left.csv"), Some("\t"));
//     let right = Table::from_csv(include_str!("data/right.csv"), Some("\t"));
//     println!("left:");
//     left.select("*");
//     println!("\nright:");
//     right.select("*");
//     println!("\njoin on name:");
//     left.left_join(&right, "name", "name", false)
//         .select("name, cowdung, value");
//     println!("\nleft join on name:");
//     left.left_join(&right, "name", "name", true).select("*");
//     println!("\nright join on name:");
//     left.right_join(&right, "name", "name", true)
//         .select("name,cowdung,value");
// }
