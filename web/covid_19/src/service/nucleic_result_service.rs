use mysql::prelude::Queryable;

use crate::common::dbconfig;
use crate::domain::nucleic_result;
use super::NucleicResultService;

type NucleicResult = nucleic_result::NucleicResultObject;

impl NucleicResultService for NucleicResult {

    fn insert_nucleic_result(&self) -> bool {

        let mut conn = dbconfig::get_conn().unwrap();

        let result = conn.exec_drop("INSERT INTO nucleic_test_result (id, result_type, institution_id, registe_id, create_time) VALUES (?,?,?,?,?)", 
                        &self.id, &self.result_type, &self.institution_id, &self.registry_id, &self.create_time);

        if result.is_ok() {
            return true;
        }

        false
    }
}
