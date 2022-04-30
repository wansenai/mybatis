use std::borrow::Borrow;
use std::fmt::{Display, Write};

use mysql::prelude::{Queryable, WithParams};

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
        let mut add_symbols: Vec<String> = Vec::new();
        let mut request_params: Vec<String> = Vec::new();

        let mut sql_stmt = String::from("UPDATE nucleic_test_institution SET ");
        let mut coon = dbconfig::get_conn().unwrap();
        let data = self.into();

        match data {
            Some(data) => {
                match &data.institution_name {
                    Some(institution_name) => {
                        request_params.push(institution_name.to_string());
                        add_symbols.push(String::from("institution_name = ? "));
                    }
                    None => (),
                }
                match &data.institution_address {
                    Some(institution_address) => {
                        request_params.push(institution_address.to_string());
                        add_symbols.push(String::from("institution_address = ? "));
                    },
                    None => (),
                }
                match &data.institution_phone {
                    Some(institution_phone) => {
                        request_params.push(institution_phone.to_string());
                        add_symbols.push(String::from("institution_phone = ? "));
                    },
                    None => (),
                }
                match &data.institution_region {
                    Some(institution_region) => {
                        request_params.push(institution_region.to_string());
                        add_symbols.push(String::from("institution_region = ? "));
                    },
                    None => (),
                }
                match &data.id {
                    Some(id) => {
                        request_params.push(id.to_string());
                    }
                    None => (),
                }
            }
            None => (),
        }
        if add_symbols.len() > 1 {

            let mut v = String::new();
            let last_item = &add_symbols.last();
                
            match last_item {
                Some(last_item) => v.push_str(last_item),
                None => (),
            }
            for mut item in add_symbols {
                if item != v {
                    item.push_str(",");
                    sql_stmt.push_str(&item);
                } else {
                    sql_stmt.push_str(&item);
                }
            }
          
        } else {
            sql_stmt.push_str(&add_symbols[0]);
        }
        sql_stmt.push_str(" WHERE id = ?");


        println!("sql: {}", sql_stmt);
        println!("params: {:?}", request_params);


        let result = coon.exec_drop(sql_stmt, request_params);

        if result.is_ok(){
            return true;
        } 
        false
    }

    fn query_nucleic_institution_byregion(region: &str) -> bool {

        false
    }

    fn query_nucleic_institution_byname(name: &str) -> bool{
        
        false
    }
}
 