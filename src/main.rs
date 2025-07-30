#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(iter_chain)]

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::iter::{chain, zip};
use std::marker::ConstParamTy;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

use num::{One, Zero};
use thiserror::Error;

trait Field: Add + Sub + Mul + Div + Display + One + Zero + PartialEq + Clone + AddAssign {}
impl<T: Add + Sub + Mul + Div + Display + One + Zero + PartialEq + Clone + AddAssign> Field for T {}

trait Power:
    Add + Display + One + Zero + PartialEq + PartialOrd + Eq + Hash + Clone + AddAssign
{
}
impl<T: Add + Display + One + Zero + PartialEq + PartialOrd + Eq + Hash + Clone + AddAssign> Power
    for T
{
}

#[derive(ConstParamTy, PartialEq, Eq)]
enum IsPolynomialReduced {
    Reduced,
    NotReduced,
}

#[derive(Debug)]
struct Polynomial<const NUM_VARS: usize, K, P, const IS_RED: IsPolynomialReduced>
where
    K: Field,
    P: Power,
{
    terms: Vec<Term<NUM_VARS, K, P>>,
}

impl<const NUM_VARS: usize, K, P, const IS_RED: IsPolynomialReduced> Display
    for Polynomial<NUM_VARS, K, P, IS_RED>
where
    K: Field,
    P: Power,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, term) in self.terms.iter().enumerate() {
            if i > 0 {
                f.write_str("+")?;
            }
            write!(f, "{term}")?;
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
#[error("polynomial was not reduced")]
struct PolynomialReducedErr<const NUM_VARS: usize, K, P, const IS_RED: IsPolynomialReduced>
where
    K: Field,
    P: Power,
{
    poly: Polynomial<NUM_VARS, K, P, IS_RED>,
}

impl<const NUM_VARS: usize, K, P> Polynomial<NUM_VARS, K, P, { IsPolynomialReduced::NotReduced }>
where
    K: Field,
    P: Power,
{
    fn try_as_reduced(
        self,
    ) -> Result<
        Polynomial<NUM_VARS, K, P, { IsPolynomialReduced::Reduced }>,
        PolynomialReducedErr<NUM_VARS, K, P, { IsPolynomialReduced::NotReduced }>,
    > {
        let mut set = HashSet::with_capacity(self.terms.len());
        let is_reduced = self
            .terms
            .iter()
            .all(|t| set.insert(t.mono.clone()) & !t.coeff.is_zero());
        if is_reduced {
            Ok(Polynomial { terms: self.terms })
        } else {
            Err(PolynomialReducedErr { poly: self })
        }
    }

    fn reduce(self) -> Polynomial<NUM_VARS, K, P, { IsPolynomialReduced::Reduced }> {
        // TODO: Hack
        self + Self { terms: Vec::new() }
    }
}

impl<const NUM_VARS: usize, K, P, const IS_RED: IsPolynomialReduced>
    Add<Polynomial<NUM_VARS, K, P, { IsPolynomialReduced::NotReduced }>>
    for Polynomial<NUM_VARS, K, P, IS_RED>
where
    K: Field,
    P: Power,
{
    type Output = Polynomial<NUM_VARS, K, P, { IsPolynomialReduced::Reduced }>;

    fn add(
        self,
        rhs: Polynomial<NUM_VARS, K, P, { IsPolynomialReduced::NotReduced }>,
    ) -> Self::Output {
        let mut map = HashMap::new();
        for term in chain(self.terms, rhs.terms) {
            let entry = map.entry(term.mono).or_insert(K::zero());
            *entry += term.coeff;
        }
        Polynomial {
            terms: map
                .into_iter()
                .filter(|(_mono, coeff)| !coeff.is_zero())
                .map(|(mono, coeff)| Term { mono, coeff })
                .collect(),
        }
    }
}

impl<const NUM_VARS: usize, K, P, const IS_RED: IsPolynomialReduced>
    Add<Polynomial<NUM_VARS, K, P, { IsPolynomialReduced::Reduced }>>
    for Polynomial<NUM_VARS, K, P, IS_RED>
where
    K: Field,
    P: Power,
{
    type Output = Polynomial<NUM_VARS, K, P, { IsPolynomialReduced::Reduced }>;

    fn add(
        self,
        rhs: Polynomial<NUM_VARS, K, P, { IsPolynomialReduced::Reduced }>,
    ) -> Self::Output {
        let mut map = HashMap::new();
        for term in chain(self.terms, rhs.terms) {
            let entry = map.entry(term.mono).or_insert(K::zero());
            *entry += term.coeff;
        }
        Polynomial {
            terms: map
                .into_iter()
                .filter(|(_mono, coeff)| !coeff.is_zero())
                .map(|(mono, coeff)| Term { mono, coeff })
                .collect(),
        }
    }
}

#[derive(Debug)]
struct Term<const NUM_VARS: usize, K, P>
where
    K: Field,
    P: Power,
{
    coeff: K,
    mono: Monomial<NUM_VARS, P>,
}

impl<const NUM_VARS: usize, K, P> Display for Term<NUM_VARS, K, P>
where
    K: Field,
    P: Power,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.coeff.is_one() {
            write!(f, "{}*", self.coeff)?;
        }
        write!(f, "{}", self.mono)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Monomial<const NUM_VARS: usize, P>
where
    P: Power,
{
    index: [P; NUM_VARS],
}

impl<const NUM_VARS: usize, P> Display for Monomial<NUM_VARS, P>
where
    P: Power,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, power) in self.index.iter().filter(|a| !a.is_zero()).enumerate() {
            if i > 0 {
                f.write_str("*")?;
            }
            if power.is_zero() {
                continue;
            }
            write!(f, "x_{}", i + 1)?;
            if !power.is_one() {
                write!(f, "^{power}")?;
            }
        }
        Ok(())
    }
}

impl<const NUM_VARS: usize, P> Monomial<NUM_VARS, P>
where
    P: Power,
{
    fn divides(&self, other: &Monomial<NUM_VARS, P>) -> bool {
        zip(&self.index, &other.index).all(|(a, b)| a <= b)
    }
}

#[derive(ConstParamTy, PartialEq, Eq)]
enum IsMonomialIdealMinimallyGenerated {
    Minimal,
    NotMinimal,
}

struct MonomialIdeal<const NUM_VARS: usize, K, P, const IS_MIN: IsMonomialIdealMinimallyGenerated>
where
    K: Field,
    P: Power,
{
    gens: Vec<Term<NUM_VARS, K, P>>,
}

fn main() {
    let f: Polynomial<_, f64, u32, { IsPolynomialReduced::NotReduced }> = Polynomial {
        terms: vec![
            Term {
                coeff: 2.0,
                mono: Monomial { index: [1, 2] },
            },
            Term {
                coeff: 1.0,
                mono: Monomial { index: [1, 2] },
            },
        ],
    };
    let f = f.try_as_reduced().unwrap_or_else(|PolynomialReducedErr { poly }| poly.reduce());
    let g: Polynomial<_, f64, u32, { IsPolynomialReduced::NotReduced }> = Polynomial {
        terms: vec![
            Term {
                coeff: 3.0,
                mono: Monomial { index: [4, 0] },
            },
        ],
    };
    let g = g.try_as_reduced().unwrap_or_else(|PolynomialReducedErr { poly }| poly.reduce());
    println!("f = {f}");
    println!("g = {g}");
    println!("f + g = {}", f + g);
}
