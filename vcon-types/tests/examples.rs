use vcon_types::Vcon;

const EXAMPLES: [&'static str; 1] = [
    include_str!("../examples/json/email-thread-text.json"),
    // include_str!("../examples/json/email-thread-multipart.json"),
    // include_str!("../examples/json/two-party-call-with-analysis.json"),
    // include_str!("../examples/json/two-party-call-with-external-reference-recording.json"),
    // include_str!("../examples/json/two-party-call-with-inline-recording.json"),
];

#[test]
#[ignore]
fn json_examples_should_work() {
    for example in EXAMPLES {
        let deser = serde_json::from_str::<Vcon>(example).unwrap();
        let ser = serde_json::to_value(&deser).unwrap();
        // compare json values and not raw strings to prevents issues with misalignment
        let example = serde_json::from_str::<serde_json::Value>(example).unwrap();
        assert_eq!(ser, example)
    }
}
