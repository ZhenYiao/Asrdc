use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use actix_web::body::BoxBody;
use actix_web::dev::JsonBody;
use actix_web::{HttpRequest, HttpResponse, Responder};

#[derive(Deserialize,Serialize)]
pub struct AppResponse<T: Serialize + Default + 'static> {
    pub code: u16,
    pub data: T,
    pub msg: String,
}
impl <T> AppResponse<T>
where  T: Serialize + Default + 'static,

{
    pub fn ok(data: T) -> AppResponse<T> {
        Self{
            code: 200,
            data,
            msg: "Ok".to_string(),
        }
    }
    pub fn err(msg: &str) -> AppResponse<T> {
        Self{
            code: 400,
            data: Default::default(),
            msg: msg.to_string(),
        }
    }
}
impl <T>Responder for AppResponse<T>
where
    T: Serialize + Default + 'static,
{
    type Body = String;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let serde = serde_json::to_string(&self).unwrap();
        HttpResponse::new(StatusCode::from_u16(self.code).unwrap())
            .set_body(serde)
    }
}
