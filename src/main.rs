// Allow Apple's naming convention for constants
#![allow(non_upper_case_globals)]

extern crate IOKit_sys as io;

const kOpen: u32 = 0;
const kClose: u32 = 1;

const kGetMuxState: u32 = 3;

const muxGraphicsCard: u64 = 7;

const kDriverClassName: &str = "AppleGraphicsControl\0"; // Note: str slices are not zero-terminated

fn main() {
    let mut connection: io::io_connect_t = io::IO_OBJECT_NULL;
    let mut output = 0u64;

    unsafe {
        let mut iterator: io::io_iterator_t = io::IO_OBJECT_NULL;

        // Create a service name matching dictionary for the graphics control service. This creates
        // a new CFDictionary that is later consumed by IOServiceGetMatchingServices, thus not
        // requiring a release.
        let service = io::IOServiceMatching(kDriverClassName.as_ptr() as *const std::ffi::c_char);

        // Create a services iterator that finds the graphics control service.
        let result =
            io::IOServiceGetMatchingServices(io::kIOMasterPortDefault, service, &mut iterator);
        if result != mach::kern_return::KERN_SUCCESS {
            panic!("IOServiceGetMatchingServices returned non-zero result");
        }

        // Since there is only one graphics control service, the next item will always be the
        // service that should be used. The iterator is released afterwards.
        let service = io::IOIteratorNext(iterator);
        let result = io::IOObjectRelease(iterator);
        if result != mach::kern_return::KERN_SUCCESS {
            panic!("IOObjectRelease returned non-zero result");
        }

        if service == io::IO_OBJECT_NULL {
            panic!("No matching driver found");
        }

        // Open a connection to the service.
        let result = io::IOServiceOpen(
            service,
            mach::traps::mach_task_self() as mach::mach_types::task_port_t,
            0,
            &mut connection,
        );
        if result != mach::kern_return::KERN_SUCCESS {
            panic!("IOServiceOpen returned non-zero result");
        }

        // Call the open method on the connection to finalize the connection.
        let result = io::IOConnectCallScalarMethod(
            connection,
            kOpen,
            std::ptr::null(),
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if result != mach::kern_return::KERN_SUCCESS {
            panic!("IOConnectCallScalarMethod(kOpen) returned non-zero result");
        }

        // Invoke the GetMuxState method on the service with the muxGraphicsCard subcommand to get
        // the currently active GPU.
        let input = [1u64, muxGraphicsCard];
        let mut output_count = 1u32;

        let result = io::IOConnectCallScalarMethod(
            connection,
            kGetMuxState,
            input.as_ptr(),
            2,
            &mut output,
            &mut output_count,
        );
        if result != mach::kern_return::KERN_SUCCESS {
            panic!("IOServiceGetMatchingServices returned non-zero result");
        }
    }

    // Print the result message
    match output {
        0 => println!("Currently using the discrete GPU"),
        1 => println!("Currently using the integrated GPU"),
        _ => println!("Currently using an unknown GPU"),
    }

    unsafe {
        // Finalize the connection
        let result = io::IOConnectCallScalarMethod(
            connection,
            kClose,
            std::ptr::null(),
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if result != mach::kern_return::KERN_SUCCESS {
            panic!("IOConnectCallScalarMethod(kClose) returned non-zero result");
        }

        // Close the service
        let result = io::IOServiceClose(connection);
        if result != mach::kern_return::KERN_SUCCESS {
            panic!("IOServiceClose returned non-zero result");
        }
    }
}
