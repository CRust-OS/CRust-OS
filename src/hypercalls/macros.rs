#[cfg(target_arch = "x86_64")]
macro_rules! hypercall {
    // Calling convention described here:
    // http://stackoverflow.com/questions/2535989/what-are-the-calling-conventions-for-unix-linux-system-calls-on-x86-64
    // This implementation based on the Mini-OS implementation of _hypercall0, _hypercall1, etc.
    ($ty : ty, $op : expr) => {
        unsafe { 
            let result : i64;
            asm!("call HYPERCALL_PAGE + ${1:c}"
                : "={rax}" (result)
                : "i" ($op as i64 * 32)
                : "memory" 
                : "volatile"
            );					
            result as $ty;
        }
    };
    ($ty : ty, $op : expr, $a1 : expr) => {
        unsafe { 
            let result : i64;
            let __ign1 : i64;
            asm!("call HYPERCALL_PAGE + ${2:c}"
                : "={rax}" (result), 
                  "={rdi}" (__ign1)
                : "i" ($op as i64 * 32),
                  "1" ($a1 as i64)
                : "memory" 
                : "volatile"
            );					
            result as $ty;
        }
    };
    ($ty : ty, $op : expr, $a1 : expr, $a2 : expr) => {
        unsafe { 
            let result : i64;
            let __ign1 : i64;
            let __ign2 : i64;
            asm!("call HYPERCALL_PAGE + ${3:c}"
                : "={rax}" (result),
                  "={rdi}" (__ign1), "={rsi}" (__ign2)
                : "i" ($op as i64 * 32),
                  "1" ($a1 as i64), "2" ($a2 as i64)
                : "memory" 
                : "volatile"
            );					
            result as $ty;
        }
    };
    ($ty : ty, $op : expr, $a1 : expr, $a2 : expr, $a3 : expr) => {
        unsafe { 
            let result : i64;
            let __ign1 : i64;
            let __ign2 : i64;
            let __ign3 : i64;
            asm!("call HYPERCALL_PAGE + ${4:c}"
                : "={rax}" (result),
                  "={rdi}" (__ign1), "={rsi}" (__ign2), "={rdx}" (__ign3)
                : "i" ($op as i64 * 32),
                  "1" ($a1 as i64), "2" ($a2 as i64), "3" ($a3 as i64)
                : "memory" 
                : "volatile"
            );
            result as $ty;
        }
    };
    ($ty : ty, $op : expr, $a1 : expr, $a2 : expr, $a3 : expr, $a4 : expr) => {
        unsafe { 
            let result : i64;
            let __ign1 : i64;
            let __ign2 : i64;
            let __ign3 : i64;
            let __ign4 : i64;
            asm!("call HYPERCALL_PAGE + ${5:c}"
                : "={rax}" (result),
                  "={rdi}" (__ign1), "={rsi}" (__ign2), "={rdx}" (__ign3), "={r10}" (__ign4)
                : "i" ($op as i64 * 32),
                  "1" ($a1 as i64), "2" ($a2 as i64), "3" ($a3 as i64), "4" ($a4 as i64)
                : "memory" 
                : "volatile"
            );
            result as $ty;
        }
    };
    ($ty : ty, $op : expr, $a1 : expr, $a2 : expr, $a3 : expr, $a4 : expr) => {
        unsafe { 
            let result : i64;
            let __ign1 : i64;
            let __ign2 : i64;
            let __ign3 : i64;
            let __ign4 : i64;
            let __ign5 : i64;
            asm!("call HYPERCALL_PAGE + ${6:c}"
                : "={rax}" (result),
                  "={rdi}" (__ign1), "={rsi}" (__ign2), "={rdx}" (__ign3), "={r10}" (__ign4), "={r8}" (__ign5)
                : "i" ($op as i64 * 32),
                  "1" ($a1 as i64), "2" ($a2 as i64), "3" ($a3 as i64), "4" ($a4 as i64), "5" ($a5 as i64)
                : "memory" 
                : "volatile"
            );
            result as $ty;
        }
    };
}
