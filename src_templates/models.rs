//////////////////////////////////////////////////
////////////////////// GROQ //////////////////////

// {const_assignments}

// Static mapping accessible from other files
pub const GROQ_MODEL_MAPPING: &[(&str, &str)] = &[
  // This will be replaced by build.rs:
  // {groq_model_hashmap}
];

pub fn get_groq_model(model_id: &str) -> &str {
  GROQ_MODEL_MAPPING
    .iter()
    .find(|(key, _)| key == &model_id)
    .map_or(model_id, |(_, value)| *value)
}

macro_rules! groq_models_pretty {
  ($prefix: expr) => {
    // This will be replaced by build.rs
    concat!($prefix, "\n", "{groq_models_pretty}")
  };
}

//////////////////////////////////////////////////
///////////////////// OLLAMA /////////////////////

// Pretty-printed string representation of the hashmap
pub const OLLAMA_MODEL_MAPPING: &[(&str, &str)] = &[
  // This will be replaced by build.rs:
  // {ollama_model_hashmap}
];

pub fn get_ollama_model(model_id: &str) -> &str {
  OLLAMA_MODEL_MAPPING
    .iter()
    .find(|(key, _)| key == &model_id)
    .map_or(model_id, |(_, value)| *value)
}

macro_rules! ollama_models_pretty {
  ($prefix: expr) => {
    // This will be replaced by build.rs
    concat!($prefix, "\n", "{ollama_models_pretty}")
  };
}

//////////////////////////////////////////////////
///////////////////// OPENAI /////////////////////

// Pretty-printed string representation of the hashmap
pub const OPENAI_MODEL_MAPPING: &[(&str, &str)] = &[
  // This will be replaced by build.rs:
  // {openai_model_hashmap}
];

pub fn get_openai_model(model_id: &str) -> &str {
  OPENAI_MODEL_MAPPING
    .iter()
    .find(|(key, _)| key == &model_id)
    .map_or(model_id, |(_, value)| *value)
}

macro_rules! openai_models_pretty {
  ($prefix: expr) => {
    // This will be replaced by build.rs
    concat!($prefix, "\n", "{openai_models_pretty}")
  };
}

//////////////////////////////////////////////////
/////////////////// ANTHROPIC ////////////////////

// Pretty-printed string representation of the hashmap
pub const ANTHROPIC_MODEL_MAPPING: &[(&str, &str)] = &[
  // This will be replaced by build.rs:
  // {anthropic_model_hashmap}
];

pub fn get_anthropic_model(model_id: &str) -> &str {
  ANTHROPIC_MODEL_MAPPING
    .iter()
    .find(|(key, _)| key == &model_id)
    .map_or(model_id, |(_, value)| *value)
}

macro_rules! anthropic_models_pretty {
  ($prefix: expr) => {
    // This will be replaced by build.rs
    concat!($prefix, "\n", "{anthropic_models_pretty}")
  };
}
