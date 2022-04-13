use fraction::Fraction;

/// ∀ x > 0, 0 < ε < x < M
///
/// ε² = 0
/// M² = 0
/// M*ε = 0
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SuperReal {
    m: Fraction,
    x: Fraction,
    e: Fraction,
}

impl SuperReal {
    pub fn new(m: Fraction, x: Fraction, e: Fraction) -> Self {
        Self { m, x, e }
    }

    #[inline]
    pub fn into_inner(self) -> (Fraction, Fraction, Fraction) {
        (self.m, self.x, self.e)
    }

    #[inline]
    pub fn real(&self) -> Fraction {
        self.x
    }

    #[inline]
    pub fn epsilon(&self) -> Fraction {
        self.e
    }

    #[inline]
    pub fn em(&self) -> Fraction {
        self.m
    }

    #[inline]
    pub fn conj(&self) -> SuperReal {
        Self {
            m: -self.m,
            x: self.x,
            e: -self.e,
        }
    }

    // we have `r * conj(r) = Real(r)²`
}

impl From<Fraction> for SuperReal {
    fn from(f: Fraction) -> SuperReal {
        SuperReal::new(Fraction::from(0), f, Fraction::from(0))
    }
}

// impl From<u64> for SuperReal {
//     fn from(x: u64) -> SuperReal {
//         SuperReal::new(Fraction::from(0), Fraction::from(x), Fraction::from(0))
//     }
// }

// impl From<(u64, u64, u64)> for SuperReal {
//     fn from((m, x, e): (u64, u64, u64)) -> SuperReal {
//         SuperReal::new(Fraction::from(m), Fraction::from(x), Fraction::from(e))
//     }
// }

// impl From<[u64; 3]> for SuperReal {
//     fn from([m, x, e]: [u64; 3]) -> SuperReal {
//         SuperReal::new(Fraction::from(m), Fraction::from(x), Fraction::from(e))
//     }
// }

impl From<i64> for SuperReal {
    fn from(x: i64) -> SuperReal {
        SuperReal::new(Fraction::from(0), Fraction::from(x), Fraction::from(0))
    }
}

impl From<(i64, i64, i64)> for SuperReal {
    fn from((m, x, e): (i64, i64, i64)) -> SuperReal {
        SuperReal::new(Fraction::from(m), Fraction::from(x), Fraction::from(e))
    }
}

impl From<[i64; 3]> for SuperReal {
    fn from([m, x, e]: [i64; 3]) -> SuperReal {
        SuperReal::new(Fraction::from(m), Fraction::from(x), Fraction::from(e))
    }
}

impl std::ops::Add<SuperReal> for SuperReal {
    type Output = Self;

    #[inline]
    fn add(self, other: SuperReal) -> Self {
        Self {
            m: self.m + other.m,
            x: self.x + other.x,
            e: self.e + other.e,
        }
    }
}

impl std::ops::Sub<SuperReal> for SuperReal {
    type Output = Self;

    #[inline]
    fn sub(self, other: SuperReal) -> Self {
        Self {
            m: self.m - other.m,
            x: self.x - other.x,
            e: self.e - other.e,
        }
    }
}

impl std::ops::Mul<SuperReal> for SuperReal {
    type Output = Self;

    #[inline]
    fn mul(self, other: SuperReal) -> Self {
        Self {
            m: self.x * other.m + self.m * other.x,
            x: self.x * other.x,
            e: self.x * other.e + self.e * other.x,
        }
    }
}

impl std::ops::Mul<Fraction> for SuperReal {
    type Output = Self;

    #[inline]
    fn mul(self, by: Fraction) -> Self {
        Self {
            m: self.m * by,
            x: self.x * by,
            e: self.e * by,
        }
    }
}

impl std::ops::Div<Fraction> for SuperReal {
    type Output = Self;

    #[inline]
    fn div(self, by: Fraction) -> Self {
        Self {
            m: self.m / by,
            x: self.x / by,
            e: self.e / by,
        }
    }
}

impl std::ops::Div<SuperReal> for SuperReal {
    type Output = Self;

    #[inline]
    fn div(self, other: SuperReal) -> Self {
        let other_mul_conj = other * other.conj();
        debug_assert!(other_mul_conj.em() == Fraction::from(0));
        debug_assert!(other_mul_conj.epsilon() == Fraction::from(0));

        if other_mul_conj.real() == Fraction::from(0) {
            Self {
                m: Fraction::from(0),
                x: Fraction::from(0),
                e: Fraction::from(0),
            }
        } else {
            self * other.conj() / other_mul_conj.real()
        }
    }
}

impl std::ops::Neg for SuperReal {
    type Output = SuperReal;

    #[inline]
    fn neg(mut self) -> Self {
        self.m = -self.m;
        self.x = -self.x;
        self.e = -self.e;

        self
    }
}

impl std::fmt::Display for SuperReal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.m != Fraction::from(0) || self.e != Fraction::from(0) {
            write!(f, "({:+}M+{:+}+{:+}ε)", self.m, self.x, self.e)
        } else if self.x == Fraction::from(0) {
            write!(f, "0")
        } else {
            write!(f, "{:+}", self.x)
        }
    }
}

impl std::cmp::PartialOrd for SuperReal {
    fn partial_cmp(&self, other: &SuperReal) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;

        match self.m.partial_cmp(&other.m) {
            Some(Less) => return Some(Less),
            Some(Greater) => return Some(Greater),
            None => return None,
            _ => {}
        }

        match self.x.partial_cmp(&other.x) {
            Some(Less) => return Some(Less),
            Some(Greater) => return Some(Greater),
            None => return None,
            _ => {}
        }

        self.e.partial_cmp(&other.e)
    }
}
