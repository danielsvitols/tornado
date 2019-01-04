use criterion::Criterion;
use std::fs;
use tornado_engine_matcher::accessor::AccessorBuilder;
use tornado_common_api::Event;
use tornado_engine_matcher::model::ProcessedEvent;

pub fn bench_jmespath(c: &mut Criterion) {

    let filename = "./test_resources/event_nested_01.json";
    let event_json =
        fs::read_to_string(filename).expect(&format!("Unable to open the file [{}]", filename));

    let expr = jmespath::compile("type").unwrap();

    c.bench_function("Extract type - jmespath", move |b| b.iter(|| {
        let data = jmespath::Variable::from_json(&event_json).unwrap();
        let result = expr.search(data).unwrap();
        // Assert
        assert_eq!("email", result.as_string().unwrap());
    }));

}

pub fn bench_accessor(c: &mut Criterion) {

    let filename = "./test_resources/event_nested_01.json";
    let event_json =
        fs::read_to_string(filename).expect(&format!("Unable to open the file [{}]", filename));

    let builder = AccessorBuilder::new();
    let value = "${event.type}".to_owned();
    let accessor = builder.build("", &value).unwrap();

    c.bench_function("Extract type - accessor", move |b| b.iter(|| {

        let event = serde_json::from_str::<Event>(&event_json).unwrap();
        let processed_event = ProcessedEvent::new(event);

        let result = accessor.get(&processed_event);

        // Assert
        assert_eq!("email", result.unwrap().as_ref().get_text().unwrap());
    }));

}
