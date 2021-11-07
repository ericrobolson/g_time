pub(crate) type InnerDuration = std::time::Duration;

/// Struct representing a Duration.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Duration(InnerDuration);

impl Duration {
    pub(crate) fn from_inner(inner: InnerDuration) -> Self {
        Self(inner)
    }

    /// Creates the duration from seconds.
    pub fn from_secs(secs: u64) -> Self {
        Self(InnerDuration::from_secs(secs))
    }

    /// Returns the given duration in seconds.
    pub fn as_secs_f32(&self) -> f32 {
        self.0.as_secs_f32()
    }
}

impl std::ops::AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Div<u32> for Duration {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        let d = if rhs == 0 {
            InnerDuration::MAX
        } else {
            self.0 / rhs
        };

        Self(d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_assign() {
        let mut duration = Duration::from_secs(1);

        duration += Duration::from_secs(3);

        assert_eq!(InnerDuration::from_secs(4), duration.0);
    }

    #[test]
    fn div_by_zero_returns_max() {
        let duration = Duration::from_secs(1) / 0;

        assert_eq!(InnerDuration::MAX, duration.0);
    }

    #[test]
    fn div_returns_divided() {
        let duration = Duration::from_secs(1) / 2;

        assert_eq!(InnerDuration::from_secs(1) / 2, duration.0);
    }
}
