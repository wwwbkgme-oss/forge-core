//! Canonical WorldTick test vectors.

#[cfg(test)]
mod tests {
    use types::{WorldTick, DAY_LENGTH_TICKS};

    #[test]
    fn zero_is_zero() {
        assert_eq!(WorldTick::ZERO.0, 0);
    }

    #[test]
    fn advance_adds() {
        let t = WorldTick(100);
        assert_eq!(t.advance(50).0, 150);
        assert_eq!(t.advance(0).0, 100);
    }

    #[test]
    fn advance_wraps_on_overflow() {
        let t = WorldTick(u64::MAX);
        // advance uses wrapping arithmetic indirectly through u64 addition.
        // This just must not panic.
        let _ = t.advance(1);
    }

    #[test]
    fn elapsed_since_saturates() {
        let t = WorldTick(10);
        let earlier = WorldTick(20);
        // t < earlier → elapsed = 0 (saturating_sub)
        assert_eq!(t.elapsed_since(earlier), 0);
    }

    #[test]
    fn elapsed_since_normal() {
        let t = WorldTick(100);
        let earlier = WorldTick(40);
        assert_eq!(t.elapsed_since(earlier), 60);
    }

    #[test]
    fn from_into_u64() {
        let t: WorldTick = 42u64.into();
        let v: u64 = t.into();
        assert_eq!(v, 42);
    }

    #[test]
    fn ordering() {
        let a = WorldTick(1);
        let b = WorldTick(2);
        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, WorldTick(1));
    }

    #[test]
    fn day_length_constant() {
        // DAY_LENGTH_TICKS must be 2400 across the federation.
        assert_eq!(DAY_LENGTH_TICKS, 2400);
    }

    #[test]
    fn display_format() {
        assert_eq!(WorldTick(7).to_string(), "tick(7)");
    }
}
