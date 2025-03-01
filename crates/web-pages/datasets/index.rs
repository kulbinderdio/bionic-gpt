#![allow(non_snake_case)]
use crate::app_layout::Layout;
use crate::app_layout::SideBar;
use crate::i18n_helper::translate;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::queries::{datasets::Dataset, models::Model};
use dioxus::prelude::*;

pub fn page(
    rbac: Rbac,
    team_id: i32,
    datasets: Vec<Dataset>,
    models: Vec<Model>,
    can_set_visibility_to_company: bool,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Datasets,
            team_id: team_id,
            rbac: rbac.clone(),
            title: "{translate(\"datasets-title\")}",
            header: rsx!(
                h3 { "{translate(\"datasets-title\")}" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "new-dataset-form",
                    button_scheme: ButtonScheme::Primary,
                    "{translate(\"datasets-create\")}"
                }
            ),

            if datasets.is_empty() {
                BlankSlate {
                    heading: "{translate(\"datasets-empty\")}",
                    visual: nav_ccsds_data_svg.name,
                    description: "{translate(\"datasets-empty-description\")}"
                }
            } else {
                Card {
                    class: "has-data-table",
                    CardHeader {
                        title: "{translate(\"datasets-title\")}"
                    }
                    CardBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th { "{translate(\"form-name\")}" }
                                th { "{translate(\"form-status\")}" }
                                th {
                                    class: "max-sm:hidden",
                                    "{translate(\"documents-title\")}"
                                }
                                th {
                                    class: "max-sm:hidden",
                                    "Chunking Strategy"
                                }
                                th {
                                    class: "text-right",
                                    "{translate(\"common-actions\")}"
                                }
                            }
                            tbody {

                                for dataset in &datasets {
                                    tr {
                                        td {
                                            a {
                                                href: crate::routes::documents::Index{team_id, dataset_id: dataset.id}.to_string(),
                                                "{dataset.name}"
                                            }
                                        }
                                        td {
                                            crate::prompts::visibility::VisLabel {
                                                visibility: dataset.visibility
                                            }
                                        }
                                        td {
                                            class: "max-sm:hidden",
                                            "{dataset.count}"
                                        }
                                        td {
                                            class: "max-sm:hidden",
                                            Label {
                                                label_role: LabelRole::Highlight,
                                                "By Title"
                                            }
                                            }
                                        td {
                                            class: "text-right",
                                            DropDown {
                                                direction: Direction::Left,
                                                button_text: "...",
                                                DropDownLink {
                                                    href: crate::routes::documents::Index{team_id, dataset_id: dataset.id}.to_string(),
                                                    target: "_top",
                                                    "{translate(\"common-view\")}"
                                                }

                                                if rbac.can_edit_dataset(dataset) {
                                                    DropDownLink {
                                                        drawer_trigger: format!("edit-trigger-{}-{}",
                                                            dataset.id, team_id),
                                                        href: "#",
                                                        target: "_top",
                                                        "{translate(\"common-edit\")}"
                                                    }
                                                }
                                                DropDownLink {
                                                    drawer_trigger: format!("delete-trigger-{}-{}",
                                                        dataset.id, team_id),
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

                    for dataset in datasets {
                        super::delete::DeleteDrawer {
                            team_id: team_id,
                            id: dataset.id,
                            trigger_id: format!("delete-trigger-{}-{}", dataset.id, team_id)
                        }

                        super::upsert::Upsert {
                            id: dataset.id,
                            trigger_id: format!("edit-trigger-{}-{}", dataset.id, team_id),
                            name: dataset.name,
                            models: models.clone(),
                            team_id: team_id,
                            combine_under_n_chars: dataset.combine_under_n_chars,
                            new_after_n_chars: dataset.new_after_n_chars,
                            _multipage_sections: true,
                            visibility: dataset.visibility,
                            can_set_visibility_to_company
                        }
                    }
                }
            }

            super::upsert::Upsert {
                trigger_id: "new-dataset-form",
                name: "".to_string(),
                models: models.clone(),
                team_id: team_id,
                combine_under_n_chars: 500,
                new_after_n_chars: 1000,
                _multipage_sections: true,
                visibility: db::Visibility::Private,
                can_set_visibility_to_company
            }
        }
    };

    crate::render(page)
}
