use itertools::Itertools;


pub fn transpose_map<T: Sized + Copy>(map: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut tm = vec![];
    for i in 0..map[0].len() {
        tm.push(map.iter().cloned().map(|l| l[i]).collect_vec());
    }
    tm
}

pub fn rot90<T: Sized + Copy + Default>(map: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut rm = vec![vec![T::default(); map.len()]; map[0].len()];
    for i in 0..map.len() {
        (0..map[0].len()).for_each(|j| {
            rm[j][map.len() - i - 1] = map[i][j];
        });
    }
    rm
}

pub fn rrot90<T: Sized + Copy + Default>(map: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut rm = vec![vec![T::default(); map.len()]; map[0].len()];
    for i in 0..map.len() {
        (0..map[0].len()).for_each(|j| {
            rm[map[0].len() - j - 1][i] = map[i][j];
        });
    }
    rm
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rrot90() {
        let map = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let rmap = vec![
            vec![3, 6, 9],
            vec![2, 5, 8],
            vec![1, 4, 7],
        ];
        assert_eq!(rrot90(&map), rmap);
    }
}