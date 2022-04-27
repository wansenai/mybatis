use crate::domain::nucleic_institution;
use super::NucleicInstitutionService;

type NucleicInstitution = nucleic_institution::InstitutionObject;

impl NucleicInstitutionService for NucleicInstitution {
    fn insert_nucleic_institution(&self) -> bool {

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
 