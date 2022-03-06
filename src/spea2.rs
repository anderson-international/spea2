use crate::sack::Sack;

pub fn evolve(p: Vec<Sack>) -> (Vec<i32>, Vec<Vec<usize>>, Vec<i32>) {
    let a = vec![];

    let mut u = p.clone();
    u.append(&mut a.clone());
    let len = u.len();

    let mut st = vec![0; len];
    let mut dm: Vec<Vec<usize>> = vec![vec![]; len];
    let mut ds: Vec<String> = vec![];
    for i in 0..len {
        let Sack {
            weight: w1,
            value: v1,
            ..
        } = u[i];
        for j in i + 1..len {
            let Sack {
                weight: w2,
                value: v2,
                ..
            } = u[j];
            if w1 < w2 && v1 > v2 {
                st[i] += 1;
                dm[j].push(i);
            } else if w2 < w1 && v2 > v1 {
                st[j] += 1;
                dm[i].push(j);
            }
            // let d = ((i32::pow(w1 - w2, 2) + i32::pow(v1 - v2, 2)) as f32).sqrt();
            // for k in 0..len {}
            ds.push(format!("{} = {}-{}", ds.len(), i, j));
        }
    }
    println!("{:#?}", ds);

    let mut start: usize = 0;
    let mut end: usize = 0;
    for i in 0..len {
        let offset = len - i - 1;
        end += offset;
        let d = &ds[start..end];
        println!("{:?}", d);
        start += offset;
    }

    //raw fitness
    let mut rf = vec![0; len];
    for i in 0..len {
        for j in 0..dm[i].len() {
            rf[i] += st[dm[i][j]];
        }
    }

    (st, dm, rf)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sack::Sack;
    fn mock_sack(weight: i32, value: i32) -> Sack {
        Sack {
            weight,
            value,
            items: vec![],
        }
    }
    #[test]
    pub fn test_get_strengths() {
        let p = vec![
            mock_sack(2, 4),
            mock_sack(3, 3),
            mock_sack(3, 2),
            mock_sack(1, 5),
        ];
        let (s, d, rf) = evolve(p);

        println!("{:?}", s);
        assert_eq!(s, [2, 0, 0, 3]);

        println!("{:?}", d);
        assert_eq!(d[0], [3]);
        assert_eq!(d[1], [0, 3]);
        assert_eq!(d[2], [0, 3]);
        assert_eq!(d[3], []);

        println!("{:?}", rf);
        assert_eq!(rf[0], 3);
        assert_eq!(rf[1], 5);
        assert_eq!(rf[2], 5);
        assert_eq!(rf[3], 0);
    }
}
