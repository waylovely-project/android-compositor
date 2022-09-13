// SPDX-FileCopyrightText: 2022 Nefo Fortressia <nefothingy@hotmail.com>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub mod bindings;
mod file_chooser;
pub mod glob_to_mime;
use file_chooser::FileChooser;
use jni::{InitArgsBuilder, JNIVersion, JavaVM};
use once_cell::sync::OnceCell;
use zbus::{Connection, ConnectionBuilder};
mod handle_token;
mod open_uri;
use open_uri::OpenURI;
mod helpers;
pub static JVM: OnceCell<JavaVM> = OnceCell::new();
pub static CONNECTION: OnceCell<Connection> = OnceCell::new();

fn init_jvm() -> jni::errors::Result<()> {
    // Build the VM properties
    let jvm_args = InitArgsBuilder::new()
        // Pass the JNI API version (default is 8)
        .version(JNIVersion::V8)
        // You can additionally pass any JVM options (standard, like a system property,
        // or VM-specific).
        // Here we enable some extra JNI checks useful during development
        .option("-Xcheck:jni")
        .build()
        .unwrap();

    // Create a new VM
    let jvm = JavaVM::new(jvm_args)?;
    jvm.attach_current_thread()?;

    if JVM.set(jvm).is_err() {
        panic!("Can't assign to JVM once cell variable");
    };
    // Attach the current thread to call into Java — see extra options in
    // "Attaching Native Threads" section.
    //
    // This method returns the guard that will detach the current thread when dropped,
    // also freeing any local references created in it

    Ok(())
}

async fn init_zbus() -> zbus::Result<()> {
    let connection = ConnectionBuilder::session()?
        .name("org.freedesktop.impl.portal.desktop.android")?
        .serve_at("/org/freedesktop/portal/desktop", OpenURI {})?
        .serve_at("/org/freedesktop/portal/desktop", FileChooser {})?
        .build()
        .await?;
    if CONNECTION.set(connection).is_err() {
        panic!("Can't set the CONNECTION once cell variable")
    };

    loop {
        std::future::pending::<()>().await;
    }
}
pub enum Errors {
    JNIError(jni::errors::Error),
    ZbusError(zbus::Error),
}
pub async fn init() -> Result<(), Errors> {
    if let Err(error) = init_jvm() {
        return Err(Errors::JNIError(error));
    };

    if let Err(error) = init_zbus().await {
        return Err(Errors::ZbusError(error));
    } else {
        return Ok(());
    }
}
