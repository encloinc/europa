use maud::{Markup, html};

use crate::config::BitcoinNetwork;

pub fn render(network: BitcoinNetwork) -> Markup {
    html! {
        section
            id="menu-screen"
            class="screen card wallet-card hidden"
            data-scroll-fade-shell
            data-at-bottom="false"
            data-has-overflow="true" {
            div id="wallet-card-scroll" class="wallet-card-scroll" data-scroll-fade-target {
                div class="wallet-card-body" {
                    button type="button" id="wallet-account-card" class="wallet-account-card" aria-label="Ver cuentas" {
                        div class="wallet-account-leading" {
                            img class="wallet-account-icon" src="/assets/svgs/mibilleterabitcoin-icon.svg" alt="";
                            div class="wallet-account-copy" {
                                p id="wallet-account-name" class="wallet-account-title" { "Billetera #0" }
                                p id="wallet-address" class="wallet-account-address" { "tb1qvcw8t...6sm7pudqz" }
                            }
                        }
                        img class="wallet-account-chevron" src="/assets/svgs/caret.svg" alt="" aria-hidden="true";
                    }

                    div class="wallet-balance-block" {
                        p class="wallet-balance-main" {
                            img class="wallet-balance-icon" src=(network.bitcoin_icon_src()) alt="";
                            span id="wallet-balance-primary" { "-- " (network.bitcoin_symbol()) }
                        }
                        p id="wallet-balance-fiat" class="wallet-balance-fiat" { "≈ -- MXN" }
                    }

                    div class="wallet-actions-grid" {
                        button type="button" class="wallet-action-tile" id="wallet-send-action" {
                            div class="wallet-action-visual" {
                                span class="wallet-action-icon-shell" {
                                    img class="wallet-action-icon" src="/assets/svgs/plane.svg" alt="";
                                }
                            }
                            div class="wallet-action-copy" {
                                span class="wallet-action-label" { "Enviar" }
                            }
                        }

                        button type="button" class="wallet-action-tile" id="wallet-receive-action" {
                            div class="wallet-action-visual" {
                                span class="wallet-action-icon-shell wallet-action-icon-shell-qr" aria-hidden="true" {
                                    img class="wallet-action-icon wallet-action-icon-qr" src="/assets/svgs/qr.svg" alt="";
                                }
                            }
                            div class="wallet-action-copy" {
                                span class="wallet-action-label" { "Recibir" }
                            }
                        }

                        button type="button" class="wallet-action-tile" id="wallet-backup-action" {
                            div class="wallet-action-visual" {
                                span class="wallet-action-icon-shell" {
                                    img class="wallet-action-icon" src="/assets/svgs/lock.svg" alt="";
                                }
                            }
                            div class="wallet-action-copy" {
                                span class="wallet-action-label" { "Respaldar" }
                            }
                        }
                    }

                    div id="wallet-transactions-section" class="wallet-transactions-section hidden" data-drag-scroll-area {
                        h3 class="wallet-section-title" { "Transacciones" }

                        div id="wallet-transactions-list" class="wallet-transactions-list" {}
                    }

                    div id="wallet-empty-state" class="wallet-empty-state hidden" {
                        div class="wallet-empty-icon" aria-hidden="true" {}
                        p class="wallet-empty-copy" { "Ninguna transaccion aqui todavia" }
                    }
                }
            }

            div class="wallet-scroll-fade" aria-hidden="true" {
                img class="wallet-scroll-fade-caret" src="/assets/svgs/caret.svg" alt="";
            }
        }
    }
}
