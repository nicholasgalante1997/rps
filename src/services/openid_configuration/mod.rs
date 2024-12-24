pub mod v1 {
    use actix_web::{post, Error};

    #[post("/openid-configuration")]
    pub async fn openid_configuration_post_handler() -> Result<String, Error> {
        Ok(String::from("Test"))
    }
}
