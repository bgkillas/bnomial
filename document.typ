= Derivations

suppose we have $n$ $b$ sided dice and wish to know how many ways they sum up to $k$,

to make it simpler we will just consider a $b$ sided dice as having values $D_b={s in NN | 0<=s<b}$,
as you may just shift $k$ by $n$ to get ${s in NN | 1<s<=b}$, then with this simplification we have

$
sum_(i=1)^n x_i=k "where" x_i in D_b
$

where the amount $x_i$ combinations that solve the above equation is the amount of ways to roll the dice

we will define this as $binom(n,k)_b$, as $binom(n,k)_2=binom(n,k)$.

first we will find a solution inductively

=== recursive derivation

let $b$ be arbitrary

base case, $n=0$, the sum is empty so we have $0=k$, so $k=0$ has 1 set of combinations which is the empty set, so $binom(0,0)_b=1$,
and for $k != 0.$ $binom(0,k)_b=0$

let $n>0,k$ be arbitrary, by induction we have

$
sum_(i=1)^(n-1) x_i=k "where" x_i in D_b
$

and the amount of combinations is $binom(n-1,k)_b$, so we have

$sum_(i=1)^n x_i=k "where" x_i in D_b$

$x_n + sum_(i=1)^(n-1) x_i=k "where" x_i in D_b$

$x_n + sum_(i=1)^(n-1) x_i=k "where" x_i in D_b$

$sum_(i=1)^(n-1) x_i=k-x_n "where" x_i in D_b$

$sum_(i=1)^(n-1) x_i=k' "where" x_i in D_b$, for $k'=k-x_n$

so we may fix $x_n$ for each value in $D_b$, then sum up the new values of $k'$ by our inductive step, so we have that

$
binom(n,k)_b=sum_(i=0)^(b-1) binom(n-1,k-i)_b
$

if $k$ is reachable, $k<=n(b-1)$ as if $n$ dice of most $b-1$ may only sum upto $n(b-1)$, so then we may define for all $n,k$ to be

$
binom(n, k)_b = cases(
    sum_(i=0)^(b-1) binom(n-1,k-i)_b \, n>0 and 0<=k<=n(b-1),
    1 \, n=0 and k=0,
    0 \, "otherwise"
)
$

#pagebreak()

=== closed-form derivation

as shown in class $sum_(i=1)^(n) x_i=k$ where $x_i in NN$ has $binom(k+n-1,n-1)$ combinations of solutions

if we limit $x_1<b$, then we have $binom(k+n-1,n-1)-binom(k+n-1-b,n-1)$, as we consider all solutions then subtract solutions of $...=k-b$ from all solutions

if we limit $x_1,x_2<b$, then we have to apply the previous logic once at a time

so we have $binom(k+n-1,n-1)-v_1-v_2-v_3$ where $v_1=v_2=binom(n-1+k-b,n-1)-binom(n-1+k-2b,n-1)$

and $v_3=binom(n-1+k-2b,n-1)$, so $v_1$ is from looking at $x_1$ first and then $x_2$

$v_2$ is from looking at $x_2$ first then $x_1$

$v_3$ is from doing both $x_1$ and $x_2$ at the same time

so we have $binom(k+n-1,n-1)-2binom(n-1+k-b,n-1)+binom(n-1+k-2b,n-1)$

note that $binom(-n,k)=binom(n,n+l)=binom(n,-l)=0$ for $n,l>0,0<=k<=n$

from this we may hope that

$
binom(n,k)_b=sum_(i=0)^n (-1)^i binom(n,i)binom(n-1+k-i b,n-1)
$

then we prove by induction, base case dealt with above,

let $n$ be arbitrary, we assume $n-1$ holds then we want to prove $n$

so we have $sum_(i=1)^(n-1) x_i=k'. x_i in D_b$, where $k'=k-x_n$, has

$sum_(i=0)^(n-1) (-1)^i binom(n-1,i)binom(n-2+k'-i b,n-2)$

solutions, we then apply

TODO

we may also see

$
binom(n,k)_b=sum_(i=0)^(floor(k/b)) (-1)^i binom(n,i)binom(n-1+k-i b,n-1)
$

as suppose $i>floor(k/b)$, then $k-i b<0$, and therefore $binom(n-1+k-i b,n-1)=0$

#pagebreak()

$
star_1
binom(n,k)_2=binom(n,k)
$

combinatorically we may simply see that counting the amount of ways $n$ 2 sided dice sum up to $k$(LHS) is equivalent to
the amount of subsets of a $n$ sized set of size $k$(LHS)

this is simply because we may count the right side as binary, where 1 is if the element is in the subset or not, and those 1's sum upto $k$, and we see that coinsides with how we do the LHS

$sum_(i=1)^n x_i=k "where" x_i in D_b$

as $D_b={0,1}$.

=== alternate

more simply we can notice that the recursive definition of the LHS is equivalent to the recursive definition of the RHS

#pagebreak()

$
star_2
binom(n,k)_b=binom(n,n (b-1) - k)_b
$

combinatorically we may consider that the value the dice has been rolled to $l$ may be turned to \
$b-1-l$, doing this for each dice shows that these both are the same

=== alternate

proof by induction, we will make a symmetry argument

base case $n=0$, for $n=0$ we have 1 valid value $k=0$ and this equation trivially is true

let $n$ be arbitrary

assume $n-1$ want to show for $n$

recall the recursive definition

$binom(n, k)_b = cases(
    sum_(i=0)^(b-1) binom(n-1,k-i)_b \, n>0 and 0<=k<=n(b-1),
    1 \, n=0 and k=0,
    0 \, "otherwise"
)$

consider

$binom(n, n(b-1)-k)_b = cases(
    sum_(i=0)^(b-1) binom(n-1,n(b-1)-k-i)_b \, n>0 and 0<=k<=n(b-1),
    1 \, n=0 and k=0,
    0 \, "otherwise"
)$

by inductive statement we have

$binom(n, n(b-1)-k)_b = cases(
    sum_(i=0)^(b-1) binom(n-1,k-i)_b \, n>0 and 0<=k<=n(b-1),
    1 \, n=0 and k=0,
    0 \, "otherwise"
)$

so $binom(n, k)_b=binom(n, n(b-1)-k)_b$

#pagebreak()

$
star_3
(sum_(i=0)^(b-1) x^i)^n=sum_(i=0)^(n(b-1)) x^i binom(n,i)_b
$

combinatorical proof,

$(sum_(i=0)^(b-1) x^i)^n$

$=sum_(i_1=0)^(b-1) ... sum_(i_n=0)^(b-1) product_(j=1)^n x^(i_j)$

$=sum_(i_1=0)^(b-1) ... sum_(i_n=0)^(b-1) x^(sum_j i_j)$

this is equivalent to $sum_(i=1)^n x_i=k "where" x_i in D_b$, as for each specific $x^l$ we are
finding all possible sums for $n$ dice from $0$ to $b-1$ so therefore the coeffecient is $binom(n,i)_b$ and we have

$=sum_(i=0)^(n(b-1)) binom(n,i)_b x^i$

=== alternate

proof by strong induction, $n=0$ is trivially true

let $n$ be arbitrary, assume $<n$ true, want to show $n$

$(sum_(i=0)^(b-1) x^i)^n$

$=(sum_(i=0)^(b-1) x^i)(sum_(i=0)^(b-1) x^i)^(n-1)$

$=(sum_(i=0)^(b-1) x^i)(sum_(j=0)^((n-1)(b-1)) x^j binom(n-1,j)_b)$

$=sum_(i=0)^(b-1) sum_(j=0)^((n-1)(b-1)) x^i x^j binom(n-1,j)_b$

#pagebreak()

we may also define

$binom(D,k)$ where $D_n={b_i : b_i in ZZ_(>0) and 1<=i<=n}$, $D'_n={b_i in D_n : i>0}$, $sum D_n = sum_(i=1)^n b_i-1$

$
star_4
binom(D_n,k)=cases(
    sum_(i=0)^(b_0-1) binom(D'_n,k-i) \, n>0 and 0<=k<=sum D'_n,
    1 \, n=0 and k=0,
    0 \, "otherwise"
)
$

due to notation being annoying i will not prove this

#pagebreak()

$
star_5
binom(m,k)_b equiv 1 mod b <=> b "is prime" and m=(b^n-1)/(b-1)
$

unsure if actually true,

this site shows those for small prime bases, as larger gets stretched too much to be noticable

https://bgkillas.github.io/bnomial/#3

shows a pascals triangle for $b=3$ mod $b$, you may add do "\#b,n" for n columns



#pagebreak()

$
star_6
sum_(k=0)^(m(b-1)) (binom(m, k)_b mod b) = b <=> b "is prime" and m=b^n
$

unsure if actually true, see $star_5$ for site example

#pagebreak()

= Related Material

a different extension, https://en.wikipedia.org/wiki/Multinomial_theorem,
see https://en.wikipedia.org/wiki/Multinomial_theorem#Generalized_Pascal's_triangle for how they relate

also $binom(n,k)_b$ is sort of a "flattening" of https://en.wikipedia.org/wiki/Pascal's_pyramid