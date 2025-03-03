#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::i18n_helper::translate;
use assets::files::*;
use daisy_rsx::*;
use db::authz::Rbac;
use db::{Invitation, Member, Team, User};
use dioxus::prelude::*;

pub fn page(
    rbac: Rbac,
    members: Vec<Member>,
    invites: Vec<Invitation>,
    team: Team,
    user: User,
    team_name: String,
) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::Team,
            team_id: team.id,
            rbac: rbac.clone(),
            title: "{translate(\"teams-members\")}",
            header: rsx!(
                h3 { "{translate(\"teams-members\")}" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "create-invite-form",
                    button_scheme: ButtonScheme::Primary,
                    "{translate(\"teams-invite-new\")}"
                }
            ),

            // If the user hasn't set their org name or their own name
            // get them to do it.
            if rbac.can_make_invitations() && (user.first_name.is_none() || team.name.is_none()) {
                Card {
                    class: "mb-3",
                    CardHeader {
                        title: "{translate(\"teams-before-invite\")}"
                    }
                    CardBody {
                        if team.name.is_none() {
                            p {
                                "{translate(\"teams-set-name\")} "
                                a {
                                    href: "#",
                                    "data-drawer-target": "set-name-drawer",
                                    "{translate(\"teams-set-name\")}"
                                }
                            }
                        }
                        if user.first_name.is_none() {
                            p {
                                "{translate(\"teams-set-your-name\")} "
                                a {
                                    href: crate::routes::profile::Profile{team_id: team.id}.to_string(),
                                    "{translate(\"form-name\")}"
                                }
                            }
                        }
                    }
                }
            }

            Card {
                class: "has-data-table",
                CardHeader {
                    title: &team_name,
                    Button {
                        class: "ml-2",
                        drawer_trigger: "set-name-drawer",
                        button_size: ButtonSize::Small,
                        "{translate(\"teams-edit-name\")}"
                    }
                }
                CardBody {
                    table {
                        class: "table table-sm",
                        thead {
                            th { "{translate(\"teams-name-or-email\")}" }
                            th { "{translate(\"teams-status\")}" }
                            th {
                                class: "max-sm:hidden",
                                "{translate(\"teams-special-privileges\")}"
                            }
                            if rbac.can_make_invitations() {
                                th {
                                    class: "text-right",
                                    "{translate(\"teams-action\")}"
                                }
                            }
                        }
                        tbody {
                            for member in &members {
                                tr {
                                    td {
                                        if let (Some(first_name), Some(last_name)) = (&member.first_name, &member.last_name) {
                                            Avatar {
                                                name: "{first_name}",
                                                avatar_type: avatar::AvatarType::User
                                            }
                                            span {
                                                class: "ml-2",
                                                "{first_name} {last_name}"
                                            }
                                        } else {
                                            Avatar {
                                                name: "{member.email}",
                                                avatar_type: avatar::AvatarType::User
                                            }
                                            span {
                                                class: "ml-2",
                                                "{member.email}"
                                            }
                                        }
                                    }
                                    td {
                                        Label {
                                            label_role: LabelRole::Success,
                                            "{translate(\"teams-active\")}"
                                        }
                                    }
                                    td {
                                        class: "max-sm:hidden",
                                        for role in member.roles.clone() {
                                            super::team_role::Role {
                                                role: role
                                            }
                                        }
                                    }
                                    if rbac.can_make_invitations() && rbac.email != member.email {
                                        td {
                                            class: "text-right",
                                            DropDown {
                                                direction: Direction::Left,
                                                button_text: "...",
                                                DropDownLink {
                                                    drawer_trigger: format!("remove-member-trigger-{}-{}",
                                                        member.id, member.team_id),
                                                    href: "#",
                                                    target: "_top",
                                                    "{translate(\"teams-remove-user\")}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            for invite in &invites {
                                tr {
                                    td {
                                            Avatar {
                                                name: "{invite.first_name}",
                                                avatar_type: avatar::AvatarType::User
                                            }
                                            span {
                                                class: "ml-2",
                                                "{invite.first_name} {invite.last_name}"
                                            }
                                    }
                                    td {
                                        Label {
                                            label_role: LabelRole::Highlight,
                                            "{translate(\"teams-invite-pending\")}"
                                        }
                                    }
                                    td {
                                        for role in invite.roles.clone() {
                                            super::team_role::Role {
                                                role
                                            }
                                        }
                                    }

                                    if rbac.can_make_invitations() {
                                        td {
                                            class: "text-right",
                                            DropDown {
                                                direction: Direction::Left,
                                                button_text: "...",
                                                DropDownLink {
                                                    drawer_trigger: format!("remove-invite-trigger-{}-{}",
                                                        invite.id, invite.team_id),
                                                    href: "#",
                                                    target: "_top",
                                                    "{translate(\"teams-delete-invite\")}"
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

            for member in members {
                super::remove_member::RemoveMemberDrawer {
                    team_id: member.team_id,
                    user_id: member.id,
                    email: member.email.clone(),
                    trigger_id: format!("remove-member-trigger-{}-{}", member.id, member.team_id)
                }
            }

            for invite in invites {
                super::remove_invite::RemoveInviteDrawer {
                    team_id: invite.team_id,
                    invite_id: invite.id,
                    trigger_id: format!("remove-invite-trigger-{}-{}", invite.id, invite.team_id)
                }
            }

            // The form to create an invitation
            super::invitation_form::InvitationForm {
                submit_action: crate::routes::team::CreateInvite{team_id:team.id}.to_string()
            }

            // Form to set he org name
            super::team_name_form::TeamNameForm {
                submit_action: crate::routes::team::SetName{team_id:team.id}.to_string()
            }
        }
    };

    crate::render(page)
}
