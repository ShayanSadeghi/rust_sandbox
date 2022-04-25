extern crate cursive;

use cursive::views::TextView;

fn main() {
    let mut siv = cursive::default();

    let cat_text = "Meow!
  \\
   \\
    /\\__/\\
    ( o o )
    =( I )=";

    siv.add_layer(TextView::new(cat_text));
    siv.run();
}
