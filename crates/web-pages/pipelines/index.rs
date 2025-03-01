#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::i18n_helper::translate;
use assets::files::empty_api_keys_svg;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{Dataset, DocumentPipeline};
use dioxus::prelude::*;

pub fn page(
    team_id: i32,
    rbac: Rbac,
    pipelines: Vec<DocumentPipeline>,
    datasets: Vec<Dataset>,
) -> String {
    let page = rsx! {
        if pipelines.is_empty() {
            Layout {
                section_class: "p-4",
                selected_item: SideBar::DocumentPipelines,
                team_id: team_id,
                rbac: rbac,
                title: "{translate(\"document-pipelines-title\")}",
                header: rsx!(
                    h3 { "{translate(\"document-pipelines-title\")}" }
                ),
                BlankSlate {
                    heading: "{translate(\"document-pipelines-automate\")}",
                    visual: empty_api_keys_svg.name,
                    description: "{translate(\"document-pipelines-description\")}",
                    primary_action_drawer: Some((translate("document-pipelines-create").to_string(), "create-api-key".to_string()))
                }

                super::key_drawer::KeyDrawer {
                    datasets: datasets.clone(),
                    team_id: team_id,
                }
            }
        } else {
            Layout {
                section_class: "p-4",
                selected_item: SideBar::DocumentPipelines,
                team_id: team_id,
                rbac: rbac,
                title: "{translate(\"document-pipelines-title\")}",
                header: rsx!(
                    h3 { "{translate(\"document-pipelines-title\")}" }
                    Button {
                        drawer_trigger: "create-api-key",
                        button_scheme: ButtonScheme::Primary,
                        "{translate(\"document-pipelines-new\")}"
                    }
                ),
                Card {
                    class: "has-data-table",
                    CardHeader {
                        title: "{translate(\"document-pipelines-title\")}"
                    }
                    CardBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th { "{translate(\"document-pipelines-name\")}" }
                                th { "{translate(\"document-pipelines-api-key\")}" }
                                th { "{translate(\"document-pipelines-dataset\")}" }
                                th {
                                    class: "text-right",
                                    "{translate(\"document-pipelines-action\")}"
                                }
                            }
                            tbody {
                                for key in &pipelines {
                                    tr {
                                        td {
                                            "{key.name}"
                                        }
                                        td {
                                            Input {
                                                value: key.api_key.clone(),
                                                name: "api_key"
                                            }
                                        }
                                        td {
                                            "{key.dataset_name}"
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

                for item in pipelines {
                    super::delete::DeleteDrawer {
                        team_id: team_id,
                        id: item.id,
                        trigger_id: format!("delete-trigger-{}-{}", item.id, team_id)
                    }
                }

                super::key_drawer::KeyDrawer {
                    datasets: datasets.clone(),
                    team_id: team_id,
                }
            }
        }
    };

    crate::render(page)
}
