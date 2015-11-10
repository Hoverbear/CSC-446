---
title: CSC 446 A3
author: Andrew Hobden (V00788452)
---

## Question 1

See `./code/q1/src/main.rs`. You can [install Rust](https://doc.rust-lang.org/stable/book/installing-rust.html) and run it with `cargo run` from the `./code/q1/` directory.

## Question 2

See `./code/q2/src/main.rs`. You can [install Rust](https://doc.rust-lang.org/stable/book/installing-rust.html) and run it with `cargo run` from the `./code/q2/` directory.

## Question 3

```
a=11, m=16, x_0=7 -> [7, 13, 15, 5]
a=11, m=16, x_0=8 -> [8]
a=7, m=16, x_0=7 -> [7, 1]
a=7, m=16, x_0=8 -> [8]
```

See `./code/q3/src/main.rs` for procedure.

This method is not good for generating random numbers without using the increment. The maximum period is the `modulus` so it isn't achieved.

## Question 4

<!-- TODO -->




## Question 5

Calculating $M$:
$$ 1 + (M+1)m \le N $$
$$ 1 + (M+1)3 \le 20 $$
$$ M \le \frac{20 - 1}{3} - 1 $$
$$ M = \frac{16}{3} \approx 5 $$

Every third number consists of: `[0.594, 0.055, 0.262, 0.442, 0.227, 0.825, 0.929]`

$$ s = (0.594)(0.055) + (0.055)(0.262) + (0.262)(0.442) + (0.442)(0.227) + (0.227)(0.825) + (0.825)(0.929) $$
$$ \rho_{13} = \frac{1}{5+1}(s) - 0.25 = -0.047180\bar3 $$
$$ \sigma_{\rho_{13}} = \frac{ \sqrt{13(5)+7} }{ 12((5)+1) } = \frac{1}{6\sqrt{2}} $$

$\therefore$ Low random numbers tend to be followed by high ones, and vice versa.

$$ Z_0 = \frac{-0.047180\bar3}{ \frac{1}{6\sqrt{2}} } = -0.400338 $$
