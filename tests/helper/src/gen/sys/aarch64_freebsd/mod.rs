// This file is @generated by portable-atomic-internal-codegen
// (gen function at tools/codegen/src/ffi.rs).
// It is not intended for manual editing.

#![cfg_attr(rustfmt, rustfmt::skip)]
mod sys_auxv;
pub use sys_auxv::elf_aux_info;
mod sys_elf_common;
pub use sys_elf_common::{AT_HWCAP, AT_HWCAP2};
mod machine_elf;
pub use machine_elf::{
    HWCAP2_BF16, HWCAP2_BTI, HWCAP2_DCPODP, HWCAP2_DGH, HWCAP2_FLAGM2, HWCAP2_FRINT,
    HWCAP2_I8MM, HWCAP2_RNG, HWCAP2_SVE2, HWCAP2_SVEAES, HWCAP2_SVEBF16,
    HWCAP2_SVEBITPERM, HWCAP2_SVEF32MM, HWCAP2_SVEF64MM, HWCAP2_SVEI8MM, HWCAP2_SVEPMULL,
    HWCAP2_SVESHA3, HWCAP2_SVESM4, HWCAP_AES, HWCAP_ASIMD, HWCAP_ASIMDDP, HWCAP_ASIMDFHM,
    HWCAP_ASIMDHP, HWCAP_ASIMDRDM, HWCAP_ATOMICS, HWCAP_CPUID, HWCAP_CRC32, HWCAP_DCPOP,
    HWCAP_DIT, HWCAP_EVTSTRM, HWCAP_FCMA, HWCAP_FLAGM, HWCAP_FP, HWCAP_FPHP,
    HWCAP_ILRCPC, HWCAP_JSCVT, HWCAP_LRCPC, HWCAP_PACA, HWCAP_PACG, HWCAP_PMULL,
    HWCAP_SB, HWCAP_SHA1, HWCAP_SHA2, HWCAP_SHA3, HWCAP_SHA512, HWCAP_SM3, HWCAP_SM4,
    HWCAP_SSBS, HWCAP_SVE, HWCAP_USCAT,
};