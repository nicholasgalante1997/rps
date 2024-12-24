pub mod v1 {
    use crate::models::pods::ResourceRequestQueryInfo;
    use actix_web::{
        get,
        http::header::{HeaderValue, ACCEPT},
        web::Query,
        HttpRequest, HttpResponse, Responder,
    };
    use log::{error, info, warn};

    #[get("/pods")]
    pub async fn resource_handler(
        req: HttpRequest,
        query: Query<ResourceRequestQueryInfo>,
    ) -> impl Responder {
        let headers = req.headers();
        let accept_as_header_value = headers
            .get(ACCEPT)
            .unwrap_or(&HeaderValue::from_bytes(b"application/json").unwrap())
            .clone();

        let accept_as_str = accept_as_header_value.clone().to_str().unwrap().to_string();

        info!("Requested Content Type: {}", &accept_as_str);
        let resource_iri = query.iri.clone();
        
        println!("***");
        println!("{}", resource_iri);
        println!("***");

        HttpResponse::Ok() 
    }
}
