pub fn main() {
    let input = advent::input_from_args();

    let reports = input
        .map(|line| line.expect("unable to get line"))
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().expect("unable to parse level"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();
    let (safe, dapened_safe) = count_safe_reports(reports);
    println!("safety: {}", safe);
    println!("safety dampened: {}", dapened_safe);
}

fn count_safe_reports(reports: Vec<Vec<u32>>) -> (u32, u32) {
    fn is_safe_report(report: &[u32]) -> bool {
        let safe = report.windows(2).fold(true, |acc, window| {
            let (prev, num) = (window[0], window[1]);
            let diff = num.abs_diff(prev);

            acc && !(diff > 3 || diff == 0)
        });

        let ordered = report.windows(3).all(|window| {
            let (prev, num, next) = (window[0], window[1], window[2]);

            !(prev > num && num < next || prev < num && num > next)
        });

        safe && ordered
    }

    fn can_be_made_safe(report: &[u32]) -> bool {
        for i in 0..report.len() {
            let mut modified_report = report.to_owned();
            modified_report.remove(i);
            if is_safe_report(&modified_report) {
                return true;
            }
        }

        false
    }

    let total_safe_reports = reports
        .iter()
        .filter(|&report| is_safe_report(report))
        .count();

    let total_damp_reports = reports
        .iter()
        .filter(|&report| is_safe_report(report) || can_be_made_safe(report))
        .count();

    (total_safe_reports as u32, total_damp_reports as u32)
}

#[cfg(test)]
mod test {
    use crate::count_safe_reports;

    #[test]
    fn test_safety() {
        let report = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let (safe, damp_safe) = count_safe_reports(report);

        assert_eq!(safe, 2);
        assert_eq!(damp_safe, 4);
    }
}
