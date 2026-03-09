use maud::{Markup, html};

use crate::onboard::components::{flow_header, input_field, link_button, password_toggle};

pub fn render() -> Markup {
    html! {
        section id="wallet-backup-password-screen" class="screen card card-compact flow-card hidden" {
            (flow_header(
                Some("menu-screen"),
                Some((1, 2)),
                "Respaldar billetera",
                "Escribe tu contraseña para revelar tu frase de recuperación.",
            ))

            form id="wallet-backup-form" class="stack flow-form wallet-backup-form" autocomplete="off" {
                (input_field(
                    Some(html! {
                        label class="input-label" for="wallet-backup-password" { "Contraseña:" }
                    }),
                    html! {
                        input
                            class="input-control"
                            id="wallet-backup-password"
                            type="password"
                            inputmode="text"
                            spellcheck="false"
                            autocapitalize="off"
                            autocomplete="off"
                            data-1p-ignore="true"
                            data-lpignore="true"
                            required;
                    },
                    None,
                    Some(password_toggle("wallet-backup-password")),
                ))

                div class="actions flow-actions" {
                    (link_button(
                        "screen-submit",
                        "/wallet/backup/reveal",
                        Some("wallet-backup-submit"),
                        Some("wallet-backup-form"),
                        true,
                        html! { "Revelar frase de recuperación" },
                    ))
                }
            }
        }
    }
}
