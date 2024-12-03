fn main() {
    let input = include_str!("../input.txt");

    let mut safe = 0;
    for report in input.split('\n') {
        let report = report
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("expected u32 integer"))
            .collect::<Vec<_>>();

        if !report.is_empty() && validate(&report) {
            safe += 1;
        }
    }

    println!("{safe}");
}

fn validate(report: &[u32]) -> bool {
    let mut increasing = None;

    for level in report.windows(2) {
        if level[0].abs_diff(level[1]) > 3 {
            return false;
        }

        if level[0] > level[1] {
            if let Some(increasing) = increasing {
                if increasing {
                    return false;
                }
            } else {
                increasing = Some(false);
            }
        } else if level[0] < level[1] {
            if let Some(increasing) = increasing {
                if !increasing {
                    return false;
                }
            } else {
                increasing = Some(true);
            }
        } else {
            return false;
        }
    }

    return true;
}
