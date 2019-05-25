use cursive::views::TextView;
use cursive::Cursive;

fn main() {
    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(TextView::new("Hello cursive"));
    siv.run();
}
