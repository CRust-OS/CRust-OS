macro_rules! hypercall {
    // Calling convention described here:
    // http://stackoverflow.com/questions/2535989/what-are-the-calling-conventions-for-unix-linux-system-calls-on-x86-64
    // This implementation based on the Mini-OS implementation of _hypercall0, _hypercall1, etc.
    ($ty : ty, $op : expr) => {
        #[allow(unused_unsafe)] //https://github.com/rust-lang/rust/issues/8472
        unsafe { 
            let result : isize;
            asm!("call HYPERCALL_PAGE + ${1:c}"
                : "={rax}" (result)
                : "i" ($op as isize * 32)
                : "memory" 
                : "volatile"
            );					
            result as $ty;
        }
    };
    ($ty : ty, $op : expr, $a1 : expr) => {
        #[allow(unused_unsafe)] //https://github.com/rust-lang/rust/issues/8472
        unsafe { 
            let result : isize;
            let __ign1 : isize;
            asm!("call HYPERCALL_PAGE + ${2:c}"
                : "={rax}" (result), 
                  "={rdi}" (__ign1)
                : "i" ($op as isize * 32),
                  "1" ($a1)
                : "memory" 
                : "volatile"
            );					
            result as $ty;
        }
    };
    ($ty : ty, $op : expr, $a1 : expr, $a2 : expr) => {
        #[allow(unused_unsafe)] //https://github.com/rust-lang/rust/issues/8472
        unsafe { 
            let result : isize;
            let __ign1 : isize;
            let __ign2 : isize;
            asm!("call HYPERCALL_PAGE + ${3:c}"
                : "={rax}" (result),
                  "={rdi}" (__ign1), "={rsi}" (__ign2)
                : "i" ($op as isize * 32),
                  "1" ($a1), "2" ($a2)
                : "memory" 
                : "volatile"
            );					
            result as $ty;
        }
    };
    ($ty : ty, $op : expr, $a1 : expr, $a2 : expr, $a3 : expr) => {
        #[allow(unused_unsafe)] //https://github.com/rust-lang/rust/issues/8472
        unsafe { 
            let result : isize;
            let __ign1 : isize;
            let __ign2 : isize;
            let __ign3 : isize;
            asm!("call HYPERCALL_PAGE + ${4:c}"
                : "={rax}" (result),
                  "={rdi}" (__ign1), "={rsi}" (__ign2), "={rdx}" (__ign3)
                : "i" ($op as isize * 32),
                  "1" ($a1), "2" ($a2), "3" ($a3)
                : "memory" 
                : "volatile"
            );
            result as $ty;
        }
    };
    ($ty : ty, $op : expr, $a1 : expr, $a2 : expr, $a3 : expr, $a4 : expr) => {
        #[allow(unused_unsafe)] //https://github.com/rust-lang/rust/issues/8472
        unsafe { 
            let result : isize;
            let __ign1 : isize;
            let __ign2 : isize;
            let __ign3 : isize;
            let __ign4 : isize;
            asm!("call HYPERCALL_PAGE + ${5:c}"
                : "={rax}" (result),
                  "={rdi}" (__ign1), "={rsi}" (__ign2), "={rdx}" (__ign3), "={r10}" (__ign4)
                : "i" ($op as isize * 32),
                  "1" ($a1), "2" ($a2), "3" ($a3), "4" ($a4)
                : "memory" 
                : "volatile"
            );
            result as $ty;
        }
    };
    ($ty : ty, $op : expr, $a1 : expr, $a2 : expr, $a3 : expr, $a4 : expr) => {
        #[allow(unused_unsafe)] //https://github.com/rust-lang/rust/issues/8472
        unsafe { 
            let result : isize;
            let __ign1 : isize;
            let __ign2 : isize;
            let __ign3 : isize;
            let __ign4 : isize;
            let __ign5 : isize;
            asm!("call HYPERCALL_PAGE + ${6:c}"
                : "={rax}" (result),
                  "={rdi}" (__ign1), "={rsi}" (__ign2), "={rdx}" (__ign3), "={r10}" (__ign4), "={r8}" (__ign5)
                : "i" ($op as isize * 32),
                  "1" ($a1), "2" ($a2), "3" ($a3), "4" ($a4), "5" ($a5)
                : "memory" 
                : "volatile"
            );
            result as $ty;
        }
    };
}
