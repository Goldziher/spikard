use magnus::Ruby;

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), magnus::Error> {
    spikard_rb_core::init(ruby)
}
