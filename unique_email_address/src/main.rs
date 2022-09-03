use std::collections::HashSet;

struct Solution {}

impl Solution {
    // Using Destructuring Tuple
    pub fn num_unique_emails_1(emails: Vec<String>) -> i32 {
        let mut unique_emails = HashSet::new();

        for email in emails {
            let mut clean_email = String::from("");
            let mut encount_at = false;
            let mut encount_plus = false;

            for c in email.chars() {
                match (c, encount_at, encount_plus) {
                    ('.', false, _) => {}
                    ('@', _, _) => {
                        encount_at = true;
                        encount_plus = false;
                        clean_email.push(c);
                    }
                    ('+', _, _) => {
                        encount_plus = true;
                    }
                    (x, _, false) => clean_email.push(x),
                    (_, _, _) => {}
                }
            }

            unique_emails.insert(clean_email);
        }

        unique_emails.len() as _
    }

    // Using Guards
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique_emails = HashSet::new();

        for email in emails {
            let mut clean_email = String::from("");
            let mut encount_at = false;
            let mut encount_plus = false;

            for c in email.chars() {
                match c {
                    '.' if !encount_at => {},
                    '@' => {
                        encount_at = true;
                        encount_plus = false;
                        clean_email.push(c);
                    },
                    '+' => {
                        encount_plus = true;
                    },
                    _ if encount_plus => {},
                    _ => clean_email.push(c),
                }
            }

            unique_emails.insert(clean_email);
        }

        unique_emails.len() as _
    }
}

fn main() {
    let emails = vec![
        "test.email+alex@leetcode.com".to_string(),
        "test.e.mail+bob.cathy@leetcode.com".to_string(),
        "testemail+david@lee.tcode.com".to_string(),
    ];
    println!("{}", Solution::num_unique_emails_1(emails.clone()));
    println!("{}", Solution::num_unique_emails(emails));
}
