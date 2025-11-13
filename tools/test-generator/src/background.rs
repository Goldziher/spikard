use anyhow::Result;
use serde_json::Value;
use spikard_codegen::openapi::Fixture;

#[derive(Clone)]
pub struct BackgroundFixtureData {
    pub state_path: String,
    pub state_key: String,
    pub value_field: String,
    pub expected_state: Vec<Value>,
}

pub fn background_data(fixture: &Fixture) -> Result<Option<BackgroundFixtureData>> {
    Ok(fixture.background.as_ref().map(|bg| BackgroundFixtureData {
        state_path: bg.state_path.clone(),
        state_key: bg.state_key.clone(),
        value_field: bg.value_field.clone(),
        expected_state: bg.expected_state.clone(),
    }))
}
