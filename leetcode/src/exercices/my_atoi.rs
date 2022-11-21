pub struct Solution {}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut s: Vec<char> = s.trim().chars().collect();
        let mut positive_flag = 1;
        let mut nbr: i32 = 0;

        if s.len() == 0 {
            return 0;
        }
        if (s[0] == '-') || (s[0] == '+') {
            if s[0] == '-' {
                positive_flag = -1;
            }
            s.remove(0);
        }
        for i in 0..s.len() {
            if !s[i].is_digit(10) {
                break;
            }
            nbr = match nbr.checked_mul(10) {
                Some(v) => v,
                None => {
                    if positive_flag == 1 {
                        return i32::MAX;
                    } else {
                        return i32::MIN;
                    }
                }
            };

            nbr = match nbr.checked_add((s[i] as i32) - ('0' as i32)) {
                Some(v) => v,
                None => {
                    if positive_flag == 1 {
                        return i32::MAX;
                    } else {
                        return i32::MIN;
                    }
                }
            };
        }
        nbr * positive_flag
    }
}
