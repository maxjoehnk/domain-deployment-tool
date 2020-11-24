use windows_service::{define_windows_service, service_dispatcher};
use std::ffi::OsString;
use crate::flags::Flags;

define_windows_service!(ffi_service_main, service_main);

fn service_main(args: Vec<OsString>) {
}

pub fn install_service(flags: Flags) -> anyhow::Result<()> {
    service_dispatcher::start("", ffi_service_main)?;

    Ok(())
}
