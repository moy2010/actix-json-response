pub struct JsonResponse<T: serde::Serialize> {
    pub value: T,
    pub status_code: Option<actix_web::http::StatusCode>,
}

impl<T: serde::Serialize> From<T> for JsonResponse<T> {
    fn from(value: T) -> Self {
        Self {
            value,
            status_code: None,
        }
    }
}

impl<T: serde::Serialize> JsonResponse<T> {
    pub fn with_status_code(mut self, status_code: actix_web::http::StatusCode) -> Self {
        self.status_code = Some(status_code);

        self
    }
}

impl<T: serde::Serialize> actix_web::Responder for JsonResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match serde_json::to_string(&self.value) {
            Err(err) => actix_web::HttpResponse::from_error(err),
            Ok(value) => actix_web::HttpResponseBuilder::new(
                self.status_code.unwrap_or(actix_web::http::StatusCode::OK),
            )
            .content_type("application/json")
            .body(value),
        }
    }
}
