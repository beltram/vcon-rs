use assert_json_diff::{CompareMode, FloatCompareMode, NumericMode};
use vcon_types::Vcon;

const EXAMPLES: [&'static str; 5] = [
    include_str!("../examples/json/email-thread-text.json"),
    include_str!("../examples/json/email-thread-multipart.json"),
    include_str!("../examples/json/two-party-call-with-analysis.json"),
    include_str!("../examples/json/two-party-call-with-external-reference-recording.json"),
    include_str!("../examples/json/two-party-call-with-inline-recording.json"),
];

#[test]
fn json_examples_should_work() {
    for example in EXAMPLES {
        let deser = serde_json::from_str::<Vcon>(example).unwrap();
        let ser = serde_json::to_value(&deser).unwrap();

        let example = serde_json::from_str::<serde_json::Value>(example).unwrap();
        let config = assert_json_diff::Config::new(CompareMode::Inclusive)
            .numeric_mode(NumericMode::AssumeFloat)
            .float_compare_mode(FloatCompareMode::Epsilon(0.01));
        assert_json_diff::assert_json_matches!(ser, example, config);
    }
}
