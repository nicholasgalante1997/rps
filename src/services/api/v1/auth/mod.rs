use crate::models::errors::UnresolvedOwnerException;
use crate::models::user::{LoginRequest, MinimalUser, WebID};
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use bcrypt::{hash, BcryptError, DEFAULT_COST};

#[post("/owner/login")]
pub async fn login(session: Session, user_info: web::Json<LoginRequest>) -> impl Responder {
    let owner = get_owner_user_info();
    match owner {
        Ok(owner) => {
            let web_id = WebID {
                user_reference_uri: owner.username.clone(),
            };
            HttpResponse::Ok().json(web_id)
        }
        Err(bcrypt_err) => {
            eprint!("{:#?}", bcrypt_err);
            HttpResponse::InternalServerError().json(UnresolvedOwnerException::new())
        }
    }
}

fn get_owner_user_info() -> Result<MinimalUser, Box<BcryptError>> {
    let password = hash(
        std::env::var("RPS_OWNER_USER_PASSPHRASE").expect("Owner Passphrase should be set"),
        DEFAULT_COST,
    )?;
    Ok(MinimalUser {
        username: String::from("<http://localhost:8080/pod/profile/card#me>"),
        password,
        is_owner: true,
    })
}
