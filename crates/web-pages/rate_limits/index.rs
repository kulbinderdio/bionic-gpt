#![allow(non_snake_case)]
use crate::app_layout::{Layout, SideBar};
use crate::i18n_helper::translate;
use assets::files::*;
use daisy_rsx::*;
use db::{authz::Rbac, Model, RateLimit};
use dioxus::prelude::*;

pub fn page(rbac: Rbac, team_id: i32, rate_limits: Vec<RateLimit>, models: Vec<Model>) -> String {
    let page = rsx! {
        Layout {
            section_class: "p-4",
            selected_item: SideBar::RateLimits,
            team_id: team_id,
            rbac: rbac,
            title: "{translate(\"rate-limits-title\")}",
            header: rsx! {
                h3 { "{translate(\"rate-limits-title\")}" }
                Button {
                    prefix_image_src: "{button_plus_svg.name}",
                    drawer_trigger: "create-limit",
                    button_scheme: ButtonScheme::Primary,
                    "{translate(\"rate-limits-add\")}"
                }
            },
            BlankSlate {
                heading: "{translate(\"rate-limits-assign-token\")}",
                visual: limits_svg.name,
                description: "{translate(\"rate-limits-roles-mapped\")}"
            }

            super::RateTable { rate_limits: rate_limits.clone(), team_id }

            for item in rate_limits {
                super::delete::DeleteDrawer {
                    team_id: team_id,
                    id: item.id,
                    trigger_id: format!("delete-trigger-{}-{}", item.id, team_id)
                }
            }

            // Our pop out drawer to add limits
            super::form::Form {
                team_id: team_id,
                models
            }
        }
    };

    crate::render(page)
}
