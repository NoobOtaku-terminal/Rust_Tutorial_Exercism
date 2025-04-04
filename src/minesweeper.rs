pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let n = minefield.len();
    if n == 0 {
        return vec![];
    }
    
    let m = minefield[0].len();
    let mut ans = Vec::<String>::new();

    let directions: [(i32, i32); 8] = [
        (1, 0), (-1, 0), (0, 1), (0, -1), 
        (1, 1), (-1, -1), (1, -1), (-1, 1)
    ];

    for i in 0..n {
        let mut temp = String::new();
        for j in 0..m {
            if minefield[i].as_bytes()[j] == b'*' {
                temp.push('*');
                continue;
            }

            let mut count = 0;
            for &(dx, dy) in &directions {
                let ni = i as i32 + dx;
                let nj = j as i32 + dy;
                
                if ni >= 0 && ni < n as i32 && nj >= 0 && nj < m as i32 {
                    if minefield[ni as usize].as_bytes()[nj as usize] == b'*' {
                        count += 1;
                    }
                }
            }

            if count > 0 {
                temp.push((b'0' + count as u8) as char);
            } else {
                temp.push(' ');
            }
        }
        ans.push(temp);
    }

    ans
}
