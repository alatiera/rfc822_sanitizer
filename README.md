# RFC822/2822_sanitizer
[![pipeline status](https://gitlab.com/alatiera/rfc822_sanitizer/badges/master/pipeline.svg)](https://gitlab.com/alatiera/rfc822_sanitizer/commits/master)
[![Build Status](https://travis-ci.org/alatiera/rfc822_sanitizer.svg?branch=master)](https://travis-ci.org/alatiera/rfc822_sanitizer)
[![Build status](https://ci.appveyor.com/api/projects/status/1xywfcpygqxlr2ng?svg=true)](https://ci.appveyor.com/project/alatiera/rfc822-sanitizer)
[![Crates.io](https://img.shields.io/crates/v/rfc822_sanitizer.svg)](https://crates.io/crates/rfc822_sanitizer)
[![codecov](https://codecov.io/gh/alatiera/rfc822_sanitizer/branch/master/graph/badge.svg)](https://codecov.io/gh/alatiera/rfc822_sanitizer)
[![Dependency Status](https://dependencyci.com/github/alatiera/rfc822_sanitizer/badge)](https://dependencyci.com/github/alatiera/rfc822_sanitizer)

[Documentation](https://docs.rs/rfc822_sanitizer/)

A helper funtion that does what you wanted not what you said!

The world is full of broken code and everyone seem to reinvent daytimes.

If it encounters an invalid rfc822/2822 daytime it tries to fix it first before returning Errors. 

This crates soul purpose is to try to fix the mess of the generators that do stuff like this:

`Thu, 05 Aug 2016 06:00:00 -0400`

`Mon, 31 July 2017 16:00:00 PDT`

`Wed, 20 Sep 2017 10:00:00 -0000`

`30 Aug 2017 1:30:00 PDT`

All of the above look fine at first glance. But all are invalid and would fail to be parsed with [`chrono::Datetime::parse_from_rfc2822`](https://docs.rs/chrono/0.4.0/chrono/struct.DateTime.html#method.parse_from_rfc2822)

Playground [link](https://play.rust-lang.org/?gist=52016537b3af751812d172d0c29ea399&version=stable).

```rust
Err(ParseError(Impossible))
Err(ParseError(Invalid))
Err(ParseError(NotEnough))
Err(ParseError(Invalid))
```

* The first one `Thu, 05 Aug 2016 06:00:00 -0400` is actually a Friday.

* The second `Mon, 31 July 2017 16:00:00 PDT` uses full lenght month Name.

* The third `Wed, 20 Sep 2017 10:00:00 -0000` has -0000 as the timezone, which is sort of undefined behaviour. For more see [#102](https://github.com/chronotope/chrono/issues/102).

* The forth `30 Aug 2017 1:30:00 PDT` uses single digit notation for Hour.

The dates above have been encountered while trying to parse rss feeds from the wild west eer the Internet.

The [RSS spec](http://www.rssboard.org/rss-specification#optionalChannelElements) specifies the use of RFC822 for the date format, which is forward compatible with RFC2822. This crate proves that people/generators still get wrong a format witch was published in 1982.

Now if we were to use the sanitizer, we would actually get a correct datetime.

## Usage:

Add the following to your Cargo.toml.
```toml
[dependencies]
rfc822_sanitizer = "0.3"
```

### from [`examples/simple.rs`](examples/simple.rs)

```rust
extern crate rfc822_sanitizer;
use rfc822_sanitizer::parse_from_rfc2822_with_fallback;

fn main() {
    let dates = vec![
        "Thu, 05 Aug 2016 06:00:00 -0400",
        "Mon, 31 July 2017 16:00:00 PDT",
        "Wed, 20 Sep 2017 10:00:00 -0000",
        "30 Aug 2017 1:30:00 PDT",
    ];

    for foo in dates.iter() {
        let fallback = parse_from_rfc2822_with_fallback(&foo);
        println!("{:?}", fallback);
    }
}
```

Output:
```rust
Ok(2016-08-05T06:00:00-04:00)
Ok(2017-07-31T16:00:00-07:00)
Ok(2017-09-20T10:00:00+00:00)
Ok(2017-08-30T01:30:00-07:00)
```

Though keep in mind that it would consume more resources.

```shell
$ cargo bench -q
running 4 tests
test bench_correct_dates_normal_parse        ... bench:      20,741 ns/iter (+/- 1,000)
test bench_correct_dates_with_fallback       ... bench:      21,133 ns/iter (+/- 282)
test bench_parse_invalid_dates_with_fallback ... bench:     119,116 ns/iter (+/- 3,706)
test mixed_sample_case                       ... bench:     431,814 ns/iter (+/- 8,526)
```
