use std::ops::Deref;

use jni::{
    errors::Result,
    objects::{JObject, JString, JValue},
    JNIEnv,
};

use crate::helpers::get_env;

use super::uri::Uri;

pub struct Intent<'a>(JObject<'a>);

impl Intent<'_> {
    pub const ID: &'static str = "android/content/Intent";

    pub fn with_action(action: Action) -> Self {
        let env: JNIEnv = get_env();
        let intent = env
            .new_object(
                Self::ID,
                "(Ljava/lang/String;)Landroid/content/Intent;",
                &[action.into()],
            )
            .unwrap();
        Self(intent)
    }
    pub fn with_uri(action: Action, juri: &Uri) -> Self {
        let env: JNIEnv = get_env();
        let intent = env
            .new_object(
                Self::ID,
                "(Ljava/lang/String;Landroid/net/Uri;)Landroid/content/Intent;",
                &[action.into(), **juri],
            )
            .unwrap();
        Self(intent)
    }

    pub fn set_type(&self, mime: &str) -> Result<JValue> {
        let env: JNIEnv = get_env();

        env.call_static_method(Self::ID, "setType", "()", &[])
    }

    pub fn normalize_mime_type(mime: &str) -> Result<JValue> {
        let env: JNIEnv = get_env();

        env.call_static_method(
            Self::ID,
            "normalizeMimeType",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[env.new_string(mime).unwrap().into()],
        )
    }

    pub fn set_type_and_normalize(&self, mime: &str) ->  Result<JValue> {
        let env: JNIEnv = get_env();

        env.call_static_method(
            Self::ID,
            "setTypeAndNormalize",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[env.new_string(mime).unwrap().into()],
        )
    }
}

impl<'a> From<Intent<'a>> for JValue<'a> {
    fn from(intent: Intent<'a>) -> Self {
        Self::from(intent.0)
    }
}

impl<'a> Deref for Intent<'a> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Action(&'static str);

impl Action {
    pub const VIEW: Action = Action("android.intent.action.VIEW");
    pub const GET_CONTENT: Action = Action("android.intent.action.GET_CONTENT");
}

impl<'a> Into<JString<'a>> for Action {
    fn into(self) -> JString<'a> {
        get_env().new_string(self.0).unwrap()
    }
}

impl<'a> Into<JValue<'a>> for Action {
    fn into(self) -> JValue<'a> {
        JValue::from(Into::<JString>::into(self))
    }
}
