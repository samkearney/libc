pub type c_char = i8;
pub type wchar_t = u32;
pub type c_long = i64;
pub type c_ulong = u64;
pub type time_t = u32;

s! {
    pub struct x86_cpu_registers {
        pub edi: u32,
        pub esi: u32,
        pub ebp: u32,
        pub exx: u32,
        pub ebx: u32,
        pub edx: u32,
        pub ecx: u32,
        pub eax: u32,
        pub eip: u32,
        pub cs: u32,
        pub efl: u32,
        pub esp: u32,
        pub ss: u32,
    }

    pub struct mcontext_t {
        pub cpu: x86_cpu_registers,
        #[cfg(libc_union)]
        pub fpu: x86_fpu_registers,
        #[cfg(not(libc_union))]
        __reserved: [u8; 512],
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_size: ::size_t,
        pub ss_flags: ::c_int,
    }

    pub struct fsave_area {
        pub fpu_control_word: u32,
        pub fpu_status_word: u32,
        pub fpu_tag_word: u32,
        pub fpu_ip: u32,
        pub fpu_cs: u32,
        pub fpu_op: u32,
        pub fpu_ds: u32,
        pub st_regs: [u8; 80],
    }

    pub struct fxsave_area {
        pub fpu_control_word: u16,
        pub fpu_status_word: u16,
        pub fpu_tag_word: u16,
        pub fpu_operand: u16,
        pub fpu_ip: u32,
        pub fpu_cs: u32,
        pub fpu_op: u32,
        pub fpu_ds: u32,
        pub mxcsr: u32,
        pub mxcsr_mask: u32,
        pub st_regs: [u8; 128],
        pub xmm_regs: [u8; 128],
        reserved2: [u8; 224],
    }
}

s_no_extra_traits! {
    #[cfg(libc_union)]
    pub union x86_fpu_registers {
        pub fsave_area: fsave_area,
        pub fxsave_area: fxsave_area,
        pub data: [u8; 512],
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        #[cfg(libc_union)]
        impl Eq for x86_fpu_registers {}

        #[cfg(libc_union)]
        impl PartialEq for x86_fpu_registers {
            fn eq(&self, other: &x86_64_fpu_registers) -> bool {
                unsafe {
                    self.fsave_area == other.fsave_area
                        || self.fxsave_area == other.fxsave_area
                }
            }
        }

        #[cfg(libc_union)]
        impl ::fmt::Debug for x86_fpu_registers {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                unsafe {
                    f.debug_struct("x86_fpu_registers")
                        .field("fsave_area", &self.fsave_area)
                        .field("fxsave_area", &self.fxsave_area)
                        .finish()
                }
            }
        }

        #[cfg(libc_union)]
        impl ::hash::Hash for x86_fpu_registers {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                unsafe {
                    self.fsave_area.hash(state);
                    self.fxsave_area.hash(state);
                }
            }
        }
    }
}
