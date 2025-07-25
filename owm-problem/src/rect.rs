use std::{cmp::PartialOrd, num::NonZeroUsize};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rect {
    pub pos: Pos,
    pub size: Size,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: NonZeroUsize,
    pub height: NonZeroUsize,
}

impl Rect {
    pub fn new(x: usize, y: usize, width: NonZeroUsize, height: NonZeroUsize) -> Self {
        Self {
            pos: Pos::new(x, y),
            size: Size::new(width, height),
        }
    }

    pub fn new_checked(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            pos: Pos::new(x, y),
            size: Size::new_checked(width, height),
        }
    }

    pub fn x(&self) -> usize {
        self.pos.x
    }

    pub fn y(&self) -> usize {
        self.pos.y
    }

    pub fn width(&self) -> NonZeroUsize {
        self.size.width
    }

    pub fn height(&self) -> NonZeroUsize {
        self.size.height
    }

    pub fn left(&self) -> usize {
        self.pos.x
    }

    pub fn right(&self) -> usize {
        self.pos.x + self.size.width.get()
    }

    pub fn top(&self) -> usize {
        self.pos.y
    }

    pub fn bottom(&self) -> usize {
        self.pos.y + self.size.height.get()
    }

    pub fn center(&self) -> Pos {
        Pos::new(self.center_x(), self.center_y())
    }

    pub fn center_x(&self) -> usize {
        self.left() + self.size.width.get() / 2
    }

    pub fn center_y(&self) -> usize {
        self.top() + self.size.height.get() / 2
    }

    pub fn top_left(&self) -> Pos {
        Pos {
            y: self.top(),
            x: self.left(),
        }
    }

    pub fn top_right(&self) -> Pos {
        Pos {
            y: self.top(),
            x: self.right(),
        }
    }

    pub fn bottom_left(&self) -> Pos {
        Pos {
            y: self.bottom(),
            x: self.left(),
        }
    }

    pub fn bottom_right(&self) -> Pos {
        Pos {
            y: self.bottom(),
            x: self.right(),
        }
    }

    pub fn expand_left(&mut self, value: usize) {
        self.pos.x -= value;
        // Values should be small enough,
        // we do not expect overflow.
        self.size.width = unsafe { NonZeroUsize::new_unchecked(self.size.width.get() + value) };
    }

    pub fn expand_right(&mut self, value: usize) {
        // Values should be small enough,
        // we do not expect overflow.
        self.size.width = unsafe { NonZeroUsize::new_unchecked(self.size.width.get() + value) };
    }

    pub fn expand_top(&mut self, value: usize) {
        self.pos.y -= value;
        // Values should be small enough,
        // we do not expect overflow.
        self.size.height = unsafe { NonZeroUsize::new_unchecked(self.size.height.get() + value) };
    }

    pub fn expand_bottom(&mut self, value: usize) {
        // Values should be small enough,
        // we do not expect overflow.
        self.size.height = unsafe { NonZeroUsize::new_unchecked(self.size.height.get() + value) };
    }

    pub fn x_range_exclusive(&self) -> RangeExclusive<usize> {
        RangeExclusive(self.left(), self.right())
    }

    pub fn y_range_exclusive(&self) -> RangeExclusive<usize> {
        RangeExclusive(self.top(), self.bottom())
    }

    pub fn area(&self) -> NonZeroUsize {
        self.size.area()
    }

    pub fn overlap(&self, other: &Rect) -> Option<Rect> {
        let left = self.left().max(other.left());
        let right = self.right().min(other.right());
        let top = self.top().max(other.top());
        let bottom = self.bottom().min(other.bottom());

        if left < right && top < bottom {
            Some(Rect {
                pos: Pos { x: left, y: top },
                // We already checked
                // these values are not equal.
                size: Size {
                    width: unsafe { NonZeroUsize::new_unchecked(right - left) },
                    height: unsafe { NonZeroUsize::new_unchecked(bottom - top) },
                },
            })
        } else {
            None
        }
    }

    pub fn diff(self, other: Self) -> usize {
        self.pos.dist(other.pos) + self.size.diff(other.size)
    }
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    /// Return manhattan distance between positions.
    pub fn dist(self, other: Pos) -> usize {
        (if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        }) + (if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        })
    }
}

impl Size {
    pub fn new(width: NonZeroUsize, height: NonZeroUsize) -> Self {
        Self { width, height }
    }

    pub fn new_checked(width: usize, height: usize) -> Self {
        Self {
            width: width.try_into().unwrap(),
            height: height.try_into().unwrap(),
        }
    }

    pub fn area(&self) -> NonZeroUsize {
        // Area cannot be zero
        // if dimensions are not.
        unsafe { NonZeroUsize::new_unchecked(self.width.get() * self.height.get()) }
    }

    pub fn diff(self, other: Size) -> usize {
        Pos::from(self).dist(other.into())
    }
}

impl From<Size> for Pos {
    fn from(value: Size) -> Self {
        Pos {
            x: value.width.get(),
            y: value.height.get(),
        }
    }
}

// Adapted from a solution by `m-hgn` on Code Wars,
// <https://www.codewars.com/kata/reviews/6380bc55c34ac10001dde712/groups/63b6d7c8ec0d060001ce20f1>.
// This could be optimized using segment trees.
/// Return the total area of a union of rectangles.
pub fn covered_area(rects: &[Rect]) -> usize {
    let mut xs = rects
        .iter()
        .flat_map(|rect| [rect.left(), rect.right()])
        .collect_vec();
    xs.sort();
    xs.dedup();

    let mut rects = rects.to_vec();
    rects.sort_by_key(|rect| rect.top());

    xs.into_iter()
        .tuple_windows()
        .map(|(left, right)| {
            let width = right - left;
            let mut last_y2 = usize::MIN;
            rects
                .iter()
                .filter(|rect| rect.left() <= left && right <= rect.right())
                .map(|rect| {
                    let ret = width * rect.bottom().saturating_sub(last_y2.max(rect.top()));
                    last_y2 = rect.bottom().max(last_y2);
                    ret
                })
                .sum::<usize>()
        })
        .sum()
}

/// Return the total area obscured in a set of rectangles.
/// If `n` rectangles are overlapped by an `n + 1`th rectangle,
/// the overlapped area will be counted `n` times,
/// but not `n + 1` times.
pub fn obscured_area(rects: &[Rect]) -> usize {
    if rects.len() < 2 {
        0
    } else {
        let overlaps = rects
            .iter()
            .enumerate()
            .map(|(i, rect)| {
                rects
                    .iter()
                    .enumerate()
                    .filter(|(other_i, _)| i != *other_i)
                    .filter_map(|(_, other)| rect.overlap(other))
                    .collect_vec()
            })
            .collect_vec();
        overlaps.iter().map(|x| covered_area(x)).sum::<usize>()
            - covered_area(&overlaps.into_iter().flatten().collect_vec())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeExclusive<T>(pub T, pub T);

impl<T> RangeExclusive<T> {
    pub fn intersects(self, other: RangeExclusive<T>) -> bool
    where
        T: Copy + PartialOrd,
    {
        self == other || self.contains_either(other) || other.contains_either(self)
    }

    fn contains_either(self, other: RangeExclusive<T>) -> bool
    where
        T: Copy + PartialOrd,
    {
        self.contains(other.0) || self.contains(other.1)
    }

    pub fn contains(self, x: T) -> bool
    where
        T: Copy + PartialOrd,
    {
        x > self.0 && x < self.1
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use test_strategy::proptest;

    use crate::testing::{ContainedRects, ContainedRectsParams};

    use super::*;

    #[test]
    fn covered_area_is_zero_if_no_rects() {
        assert!(covered_area(&[]) == 0)
    }

    #[proptest]
    fn covered_area_is_non_zero_if_rects(
        #[strategy(ContainedRects::arbitrary_with(ContainedRectsParams::from_len_range(1..=16)))]
        rects: ContainedRects,
    ) {
        prop_assert!(covered_area(&rects.rects) > 0)
    }

    #[test]
    fn range_exclusive_intersects_works_for_simple_cases() {
        assert!(RangeExclusive(0, 2).intersects(RangeExclusive(1, 2)));
        assert!(RangeExclusive(0, 3).intersects(RangeExclusive(1, 2)));
        assert!(!RangeExclusive(0, 1).intersects(RangeExclusive(1, 2)));
    }

    #[proptest]
    fn range_exclusive_intersects_with_itself(x: RangeExclusive<usize>) {
        prop_assert!(x.intersects(x));
    }

    #[proptest]
    fn range_exclusive_intersects_is_symmetrical(
        x: RangeExclusive<usize>,
        y: RangeExclusive<usize>,
    ) {
        prop_assert_eq!(x.intersects(y), y.intersects(x));
    }
}
