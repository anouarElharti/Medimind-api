use uuid::Uuid;
use actix_web::{web, HttpResponse, post, get, put, delete};
use crate::domain::models::speciality::Speciality;
use crate::domain::repositories::speciality_repository::SpecialityRepository;
use crate::application::usecases::speciality_use_cases::SpecialityUseCases;


#[post("/specialities/create")]
pub async fn create_speciality(
    speciality: web::Json<Speciality>,
    use_cases: web::Data<SpecialityUseCases<Box<dyn SpecialityRepository>>>,
) -> HttpResponse {
    match use_cases.create_speciality(speciality.into_inner()).await {
        Ok(created) => HttpResponse::Created().json(created),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {:?}", err)),
    }
}

#[get("/specialities")]
pub async fn get_specialities(
    use_cases: web::Data<SpecialityUseCases<Box<dyn SpecialityRepository>>>,
) -> HttpResponse {
    match use_cases.list_specialities().await {
        Ok(specialities) => HttpResponse::Ok().json(specialities),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[get("/specialities/{id}")]
pub async fn get_speciality(
    id: web::Path<Uuid>,
    use_cases: web::Data<SpecialityUseCases<Box<dyn SpecialityRepository>>>,
) -> HttpResponse {
    match use_cases.get_speciality(id.into_inner()).await {
        Ok(Some(speciality)) => HttpResponse::Ok().json(speciality),
        Ok(None) => HttpResponse::NotFound().json("Speciality not found"),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}

#[put("/specialities/update")]
pub async fn update_speciality(
    speciality: web::Json<Speciality>,
    use_cases: web::Data<SpecialityUseCases<Box<dyn SpecialityRepository>>>,
) -> HttpResponse {
    match use_cases.update_speciality(speciality.into_inner()).await {
        Ok(updated) => HttpResponse::Ok().json(updated),
        Err(err) => HttpResponse::BadRequest().json(format!("Error: {:?}", err)),
    }
}

#[delete("/specialities/delete/{id}")]
pub async fn delete_speciality(
    id: web::Path<Uuid>,
    use_cases: web::Data<SpecialityUseCases<Box<dyn SpecialityRepository>>>,
) -> HttpResponse {
    match use_cases.delete_speciality(id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::InternalServerError().json(format!("Error: {:?}", err)),
    }
}
