fn main () {
}
fn solve2(s_pos: usize, t_pos: usize, memo: &mut [[isize; 100]]) -> isize {
    let s = "aatcoodeerr".chars().collect::<Vec<char>>();
    let t = "atcoder".chars().collect::<Vec<char>>();
    let mut cnt = 0;
    if t.len()-1 == t_pos && s.len() > s_pos && s[s_pos] == t[t_pos] {
        cnt += 1;
    }
    if s.len()-1 == s_pos {
        return cnt;
    }

    if memo[s_pos][t_pos] != -1 {
        return memo[s_pos][t_pos];
    }

    if t.len()-1 > t_pos && s[s_pos] == t[t_pos] {
        cnt += solve2(s_pos+1, t_pos+1, memo);
    }
    cnt += solve2(s_pos+1, t_pos, memo);
    memo[s_pos][t_pos] = cnt;
    cnt
}
