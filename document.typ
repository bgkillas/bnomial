$
binom(n, k)_b = cases(
    sum_(i=0)^(b-1) binom(n-1,k-i) \, n>0 and 0<=k<=n(b-1),
    1 \, n=0 and k=0,
    0 \, "otherwise"
)
$

$binom(m,k)_b equiv 1 mod b <=> b "is prime" and m=(b^n-1)/(b-1)$

$sum_(k=0)^(m(b-1)) (binom(m, k)_b mod b) = b <=> b "is prime" and m=b^n$

$(sum_(i=0)^(b-1) x^i)^n=sum_(i=0)^(n(b-1)) x^i binom(n,i)_b$