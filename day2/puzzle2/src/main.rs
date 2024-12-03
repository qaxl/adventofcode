fn main() {
    let input = include_str!("../input.txt");

    let mut safe = 0;
    for report in input.split('\n') {
        let report = report
            .split_whitespace()
            .map(|x| x.parse::<u32>().expect("expected u32 integer"))
            .collect::<Vec<_>>();

        if report.is_empty() {
            continue;
        }


        if check_if_safe_when_one_invalid_level_is_removed(&report) {
            safe += 1;
            println!("{report:?} safe");
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

fn check_if_safe_when_one_invalid_level_is_removed(report: &[u32]) -> bool {
    for n in 0..report.len() {
        let report = report.iter().enumerate().filter(|&(i, _)| i != n).map(|(_, v)| *v).collect::<Vec<_>>();

        if validate(&report) {
            return true;
        }
    }

    return false;
}
