use mysql::prelude::Queryable;

use crate::common::dbconfig;
use crate::domain::nucleic_institution;
use super::NucleicInstitutionService;

type NucleicInstitution = nucleic_institution::InstitutionObject;

impl NucleicInstitutionService for NucleicInstitution {
    fn insert_nucleic_institution(&self) -> bool {
        let mut coon = dbconfig::get_conn().unwrap();
        
        let result = coon.exec_drop("INSERT INTO nucleic_test_institution (id, institution_name, institution_address, institution_phone, institution_region, create_time, update_time)
         VALUES (?, ?, ?, ?, ?, ?, ?)", (&self.id, &self.institution_name, &self.institution_address, &self.institution_phone, 
         &self.institution_region, &self.create_time, &self.update_time,));
        
        if result.is_ok(){
            return true;
        } 
        false
    }

    fn update_nucleic_institution(&self) -> bool {

        false
    }

    fn query_nucleic_institution_byregion(region: &str) -> bool {

        false
    }

    fn query_nucleic_institution_byname(name: &str) -> bool{
        
        false
    }
}
 