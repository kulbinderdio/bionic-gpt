#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::i18n_helper::translate;
use assets::files::button_plus_svg;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{InviteSummary, TeamOwner};
use dioxus::prelude::*;

pub fn page(
    rbac: Rbac,
    team_id: i32,
    teams: Vec<TeamOwner>,
    invites: Vec<InviteSummary>,
    current_user_email: String,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Switch,
            team_id: team_id,
            rbac: rbac,
            title: "{translate(\"your-teams-title\")}",
            header: rsx!(
                h3 { "{translate(\"your-teams-title\")}" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "create-new-team",
                    button_scheme: ButtonScheme::Primary,
                    "{translate(\"your-teams-create\")}"
                }
            ),
            Card {
                class: "has-data-table",
                CardHeader {
                    title: "{translate(\"your-teams-teams\")}"
                }
                CardBody {
                    table {
                        class: "table table-sm",
                        thead {
                            th { "{translate(\"your-teams-team\")}" }
                            th { "{translate(\"your-teams-creator\")}" }
                            th {
                                class: "text-right",
                                "{translate(\"your-teams-action\")}"
                            }
                        }
                        tbody {
                            for team in &teams {

                                if let Some(name) = &team.team_name {
                                    tr {
                                        td {
                                            Avatar {
                                                name: "{name}",
                                                avatar_type: avatar::AvatarType::Team
                                            }
                                            span {
                                                class: "ml-2 mr-2",
                                                "{name}"
                                            }
                                            if team.id != team_id {
                                                a {
                                                    "data-turbo-frame": "_top",
                                                    href: crate::routes::team::Index{ team_id: team.id }.to_string(),
                                                    "({translate(\"your-teams-switch\")})"
                                                }
                                            }
                                        }
                                        td {
                                            strong {
                                                "{team.team_owner}"
                                            }
                                        }
                                        if team.team_owner == current_user_email && teams.len() > 1 {
                                            td {
                                                class: "text-right",
                                                DropDown {
                                                    direction: Direction::Left,
                                                    button_text: "...",
                                                    DropDownLink {
                                                        drawer_trigger: format!("delete-trigger-{}", team.id),
                                                        href: "#",
                                                        target: "_top",
                                                        "{translate(\"your-teams-delete\")}"
                                                    }
                                                }
                                            }
                                        } else {
                                            td {
                                                class: "text-right",
                                            }
                                        }
                                    }
                                } else {
                                    tr {
                                        td {
                                            Avatar {
                                                avatar_type: avatar::AvatarType::Team
                                            }
                                                span {
                                                    class: "ml-2 mr-2",
                                                    "{translate(\"your-teams-name-not-set\")}"
                                                }
                                                if team.id != team_id {
                                                    a {
                                                        "data-turbo-frame": "_top",
                                                        href: crate::routes::team::Index{ team_id: team.id }.to_string(),
                                                        "({translate(\"your-teams-switch\")})"
                                                }
                                            }
                                        }
                                        td {
                                            strong {
                                                "{team.team_owner}"
                                            }
                                        }
                                        if team.team_owner == current_user_email && teams.len() > 1 {
                                            td {
                                                class: "text-right",
                                                DropDown {
                                                    direction: Direction::Left,
                                                    button_text: "...",
                                                    DropDownLink {
                                                        drawer_trigger: format!("delete-trigger-{}", team.id),
                                                        href: "#",
                                                        target: "_top",
                                                        "{translate(\"your-teams-delete\")}"
                                                    }
                                                }
                                            }
                                        } else {
                                            td {
                                                class: "text-right",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }


            Card {
                class: "has-data-table mt-8",
                CardHeader {
                    title: "{translate(\"your-teams-invitations\")}"
                }
                CardBody {
                    table {
                        class: "table table-sm",
                        thead {
                            th { "{translate(\"your-teams-team\")}" }
                            th {
                                "{translate(\"your-teams-creator\")}"
                            }
                            th {
                                class: "text-right",
                                "{translate(\"your-teams-action\")}"
                            }
                        }
                        tbody {
                            for invite in &invites {
                                td {
                                    "{invite.team_name}"
                                }
                                td {
                                    "{invite.created_by}"
                                }
                                td {
                                    class: "text-right",
                                    DropDown {
                                        direction: Direction::Left,
                                        button_text: "...",
                                        DropDownLink {
                                            drawer_trigger: format!("accept-invite-trigger-{}", invite.id),
                                            href: "#",
                                            target: "_top",
                                            "{translate(\"your-teams-accept\")}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            for invite in invites {
                super::accept_invitation::AcceptInvite {
                    invite,
                    team_id
                }
            }

            for team in teams {
                super::delete::DeleteDrawer {
                    team_id: team.id,
                    trigger_id: format!("delete-trigger-{}", team.id)
                }
            }

            // The for to create new teams
            form {
                method: "post",
                "data-turbo-frame": "_top",
                action: crate::routes::teams::New{team_id}.to_string(),
                Drawer {
                    label: "{translate(\"your-teams-create-new\")}",
                    trigger_id: "create-new-team",
                    DrawerBody {
                        div {
                            class: "flex flex-col",
                            Input {
                                input_type: InputType::Text,
                                placeholder: "{translate(\"your-teams-team\")}",
                                help_text: "{translate(\"your-teams-give-name\")}",
                                required: true,
                                label: "{translate(\"your-teams-name\")}",
                                name: "name"
                            }
                        }
                    }
                    DrawerFooter {
                        Button {
                            button_type: ButtonType::Submit,
                            button_scheme: ButtonScheme::Primary,
                            "{translate(\"teams-create\")}"
                        }
                    }
                }
            }
        }
    };

    crate::render(page)
}
