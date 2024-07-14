#[inline(always)]
pub fn partial_max<T: PartialOrd>(left: T, right: T) -> T {
    if left > right {
        left
    } else {
        right
    }
}

pub fn partial_max_iter<T: PartialOrd>(mut iter: impl Iterator<Item = T>) -> Option<T> {
    let mut max = iter.next()?;
    while let Some(next) = iter.next() {
        if next > max {
            max = next;
        }
    }
    Some(max)
}

pub fn partial_min_iter<T: PartialOrd>(mut iter: impl Iterator<Item = T>) -> Option<T> {
    let mut max = iter.next()?;
    while let Some(next) = iter.next() {
        if next < max {
            max = next;
        }
    }
    Some(max)
}

#[inline(always)]
pub fn partial_min<T: PartialOrd>(left: T, right: T) -> T {
    if left < right {
        left
    } else {
        right
    }
}

#[macro_export]
macro_rules! max_many {
    ($left:expr, $right: expr) => {
        $crate::partial_max($left, $right)
    };
    ($first:expr, $second: expr, $third: expr, $fourth:expr) => {
        $crate::partial_max($crate::partial_max($first, $second), $crate::partial_max($third, $fourth))
    };
    ($first:expr, $($others:expr),*) => {
        $crate::partial_max($first, max_many!($($others),*))
    };
}

#[macro_export]
macro_rules! min_many {
    ($left:expr, $right: expr) => {
        $crate::partial_min($left, $right)
    };
    ($first:expr, $second: expr, $third: expr, $fourth:expr) => {
        $crate::partial_min($crate::partial_min($first, $second), $crate::partial_min($third, $fourth))
    };
    ($first:expr, $($others:expr),*) => {
        $crate::partial_min($first, min_many!($($others),*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(max_many!(1, 2), 2);
        assert_eq!(max_many!(1, 2, 3), 3);
        assert_eq!(max_many!(4, 1, 2, 3), 4);
        assert_eq!(max_many!(4, 1, 5, 2, 3), 5);
        assert_eq!(max_many!(4, 1, 5, 2, 3, 6), 6);

        assert_eq!(min_many!(1, 2), 1);
        assert_eq!(min_many!(1, 2, 3), 1);
        assert_eq!(min_many!(4, 1, 2, 3), 1);
        assert_eq!(min_many!(4, 1, 5, 2, 3), 1);
        assert_eq!(min_many!(4, 1, 5, 2, 3, 6), 1);
    }

    #[test]
    fn even_with_floats() {
        assert_eq!(max_many!(1.0, 2.5), 2.5);
        assert_eq!(max_many!(1.0, 2.0, 3.0), 3.0);
        assert_eq!(max_many!(4.0, 1.0, 2.0, 3.0), 4.0);
        assert_eq!(max_many!(4.0, 1.0, 5.33, 2.0, 3.0), 5.33);
        assert_eq!(max_many!(4.0, 1.0, 5.0, 2.0, 3.0, 6.0), 6.0);

        assert_eq!(min_many!(1.0, 2.0), 1.0);
        assert_eq!(min_many!(1.0, 2.0, 3.0), 1.0);
        assert_eq!(min_many!(4.0, 1.0, 2.0, 3.0), 1.0);
        assert_eq!(min_many!(4.0, 1.0, 5.0, 2.0, 3.0), 1.0);
        assert_eq!(min_many!(4.0, 1.0, 5.0, 2.0, 3.0, 6.0), 1.0);
    }
}
