pub mod arrays {
    use std::{
        ops::{
            Shl,
            ShlAssign,
            SubAssign,
            Index,
        },
        slice::Iter,
    };

    use super::super::airport::Airport;

    pub struct Airports(Vec<Airport>);

    impl Airports {
        pub fn new() -> Airports {
            Airports(Vec::new())
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn iter(&self) -> Iter<Airport> {
            self.0.iter()
        }
    }

    impl Shl<Airport> for &mut Airports {
        type Output = Self;

        fn shl(self, rhs: Airport) -> Self::Output {
            self.0.push(rhs);
            self
        }
    }

    impl ShlAssign<Airport> for Airports {
        fn shl_assign(&mut self, rhs: Airport) {
            self.0.push(rhs);
        }
    }

    impl Index<usize> for Airports {
        type Output = Airport;

        fn index(&self, index: usize) -> &Self::Output {
            &self.0[index]
        }
    }

    impl SubAssign<i64> for Airports {
        fn sub_assign(&mut self, rhs: i64) {
            let i = self.0.iter().position(|a| a.id == rhs);
            match i {
                Some(i) => _ = self.0.remove(i),
                None => (),
            };
        }
    }
}