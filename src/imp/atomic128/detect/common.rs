#[derive(Clone, Copy)]
pub(crate) struct CpuInfo(u32);

impl CpuInfo {
    const INIT: u32 = 0;

    #[inline]
    fn set(&mut self, bit: u32) {
        self.0 = set(self.0, bit);
    }
    #[inline]
    fn test(self, bit: u32) -> bool {
        test(self.0, bit)
    }
}

#[inline]
fn set(x: u32, bit: u32) -> u32 {
    x | 1 << bit
}
#[inline]
fn test(x: u32, bit: u32) -> bool {
    x & (1 << bit) != 0
}

#[inline]
pub(crate) fn detect() -> CpuInfo {
    use core::sync::atomic::{AtomicU32, Ordering};

    static CACHE: AtomicU32 = AtomicU32::new(0);
    let mut info = CpuInfo(CACHE.load(Ordering::Relaxed));
    if info.0 != 0 {
        return info;
    }
    info.set(CpuInfo::INIT);
    // Note: This cfg is intended to make it easy for portable-atomic developers
    // to test has_cmpxchg16b == false or has_lse == false cases,
    // and is not a public API.
    if !cfg!(portable_atomic_test_outline_atomics_detect_false) {
        _detect(&mut info);
    }
    CACHE.store(info.0, Ordering::Relaxed);
    info
}

#[cfg(target_arch = "aarch64")]
impl CpuInfo {
    /// Whether FEAT_LSE is available
    const HAS_LSE: u32 = 1;
    /// Whether FEAT_LSE2 is available
    // This is currently only used in tests.
    #[cfg(test)]
    const HAS_LSE2: u32 = 2;
    /// Whether FEAT_LSE128 is available
    // This is currently only used in tests.
    #[cfg(test)]
    const HAS_LSE128: u32 = 3;
    /// Whether FEAT_LRCPC3 is available
    // This is currently only used in tests.
    #[cfg(test)]
    const HAS_RCPC3: u32 = 4;

    #[allow(clippy::unused_self)]
    #[inline]
    pub(crate) fn has_lse(self) -> bool {
        #[cfg(any(target_feature = "lse", portable_atomic_target_feature = "lse"))]
        {
            // FEAT_LSE is available at compile-time.
            true
        }
        #[cfg(not(any(target_feature = "lse", portable_atomic_target_feature = "lse")))]
        {
            self.test(CpuInfo::HAS_LSE)
        }
    }
}

#[cfg(target_arch = "x86_64")]
impl CpuInfo {
    /// Whether CMPXCHG16B is available
    const HAS_CMPXCHG16B: u32 = 1;
    /// Whether VMOVDQA is atomic
    const HAS_VMOVDQA_ATOMIC: u32 = 2;

    #[allow(clippy::unused_self)]
    #[inline]
    pub(crate) fn has_cmpxchg16b(self) -> bool {
        #[cfg(any(target_feature = "cmpxchg16b", portable_atomic_target_feature = "cmpxchg16b"))]
        {
            // CMPXCHG16B is available at compile-time.
            true
        }
        #[cfg(not(any(
            target_feature = "cmpxchg16b",
            portable_atomic_target_feature = "cmpxchg16b",
        )))]
        {
            self.test(CpuInfo::HAS_CMPXCHG16B)
        }
    }
    #[inline]
    pub(crate) fn has_vmovdqa_atomic(self) -> bool {
        self.test(CpuInfo::HAS_VMOVDQA_ATOMIC)
    }
}

// core::ffi::c_* (except c_void) requires Rust 1.64, libc will soon require Rust 1.47
#[cfg(target_arch = "aarch64")]
#[cfg(not(windows))]
#[allow(dead_code, non_camel_case_types)]
mod c_types {
    pub(crate) type c_void = core::ffi::c_void;
    // c_{,u}int is {i,u}32 on non-16-bit architectures
    // https://github.com/rust-lang/rust/blob/1.69.0/library/core/src/ffi/mod.rs#L160
    pub(crate) type c_int = i32;
    pub(crate) type c_uint = u32;
    // c_{,u}long is {i,u}64 on non-Windows 64-bit targets, otherwise is {i,u}32
    // https://github.com/rust-lang/rust/blob/1.69.0/library/core/src/ffi/mod.rs#L176
    // (Windows currently doesn't use this module - this module is cfg(not(windows)))
    #[cfg(target_pointer_width = "64")]
    pub(crate) type c_long = i64;
    #[cfg(not(target_pointer_width = "64"))]
    pub(crate) type c_long = i32;
    #[cfg(target_pointer_width = "64")]
    pub(crate) type c_ulong = u64;
    #[cfg(not(target_pointer_width = "64"))]
    pub(crate) type c_ulong = u32;
    // c_size_t is usize
    // https://github.com/rust-lang/rust/blob/1.69.0/library/core/src/ffi/mod.rs#L88
    pub(crate) type c_size_t = usize;
    // c_char is u8 on most non-Apple/non-Windows ARM/PowerPC/RISC-V targets
    // (Linux/Android/FreeBSD/NetBSD/OpenBSD/VxWorks/Fuchsia/QNX Neutrino/Horizon)
    // https://github.com/rust-lang/rust/blob/1.69.0/library/core/src/ffi/mod.rs#L104
    // (macOS is currently the only Apple target that uses this module, and Windows currently doesn't use this module)
    #[cfg(not(target_os = "macos"))]
    pub(crate) type c_char = u8;
    // c_char is i8 on all Apple targets
    #[cfg(target_os = "macos")]
    pub(crate) type c_char = i8;

    // Static assertions for C type definitions.
    #[cfg(test)]
    const _: fn() = || {
        use test_helper::{libc, sys};
        let _: c_int = 0 as std::os::raw::c_int;
        let _: c_uint = 0 as std::os::raw::c_uint;
        let _: c_long = 0 as std::os::raw::c_long;
        let _: c_ulong = 0 as std::os::raw::c_ulong;
        let _: c_size_t = 0 as libc::size_t; // std::os::raw::c_size_t is unstable
        let _: c_char = 0 as std::os::raw::c_char;
        let _: c_char = 0 as sys::c_char;
    };
}

#[allow(
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::undocumented_unsafe_blocks,
    clippy::wildcard_imports
)]
#[cfg(test)]
mod tests_common {
    use super::*;

    #[test]
    fn test_bit_flags() {
        let mut x = CpuInfo(0);
        #[cfg(target_arch = "aarch64")]
        {
            assert!(!x.test(CpuInfo::INIT));
            assert!(!x.test(CpuInfo::HAS_LSE));
            assert!(!x.test(CpuInfo::HAS_LSE2));
            assert!(!x.test(CpuInfo::HAS_LSE128));
            assert!(!x.test(CpuInfo::HAS_RCPC3));
            x.set(CpuInfo::INIT);
            assert!(x.test(CpuInfo::INIT));
            assert!(!x.test(CpuInfo::HAS_LSE));
            assert!(!x.test(CpuInfo::HAS_LSE2));
            assert!(!x.test(CpuInfo::HAS_LSE128));
            assert!(!x.test(CpuInfo::HAS_RCPC3));
            x.set(CpuInfo::HAS_LSE);
            assert!(x.test(CpuInfo::INIT));
            assert!(x.test(CpuInfo::HAS_LSE));
            assert!(!x.test(CpuInfo::HAS_LSE2));
            assert!(!x.test(CpuInfo::HAS_LSE128));
            assert!(!x.test(CpuInfo::HAS_RCPC3));
            x.set(CpuInfo::HAS_LSE2);
            assert!(x.test(CpuInfo::INIT));
            assert!(x.test(CpuInfo::HAS_LSE));
            assert!(x.test(CpuInfo::HAS_LSE2));
            assert!(!x.test(CpuInfo::HAS_LSE128));
            assert!(!x.test(CpuInfo::HAS_RCPC3));
            x.set(CpuInfo::HAS_LSE128);
            assert!(x.test(CpuInfo::INIT));
            assert!(x.test(CpuInfo::HAS_LSE));
            assert!(x.test(CpuInfo::HAS_LSE2));
            assert!(x.test(CpuInfo::HAS_LSE128));
            assert!(!x.test(CpuInfo::HAS_RCPC3));
            x.set(CpuInfo::HAS_RCPC3);
            assert!(x.test(CpuInfo::INIT));
            assert!(x.test(CpuInfo::HAS_LSE));
            assert!(x.test(CpuInfo::HAS_LSE2));
            assert!(x.test(CpuInfo::HAS_LSE128));
            assert!(x.test(CpuInfo::HAS_RCPC3));
        }
        #[cfg(target_arch = "x86_64")]
        {
            assert!(!x.test(CpuInfo::INIT));
            assert!(!x.test(CpuInfo::HAS_CMPXCHG16B));
            assert!(!x.test(CpuInfo::HAS_VMOVDQA_ATOMIC));
            x.set(CpuInfo::INIT);
            assert!(x.test(CpuInfo::INIT));
            assert!(!x.test(CpuInfo::HAS_CMPXCHG16B));
            assert!(!x.test(CpuInfo::HAS_VMOVDQA_ATOMIC));
            x.set(CpuInfo::HAS_CMPXCHG16B);
            assert!(x.test(CpuInfo::INIT));
            assert!(x.test(CpuInfo::HAS_CMPXCHG16B));
            assert!(!x.test(CpuInfo::HAS_VMOVDQA_ATOMIC));
            x.set(CpuInfo::HAS_VMOVDQA_ATOMIC);
            assert!(x.test(CpuInfo::INIT));
            assert!(x.test(CpuInfo::HAS_CMPXCHG16B));
            assert!(x.test(CpuInfo::HAS_VMOVDQA_ATOMIC));
        }
    }

    #[test]
    fn print_features() {
        use std::{fmt::Write as _, io::Write, string::String};

        let mut features = String::new();
        #[cfg(target_arch = "aarch64")]
        {
            let _ = writeln!(features, "lse: {}", detect().test(CpuInfo::HAS_LSE));
            let _ = writeln!(features, "lse2: {}", detect().test(CpuInfo::HAS_LSE2));
            let _ = writeln!(features, "lse128: {}", detect().test(CpuInfo::HAS_LSE128));
            let _ = writeln!(features, "rcpc3: {}", detect().test(CpuInfo::HAS_RCPC3));
        }
        #[cfg(target_arch = "x86_64")]
        {
            let _ = writeln!(features, "cmpxchg16b: {}", detect().test(CpuInfo::HAS_CMPXCHG16B));
            let _ = writeln!(
                features,
                "vmovdqa-atomic: {}",
                detect().test(CpuInfo::HAS_VMOVDQA_ATOMIC)
            );
        }
        let stdout = std::io::stderr();
        let mut stdout = stdout.lock();
        let _ = stdout.write_all(features.as_bytes());
    }

    #[cfg(not(portable_atomic_test_outline_atomics_detect_false))]
    #[cfg(target_arch = "aarch64")]
    #[test]
    fn test_detect() {
        let proc_cpuinfo = test_helper::cpuinfo::ProcCpuinfo::new();
        if detect().has_lse() {
            assert!(detect().test(CpuInfo::HAS_LSE));
            if let Ok(proc_cpuinfo) = proc_cpuinfo {
                assert!(proc_cpuinfo.lse);
            }
        } else {
            assert!(!detect().test(CpuInfo::HAS_LSE));
            if let Ok(proc_cpuinfo) = proc_cpuinfo {
                assert!(!proc_cpuinfo.lse);
            }
        }
        if detect().test(CpuInfo::HAS_LSE2) {
            assert!(detect().test(CpuInfo::HAS_LSE));
            assert!(detect().test(CpuInfo::HAS_LSE2));
            if let Ok(test_helper::cpuinfo::ProcCpuinfo { lse2: Some(lse2), .. }) = proc_cpuinfo {
                assert!(lse2);
            }
        } else {
            assert!(!detect().test(CpuInfo::HAS_LSE2));
            if let Ok(test_helper::cpuinfo::ProcCpuinfo { lse2: Some(lse2), .. }) = proc_cpuinfo {
                assert!(!lse2);
            }
        }
        if detect().test(CpuInfo::HAS_LSE128) {
            assert!(detect().test(CpuInfo::HAS_LSE));
            assert!(detect().test(CpuInfo::HAS_LSE2));
            assert!(detect().test(CpuInfo::HAS_LSE128));
        } else {
            assert!(!detect().test(CpuInfo::HAS_LSE128));
        }
        if detect().test(CpuInfo::HAS_RCPC3) {
            assert!(detect().test(CpuInfo::HAS_RCPC3));
        } else {
            assert!(!detect().test(CpuInfo::HAS_RCPC3));
        }
    }
}
