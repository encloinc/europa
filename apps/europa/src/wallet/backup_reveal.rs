use maud::{Markup, html};

use crate::onboard::components::{flow_topbar, link_button};

pub fn render() -> Markup {
    html! {
        section id="wallet-backup-reveal-screen" class="screen card card-compact flow-card hidden" {
            div class="flow-header wallet-backup-reveal-header" {
                (flow_topbar(Some("wallet-backup-password-screen"), Some((2, 2))))
            }

            div class="backup-warning wallet-backup-reveal-warning" {
                span class="backup-warning-icon" aria-hidden="true" {}
                p {
                    "No compartas esta frase de recuperación con nadie o podrían robar tus fondos."
                }
            }

            div id="wallet-backup-mnemonic-grid" class="seed-grid wallet-backup-seed-grid" {
                @for index in 0..12 {
                    div class="seed-chip" data-wallet-backup-word-slot=(index) {
                        span class="seed-index" { (format!("{}.", index + 1)) }
                        span class="seed-text word-value" { "••••" }
                    }
                }
            }

            div class="actions flow-actions" {
                (link_button(
                    "screen-submit",
                    "/wallet",
                    Some("wallet-backup-return"),
                    None,
                    false,
                    html! { "Regresar a menu" },
                ))
            }
        }
    }
}
