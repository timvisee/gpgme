#[macro_use]
extern crate gpgme;
#[macro_use]
extern crate lazy_static;
extern crate tempdir;

use gpgme::{Context, PinentryMode, Error};

#[macro_use]
mod support;

require_gpgme_ver! {
    (1, 4) => {
        test_case! {
            test_pinentry_mode(test) {
                let mode = PinentryMode::Loopback;
                let mut ctx = test.create_context();
                match ctx.set_pinentry_mode(mode) {
                    Ok(()) => {
                        assert_eq!(mode, Context::pinentry_mode(&ctx));
                    }
                    Err(Error::UNSUPPORTED_OPERATION) => (),
                    e @ Err(_) => fail_if_err!(e),
                }
            }
        }
    }
}
