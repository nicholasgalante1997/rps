pub mod v1 {
    use crate::models::pods::ResourceRequestQueryInfo;
    use log::{info, warn, error};
    use actix_web::{
        get,
        http::header::{HeaderValue, ACCEPT},
        web::Query,
        HttpRequest, HttpResponse, Responder,
    };

    
    #[get("/pods")]
    pub async fn resource_handler(
        req: HttpRequest,
        query: Query<ResourceRequestQueryInfo>,
    ) -> impl Responder {
        let headers = req.headers();
        let accept_as_header_value = headers
            .get(ACCEPT)
            .unwrap_or(&HeaderValue::from_bytes(b"application/json").unwrap()).clone();

        let accept_as_str = accept_as_header_value.clone().to_str().unwrap().to_string();

        info!("Requested Content Type: {}", &accept_as_str); 

        let public: Vec<String> = vec![];
        let protected: Vec<String> = vec![];

        let resource_iri = query.resource_iri.clone();

        HttpResponse::Ok() // TODO Change this
    }
}
