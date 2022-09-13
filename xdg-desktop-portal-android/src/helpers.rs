// SPDX-FileCopyrightText: 2022 Nefo Fortressia <nefothingy@hotmail.com>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::JVM;
use crate::{handle_token::HandleToken, CONNECTION};
use jni::{JNIEnv, JavaVM};
use zbus::zvariant::OwnedObjectPath;

pub fn get_env<'a>() -> JNIEnv<'a> {
    let vm: &JavaVM = JVM.get().unwrap();
    vm.get_env().unwrap()
}
/**
 *
 * Copied from https://github.com/bilelmoussaoui/ashpd/blob/master/src/desktop/request.rs#L191-L197
 *
 * Upgraded to LGPLv2.1-or-later!
 */
pub fn get_object_path(handle_token: HandleToken) -> OwnedObjectPath {
    let connection = CONNECTION.get().unwrap();
    let unique_name = connection.unique_name().unwrap();
    let unique_identifier = unique_name.trim_start_matches(':').replace('.', "_");

    OwnedObjectPath::try_from(format!(
        "/org/freedesktop/portal/desktop/request/{}/{}",
        unique_identifier, handle_token
    ))
    .unwrap()
}
