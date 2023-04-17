use std::ops;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) struct Loc(pub(crate) usize, pub(crate) usize);
impl Loc {
    pub(crate) fn from_notation(notation: &str) -> Self {
        let mut chars = notation.chars();
        let file = chars.next().unwrap() as usize - 'a' as usize;
        let rank = chars.next().unwrap() as usize - '1' as usize;
        Self(file, rank)
    }

    pub(crate) fn as_notation(&self) -> String {
        let mut notation = String::new();
        notation.push((self.0 as u8 + b'a') as char);
        notation.push((self.1 as u8 + b'1') as char);
        notation
    }

    #[allow(non_snake_case)]
    pub(crate) fn as_iLoc(&self) -> ILoc {
        ILoc(self.0 as i16, self.1 as i16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[cfg_attr(not(feature = "bare"), derive(schemars::JsonSchema))]
pub(crate) struct ILoc(pub(crate) i16, pub(crate) i16);
impl ILoc {
    pub(crate) fn try_as_loc(&self) -> Option<Loc> {
        if self.0 < 0 || self.1 < 0 {
            return None;
        }
        Some(Loc(self.0 as usize, self.1 as usize))
    }
}

#[test]
fn test_loc_notation() {
    // From notation
    assert_eq!(Loc::from_notation("a1"), Loc(0, 0));
    assert_eq!(Loc::from_notation("h8"), Loc(7, 7));
    assert_eq!(Loc::from_notation("e4"), Loc(4, 3));

    // To notation
    assert_eq!(Loc(0, 0).as_notation(), "a1");
    assert_eq!(Loc(7, 7).as_notation(), "h8");
    assert_eq!(Loc(4, 3).as_notation(), "e4");
}

impl From<Loc> for ILoc {
    fn from(loc: Loc) -> Self {
        Self(loc.0 as i16, loc.1 as i16)
    }
}
impl From<ILoc> for Loc {
    fn from(loc: ILoc) -> Self {
        Self(loc.0 as usize, loc.1 as usize)
    }
}

impl ops::Add for Loc {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl ops::AddAssign for Loc {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl ops::Sub for Loc {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl ops::SubAssign for Loc {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl ops::Add for ILoc {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl ops::AddAssign for ILoc {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl ops::Sub for ILoc {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl ops::SubAssign for ILoc {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

#[macro_export]
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { $crate::hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = $crate::hashmap!(@count $($key),*);
            let mut _map = rustc_hash::FxHashMap::with_capacity_and_hasher(_cap, Default::default());
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}
