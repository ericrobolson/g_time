use crate::Duration;

type InnerInstant = std::time::Instant;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Instant(InnerInstant);

impl Instant {
    pub fn now() -> Self {
        Self(InnerInstant::now())
    }
}

impl std::ops::Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        let dur = self.0 - rhs.0;

        Duration::from_inner(dur)
    }
}
