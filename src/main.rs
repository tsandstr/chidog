use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt::Display;
use std::hash::Hash;
use std::iter::zip;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use num::{BigRational, Num, One, PrimInt, Unsigned, Zero};

trait Ring<T: RingElement> {}

trait RingOps: Add + Sub + Mul + One + Zero + AddAssign + SubAssign + MulAssign {}
impl<T> RingOps for T where T: Add + Sub + Mul + One + Zero + AddAssign + SubAssign + MulAssign {}

trait RingElement: Sized + RingOps {}

struct PolynomialRing<'a, R, V> {
    vars: Vec<V>,
    base: &'a R,
}

impl<R, V> PolynomialRing<'_, R, V>
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

#[derive(Clone)]
struct Polynomial<'a, R, V, K, P>
where
    P: Hash,
{
    elem_of: &'a PolynomialRing<'a, R, V>,
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
        let mut terms = HashMap::<Monomial<P>, K>::new();
        for (m1, c1) in self.terms.iter() {
            for (m2, c2) in rhs.terms.iter() {
                let prod_monomial = zip(m1.powers.iter(), m2.powers.iter())
                    .map(|(m1, m2)| *m1 + *m2)
                    .collect();
                match terms.entry(Monomial {
                    powers: prod_monomial,
                }) {
                    Entry::Occupied(mut entry) => {
                        *entry.get_mut() += c1.clone() * c2.clone();
                        if entry.get().is_zero() {
                            entry.remove();
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(c1.clone() * c2.clone());
                    }
                }
            }
        }
        Self {
            elem_of: self.elem_of,
            terms,
        }
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

impl<R, V, K, P> SubAssign for Polynomial<'_, R, V, K, P>
where
    R: Ring<K>,
    K: RingElement + Clone,
    P: Hash + PrimInt + Unsigned + Clone,
    V: Eq,
{
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl<R, V, K, P> MulAssign for Polynomial<'_, R, V, K, P>
where
    R: Ring<K>,
    K: RingElement + Clone,
    P: Hash + PrimInt + Unsigned + Clone,
    V: Eq,
{
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Monomial<P> {
    powers: Vec<P>,
}

impl<R, V, K, P> Ring<Polynomial<'_, R, V, K, P>> for PolynomialRing<'_, R, V>
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

#[derive(Clone)]
struct AlreadyRing<T> {
    phantom: PhantomData<T>,
}
impl<T> Ring<T> for AlreadyRing<T> where T: Num + RingOps {}
impl<T> RingElement for T where T: Num + RingOps {}

fn main() {
    let my_ring = PolynomialRing {
        vars: vec!["x", "y", "z"]
            .into_iter()
            .map(|s| String::from(s))
            .collect(),
        base: &AlreadyRing { phantom: PhantomData::<BigRational> },
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
    println!("f     = {f}");
    println!("g     = {g}");
    println!("f + g = {}", f + g);

    println!();

    let your_ring = PolynomialRing {
        vars: vec!["a", "b"],
        base: &AlreadyRing { phantom: PhantomData::<BigRational> },
    };
    let foo = Polynomial {
        elem_of: &your_ring,
        terms: HashMap::<Monomial<u32>, BigRational>::from([
            (
                Monomial { powers: vec![1, 0] },
                BigRational::from_float(1.0).unwrap(),
            ),
            (
                Monomial { powers: vec![0, 1] },
                BigRational::from_float(1.0).unwrap(),
            ),
        ]),
    };
    let bar = foo.clone();
    println!("foo   = {foo}");
    println!("bar   = foo");
    println!("foo^2 = {}", foo * bar);
}
