pub mod domain {
    pub mod models {
        pub mod hospital;
        pub mod patient;
        pub mod doctor;
        pub mod speciality;
    }
    pub mod repositories {
        pub mod hospital_repository;
        pub mod patient_repository;
        pub mod doctor_repository;
        pub mod speciality_repository;
    }
}

pub mod application {
    pub mod usecases {
        pub mod hospital_use_cases;
        pub mod patient_use_cases;
        pub mod doctor_use_cases;
        pub mod speciality_use_cases;
    }
}

pub mod api {
    pub mod hospitals;
    pub mod patients;
    pub mod doctors;
    pub mod specialities;
    pub mod auth;
}