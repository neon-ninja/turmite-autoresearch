//! Highway detection from the ant's recent trajectory.
//!
//! A highway is a stretch where the motion is periodic with a non-zero drift:
//! for some period p and displacement d != 0, pos[t] - pos[t-p] == d for every
//! t in the window. Because a highway ant walks into fresh (colour 0) cells,
//! once it starts it never stops, so a long consistent tail is decisive.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Highway {
    pub period: u32,
    pub dx: i32,
    pub dy: i32,
}

/// Minimum stretch of consistent periodic motion before we call it a highway.
pub const MIN_SPAN: usize = 20_000;
pub const MIN_REPS: usize = 4;

/// `tail` holds the most recent positions, oldest first. Returns the smallest
/// period that explains the tail.
pub fn find_highway(tail: &[(i32, i32)], max_period: usize, min_span: usize) -> Option<Highway> {
    let last = tail.len().checked_sub(1)?;
    for p in 1..=max_period {
        let span = (MIN_REPS * p).max(min_span);
        if span + p > tail.len() {
            break;
        }
        let d = sub(tail[last], tail[last - p]);
        if d == (0, 0) {
            continue;
        }
        if (last - span..=last).rev().all(|t| sub(tail[t], tail[t - p]) == d) {
            return Some(Highway {
                period: p as u32,
                dx: d.0,
                dy: d.1,
            });
        }
    }
    None
}

fn sub(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
}

/// A highway leaves the grid in the direction it travels. Something walking
/// periodically along the grid boundary steps out sideways (perpendicular to
/// its drift), so that exit does not confirm a highway.
pub fn exit_confirms(hw: Highway, exit_heading: u8) -> bool {
    let (ex, ey) = crate::DIRS[exit_heading as usize];
    hw.dx * ex + hw.dy * ey > 0
}

#[cfg(test)]
mod tests {
    use super::*;
    fn hw(dx: i32, dy: i32) -> Highway {
        Highway { period: 24, dx, dy }
    }
    #[test]
    fn exit_direction() {
        // Headings: 0 N (0,-1), 1 E, 2 S, 3 W.
        assert!(exit_confirms(hw(2, 2), 1)); // diagonal SE leaving east
        assert!(exit_confirms(hw(2, 2), 2)); // ...or south
        assert!(!exit_confirms(hw(2, 2), 0)); // never north
        assert!(!exit_confirms(hw(4, 0), 0)); // walking east along top edge, stepping north
        assert!(exit_confirms(hw(0, -4), 0));
        assert!(!exit_confirms(hw(0, -4), 2));
    }
}
