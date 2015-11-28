---
title: CSC 446 A4
author: Andrew Hobden (V00788452)
---

## Question 1

## Q1.a

Let $n=1$, $F^{-1}(\rho,\lambda) = \frac{-\ln(1.0-\rho)}{ \lambda }$.

![](q1_chart.png)\


The line is approximately straight and has an approximate slope of 1. Therefore accept it.

## Q1.b

The mean is calculated with $\bar X = \frac{ \sum_{i=1}^{n} X_i }{ n }$.

```rust
let mean = values.iter().fold(0f64,|acc, &(expo, _)| acc + expo) / SAMPLES as f64;
```

The variance is calculated with $S^2 = \frac{ \sum_{i=1}^n X_i^2 - n\bar X^2 }{ n - 1 }$.

```rust
let variance = (values.iter().fold(0f64,|acc, &(expo, _)| {
    acc + expo.powf(2.0)
}) - (SAMPLES as f64 * mean.powf(2.0))) / (SAMPLES -1) as f64;
```

$$ \bar X = \frac{ \sum_{i=1}^{n} X_i }{ n } = 0.9614169978283242 $$

$$ S^2 = \frac{ \sum_{i=1}^n X_i^2 - n\bar X^2 }{ n - 1 } = 0.9162704872993767 $$
