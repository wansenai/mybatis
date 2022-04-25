use crate::domain::nucleic_institution;
use super::Nucleic;

type NucleicInstitution = nucleic_institution::InstitutionObject;

impl Nucleic for NucleicInstitution {
    fn query_map(&self) ->i32 {
        6
    }

    fn test(&self) -> i32 {
        9999
    }
}
 