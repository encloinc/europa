use maud::{Markup, html};

pub fn render() -> Markup {
    html! {
        section id="wallet-send-scan-screen" class="screen card card-compact wallet-send-scan-screen hidden" {
            div class="wallet-send-scan-topbar" {
                button type="button" class="back-arrow" data-back="wallet-send-screen" aria-label="Regresar" {
                    img class="back-arrow-icon" src="/assets/svgs/back.svg" alt="";
                }
            }

            div id="wallet-send-scan-shell" class="wallet-send-scan-shell" data-camera-ready="false" {
                video
                    id="wallet-send-scan-video"
                    class="wallet-send-scan-video"
                    playsinline
                    autoplay
                    muted {}

                div id="wallet-send-scan-empty" class="wallet-send-scan-empty" {
                    img class="wallet-send-scan-empty-icon" src="/assets/svgs/camera-slash.svg" alt="";
                    p class="wallet-empty-copy" { "Camara no configurada" }
                }
            }
        }
    }
}
