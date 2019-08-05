use super::{DialogData, FirmwareUpdateDialog};
use crate::{Entity, FirmwareEvent, FwupdDevice, FwupdRelease};
use gtk::{self, prelude::*};
use std::{collections::BTreeSet, sync::Arc};

pub(crate) struct FwupdDialogData {
    pub entity:   Entity,
    pub device:   Arc<FwupdDevice>,
    pub releases: BTreeSet<FwupdRelease>,
    pub shared:    DialogData,
    pub latest:   Box<str>,
}

pub(crate) fn fwupd_dialog(
    data: &FwupdDialogData,
    upgradeable: bool,
    has_battery: bool,
    upgrade_button: bool,
) {
    let &FwupdDialogData { entity, device, releases, latest, shared } = &data;
    let &DialogData { sender, stack, .. } = &shared;

    let response = if !upgrade_button || device.needs_reboot() {

        let log_entries = releases
            .iter()
            .rev()
            .map(|release| (release.version.as_ref(), release.description.as_ref()));

        let dialog = FirmwareUpdateDialog::new(
            latest,
            log_entries,
            upgradeable,
            device.needs_reboot(),
            has_battery,
        );

        let response = dialog.run();
        dialog.destroy();
        response
    } else {
        gtk::ResponseType::Accept
    };

    if gtk::ResponseType::Accept == response {
        // Exchange the button for a progress bar.
        if let Some(stack) = stack.upgrade() {
            stack.switch_to_waiting();
        }

        let _ = sender.send(FirmwareEvent::Fwupd(
            *entity,
            device.clone(),
            Arc::new(releases.iter().last().expect("no release found").clone()),
        ));
    }
}
