// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2019-2020 Andre Richter <andre.o.richter@gmail.com>

//! x86.

use crate::QEMUExit;

const EXIT_FAILURE: u32 = 0; // since ((0 << 1) | 1) = 1.

/// x86 configuration.
pub struct X86 {
    /// Port number of the isa-debug-exit device.
    io_port: u16,
    /// Since QEMU's isa-debug-exit cannot exit(0), choose a value that represents success for you.
    ///
    /// Note: Only odd values will work.
    custom_exit_success: u32,
}

impl X86 {
    /// Create an instance.
    pub const fn new(io_port: u16, custom_exit_success: u32) -> Self {
        assert!((custom_exit_success & 1) == 1);

        X86 {
            io_port,
            custom_exit_success,
        }
    }
}

impl QEMUExit for X86 {
    fn exit(&self, code: u32) -> ! {
        use x86_64::instructions::port::Port;

        let mut port = Port::<u32>::new(self.io_port);
        unsafe { port.write(code.into()) }; // QEMU will execute `exit(((code << 1) | 1))`.

        // For the case that the QEMU exit attempt did not work, transition into an infinite loop.
        // Calling `panic!()` here is unfeasible, since there is a good chance this function here is
        // the last expression in the `panic!()` handler itself. This prevents a possible infinite
        // loop.
        loop {}
    }

    fn exit_success(&self) -> ! {
        self.exit(self.custom_exit_success >> 1) // Shift because QEMU does ((code << 1) | 1).
    }

    fn exit_failure(&self) -> ! {
        self.exit(EXIT_FAILURE)
    }
}
