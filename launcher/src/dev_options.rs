//! Developer options: a Settings section nothing in the window points to. Seven clicks on the
//! sidebar's "Launcher x.y.z" reveal it for this run; it stays shown while one of its options is
//! on. Its one option turns offline play off (`backend::Settings::require_account`): offline names
//! play without a Microsoft account unless it is on. Either way the launcher downloads Mojang's
//! files (client, libraries, assets, Java) only for an owner (`backend::download_for_owner`).
use super::*;

/// Clicks on the launcher's version that reveal the section.
const CLICKS: u8 = 7;

impl Launcher {
    /// A click on the sidebar's launcher version; the last one opens Settings on the section.
    pub(super) fn version_clicked(&mut self) {
        if self.dev_options {
            return;
        }
        self.dev_clicks += 1;
        if self.dev_clicks >= CLICKS {
            self.dev_options = true;
            self.go(Page::Settings);
        }
    }

    /// The section, when revealed or when one of its options is on.
    pub(super) fn dev_options_section(&self, cx: &mut Context<Self>) -> Option<Div> {
        if !self.dev_options && !self.settings.require_account {
            return None;
        }
        let t = self.theme;
        let toggle = switch("dev-offline", self.settings.require_account, &t).on_click(cx.listener(
            |this, _: &ClickEvent, _, cx| {
                this.settings.require_account = !this.settings.require_account;
                this.save_settings();
                cx.notify();
            },
        ));
        Some(
            div()
                .flex()
                .flex_col()
                .child(
                    section_title("Developer options", "terminal", &t)
                        .mt(px(20.))
                        .mb(px(12.)),
                )
                .child(index_row(
                    0,
                    tr("Require a Microsoft account to play"),
                    Some(
                        div()
                            .child(tr(
                                "Off: offline names play without signing in. Downloading Minecraft \
                                 from Mojang always needs a Microsoft account that owns it.",
                            ))
                            .into_any_element(),
                    ),
                    Some(toggle.into_any_element()),
                    &t,
                )),
        )
    }
}
