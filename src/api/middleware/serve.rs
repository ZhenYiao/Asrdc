use std::str::FromStr;
use actix_web::body::MessageBody;
use actix_web::dev::ServiceResponse;
use actix_web::http::header;

pub async fn serve(
    mut res: ServiceResponse<impl MessageBody>,
) -> actix_web::Result<ServiceResponse<impl MessageBody>> {
    res.headers_mut().insert(header::HeaderName::from_str("Language").unwrap(), header::HeaderValue::from_static("Rust"));
    res.headers_mut().insert(header::HeaderName::from_str("Serve").unwrap(), header::HeaderValue::from_static("Actix-Web"));
    Ok(res)
}