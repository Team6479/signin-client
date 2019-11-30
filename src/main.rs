#![allow(unused_must_use)]

use cursive::Cursive;
use cursive::views::{Dialog, TextView, EditView, DummyView, Button, LinearLayout};
use cursive::traits::*;

use regex::Regex;
use chrono::{offset, Datelike};

mod cache;

fn main() {
    let mut tui = Cursive::default();

    tui.add_layer(Dialog::around(LinearLayout::vertical()
            .child(Button::new("Sign In", signin_dialog))
            .child(Button::new("Sign Out", signout_dialog))
            .child(DummyView)
            .child(Button::new("Admin", |s| s.quit())))
        .title("Options"));
    
    tui.run();
}

fn signin_dialog(s: &mut Cursive) {
    s.add_layer(Dialog::around(EditView::new()
            .on_submit(|s, text| {
                if validate_id(&text) {
                    // TODO: do something with the ID
                } else {
                    s.add_layer(Dialog::around(TextView::new("Please use your student ID number."))
                        .title("Invalid ID")
                        .button("Ok", |s| {
                            s.pop_layer();
                        }));
                }
            }))
        .title("Enter or scan your ID to sign in")
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn signout_dialog(s: &mut Cursive) {
    s.add_layer(Dialog::around(EditView::new()
            .on_submit(|s, text| {
                if validate_id(&text) {
                    // TODO: do something with the ID
                } else {
                    s.add_layer(Dialog::around(TextView::new("Please use your student ID number."))
                        .title("Invalid ID")
                        .button("Ok", |s| {
                            s.pop_layer();
                        }));
                }
            }))
        .title("Enter or scan your ID to sign out")
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn validate_id(id: &str) -> bool {
    let current_yy= offset::Local::today().year() % 100; // 19, if the current year is 2019
    let min_yy = current_yy - 1; // allows superseniors in second semester
    let max_yy = current_yy + 4; // allows freshman in first semester
    let grad_yr_regex = if min_yy / 10 == max_yy / 10 { // same decade
        format!("{}[{}-{}]", (min_yy / 10), (min_yy % 10), (max_yy % 10))
    } else { // different decades
        format!("(?:{}[{}-9])|(?:{}[0-{}])", (min_yy / 10), (min_yy % 10), (max_yy / 10), (max_yy % 10))
    };
    let mid_regex = "[0-9]{3}"; // TODO: figure out what's valid here (I've been told that it's usually 400)
    let end_regex = "[0-9]{3}"; // these numbers appear to be random
    let re = Regex::new(&format!("{}{}{}", &grad_yr_regex, &mid_regex, &end_regex)).unwrap();
    re.is_match(id)
}
