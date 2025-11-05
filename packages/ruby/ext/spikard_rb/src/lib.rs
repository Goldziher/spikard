use magnus::Ruby;

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), magnus::Error> {
    spikard_rb::init(ruby)
}
