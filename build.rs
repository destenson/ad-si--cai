use std::env;
use std::fs;
use std::path::Path;

const ANTHROPIC_MODEL_MAPPING_SRC: [(&str, &str); 26] = [
  // Default models
  ("claude-opus", "claude-3-opus-latest"),
  ("opus", "claude-3-opus-latest"),
  ("op", "claude-3-opus-latest"),
  ("o", "claude-3-opus-latest"),
  ("claude-sonnet", "claude-3-7-sonnet-latest"),
  ("sonnet", "claude-3-7-sonnet-latest"),
  ("so", "claude-3-7-sonnet-latest"),
  ("s", "claude-3-7-sonnet-latest"),
  ("claude-haiku", "claude-3-5-haiku-latest"),
  ("haiku", "claude-3-5-haiku-latest"),
  ("ha", "claude-3-5-haiku-latest"),
  ("h", "claude-3-5-haiku-latest"),
  // Version 3.7 models
  ("claude-sonnet-3-7", "claude-3-7-sonnet-latest"),
  ("sonnet-3-7", "claude-3-7-sonnet-latest"),
  // Version 3.5 models
  ("claude-sonnet-3-5", "claude-3-5-sonnet-latest"),
  ("sonnet-3-5", "claude-3-5-sonnet-latest"),
  ("claude-haiku-3-5", "claude-3-5-haiku-latest"),
  ("haiku-3-5", "claude-3-5-haiku-latest"),
  // Version 3 models
  ("claude-opus-3", "claude-3-opus-latest"),
  ("opus-3", "claude-3-opus-latest"),
  ("claude-sonnet-3", "claude-3-sonnet-20240229"),
  ("sonnet-3", "claude-3-sonnet-20240229"),
  ("claude-haiku-3", "claude-3-haiku-20240307"),
  ("haiku-3", "claude-3-haiku-20240307"),
  ("claude-sonnet-3-7", "claude-3-7-sonnet-20250219"),
  ("sonnet-3-7", "claude-3-7-sonnet-20250219"),
];

const GROQ_MODEL_MAPPING_SRC: [(&str, &str); 22] = [
  ///// Default models /////
  // Llama
  ("llama", "llama-3.1-8b-instant"),
  ("ll", "llama-3.1-8b-instant"),
  ("l", "llama-3.1-8b-instant"),
  ("llama-8b", "llama-3.3-8b-instant"),
  ("llama-70b", "llama-3.3-70b-versatile"),
  // Mixtral
  ("mixtral", "mixtral-8x7b-32768"),
  ("mi", "mixtral-8x7b-32768"),
  ("m", "mixtral-8x7b-32768"),
  // Gemma
  ("gemma", "gemma2-9b-it"),
  ("ge", "gemma2-9b-it"),
  ("g", "gemma2-9b-it"),
  ///// Specific versions /////
  // Llama 3.3
  ("llama33", "llama-3.3-8b-instant"),
  ("llama33-8b", "llama-3.3-8b-instant"),
  ("llama33-70b", "llama-3.3-70b-versatile"),
  // Llama 3.0
  ("llama3", "llama3-8b-8192"),
  ("llama3-8b", "llama3-8b-8192"),
  ("llama3-70b", "llama3-70b-8192"),
  // Mixtral
  ("mixtral-8x7b", "mixtral-8x7b-32768"),
  // Qwen
  ("qwen", "qwen-2.5-32b"),
  // DeepSeek
  ("deepseek-qwen", "deepseek-r1-distill-qwen-32b"),
  ("deepseek-llama", "deepseek-r1-distill-llama-70b"),
  ("deepseek-llama-specdec", "deepseek-r1-distill-llama-70b-specdec"),
];

const CEREBRAS_MODEL_MAPPING_SRC: [(&str, &str); 13] = [
  ///// Default models /////
  // Llama
  ("llama", "llama3.1-8b"),
  ("ll", "llama3.1-8b"),
  ("l", "llama3.1-8b"),
  ("llama-8b", "llama3.1-8b"),
  ("llama-70b", "llama-3.3-70b"),
  // Deepseek
  ("deepseek", "deepseek-r1-distill-llama-70b"),
  ("deep", "deepseek-r1-distill-llama-70b"),
  ("d", "deepseek-r1-distill-llama-70b"),
  ///// Specific versions /////
  // Llama 3.1
  ("llama31", "llama-3.1-8b"),
  ("llama31-8b", "llama-3.1-8b"),
  // Llama 3.3
  ("llama33", "llama-3.3-70b"),
  ("llama33-70b", "llama-3.3-70b"),
  // Deepseek R1
  ("deepseek-r1", "deepseek-r1-distill-llama-70b"),
];

const DEEPSEEK_MODEL_MAPPING_SRC: [(&str, &str); 2] = [
  ("chat", "deepseek-chat"),
  ("reasoner", "deepseek-reasoner"), //
];

const OLLAMA_MODEL_MAPPING_SRC: [(&str, &str); 21] = [
  // Default models
  ("llama", "llama3.1"),
  ("ll", "llama3.1"),
  ("l", "llama3.1"),
  ("mixtral", "mixtral"),
  ("mix", "mixtral"),
  ("m", "mixtral"),
  ("mistral", "mistral"),
  ("mis", "mistral"),
  ("gemma", "gemma"),
  ("ge", "gemma"),
  ("g", "gemma"),
  ("codegemma", "codegemma"),
  ("cg", "codegemma"),
  ("c", "codegemma"),
  ("command-r", "command-r"),
  ("cr", "command-r"),
  ("command-r-plus", "command-r-plus"),
  ("crp", "command-r-plus"),
  // Specific versions
  ("llama3", "llama3.1"),
  ("llama3.0", "llama3"),
  ("llama2", "llama2"),
];

const OPENAI_MODEL_MAPPING_SRC: [(&str, &str); 13] = [
  // Default models
  ("gpt", "gpt-4o"),
  ("omni", "gpt-4o"),
  ("mini", "gpt-4o-mini"),
  ("m", "gpt-4o-mini"),
  ("turbo", "gpt-4-turbo"),
  ("t", "gpt-4-turbo"),
  // Specific versions
  ("4o", "gpt-4o"),
  ("gpt4", "gpt-4"),
  ("4", "gpt-4"),
  ("turbo4", "gpt-4-turbo"),
  ("t4", "gpt-4-turbo"),
  ("turbo35", "gpt-3.5-turbo"),
  ("t35", "gpt-3.5-turbo"),
];

const XAI_MODEL_MAPPING_SRC: [(&str, &str); 8] = [
  // Default models
  ("grok", "grok-2-latest"),
  ("grok-mini", "grok-3-mini-latest"),
  ("grok-vision", "grok-2-vision-latest"),
  // Specific versions
  ("grok-2", "grok-2-1212"),
  ("grok-2-vision", "grok-2-vision-1212"),
  ("grok-3", "grok-3-latest"),
  ("grok-3-mini", "grok-3-mini-latest"),
  ("grok-3-vision", "grok-3-vision-latest"),
];

const PERPLEXITY_MODEL_MAPPING_SRC: [(&str, &str); 17] = [
  // Supported models
  ("sonar", "sonar"),
  ("s", "sonar"),
  ("sonar-pro", "sonar-pro"),
  ("sp", "sonar-pro"),
  ("sonar-reasoning", "sonar-reasoning"),
  ("sr", "sonar-reasoning"),
  ("sonar-reasoning-pro", "sonar-reasoning-pro"),  
  ("srp", "sonar-reasoning-pro"),
  ("r1-1776", "r1-1776"),
  ("r", "r1-1776"),
  ("offline", "r1-1776"),
  ("llama-small", "llama-3.1-sonar-small-128k-online"),
  ("ls", "llama-3.1-sonar-small-128k-online"),
  ("llama-large", "llama-3.1-sonar-large-128k-online"),
  ("ll", "llama-3.1-sonar-large-128k-online"),
  ("llama-huge", "llama-3.1-sonar-huge-128k-online"),
  ("lh", "llama-3.1-sonar-huge-128k-online"),
];

fn pretty_print_mapping(mapping: &[(&str, &str)]) -> String {
  mapping
    .iter()
    .map(|(alias, model)| format!("  {: <9} → {model}\n", *alias))
    .collect::<String>()
}

fn main() {
  let models_rs_content = include_str!("src_templates/models.rs");

  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = Path::new(&out_dir).join("models.rs");

  // Write the hashmap and its pretty representation to the file
  let code = models_rs_content
    .replace(
      "// {anthropic_model_hashmap}",
      &ANTHROPIC_MODEL_MAPPING_SRC
        .iter()
        .map(|(model, constant)| format!("(\"{model}\", \"{constant}\"),\n"))
        .collect::<String>(),
    )
    .replace(
      "{anthropic_models_pretty}",
      &pretty_print_mapping(&ANTHROPIC_MODEL_MAPPING_SRC),
    )
    .replace(
      "// {cerebras_model_hashmap}",
      &CEREBRAS_MODEL_MAPPING_SRC
        .iter()
        .map(|(model, constant)| format!("(\"{model}\", \"{constant}\"),\n"))
        .collect::<String>(),
    )
    .replace(
      "{cerebras_models_pretty}",
      &pretty_print_mapping(&CEREBRAS_MODEL_MAPPING_SRC),
    )
    .replace(
      "// {deepseek_model_hashmap}",
      &DEEPSEEK_MODEL_MAPPING_SRC
        .iter()
        .map(|(model, constant)| format!("(\"{model}\", \"{constant}\"),\n"))
        .collect::<String>(),
    )
    .replace(
      "{deepseek_models_pretty}",
      &pretty_print_mapping(&DEEPSEEK_MODEL_MAPPING_SRC),
    )
    .replace(
      "// {groq_model_hashmap}",
      &GROQ_MODEL_MAPPING_SRC
        .iter()
        .map(|(model, constant)| format!("(\"{model}\", \"{constant}\"),\n"))
        .collect::<String>(),
    )
    .replace(
      "{groq_models_pretty}",
      &pretty_print_mapping(&GROQ_MODEL_MAPPING_SRC),
    )
    .replace(
      "// {ollama_model_hashmap}",
      &OLLAMA_MODEL_MAPPING_SRC
        .iter()
        .map(|(model, constant)| format!("(\"{model}\", \"{constant}\"),\n"))
        .collect::<String>(),
    )
    .replace(
      "{ollama_models_pretty}",
      &pretty_print_mapping(&OLLAMA_MODEL_MAPPING_SRC),
    )
    .replace(
      "// {openai_model_hashmap}",
      &OPENAI_MODEL_MAPPING_SRC
        .iter()
        .map(|(model, constant)| format!("(\"{model}\", \"{constant}\"),\n"))
        .collect::<String>(),
    )
    .replace(
      "{openai_models_pretty}",
      &pretty_print_mapping(&OPENAI_MODEL_MAPPING_SRC),
    )
    .replace(
      "// {xai_model_hashmap}",
      &XAI_MODEL_MAPPING_SRC
        .iter()
        .map(|(model, constant)| format!("(\"{model}\", \"{constant}\"),\n"))
        .collect::<String>(),
    )
    .replace(
      "{x_models_pretty}",
      &pretty_print_mapping(&XAI_MODEL_MAPPING_SRC),
    )
    .replace(
      "// {perplexity_model_hashmap}",
      &PERPLEXITY_MODEL_MAPPING_SRC
        .iter()
        .map(|(model, constant)| format!("(\"{model}\", \"{constant}\"),\n"))
        .collect::<String>(),
    )
    .replace(
      "{perplexity_models_pretty}",
      &pretty_print_mapping(&PERPLEXITY_MODEL_MAPPING_SRC),
    );

  fs::write(&dest_path, code).unwrap();
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=src_templates/models.rs");
}
