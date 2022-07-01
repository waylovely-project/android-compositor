use jni::objects::{JValue, JString};

use crate::helpers::get_env;

use super::intent::Intent;


pub struct Activity {

}

impl Activity {
    pub fn start_activity(intent: &Intent) {
        get_env().call_static_method(
            "android/app/Activity",
            "startActivity",
            "(Landroid/content/Intent;)V",
            &[JValue::from(**intent) ],
        )
        .unwrap();

    }
}