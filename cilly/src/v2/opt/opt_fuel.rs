#[derive(Clone, Eq, PartialEq, Debug)]
pub struct OptFuel(u32);
impl OptFuel {
    /// Returns a fraction of the current fuel level. Fraction must be in range `0.0..1.0``.
    #[must_use]
    pub fn fraction(self, fraction: f32) -> Self {
        debug_assert!(
            0.0 < fraction && fraction < 1.0,
            "Invalid fraction, not in rage 0.0..1.0"
        );
        let inner = self.0;
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let scale_div = (1.0 / fraction) as u32;
        Self(inner / scale_div)
    }
    /// Creates *fuel* fuel
    #[must_use]
    pub fn new(fuel: u32) -> Self {
        Self(fuel)
    }
    /// Decreases the ammount of fuel avalible if fuel present, and returns false if not enough fuel present.
    pub fn consume(&mut self, cost: u32) -> bool {
        if self.0 >= cost {
            self.0 -= cost;
            true
        } else {
            false
        }
    }
    /// Checks if no fuel remains
    #[must_use]
    pub fn exchausted(&self) -> bool {
        self.0 == 0
    }
}
#[test]
fn opt_fuel() {
    let mut fuel = OptFuel::new(64);
    assert!(!fuel.exchausted());
    assert!(fuel.consume(64));
    assert!(fuel.exchausted());
    assert_eq!(fuel.0, 0);
    let mut fuel = OptFuel::new(64);
    assert!(!fuel.exchausted());
    assert!(!fuel.consume(128));
    assert!(fuel.consume(64));
    let mut fuel = OptFuel::new(64);
    assert!(fuel.consume(32));
    let mut fuel = fuel.fraction(0.5);
    assert!(fuel.consume(16));
    assert!(fuel.exchausted());
    assert_eq!(fuel.0, 0);
}
#[test]
fn consume() {
    let mut fuel = OptFuel::new(64);
    fuel.consume(1);
    assert_eq!(fuel.0, 63);
}
