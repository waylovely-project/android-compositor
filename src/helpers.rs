// SPDX-FileCopyrightText: 2022 Nefo Fortressia <nefothingy@hotmail.com>
//
// SPDX-License-Identifier: LGPL-2.1-or-later

use zbus::zvariant::{ OwnedObjectPath};
use jni::{JNIEnv, JavaVM};
use crate::JVM;
use crate::{handle_token::HandleToken, CONNECTION};

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
