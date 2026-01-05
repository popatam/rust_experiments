


// # префиксные суммы
// вычисление сумм/разностей среза последовательности
// O(n) на построение, 0(1) на любую сумму
fn prefix_sums(a: &[i64]) -> Vec<i64> {
    let mut sums = vec![0i64; a.len() + 1]; // +1 элемент!
    for i in 0..a.len() {
        sums[i+1] = sums[i] + a[i];
    }
    sums
}

fn range_sums(sums: &[i64], left: usize, right: usize) -> i64 {
    sums[right+1] - sums[left]
}

// #

fn main() {
    // prefix_sums
    let a = [2,1,-4, 8, -11];
    let sums_vec = prefix_sums(&a);
    let range_sum = range_sums(&sums_vec, 1, 2);
    println!("{sums_vec:?} {range_sum}");
    use std::collections::HashMap;

    fn count_subarrays_sum_k(a: &[i64], k: i64) -> i64 {
        let mut cnt: HashMap<i64, i64> = HashMap::new();
        cnt.insert(0, 1); // pref[0] = 0 встречается 1 раз

        let mut pref = 0i64;
        let mut ans = 0i64;

        for &x in a {
            pref += x;
            if let Some(c) = cnt.get(&(pref - k)) {
                ans += c;
            }
            *cnt.entry(pref).or_insert(0) += 1;
        }
        println!("{cnt:?}");
        ans
    }
    fn all_subarrays_sum_k(a: &[i64], k: i64) -> Vec<(usize, usize)> {
        // pos[sum] = все i, где pref[i] == sum
        let mut pos: HashMap<i64, Vec<usize>> = HashMap::new();
        pos.insert(0, vec![0]); // pref[0] = 0 на позиции i=0 (пустой префикс)

        let mut pref = 0i64;
        let mut res: Vec<(usize, usize)> = Vec::new();

        for (r, &x) in a.iter().enumerate() {
            pref += x; // теперь pref = pref[r+1]
            let need = pref - k;

            if let Some(starts) = pos.get(&need) {
                // каждый start = l (граница слева), подотрезок [l..r]
                for &l in starts {
                    res.push((l, r));
                }
            }

            // добавить текущую позицию префикса (r+1)
            pos.entry(pref).or_insert_with(Vec::new).push(r + 1);
        }

        res
    }
    println!("{:?}", all_subarrays_sum_k(&a, -3))
}
