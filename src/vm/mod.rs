use std::collections::HashMap;

use crate::table::Table;
use crate::value::Value;

struct Vm {
    tables: HashMap<String, Table>,
    stack: Vec<Value>,
    code: Vec<Opcode>,
    table_register: String,
    ip: usize,
}

enum Opcode {
    LoadTable(String),
    ApplyIndex(String),
    FetchRow,
    FilterRow,
    IncRowPointer,
}

impl Vm {
    fn run(&mut self) {
        for op in &self.code {
            // match op {
            //     Opcode::LoadTable(name) => {
            //         if !self.tables.contains_key(name) {
            //             let table = self.load_table(name).unwrap();
            //             self.tables.insert(name.clone(), table);
            //         }
            //         self.table_register = name.clone();
            //     }
            // }
        }
    }

    fn load_table(&self, name: &String) -> anyhow::Result<Table> {
        Ok(Table::from_csv_file(name, None)?)
    }
}
