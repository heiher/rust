// This defines the loongarch64 target for UEFI systems as described in the UEFI specification. See the
// uefi-base module for generic UEFI options.

use crate::spec::{Arch, CodeModel, LinkerFlavor, Lld, Target, TargetMetadata, base};

pub(crate) fn target() -> Target {
    let mut base = base::uefi_msvc::opts();

    base.add_pre_link_args(LinkerFlavor::Msvc(Lld::No), &["/machine:loongarch64"]);
    base.code_model = Some(CodeModel::Small);
    base.cpu = "generic".into();
    base.features = "-f,-d".into();
    base.llvm_abiname = "lp64s".into();
    base.max_atomic_width = Some(64);
    base.direct_access_external_data = Some(true);

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
        arch: Arch::LoongArch64,
        options: base,
    }
}
