use super::handle_token::HandleToken;
use crate::bindings::activity::Activity;
use crate::bindings::intent::{self, Action, Intent};
use crate::bindings::uri::Uri;
use crate::helpers::{get_env, get_object_path};
use crate::JVM;
use jni::objects::{JString, JValue};
use std::fs;
use std::os::unix::prelude::*;
use zbus::dbus_interface;
use zbus::fdo::{Error, Result};
use zbus::zvariant::{DeserializeDict, Fd, ObjectPath, OwnedObjectPath, SerializeDict, Type};

//  This lovelyyy codee is copieed from https://github.com/bilelmoussaoui/ashpd/blob/master/src/desktop/open_uri.rs#L125
//   Licensed under the lovellyy MIT License (https://github.com/bilelmoussaoui/ashpd/blob/master/LICENSE)
#[derive(SerializeDict, DeserializeDict, Type, Debug, Default)]
/// Specified options for a [`OpenURIProxy::open_file`] or
/// [`OpenURIProxy::open_uri`] request.
#[zvariant(signature = "dict")]
struct OpenUriOptions {
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

pub struct OpenURI {}

#[dbus_interface(name = "org.freedesktop.portal.OpenURI")]
impl OpenURI {
    async fn open_uri(
        &self,
        _parent_window: &str,
        uri: &str,
        options: OpenUriOptions,
    ) -> Result<OwnedObjectPath> {
        let env = get_env();

        let juri = Uri::from_string(uri);
        let intent = Intent::with_uri(Action::VIEW, &juri);

        Activity::start_activity(&intent);

        Ok(get_object_path(options.handle_token.clone()))
    }

    async fn open_directory(
        &self,
        parent_window: &str,
        directory: Fd,
        options: OpenUriOptions,
    ) -> Result<OwnedObjectPath> {
        let uri = format!("/proc/self/fd/{:?}", directory.as_raw_fd());

        if uri.starts_with("/data/data") {
            return Err(Error::AccessDenied("Files under /data/data might not be available to other apps! And we don't want to open their secret files!".to_string()));
        }
        
        match fs::canonicalize(uri) {
            Ok(path) => {
                self.open_uri(parent_window, &path.to_string_lossy(), options)
                    .await
            }
            Err(error) => Err(Error::IOError(error.to_string())),
        }
    }
    async fn open_file(
        &self,
        parent_window: &str,
        file: Fd,
        options: OpenUriOptions,
    ) -> Result<OwnedObjectPath> {
        let uri = format!("/proc/self/fd/{:?}", file.as_raw_fd());

        if uri.starts_with("/data/data") {
            return Err(Error::AccessDenied("Files under /data/data might not be available to other apps! And we don't want to open their secret files!".to_string()));
        }
        match fs::canonicalize(uri) {
            Ok(path) => {
                self.open_uri(parent_window, &path.to_string_lossy(), options)
                    .await
            }
            Err(error) => Err(Error::IOError(error.to_string())),
        }
    }
}
