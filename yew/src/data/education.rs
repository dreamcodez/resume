use crate::models::Education;

pub fn get_education() -> Vec<Education> {
    vec![Education {
        institution: "California State University".to_string(),
        degree: "Computer Science".to_string(),
        field: "Computer Science".to_string(),
        start_year: 2005,
        end_year: Some(2008),
        description: "Some upper division coursework completed".to_string(),
    }]
}
