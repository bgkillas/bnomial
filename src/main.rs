use std::hint::black_box;
type Num = usize;
fn main() {
    let tmr = std::time::Instant::now();
    black_box(bnomials(42, 2048));
    println!("{}", tmr.elapsed().as_nanos());
}
fn bnomials(base: Num, n: Num) -> Box<[Box<[Num]>]> {
    fn get_layer(base: Num, last: &[Num], n: Num) -> Box<[Num]> {
        let mut vec = Vec::with_capacity((base - 1) * n + 1);
        for k in 0..vec.capacity() {
            vec.push(
                ((k + 1).saturating_sub(base)..=k.min((base - 1) * (n - 1)))
                    .map(|k| last[k])
                    .sum(),
            );
        }
        vec.into_boxed_slice()
    }
    let mut vec = Vec::with_capacity(n);
    vec.push(vec![1].into_boxed_slice());
    for n in 1..vec.capacity() {
        let last = vec.last().unwrap();
        let set = get_layer(base, last, n);
        vec.push(set);
    }
    vec.into_boxed_slice()
}
#[expect(unused)]
fn bnomial(base: Num, n: Num, k: Num) -> Num {
    if n == 0 {
        return 1;
    }
    let a =         (0..=k / base)
            .map(|i| if i.is_multiple_of(2) { 1 } else { -1 }
                * (binomial(n, i) * binomial(n + k - (i * base + 1), n - 1)).cast_signed())
            .sum::<isize>()
            .cast_unsigned();
    #[cfg(test)]
    assert_eq!(
        a,
        ((k + 1).saturating_sub(base)..=k.min((base - 1) * (n - 1)))
            .map(|k| bnomial(base, n - 1, k))
            .sum(),
        "{base} {n} {k}"
    );
    a
}
#[expect(unused)]
fn binomial(n: Num, k: Num) -> Num {
    let mut value = 1;
    for v in (n + 1) - k..=n {
        value *= v;
    }
    for v in 2..=k {
        #[cfg(test)]
        assert!(value.is_multiple_of(v));
        value /= v;
    }
    value
}
#[test]
fn test_trinomial() {
    let pascals: [&[Num]; _] = [
        &[1],
        &[1, 1, 1],
        &[1, 2, 3, 2, 1],
        &[1, 3, 6, 7, 6, 3, 1],
        &[1, 4, 10, 16, 19, 16, 10, 4, 1],
        &[1, 5, 15, 30, 45, 51, 45, 30, 15, 5, 1],
    ];
    let binomials = bnomials(3, pascals.len());
    for (n, arr) in pascals.iter().enumerate() {
        assert_eq!(&&*binomials[n], arr, "{n}");
        for (k, v) in arr.iter().enumerate() {
            assert_eq!(*v, bnomial(3, n, k), "{n} {k}");
        }
    }
}
#[test]
fn test_binomial() {
    let pascals: [&[Num]; _] = [
        &[1],
        &[1, 1],
        &[1, 2, 1],
        &[1, 3, 3, 1],
        &[1, 4, 6, 4, 1],
        &[1, 5, 10, 10, 5, 1],
    ];
    for (n, arr) in pascals.iter().enumerate() {
        for (k, v) in arr.iter().enumerate() {
            assert_eq!(*v, binomial(n, k), "{n} {k}");
            assert_eq!(*v, bnomial(2, n, k), "{n} {k}");
        }
    }
}
