use std::ops::{
    Add,
    Shl,
    Index,
    Range,
};

struct Meters(i32);

impl Add for Meters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Meters(self.0 + rhs.0)
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
