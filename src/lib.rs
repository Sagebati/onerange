mod iter;

use std::cmp::Ordering;
use std::collections::Bound;
use std::ops::{RangeBounds};

#[derive(Copy, Clone, Debug)]
struct OneRange<T, Step = ()> {
    start: T,
    end: T,
    step: Step,
}

trait MinMax {
    fn max(_: Self) -> Self;
    fn min(_: Self) -> Self;
}

macro_rules! range {
    ($start: expr, =$end: expr, 1) => {
        OneRange {
            start: $start,
            end: $end,
            step: (),
        }
    };

    ($start: expr, =$end: expr, $step:expr) => {
        OneRange {
            start: $start,
            end: $end,
            step: $step,
        }
    };

    ($start: expr, =$end: expr) => {
        range!($start, $end, 1)
    };

    ($start :expr, $end: expr) => {
        range!($start, $end, 1)
    };

    ($start: expr, $end: expr, $step: expr) => {
        range!($start, =$end-1, $step)
    };

    (..$end: expr) => {
        range!(MinMax::min($end), $end, 1)
    };

    (..$end: expr, $step:expr) => {
        range!(MinMax::min($end), $end, $step)
    };

    (..=$end: expr, $step:expr) => {
        range!(MinMax::min($end), $end, $step)
    };

    (..=$end: expr) => {
        range!(MinMax::min($end), =$end, 1)
    };
}

macro_rules! into_iter {
    ($($t:ty),+) => {
        $(
        impl OneRange<$t, ()> {
            fn iter(&self) -> impl Iterator<Item=$t> {
                self.start..self.end
            }
        }

        impl OneRange<$t, usize> {
            fn iter(&self) -> impl Iterator<Item=$t> {
                (self.start..self.end).step_by(self.step)
            }
        }

        impl IntoIterator for OneRange<$t> {
            type Item=$t;
            type IntoIter=std::ops::RangeInclusive<$t>;

            fn into_iter(self) -> Self::IntoIter {
                self.start..=self.end
            }
        }

        impl IntoIterator for OneRange<$t, usize> {
            type Item=$t;
            type IntoIter=std::iter::StepBy<std::ops::RangeInclusive<$t>>;

            fn into_iter(self) -> Self::IntoIter {
                (self.start..=self.end).step_by(self.step)
            }
        }

        impl MinMax for $t {
            fn max(_:Self) -> Self {
                Self::MAX
            }
            fn min(_:Self) -> Self {
                Self::MIN
            }
        }
        )+
    };
}

into_iter!(u8,u16,u32,u64,u128,i8,i16,i32,i64,i128);

impl<T> RangeBounds<T> for OneRange<T> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&T> {
        Bound::Included(&self.end)
    }

    fn contains<U>(&self, item: &U) -> bool where T: PartialOrd<U>, U: ?Sized + PartialOrd<T> {
        matches!((self.start.partial_cmp(item), item.partial_cmp(&self.end)), (Some(Ordering::Equal | Ordering::Less), Some(Ordering::Equal | Ordering::Less)))
    }
}

impl<T> RangeBounds<T> for OneRange<T, usize> {
    fn start_bound(&self) -> Bound<&T> {
        Bound::Included(&self.start)
    }

    fn end_bound(&self) -> Bound<&T> {
        Bound::Included(&self.end)
    }

    fn contains<U>(&self, item: &U) -> bool where T: PartialOrd<U>, U: ?Sized + PartialOrd<T> {
        matches!((self.start.partial_cmp(item), item.partial_cmp(&self.end)), (Some(Ordering::Equal | Ordering::Less), Some(Ordering::Equal | Ordering::Less)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_inclusive() {
        let r = range!(0, =255u8);
        assert!(r.contains(&3))
    }

    #[test]
    fn range() {
        let r = range!(0, 100);
        assert!(r.contains(&3))
    }

    #[test]
    fn range_open() {
        let r = range!(..123);
        assert!(r.contains(&3));
        let r = range!(..=5u8);
        assert_eq!(r.into_iter().collect::<Vec<_>>(), [0, 1, 2, 3, 4, 5]);
    }
}
