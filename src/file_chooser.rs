use std::collections::HashMap;
use std::iter::Product;

use crate::bindings::intent::Action;
use crate::{bindings::intent::Intent, helpers::get_object_path};
use itertools::{Itertools, iproduct};
use jni::objects::{JString, JValue};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::slice::Iter;
use zbus::dbus_interface;
use zbus::fdo::{Error, Result};
use zbus::zvariant::{DeserializeDict, OwnedObjectPath, SerializeDict, Type};

use super::handle_token::HandleToken;



//  This lovelyyy codee is copieed from https://github.com/bilelmoussaoui/ashpd/blob/master/src/desktop/open_uri.rs
//   Licensed under the lovellyy MIT License (https://github.com/bilelmoussaoui/ashpd/blob/master/LICENSE)

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
/// A file filter, to limit the available file choices to a mimetype or a glob
/// pattern.
pub struct FileFilter(String, Vec<(FilterType, String)>);

#[derive(Serialize_repr, Clone, Deserialize_repr, PartialEq, Eq, Debug, Type)]
#[repr(u32)]
enum FilterType {
    GlobPattern = 0,
    MimeType = 1,
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
/// Presents the user with a choice to select from or as a checkbox.
pub struct Choice(String, String, Vec<(String, String)>, String);

#[derive(SerializeDict, DeserializeDict, Type, Clone, Debug, Default)]
/// Specified options for a [`FileChooserProxy::open_file`] request.
#[zvariant(signature = "dict")]
pub struct OpenFileOptions {
    /// A string that will be used as the last element of the handle.
    handle_token: HandleToken,
    /// Label for the accept button. Mnemonic underlines are allowed.
    accept_label: Option<String>,
    /// Whether the dialog should be modal.
    modal: Option<bool>,
    /// Whether multiple files can be selected or not.
    multiple: Option<bool>,
    /// Whether to select for folders instead of files.
    directory: Option<bool>,
    /// List of serialized file filters.
    filters: Vec<FileFilter>,
    /// Request that this filter be set by default at dialog creation.
    current_filter: Option<FileFilter>,
    /// List of serialized combo boxes to add to the file chooser
    choices: Vec<Choice>,
}

// Code copied from ASHPD ends here! <3

pub struct FileChooser {}

#[dbus_interface(name = "org.freedesktop.portal.FileChooser")]
impl FileChooser {
    async fn open_file(
        &self,
        _parent_window: &str,
        _title: &str,
        options: OpenFileOptions,
    ) -> Result<OwnedObjectPath> {
        let intent = Intent::with_action(Action::GET_CONTENT);
        let mut mimes = Vec::new();
        for filter in options.filters {
            for mime in filter.1 {
                if mime.0 == FilterType::MimeType {
                    mimes.push(mime.1);
                } else if mime.0 == FilterType::GlobPattern {
                    
                }
            }
        }

        intent.set_type_and_normalize(&mimes.join(","));
        Ok(get_object_path(options.handle_token.clone()))
    }
}
