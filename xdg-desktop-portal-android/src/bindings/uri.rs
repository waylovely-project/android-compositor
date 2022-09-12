// SPDX-FileCopyrightText: 2022 Nefo Fortressia <nefothingy@hotmail.com>
//
// SPDX-License-Identifier: LGPL-2.1-or-later

use std::ops::Deref;

use jni::objects::JValue;

use crate::JVM;

pub struct Uri<'a>(JValue<'a>);

impl Uri<'_> {
    pub fn from_string(uri: &str) -> Self {
        let jvm = JVM.get().unwrap();

        let env = jvm.get_env().unwrap();
        let juri = env
            .call_static_method(
                "android/net/Uri",
                "path",
                "(Ljava/lang/String;)Landroid/net/Uri;",
                &[JValue::from(env.new_string(uri).unwrap())],
            )
            .unwrap();

        Self(juri)
    }
}

impl<'a> Deref for Uri<'a> {
    type Target = JValue<'a>;

    fn deref(&self) -> &JValue<'a> {
        &self.0
    }
}