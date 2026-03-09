use maud::{Markup, html};

pub fn render(required_confirmations: u32) -> Markup {
    html! {
        section id="wallet-receive-screen" class="screen card card-compact wallet-receive-screen hidden" {
            div class="wallet-receive-topbar" {
                button type="button" class="back-arrow" data-back="menu-screen" aria-label="Regresar" {
                    img class="back-arrow-icon" src="/assets/svgs/back.svg" alt="";
                }
            }

            div class="wallet-receive-copy" {
                h2 class="wallet-receive-title" { "Recibir Bitcoin" }
                p class="wallet-receive-description" {
                    "Para depositar Bitcoin en tu billetera, por favor envía cualquier cantidad de Bitcoin a la siguiente dirección."
                }
            }

            div class="wallet-receive-qr-shell" {
                div id="wallet-receive-qr" class="wallet-receive-qr" aria-hidden="true" {}
                div class="wallet-receive-qr-badge" {
                    img class="wallet-receive-qr-badge-icon" src="/assets/svgs/bitcoin.svg" alt="";
                }
            }

            div class="wallet-receive-address-block" {
                p class="wallet-receive-address-label" { "Tu dirección de Bitcoin:" }
                div class="wallet-receive-address-shell" {
                    p id="wallet-receive-address" class="wallet-receive-address" { "bc1q4g0w7n3yjuzpx5s6umw03mzca49ktkmvxm976nyv0k272m2vl48slrrw5l" }
                }
            }

            div class="wallet-receive-footer" {
                img class="wallet-receive-footer-icon" src="/assets/svgs/3-bitcoin.svg" alt="";
                p class="wallet-receive-footer-copy" {
                    "Los depósitos necesitan "
                    span class="wallet-receive-footer-emphasis" { (required_confirmations) " confirmaciones" }
                    " en la blockchain para aparecer en tu billetera. Puedes revisar el estado de tu depósito en el menú principal."
                }
            }
        }
    }
}
