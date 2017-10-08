#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rfc822_sanitizer;

fuzz_target!(|data: &[u8]| {
    if let Ok(utf8) = String::from_utf8(data.to_vec()) {
        let _ = rfc822_sanitizer::sanitize_rfc822_like_date(utf8);
    }
});
