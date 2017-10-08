extern crate rfc822_sanitizer;
use rfc822_sanitizer::parse_from_rfc2822_with_fallback;

fn main() {
    let dates = vec![
        "Thu, 05 Aug 2016 06:00:00 -0400",
        "Mon, 31 July 2017 16:00:00 PDT",
        "Wed, 20 Sep 2017 10:00:00 -0000",
        "30 Aug 2017 1:30:00 PDT",
    ];

    for date in &dates {
        let fallback = parse_from_rfc2822_with_fallback(date);
        println!("{:?}", fallback);
    }
}
