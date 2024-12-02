pub fn solve(input: String) {
    let reports = read_reports(input);
    let mut count_safe = 0;
    for report in reports {
        if report.is_empty() {
            continue;
        }
        let result = check_report(&report);
        println!("{:?}: {}", report, result);
        if result {
            count_safe = count_safe + 1;
        }
    }
    println!("Total safe: {}", count_safe);
}

fn read_reports(input: String) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();
    let lines = input.split("\n");
    for line in lines {
        let values = read_report(&line);
        reports.push(values);
    }
    reports
}

fn read_report(report: &str) -> Vec<i32> {
    let mut report_values: Vec<i32> = Vec::new();
    let report: Vec<&str> = report.split_whitespace().collect();
    for value in report {
        let value: i32 = value.parse().expect("A number");
        report_values.push(value);
    }

    report_values
}

fn check_report(report: &Vec<i32>) -> bool {
    let mut last = report.get(0).expect("Should contain value");
    let mut order: Option<&str> = None;
    for index in 1..report.len() {
        let value = report.get(index).expect("A number");
        let current_order = if value > last { "asc" } else { "dsc" };
        match order {
            None => order = Some(current_order),
            Some(order) => {
                if order != current_order {
                    return false;
                }
            }
        }

        let diff = last - value;
        if diff == 0 {
            return false;
        };
        if (diff >= 4) || (diff <= -4) {
            return false;
        }
        last = value;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_dsc_safe() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(check_report(&report));
    }

    #[test]
    fn verify_asc_safe() {
        let report = vec![1, 3, 6, 7, 9];
        assert!(check_report(&report));
    }

    #[test]
    fn verify_asc_unsafe() {
        let report = vec![1, 2, 7, 8, 9];
        assert!(!check_report(&report));
    }

    #[test]
    fn verify_dsc_unsafe() {
        let report = vec![9, 7, 6, 2, 1];
        assert!(!check_report(&report));
    }

    #[test]
    fn verify_no_change() {
        let report = vec![8, 6, 4, 4, 1];
        assert!(!check_report(&report));
    }

    #[test]
    fn verify_pattern_swap() {
        let report = vec![1, 3, 2, 4, 5];
        assert!(!check_report(&report));
    }
}
