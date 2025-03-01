#![allow(non_snake_case)]
use crate::{
    app_layout::{Layout, SideBar},
    i18n_helper::translate,
    render,
};
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, ApiKey, Prompt, PromptType as DBPromptType};
use dioxus::prelude::*;

pub fn page(
    rbac: Rbac,
    team_id: i32,
    api_keys: Vec<ApiKey>,
    assistants: Vec<Prompt>,
    models: Vec<Prompt>,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::ApiKeys,
            team_id: team_id,
            rbac: rbac,
            title: "{translate(\"api-keys-title\")}",
            header: rsx! {
                h3 { "{translate(\"api-keys-title\")}" }
            },
            if api_keys.is_empty() {
                BlankSlate {
                    heading: "{translate(\"api-keys-empty\")}",
                    visual: empty_api_keys_svg.name,
                    description: "{translate(\"api-keys-description\")}",
                }
            },

            for item in &api_keys {
                super::delete::DeleteDrawer {
                    team_id: team_id,
                    id: item.id,
                    trigger_id: format!("delete-trigger-{}-{}", item.id, team_id)
                }
            }

            super::form::AssistantForm {
                team_id: team_id,
                prompts: assistants.clone()
            },
            super::form::ModelForm {
                team_id: team_id,
                prompts: models.clone()
            },

            if ! api_keys.is_empty() {

                Card {
                    class: "has-data-table",
                    CardHeader {
                        title: "{translate(\"api-keys-title\")}"
                    }
                    CardBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th { "{translate(\"api-keys-name\")}" }
                                th { "{translate(\"form-type\")}" }
                                th { "{translate(\"api-keys-key\")}" }
                                th { "{translate(\"prompts-title\")}/{translate(\"models-title\")}" }
                                th {
                                    class: "text-right",
                                    "{translate(\"common-actions\")}"
                                }
                            }
                            tbody {
                                for key in &api_keys {
                                    tr {
                                        td {
                                            "{key.name}"
                                        }
                                        td {
                                            PromptType {
                                                prompt_type: key.prompt_type
                                            }
                                        }
                                        td {
                                            div {
                                                class: "flex w-full",
                                                Input {
                                                    value: key.api_key.clone(),
                                                    name: "api_key",
                                                    input_type: InputType::Password
                                                }
                                                Button {
                                                    class: "api-keys-toggle-visibility",
                                                    "{translate(\"common-view\")}"
                                                }
                                            }
                                        }
                                        td {
                                            "{key.prompt_name}"
                                        }
                                        td {
                                            class: "text-right",
                                            DropDown {
                                                direction: Direction::Left,
                                                button_text: "...",
                                                DropDownLink {
                                                    drawer_trigger: format!("delete-trigger-{}-{}",
                                                        key.id, team_id),
                                                    href: "#",
                                                    target: "_top",
                                                    "{translate(\"common-delete\")}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            KeySelector {

            }

            OpenAICompatibility {

            }

            /***CodeExamples {

            }**/
        }
    };

    render(page)
}

#[component]
pub fn PromptType(prompt_type: DBPromptType) -> Element {
    match prompt_type {
        DBPromptType::Model => rsx!(
            Label {
                class: "mr-2 truncate",
                label_role: LabelRole::Info,
                "{translate(\"models-title\")}"
            }
        ),
        DBPromptType::Assistant => rsx!(
            Label {
                class: "mr-2 truncate",
                label_role: LabelRole::Highlight,
                "{translate(\"prompts-title\")}"
            }
        ),
    }
}

#[component]
fn OpenAICompatibility() -> Element {
    rsx! {
        // OpenAI API Compatibility Card
        Card {
            class: "mt-8 mb-8",
            CardBody {
                h2 { class: "card-title", "{translate(\"api-keys-openai-compatibility\")}" }
                p { "{translate(\"api-keys-warning\")}" }
                ul { class: "list-disc list-inside mt-4",
                    li { "{translate(\"api-keys-same-endpoints\")}" }
                    li { "{translate(\"api-keys-easy-migration\")}" }
                    li { "{translate(\"api-keys-similar-models\")}" }
                }
            }
        }
    }
}

#[component]
fn CodeExamples() -> Element {
    rsx! {
        Card {
            CardHeader {
                title: "{translate(\"api-keys-usage-example\")}"
            }
            CardBody {
                p {
                    ""
                }
                div { class: "mt-4",
                    pre {
                        code {
                            "// Example: Using the Assistant API
const response = await fetch('https://app.bionic-gpt.com/v1/chat/completions', {{
    method: 'POST',
    headers: {{
        'Authorization': 'Bearer YOUR_ASSISTANT_KEY',
        'Content-Type': 'application/json'
    }},
    body: JSON.stringify({{
        model: 'assistant',
            messages: [{{ role: 'user', content: 'Hello, how are you?' }}]
    }})
}});

const data = await response.json();
console.log(data.choices[0].message.content);"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn KeySelector() -> Element {
    rsx! {
        div {
            class: "grid grid-cols-1 md:grid-cols-2 gap-8 mb-8 mt-8",
            // Assistant Key Card
            Card {
                CardBody {
                    h2 {
                        class: "card-title",
                        "{translate(\"prompts-title\")} {translate(\"api-keys-key\")}"
                    }
                    p { "{translate(\"api-keys-turn-assistants-api\")}" }
                    ul { class: "list-disc list-inside mt-4",
                        li { "{translate(\"api-keys-access-assistants\")}" }
                        li { "{translate(\"api-keys-simplified-integration\")}" }
                        li { "{translate(\"api-keys-ideal-use-cases\")}" }
                    }
                    div { class: "card-actions justify-end mt-4",
                        Button {
                            drawer_trigger: "create-assistant-key",
                            "{translate(\"api-keys-create\")}"
                        }
                    }
                }
            }

            // Model Key Card
            Card {
                CardBody {
                    h2 { class: "card-title", "{translate(\"models-title\")} {translate(\"api-keys-key\")}" }
                    p { "{translate(\"api-keys-use-models\")}" }
                    ul { class: "list-disc list-inside mt-4",
                        li { "{translate(\"api-keys-full-control\")}" }
                        li { "{translate(\"api-keys-flexibility\")}" }
                        li { "{translate(\"api-keys-limits-applied\")}" }
                    }
                    div { class: "card-actions justify-end mt-4",
                        Button {
                            drawer_trigger: "create-model-key",
                            "{translate(\"api-keys-create\")}"
                        }
                    }
                }
            }
        }
    }
}
