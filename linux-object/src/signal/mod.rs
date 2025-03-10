//! Linux signals
#![allow(missing_docs)]
use bitflags::*;
use numeric_enum_macro::numeric_enum;

mod action;

pub use self::action::*;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        /// struct mcontext
        #[repr(C)]
        #[derive(Clone, Debug, Default)]
        pub struct MachineContext {
            // gregs
            pub r8: usize,
            pub r9: usize,
            pub r10: usize,
            pub r11: usize,
            pub r12: usize,
            pub r13: usize,
            pub r14: usize,
            pub r15: usize,
            pub rdi: usize,
            pub rsi: usize,
            pub rbp: usize,
            pub rbx: usize,
            pub rdx: usize,
            pub rax: usize,
            pub rcx: usize,
            pub rsp: usize,
            pub rip: usize,
            pub eflags: usize,
            pub cs: u16,
            pub gs: u16,
            pub fs: u16,
            pub _pad: u16,
            pub err: usize,
            pub trapno: usize,
            pub oldmask: usize,
            pub cr2: usize,
            // fpregs
            // TODO
            pub fpstate: usize,
            // reserved
            pub _reserved1: [usize; 8],
        }

        impl MachineContext {
            pub fn set_pc(&mut self, pc: usize) {
                self.rip = pc;
            }
            pub fn get_pc(&self) -> usize {
                self.rip
            }
        }
    } else if #[cfg(target_arch = "riscv64")] {
        /// struct mcontext
        #[repr(C)]
        #[derive(Clone, Debug)]
        pub struct MachineContext {
            // TODO
            pub reserved_: [usize; 16],
            // pc
            pub pc: usize,
            // TODO
            pub reserved: [usize; 17],
            // fpregs
            pub fpstate: [usize; 66],
        }

        impl Default for MachineContext {
            fn default() -> Self {
                Self {
                    reserved_: [0; 16],
                    pc: 0,
                    reserved: [0; 17],
                    fpstate: [0; 66]
                }
            }
        }

        impl MachineContext {
            pub fn set_pc(&mut self, pc: usize) {
                self.pc = pc;
            }
            pub fn get_pc(&self) -> usize {
                self.pc
            }
        }
    } else {
        /// TODO: other archs
        /// struct mcontext
        #[repr(C)]
        #[derive(Clone, Debug)]
        pub struct MachineContext {
            // TODO
            pub reserved_: [usize; 36],
        }

        impl Default for MachineContext {
            fn default() -> Self {
                Self {
                    reserved_: [0; 36],
                }
            }
        }

        impl MachineContext {
            pub fn set_pc(&mut self, _pc: usize) {}
            pub fn get_pc(&self) -> usize {
                0
            }
        }
    }
}

numeric_enum! {
    #[repr(u8)]
    #[derive(Eq, PartialEq, Debug, Copy, Clone)]
    pub enum Signal {
        SIGHUP = 1,
        SIGINT = 2,
        SIGQUIT = 3,
        SIGILL = 4,
        SIGTRAP = 5,
        SIGABRT = 6,
        SIGBUS = 7,
        SIGFPE = 8,
        SIGKILL = 9,
        SIGUSR1 = 10,
        SIGSEGV = 11,
        SIGUSR2 = 12,
        SIGPIPE = 13,
        SIGALRM = 14,
        SIGTERM = 15,
        SIGSTKFLT = 16,
        SIGCHLD = 17,
        SIGCONT = 18,
        SIGSTOP = 19,
        SIGTSTP = 20,
        SIGTTIN = 21,
        SIGTTOU = 22,
        SIGURG = 23,
        SIGXCPU = 24,
        SIGXFSZ = 25,
        SIGVTALRM = 26,
        SIGPROF = 27,
        SIGWINCH = 28,
        SIGIO = 29,
        SIGPWR = 30,
        SIGSYS = 31,
        // real time signals
        SIGRT32 = 32,
        SIGRT33 = 33,
        SIGRT34 = 34,
        SIGRT35 = 35,
        SIGRT36 = 36,
        SIGRT37 = 37,
        SIGRT38 = 38,
        SIGRT39 = 39,
        SIGRT40 = 40,
        SIGRT41 = 41,
        SIGRT42 = 42,
        SIGRT43 = 43,
        SIGRT44 = 44,
        SIGRT45 = 45,
        SIGRT46 = 46,
        SIGRT47 = 47,
        SIGRT48 = 48,
        SIGRT49 = 49,
        SIGRT50 = 50,
        SIGRT51 = 51,
        SIGRT52 = 52,
        SIGRT53 = 53,
        SIGRT54 = 54,
        SIGRT55 = 55,
        SIGRT56 = 56,
        SIGRT57 = 57,
        SIGRT58 = 58,
        SIGRT59 = 59,
        SIGRT60 = 60,
        SIGRT61 = 61,
        SIGRT62 = 62,
        SIGRT63 = 63,
        SIGRT64 = 64,
    }
}

impl Signal {
    pub const RTMIN: usize = 32;
    pub const RTMAX: usize = 64;

    pub fn is_standard(self) -> bool {
        (self as usize) < Self::RTMIN
    }
}

/// See musl struct __ucontext
///
/// Not exactly the same for now
#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct SignalUserContext {
    pub flags: usize,
    pub link: usize,
    pub stack: SignalStack,
    pub sig_mask: Sigset,
    pub context: MachineContext,
}

#[repr(C)]
#[derive(Clone)]
pub struct SignalFrame {
    /// point to ret_code
    pub ret_code_addr: usize,
    /// Signal Frame info
    pub info: SigInfo,
    /// adapt interface, a little bit waste
    pub ucontext: SignalUserContext,
    /// call sys_sigreturn
    pub ret_code: [u8; 7],
}

bitflags! {
    pub struct SignalStackFlags : u32 {
        const ONSTACK = 1;
        const DISABLE = 2;
        const AUTODISARM = 0x80000000;
    }
}

/// Linux struct stack_t
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SignalStack {
    pub sp: usize,
    pub flags: SignalStackFlags,
    pub size: usize,
}

impl Default for SignalStack {
    fn default() -> Self {
        // default to disabled
        SignalStack {
            sp: 0,
            flags: SignalStackFlags::DISABLE,
            size: 0,
        }
    }
}
