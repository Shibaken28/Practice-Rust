# 素因数分解

$2$ 以上の整数 $N$ を素因数分解する。

## 計算量
$O(\sqrt{N})$

## 補足
`factor`は素因数を列挙する。`factor2`は素因数とその個数を列挙する。

例
```rust
factor(12) -> [2, 2, 3] // 2 * 2 * 3 = 12
factor2(12) -> [(2, 2), (3, 1)] // 2^2 * 3^1 = 12
```

## Verify
- [アルゴリズムと数学　演習問題集 014 - Factorization](https://atcoder.jp/contests/math-and-algorithm/submissions/44208636)
- [アルゴリズムと数学　演習問題集 014 - Factorization](https://atcoder.jp/contests/math-and-algorithm/submissions/44208832)
