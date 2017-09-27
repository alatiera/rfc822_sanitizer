extern crate chrono;
extern crate regex;

use chrono::{DateTime, FixedOffset, ParseResult};
use regex::Regex;

use std::collections::HashMap;

pub fn from_rfc822_to_rfc2822(s: &str) -> ParseResult<DateTime<FixedOffset>> {
    let weekdays = vec![
        "Mon,",
        "Tue,",
        "Wed,",
        "Thu,",
        "Fri,",
        "Sat,",
        "Sun,",
        "Monday,",
        "Tuesday,",
        "Wednesday,",
        "Thursday,",
        "Friday,",
        "Saturday,",
        "Sunday,",
    ];

    let mut months = HashMap::new();
    months.insert("January", "Jan");
    months.insert("February", "Feb");
    months.insert("March", "Mar");
    months.insert("April ", "Apr");
    months.insert("May", "May");
    months.insert("June", "Jun");
    months.insert("July", "Jul");
    months.insert("August", "Aug");
    months.insert("September", "Sep");
    months.insert("October", "Oct");
    months.insert("November", "Nov");
    months.insert("December", "Dec");

    let mut foo = String::from(s);

    // Pad HH:MM:SS with exta zero if needed.
    let re = Regex::new(r"(\d{1,2}):(\d{1,2}):(\d{1,2})").unwrap();
    // hours, minutes, seconds = cap[1], cap[2], cap[3]
    let cap = re.captures(&s).unwrap();
    let mut newtime = Vec::new();

    cap.iter().skip(1).for_each(|x| if let Some(y) = x {
        // if y.end() - y.start() == 1 {
        if y.as_str().len() == 1 {
            newtime.push(format!("0{}", y.as_str()));
        } else {
            newtime.push(y.as_str().to_string());
        }
    });

    let ntime = &newtime.join(":");
    foo = foo.replace(cap.get(0).unwrap().as_str(), ntime);

    // Weekday name is not required for rfc2822
    // for stable, replace for_each with map and add
    // .fold((), |(),_|()) or .collect()
    weekdays.iter().for_each(|x| if foo.starts_with(x) {
        // TODO: handle to lower etc.
        // For sure someone has a weird feed with the day in lowercase
        foo = format!("{}", &foo[x.len()..]);
        foo = foo.trim().to_string();
    });

    // Replace long month names with 3 letter Abr.
    months.iter().for_each(|(k, v)| if s.contains(k) {
        foo = foo.replace(k, v);
    });

    // See #102, https://github.com/chronotope/chrono/issues/102
    // Handle -0000 as +0000
    if s.ends_with("-0000") {
        foo = format!("{}+0000", &foo[..foo.len() - 5]);
    }

    // println!("{}", foo);
    DateTime::parse_from_rfc2822(&foo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn foo() {
        // left is raw date extracted from rss feeds.
        // right is corresponding valid rfc2822
        let dates = vec![
            ("Thu, 6 July 2017 15:30:00 PDT", "6 Jul 2017 15:30:00 PDT"),
            ("Mon, 10 July 2017 16:00:00 PDT", "10 Jul 2017 16:00:00 PDT"),
            ("Mon, 17 July 2017 17:00:00 PDT", "17 Jul 2017 17:00:00 PDT"),
            ("Mon, 24 July 2017 16:00:00 PDT", "24 Jul 2017 16:00:00 PDT"),
            ("Mon, 31 July 2017 16:00:00 PDT", "31 Jul 2017 16:00:00 PDT"),
            ("Thu, 30 Aug 2017 1:30:00 PDT", "30 Aug 2017 01:30:00 PDT"),
            (
                "Wed, 20 Sep 2017 10:00:00 -0000",
                "20 Sep 2017 10:00:00 +0000",
            ),
            (
                "Wed, 13 Sep 2017 10:00:00 -0000",
                "13 Sep 2017 10:00:00 +0000",
            ),
            (
                "Wed, 09 Aug 2017 10:00:00 -0000",
                "09 Aug 2017 10:00:00 +0000",
            ),
            (
                "Wed, 02 Aug 2017 10:00:00 -0000",
                "02 Aug 2017 10:00:00 +0000",
            ),
            (
                "Wed, 26 Jul 2017 10:00:00 -0000",
                "26 Jul 2017 10:00:00 +0000",
            ),
            (
                "Wed, 19 Jul 2017 10:00:00 -0000",
                "19 Jul 2017 10:00:00 +0000",
            ),
            (
                "Wed, 12 Jul 2017 10:00:00 -0000",
                "12 Jul 2017 10:00:00 +0000",
            ),
            (
                "Wed, 28 Jun 2017 10:00:00 -0000",
                "28 Jun 2017 10:00:00 +0000",
            ),
            (
                "Wed, 21 Jun 2017 10:00:00 -0000",
                "21 Jun 2017 10:00:00 +0000",
            ),
            (
                "Wed, 14 Jun 2017 10:00:00 -0000",
                "14 Jun 2017 10:00:00 +0000",
            ),
            (
                "Wed, 07 Jun 2017 10:00:00 -0000",
                "07 Jun 2017 10:00:00 +0000",
            ),
            (
                "Wed, 31 May 2017 10:00:00 -0000",
                "31 May 2017 10:00:00 +0000",
            ),
            (
                "Wed, 24 May 2017 10:00:00 -0000",
                "24 May 2017 10:00:00 +0000",
            ),
            (
                "Wed, 17 May 2017 10:00:00 -0000",
                "17 May 2017 10:00:00 +0000",
            ),
            (
                "Wed, 10 May 2017 10:00:00 -0000",
                "10 May 2017 10:00:00 +0000",
            ),
            (
                "Wed, 03 May 2017 10:00:00 -0000",
                "03 May 2017 10:00:00 +0000",
            ),
            (
                "Wed, 19 Apr 2017 10:00:00 -0000",
                "19 Apr 2017 10:00:00 +0000",
            ),
            (
                "Wed, 12 Apr 2017 10:00:00 -0000",
                "12 Apr 2017 10:00:00 +0000",
            ),
            (
                "Wed, 05 Apr 2017 10:00:00 -0000",
                "05 Apr 2017 10:00:00 +0000",
            ),
            (
                "Wed, 29 Mar 2017 10:00:00 -0000",
                "29 Mar 2017 10:00:00 +0000",
            ),
            (
                "Wed, 22 Mar 2017 10:00:00 -0000",
                "22 Mar 2017 10:00:00 +0000",
            ),
            (
                "Wed, 15 Mar 2017 10:00:00 -0000",
                "15 Mar 2017 10:00:00 +0000",
            ),
            (
                "Wed, 08 Mar 2017 11:00:00 -0000",
                "08 Mar 2017 11:00:00 +0000",
            ),
            (
                "Wed, 01 Mar 2017 11:00:00 -0000",
                "01 Mar 2017 11:00:00 +0000",
            ),
            (
                "Wed, 22 Feb 2017 11:00:00 -0000",
                "22 Feb 2017 11:00:00 +0000",
            ),
            (
                "Wed, 15 Feb 2017 11:00:00 -0000",
                "15 Feb 2017 11:00:00 +0000",
            ),
            (
                "Wed, 08 Feb 2017 11:00:00 -0000",
                "08 Feb 2017 11:00:00 +0000",
            ),
            (
                "Wed, 01 Feb 2017 11:00:00 -0000",
                "01 Feb 2017 11:00:00 +0000",
            ),
            (
                "Wed, 25 Jan 2017 11:00:00 -0000",
                "25 Jan 2017 11:00:00 +0000",
            ),
            (
                "Fri, 13 Jan 2017 18:38:00 -0000",
                "13 Jan 2017 18:38:00 +0000",
            ),
            (
                "Wed, 20 Sep 2017 03:30:00 -0000",
                "20 Sep 2017 03:30:00 +0000",
            ),
            (
                "Wed, 13 Sep 2017 03:15:00 -0000",
                "13 Sep 2017 03:15:00 +0000",
            ),
            (
                "Wed, 06 Sep 2017 03:15:00 -0000",
                "06 Sep 2017 03:15:00 +0000",
            ),
            (
                "Wed, 30 Aug 2017 03:15:00 -0000",
                "30 Aug 2017 03:15:00 +0000",
            ),
            (
                "Wed, 23 Aug 2017 03:15:00 -0000",
                "23 Aug 2017 03:15:00 +0000",
            ),
            (
                "Wed, 16 Aug 2017 03:15:00 -0000",
                "16 Aug 2017 03:15:00 +0000",
            ),
            (
                "Wed, 09 Aug 2017 03:15:00 -0000",
                "09 Aug 2017 03:15:00 +0000",
            ),
            (
                "Wed, 02 Aug 2017 03:00:00 -0000",
                "02 Aug 2017 03:00:00 +0000",
            ),
            (
                "Tue, 11 Jul 2017 17:14:45 -0000",
                "11 Jul 2017 17:14:45 +0000",
            ),
            (
                "Thu, 03 August 2017 06:00:00 -0400",
                "03 Aug 2017 06:00:00 -0400",
            ),
            (
                "Thu, 27 July 2017 06:00:00 -0400",
                "27 Jul 2017 06:00:00 -0400",
            ),
            (
                "Thu, 20 July 2017 06:00:00 -0400",
                "20 Jul 2017 06:00:00 -0400",
            ),
            (
                "Thu, 13 July 2017 06:00:00 -0400",
                "13 Jul 2017 06:00:00 -0400",
            ),
            (
                "Thu, 06 July 2017 06:00:00 -0400",
                "06 Jul 2017 06:00:00 -0400",
            ),
            (
                "Thu, 28 June 2017 06:00:00 -0400",
                "28 Jun 2017 06:00:00 -0400",
            ),
            (
                "Thu, 17 Jul 2013 06:00:03 -0400",
                "17 Jul 2013 06:00:03 -0400",
            ),
            (
                "Thu, 02 Apr 2014 06:00:03 -0400",
                "02 Apr 2014 06:00:03 -0400",
            ),
            (
                "Wed, 14 Jan 2016 06:00:03 -0400",
                "14 Jan 2016 06:00:03 -0400",
            ),
            (
                "Thu, 22 June 2017 06:00:00 -0400",
                "22 Jun 2017 06:00:00 -0400",
            ),
            (
                "Thu, 15 June 2017 06:00:00 -0400",
                "15 Jun 2017 06:00:00 -0400",
            ),
            (
                "Thu, 7 June 2017 06:00:00 -0400",
                "7 Jun 2017 06:00:00 -0400",
            ),
            (
                "Thu, 1 June 2017 06:00:00 -0400",
                "1 Jun 2017 06:00:00 -0400",
            ),
            (
                "Thu, 23 Dec 2015 06:00:03 -0400",
                "23 Dec 2015 06:00:03 -0400",
            ),
            (
                "Thu, 14 Feb 2014 06:00:03 -0400",
                "14 Feb 2014 06:00:03 -0400",
            ),
            (
                "Thu, 04 Dec 2013 06:00:03 -0400",
                "04 Dec 2013 06:00:03 -0400",
            ),
            (
                "Thu, 20 Dec 2016 06:00:00 -0400",
                "20 Dec 2016 06:00:00 -0400",
            ),
            (
                "Thu, 23 Nov 2016 06:00:00 -0400",
                "23 Nov 2016 06:00:00 -0400",
            ),
            (
                "Thu, 05 Aug 2016 06:00:00 -0400",
                "05 Aug 2016 06:00:00 -0400",
            ),
            (
                "Fri, 09 Jun 2016 12:00:00 -0400",
                "09 Jun 2016 12:00:00 -0400",
            ),
            (
                "Thu, 10 May 2017 06:00:00 -0400",
                "10 May 2017 06:00:00 -0400",
            ),
            (
                "Thu, 22 Feb 2017 06:00:00 -0400",
                "22 Feb 2017 06:00:00 -0400",
            ),
            (
                "Thu, 15 Feb 2017 06:00:00 -0400",
                "15 Feb 2017 06:00:00 -0400",
            ),
        ];
       
        // used printing out stuff
        // dates
        //     .iter()
        //     .map(|x| from_rfc822_to_rfc2822(x))
        //     .fold((), |(), _| ());

        for (bad, good) in dates {
            assert_eq!(
                from_rfc822_to_rfc2822(bad),
                DateTime::parse_from_rfc2822(good)
            )
        }
    }
}
