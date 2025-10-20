// This defines the loongarch64 target for UEFI systems as described in the UEFI specification. See the
// uefi-base module for generic UEFI options.

use crate::spec::{LinkerFlavor, Lld, Target, TargetMetadata, base};

pub(crate) fn target() -> Target {
    let mut base = base::uefi_msvc::opts();

    base.max_atomic_width = Some(64);
    base.add_pre_link_args(LinkerFlavor::Msvc(Lld::No), &["/machine:loongarch64"]);
    base.features = "-f,-d".into();

    Target {
        llvm_target: "loongarch64-unknown-uefi".into(),
        metadata: TargetMetadata {
            description: Some("LoongArch64 UEFI".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: None,
        },
        pointer_width: 64,
        data_layout: "e-m:w-p:64:64-i64:64-i128:128-n32:64-S128".into(),
        arch: "loongarch64".into(),
        options: base,
    }
}
