use std::{
    cmp::{
        PartialEq,
        PartialOrd,
    },
    ops::{
        Add,
        Shl,
        Index,
        Range,
    },
};

struct Meters(i32);

impl Add<Meters> for Meters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Meters(self.0 + rhs.0)
    }
}

impl Add<i32> for Meters {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Meters(self.0 + rhs)
    }
}

impl PartialEq<i32> for Meters {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }

    fn ne(&self, other: &i32) -> bool {
        self.0 != *other
    }
}

impl PartialOrd<i32> for Meters {
    fn ge(&self, other: &i32) -> bool {
        self.0 >= *other
    }

    fn gt(&self, other: &i32) -> bool {
        self.0 > *other
    }

    fn le(&self, other: &i32) -> bool {
        self.0 <= *other
    }

    fn lt(&self, other: &i32) -> bool {
        self.0 < *other
    }

    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        match *other {
            x if self.0 < x => Some(std::cmp::Ordering::Less),
            x if self.eq(&x) => Some(std::cmp::Ordering::Equal),
            _ => Some(std::cmp::Ordering::Greater),
        }
    }
}

impl PartialEq<Meters> for Meters {
    fn eq(&self, other: &Meters) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Meters) -> bool {
        self.0 != other.0
    }
}

impl PartialOrd<Meters> for Meters {
    fn ge(&self, other: &Meters) -> bool {
        self.0 >= other.0
    }

    fn gt(&self, other: &Meters) -> bool {
        self.0 > other.0
    }

    fn le(&self, other: &Meters) -> bool {
        self.0 <= other.0
    }

    fn lt(&self, other: &Meters) -> bool {
        self.0 < other.0
    }

    fn partial_cmp(&self, other: &Meters) -> Option<std::cmp::Ordering> {
        match other.0 {
            val if self.0 < val => Some(std::cmp::Ordering::Less),
            val if self.0 == val => Some(std::cmp::Ordering::Equal),
            val if self.0 > val => Some(std::cmp::Ordering::Greater),
            _ => None,
        }
    }
}

struct Items(Vec<i32>);

impl Shl<i32> for Items {
    type Output = Self;

    fn shl(mut self, rhs: i32) -> Self::Output {
        self.0.push(rhs);
        self
    }
}

impl Index<Range<usize>> for Items {
    type Output = [i32];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl Index<usize> for Items {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
pub mod testing_wrappers {
    use super::Items;

    #[test]
    fn test_items() {
        let mut items = Items(vec![]);
        items = items << 1 << 2 << 3;
        println!("{:?}", items.0.clone());
        let mut q: String = items[0..2].iter()
            .enumerate()
            .map(|(i, v)| {
                let mut s = v.to_string();
                if i > 0 {
                    s.insert_str(0, ", ");
                }
                s
            }).collect();
        println!("[{}]", q);
        q = items[1..3].iter()
            .enumerate()
            .map(|(i, v)| {
                let mut s = v.to_string();
                if i > 0 {
                    s.insert_str(0, ", ");
                }
                s
            }).collect();
        println!("[{}]", q);
    }
}
