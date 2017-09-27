extern crate chrono;

use std::collections::HashMap;
use chrono::DateTime;

pub fn from_str(s:&str) -> Result<(), Error>{

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
    let re = regex::Regex::new(r"(\d{1,2}):(\d{1,2}):(\d{1,2})")?;
    // hours, minutes, seconds = cap[1], cap[2], cap[3]
    let cap = re.captures(&s).unwrap();
    let mut newtime = Vec::new();

    cap.iter().skip(1).for_each(|x| if let Some(y) = x {
        if y.end() - y.start() == 1 {
            // warn!("{}", y.as_str());
            newtime.push(format!("0{}", y.as_str()));
        } else {
            newtime.push(y.as_str().to_string());
        }
    });

    let ntime = &newtime.join(":");
    foo = foo.replace(cap.get(0).unwrap().as_str(), ntime);
    debug!("{:?}", foo);

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

    let date = DateTime::parse_from_rfc2822(&foo);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
