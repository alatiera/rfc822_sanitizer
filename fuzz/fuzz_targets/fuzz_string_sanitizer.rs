#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rfc822_sanitizer;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(utf8) = std::str::from_utf8(data) {
        let _ = rfc822_sanitizer::sanitize_rfc822_like_date(utf8);
    }
});
