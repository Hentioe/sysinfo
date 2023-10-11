// Take a look at the license at the top of the repository in the LICENSE file.

mod component;
mod cpu;
mod disk;
mod network;
pub(crate) mod network_helper;
mod process;
mod sid;
mod system;
mod tools;
mod users;
mod utils;

pub use self::component::{Component, Components};
pub use self::cpu::Cpu;
pub(crate) use self::disk::{DiskInner, DisksInner};
pub use self::network::NetworkData;
pub(crate) use self::process::ProcessInner;
pub use self::sid::Sid;
pub(crate) use self::system::SystemInner;
pub(crate) use self::users::get_users;
pub use self::users::User;

use std::time::Duration;

declare_signals! {
    (),
    Signal::Kill => (),
    _ => None,
}

#[doc = include_str!("../../md_doc/is_supported.md")]
pub const IS_SUPPORTED: bool = true;
#[doc = include_str!("../../md_doc/supported_signals.md")]
pub const SUPPORTED_SIGNALS: &[crate::Signal] = supported_signals();
#[doc = include_str!("../../md_doc/minimum_cpu_update_interval.md")]
pub const MINIMUM_CPU_UPDATE_INTERVAL: Duration = Duration::from_millis(200);
