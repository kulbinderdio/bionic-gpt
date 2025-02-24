#![allow(non_snake_case)]
use daisy_rsx::*;
use dioxus::prelude::*;
use rust_i18n::t;

#[component]
pub fn HistoryTable(team_id: i32, buckets: Vec<super::HistoryBucket>) -> Element {
    rsx!(
        for bucket in buckets {
            if ! bucket.histories.is_empty() {
                Card {
                    class: "has-data-table mb-6",
                    CardHeader {
                        title: "{bucket.name}"
                    }
                    CardBody {
                        table {
                            class: "table table-sm",
                            thead {
                                th {
                                    "width": "200",
                                    {t!("history-time")}
                                }
                                th { {t!("history-summary")} }
                            }
                            tbody {
                                for history in bucket.histories {
                                    tr {
                                        td {
                                            RelativeTime {
                                                format: RelativeTimeFormat::Relative,
                                                datetime: &history.created_at_iso
                                            }
                                        }
                                        td {
                                            a {
                                                href: crate::routes::console::Conversation{team_id, conversation_id: history.id}.to_string(),
                                                "{history.summary}"
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
    )
}
