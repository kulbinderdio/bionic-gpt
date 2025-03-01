#![allow(non_snake_case)]
use daisy_rsx::*;
use db::ModelType;
use dioxus::prelude::*;
use crate::i18n_helper::translate;

#[component]
pub fn Model(model_type: ModelType) -> Element {
    match model_type {
        ModelType::LLM => rsx!(
            Label {
                class: "truncate",
                label_role: LabelRole::Info,
                "{translate(\"model-type-llm\")}"
            }
        ),
        ModelType::Embeddings => rsx!(
            Label {
                class: "truncate",
                label_role: LabelRole::Highlight,
                "{translate(\"model-type-embeddings\")}"
            }
        ),
        ModelType::TextToSpeech => rsx!(
            Label {
                class: "truncate",
                label_role: LabelRole::Warning,
                "Text To Speech"
            }
        ),
        ModelType::Image => rsx!(
            Label {
                class: "truncate",
                label_role: LabelRole::Neutral,
                "Image Generation"
            }
        ),
    }
}
