extern crate cursive;

use cursive::traits::Identifiable;
use cursive::views::{Checkbox, Dialog, EditView, ListView, TextView};
use cursive::Cursive;

struct CatSayOptions<'a> {
    message: &'a str,
    dead: bool,
}

fn input_step(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill the form for displaying the cat")
            .content(
                ListView::new()
                    .child("Message:", EditView::new().with_name("message"))
                    .child("Dead:", Checkbox::new().with_name("dead")),
            )
            .button("OK", |s| {
                let message = s
                    .call_on_name("message", |t: &mut EditView| t.get_content())
                    .unwrap();
                let is_dead = s
                    .call_on_name("dead", |t: &mut Checkbox| t.is_checked())
                    .unwrap();
                let options = CatSayOptions {
                    message: &message,
                    dead: is_dead,
                };
                result_step(s, &options);
            }),
    )
}

fn result_step(siv: &mut Cursive, options: &CatSayOptions) {
    let eye = if options.dead { "x" } else { "o" };
    let cat_text = format!(
        "{msg}
\\ \\
/\\_/\\
( {eye} {eye} ) 
    =( I )=    ",
        msg = options.message,
        eye = eye
    );
    siv.pop_layer(); // [3] siv.add_layer( // [4]
    siv.add_layer(
        Dialog::around(TextView::new(cat_text))
            .title("The cat says...")
            .button("OK", |s| s.quit()),
    );
}

fn main() {
    let mut siv = Cursive::default();
    input_step(&mut siv);
    siv.run();
}
