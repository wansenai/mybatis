mod domain;
mod service;
mod common;
use domain::nucleic_institution;

type NucleicInstitution = nucleic_institution::InstitutionObject;
type NucleicInstitutionService = dyn service::nucleic_institution_service::Nucleic;

fn query() {
    let queryObj = NucleicInstitution::new(
        String::from("sa0as151-a71515z-891z59"), 
        1, 
        String::from("1"), 
        String::from("1"), 
        String::from("2022-04-22 17:450")
    );

    let result: i32 = NucleicInstitutionService::query_map(&queryObj);

    println!("{}", result)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(4+4, 8);
    }

    #[test]
    fn test_b() {
        query();
    }
}