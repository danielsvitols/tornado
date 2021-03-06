use self::handler::ApiHandler;
use self::http::HttpHandler;
use actix_web::{web, Scope};
use std::sync::Arc;

pub mod handler;
mod http;

pub fn new_endpoints<T: ApiHandler + 'static>(mut scope: Scope, api_handler: Arc<T>) -> Scope {
    let http = HttpHandler { api_handler };

    let http_clone = http.clone();
    scope = scope.service(
        web::resource("/config").route(web::get().to_async(move |req| http_clone.get_config(req))),
    );

    let http_clone = http.clone();
    scope = scope.service(
        web::resource("/send_event")
            .route(web::post().to_async(move |req, body| http_clone.send_event(req, body))),
    );

    scope
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::api::handler::SendEventRequest;
    use crate::error::ApiError;
    use actix_web::{
        http::{header, StatusCode},
        test, App,
    };
    use futures::{future::FutureResult, Future};
    use std::collections::HashMap;
    use tornado_engine_api_dto::event::{EventDto, ProcessType, SendEventRequestDto};
    use tornado_engine_matcher::config::MatcherConfig;
    use tornado_engine_matcher::model::{ProcessedEvent, ProcessedNode, ProcessedRules};

    struct TestApiHandler {}

    impl ApiHandler for TestApiHandler {
        fn get_config(&self) -> Box<dyn Future<Item = MatcherConfig, Error = ApiError>> {
            Box::new(FutureResult::from(Ok(MatcherConfig::Ruleset {
                name: "ruleset".to_owned(),
                rules: vec![],
            })))
        }

        fn send_event(
            &self,
            event: SendEventRequest,
        ) -> Box<dyn Future<Item = ProcessedEvent, Error = ApiError>> {
            Box::new(FutureResult::from(Ok(ProcessedEvent {
                event: event.event.into(),
                result: ProcessedNode::Ruleset {
                    name: "ruleset".to_owned(),
                    rules: ProcessedRules { rules: vec![], extracted_vars: HashMap::new() },
                },
            })))
        }
    }

    #[test]
    fn should_return_status_code_ok() {
        // Arrange
        let mut srv = test::init_service(
            App::new().service(new_endpoints(web::scope("/api"), Arc::new(TestApiHandler {}))),
        );

        // Act
        let request = test::TestRequest::get()
            .uri("/api/config")
            //.header(header::CONTENT_TYPE, "application/json")
            //.set_payload(payload)
            .to_request();

        let response = test::call_service(&mut srv, request);

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn should_return_the_matcher_config() {
        // Arrange
        let mut srv = test::init_service(
            App::new().service(new_endpoints(web::scope("/api"), Arc::new(TestApiHandler {}))),
        );

        // Act
        let request = test::TestRequest::get()
            .uri("/api/config")
            //.header(header::CONTENT_TYPE, "application/json")
            //.set_payload(payload)
            .to_request();

        // Assert
        let dto: tornado_engine_api_dto::config::MatcherConfigDto =
            test::read_response_json(&mut srv, request);
        assert_eq!(
            tornado_engine_api_dto::config::MatcherConfigDto::Ruleset {
                name: "ruleset".to_owned(),
                rules: vec![]
            },
            dto
        );
    }

    #[test]
    fn should_return_the_processed_event() {
        // Arrange
        let mut srv = test::init_service(
            App::new().service(new_endpoints(web::scope("/api"), Arc::new(TestApiHandler {}))),
        );

        let send_event_request = SendEventRequestDto {
            event: EventDto {
                event_type: "my_test_event".to_owned(),
                payload: HashMap::new(),
                created_ms: 0,
            },
            process_type: ProcessType::SkipActions,
        };

        // Act
        let request = test::TestRequest::post()
            .uri("/api/send_event")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(serde_json::to_string(&send_event_request).unwrap())
            .to_request();

        // Assert
        let dto: tornado_engine_api_dto::event::ProcessedEventDto =
            test::read_response_json(&mut srv, request);
        assert_eq!("my_test_event", dto.event.event_type);
    }
}
