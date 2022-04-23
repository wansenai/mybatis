use crate::domain::nucleic_institution;

type NucleicInstitution = nucleic_institution::InstitutionObject;

pub trait Nucleic {
    fn query_map(&self) -> i32;

    fn test(&self) -> i32;
}

impl Nucleic for NucleicInstitution {
    fn query_map(&self) ->i32 {
        6
    }

    fn test(&self) -> i32 {
        9999
    }
}
 