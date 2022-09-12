// SPDX-FileCopyrightText: 2022 Nefo Fortressia <nefothingy@hotmail.com>
//
// SPDX-License-Identifier: LGPL-2.1-or-later

use crate::helpers::get_object_path;
use crate::JVM;
use jni::objects::{JString, JValue};
use zbus::dbus_interface;
use zbus::fdo::Result;
use zbus::zvariant::{DeserializeDict, ObjectPath, OwnedObjectPath, SerializeDict, Type};

use super::handle_token::HandleToken;

//  This lovelyyy codee is copieed from https://github.com/bilelmoussaoui/ashpd/blob/master/src/desktop/open_uri.rs#L125
//  Originally licensed under the lovellyy MIT License (https://github.com/bilelmoussaoui/ashpd/blob/master/LICENSE)
//  But upgraded to LGPLv2.1-or-later!
#[derive(SerializeDict, DeserializeDict, Type, Debug, Default)]
/// Specified options for a [`ScreenCastProxy::open_file`] or
/// [`ScreenCastProxy::open_uri`] request.
#[zvariant(signature = "dict")]
struct Options {
    /// A string that will be used as the last element of the handle.
    handle_token: HandleToken,
    /// Whether to allow the chosen application to write to the file.
    /// This key only takes effect the uri points to a local file that is
    /// exported in the document portal, and the chosen application is sandboxed
    /// itself.
    writeable: Option<bool>,
    /// Whether to ask the user to choose an app. If this is not passed, or
    /// false, the portal may use a default or pick the last choice.
    ask: Option<bool>,
    // Token to activate the chosen application.
    activation_token: Option<String>,
}
// Code copied from ASHPD ends here! <3

pub struct ScreenCast {}

#[dbus_interface(name = "org.freedesktop.portal.ScreenCast")]
impl ScreenCast {

    async fn create_session(
        &self,
        options: Option,
    
    ) {
        
    }
    async fn open_uri(
        &self,
        _parent_window: &str,
        uri: &str,
        options: OpenFileOptions,
    ) -> Result<OwnedObjectPath> {
      
        Ok(get_object_path(options.handle_token.clone()))
    }
}
