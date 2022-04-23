use crate::domain::nucleic_registe;
use crate::common::dbconfig;
use mysql::prelude::Queryable;

use super::NucleicService;

type NucleicRegiste = nucleic_registe::NucleicRegiste;

impl NucleicService for NucleicRegiste {
    fn nucleic_registe(&self) -> bool {
        let mut conn =  dbconfig::get_conn().unwrap();

        let result = conn.exec_drop("INSERT INTO nucleic_test_registe (id, nucleic_type, name, address, phone, create_time, update_time)
                            VALUES(?, ?, ?, ?, ?, ?, ?)",
                            (&self.id, self.nucleic_type, &self.name, &self.address, &self.phone, &self.create_time, &self.update_time,));

        if result.is_ok() {
            return true
        }
        false
    }
}
