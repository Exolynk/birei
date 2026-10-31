/// Steps from `current` in `direction` to the next enabled entry, wrapping around the list.
///
/// Without a current entry, stepping forward starts at the first entry and backward at the
/// last. Returns `None` for an empty list or when no entry is enabled.
pub(crate) fn step_enabled_index(
    len: usize,
    current: Option<usize>,
    direction: i32,
    is_enabled: impl Fn(usize) -> bool,
) -> Option<usize> {
    let len = i64::try_from(len).ok().filter(|len| *len > 0)?;
    let direction = i64::from(direction);
    let mut index = current
        .and_then(|index| i64::try_from(index).ok())
        .unwrap_or(if direction >= 0 { -1 } else { len });

    for _ in 0..len {
        index = (index + direction).rem_euclid(len);
        let candidate = usize::try_from(index).ok()?;
        if is_enabled(candidate) {
            return Some(candidate);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::step_enabled_index;

    /// Steps skip disabled entries and wrap at both ends.
    #[test]
    fn skips_disabled_and_wraps() {
        let enabled = [true, false, true];
        let is_enabled = |index: usize| enabled[index];
        assert_eq!(step_enabled_index(3, None, 1, is_enabled), Some(0));
        assert_eq!(step_enabled_index(3, None, -1, is_enabled), Some(2));
        assert_eq!(step_enabled_index(3, Some(0), 1, is_enabled), Some(2));
        assert_eq!(step_enabled_index(3, Some(2), 1, is_enabled), Some(0));
        assert_eq!(step_enabled_index(3, Some(0), -1, is_enabled), Some(2));
    }

    /// Empty or fully disabled lists have no next entry.
    #[test]
    fn empty_or_disabled_lists_have_no_entry() {
        assert_eq!(step_enabled_index(0, None, 1, |_| true), None);
        assert_eq!(step_enabled_index(2, Some(0), 1, |_| false), None);
    }
}
