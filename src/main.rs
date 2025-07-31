#![feature(iter_intersperse)]

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Display;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, Sub};

use num::{BigRational, One, PrimInt, Unsigned, Zero};

trait Ring<T: RingElement> {}

trait RingOps: Add + Sub + Mul + One + Zero + AddAssign {}
impl<T> RingOps for T where T: Add + Sub + Mul + One + Zero + AddAssign {}

trait RingElement: Sized + RingOps {}

struct PolynomialRing<R, V> {
    vars: Vec<V>,
    phantom: PhantomData<R>,
}

impl<R, V> PolynomialRing<R, V>
where
    V: Display,
{
    fn fmt_monomial<P: Display + Zero + One + Eq>(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        m: &Monomial<P>,
    ) -> std::fmt::Result {
        if m.powers.iter().all(|p| p.is_zero()) {
            write!(f, "1")?;
        } else {
            for (i, (var_idx, p)) in m
                .powers
                .iter()
                .enumerate()
                .filter(|(_j, p)| !p.is_zero())
                .enumerate()
            {
                if i > 0 {
                    write!(f, "*")?;
                }
                write!(f, "{}", self.vars[var_idx])?;
                if !p.is_one() {
                    write!(f, "^{p}")?;
                }
            }
        }
        Ok(())
    }
}

struct Polynomial<'a, R, V, K, P>
where
    P: Hash,
{
    elem_of: &'a PolynomialRing<R, V>,
    terms: HashMap<Monomial<P>, K>,
}

impl<R, V, K, P> Add for Polynomial<'_, R, V, K, P>
where
    R: Ring<K>,
    K: RingElement + Clone,
    P: Hash + PrimInt + Unsigned + Clone,
    V: Eq,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut terms = self.terms.clone();
        for (m, c2) in rhs.terms.into_iter() {
            match terms.entry(m) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += c2;
                    if entry.get().is_zero() {
                        entry.remove();
                    }
                }
                Entry::Vacant(entry) => {
                    entry.insert_entry(c2);
                }
            }
        }
        Self {
            elem_of: self.elem_of,
            terms,
        }
    }
}
impl<R, V, K, P> Sub for Polynomial<'_, R, V, K, P>
where
    R: Ring<K>,
    K: RingElement + Clone,
    P: Hash + PrimInt + Unsigned + Clone,
    V: Eq,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl<R, V, K, P> Mul for Polynomial<'_, R, V, K, P>
where
    R: Ring<K>,
    K: RingElement + Clone,
    P: Hash + PrimInt + Unsigned + Clone,
    V: Eq,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
impl<R, V, K, P> One for Polynomial<'_, R, V, K, P>
where
    R: Ring<K>,
    K: RingElement + Clone,
    P: Hash + PrimInt + Unsigned + Clone,
    V: Eq,
{
    fn one() -> Self {
        todo!()
    }
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::one()
    }
}
impl<R, V, K, P> Zero for Polynomial<'_, R, V, K, P>
where
    R: Ring<K>,
    K: RingElement + Clone,
    P: Hash + PrimInt + Unsigned + Clone,
    V: Eq,
{
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}
impl<R, V, K, P> AddAssign for Polynomial<'_, R, V, K, P>
where
    R: Ring<K>,
    K: RingElement + Clone,
    P: Hash + PrimInt + Unsigned + Clone,
    V: Eq,
{
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Monomial<P> {
    powers: Vec<P>,
}

impl<R, V, K, P> Ring<Polynomial<'_, R, V, K, P>> for PolynomialRing<R, V>
where
    R: Ring<K>,
    K: RingElement + Clone,
    P: Hash + PrimInt + Unsigned, // TODO: Correct trait (see also impl RingElement for Polynomial)
    V: Eq,
{
}

impl<R, V, K, P> RingElement for Polynomial<'_, R, V, K, P>
where
    R: Ring<K>,
    K: RingElement + Clone,
    P: Hash + PrimInt + Unsigned,
    V: Eq,
{
}

impl<R, V, K, P> Display for Polynomial<'_, R, V, K, P>
where
    K: Display + One + Eq,
    P: Hash + Display + One + Zero + Eq,
    V: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.terms.is_empty() {
            write!(f, "0")?;
        } else {
            for (i, (m, c)) in self.terms.iter().enumerate() {
                // TODO: Handle parenthesization of coefficients;
                // probably decided trait DisplayAsCoefficient
                if !c.is_one() {
                    if i > 0 {
                        write!(f, "{c:+}")?;
                    } else {
                        write!(f, "{c}")?;
                    }
                    write!(f, "*")?;
                } else if i > 0 {
                    write!(f, "+")?;
                }
                self.elem_of.fmt_monomial(f, m)?;
            }
        }
        Ok(())
    }
}

struct AlreadyRing {}
impl<T> Ring<T> for AlreadyRing where T: RingOps + Clone {}
impl<T> RingElement for T where T: RingOps + Clone {}

fn main() {
    let my_ring = PolynomialRing {
        vars: vec!["x", "y", "z"]
            .into_iter()
            .map(|s| String::from(s))
            .collect(),
        phantom: PhantomData::<AlreadyRing>,
    };
    let f = Polynomial {
        elem_of: &my_ring,
        terms: HashMap::<Monomial<u32>, BigRational>::from([
            (
                Monomial {
                    powers: vec![1, 0, 0],
                },
                BigRational::from_float(1.0).unwrap(),
            ),
            (
                Monomial {
                    powers: vec![1, 1, 0],
                },
                BigRational::from_float(2.0).unwrap(),
            ),
            (
                Monomial {
                    powers: vec![0, 1, 1],
                },
                BigRational::from_float(3.0).unwrap(),
            ),
        ]),
    };
    let g = Polynomial {
        elem_of: &my_ring,
        terms: HashMap::<Monomial<u32>, BigRational>::from([
            (
                Monomial {
                    powers: vec![1, 0, 0],
                },
                BigRational::from_float(-1.0).unwrap(),
            ),
            (
                Monomial {
                    powers: vec![1, 1, 0],
                },
                BigRational::from_float(-3.0).unwrap(),
            ),
            (
                Monomial {
                    powers: vec![1, 1, 1],
                },
                BigRational::from_float(2.0).unwrap(),
            ),
        ]),
    };
    println!("f = {f}");
    println!("g = {g}");
    println!("f + g = {}", f + g);
}
