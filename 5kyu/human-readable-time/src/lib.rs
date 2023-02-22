#![allow(dead_code)]

use std::time::Duration;
use std::fmt;

struct DurationWrapper(Duration);

impl fmt::Display for DurationWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.0.as_secs();
        if secs < 60 {
            if secs < 10 {
                write!(f, "00:00:0{}", secs)
            } else {
                write!(f, "00:00:{}", secs)
            }
        } else if secs < 3600 {
            let m = secs / 60;
            let s = secs % 60;

            if m < 10 && s < 10 {
                write!(f, "00:0{}:0{}", m, s)
            } else if m < 10 && s >= 10 {
                write!(f, "00:0{}:{}", m, s)
            } else if m >= 10 && s < 10 {
                write!(f, "00:{}:0{}", m, s)
            } else {
                write!(f, "00:{}:{}", m, s)
            }
        } else {
            let h = secs / 3600;
            let m = (secs - h * 3600) / 60;
            let s = (secs - h * 3600) % 60;

            if h < 10 && m < 10 && s < 10 {
                write!(f, "0{}:0{}:0{}", h, m, s)
            } else if h < 10 && m >= 10 && s < 10 {
                write!(f, "0{}:{}:0{}", h, m, s)
            } else if h < 10 && m < 10 && s >= 10 {
                write!(f, "0{}:0{}:{}", h, m, s)
            } else if h < 10 && m >= 10 && s >= 10 {
                write!(f, "0{}:{}:{}", h, m, s)
            } else if h >= 10 && m < 10 && s < 10 {
                write!(f, "{}:0{}:0{}", h, m, s)
            } else if h >= 10 && m >= 10 && s < 10 {
                write!(f, "{}:{}:0{}", h, m, s)
            } else if h >= 10 && m < 10 && s >= 10 {
                write!(f, "{}:0{}:{}", h, m, s)
            }
            else {
                write!(f, "{}:{}:{}", h, m, s)
            }
        }
    }
}

fn make_readable(seconds: u32) -> String {
    DurationWrapper(Duration::from_secs(seconds.into())).to_string()
}

#[cfg(test)]
mod tests {
    use super::make_readable;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(s: u32, expected: &str) {
        assert_eq!(make_readable(s), expected, "{ERR_MSG} with seconds = {s}")
    }

    #[test]
    fn fixed_tests() {
        dotest(0, "00:00:00");
        dotest(59, "00:00:59");
        dotest(60, "00:01:00");
        dotest(3599, "00:59:59");
        dotest(3600, "01:00:00");
        dotest(86399, "23:59:59");
        dotest(86400, "24:00:00");
        dotest(359999, "99:59:59");
    }
}
