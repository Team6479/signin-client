#![allow(unused_must_use)]

use cursive::Cursive;
use cursive::views::{Dialog, TextView, EditView, DummyView, Button, LinearLayout};

mod util;
use util::{time, sess, user, traits::*};

fn main() {
    util::init();

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
            .on_submit(|s, id| {
                if user::is_actionable(&id) {
                    if sess::is_signed_in(&id) {
                        s.pop_layer();
                        s.add_layer(Dialog::around(TextView::new(format!("Users cannot sign in twice. Sign out?")))
                            .title("Already signed in")
                            .button("No", |s| {
                                s.pop_layer();
                            })
                            .button("Yes", |s| {
                                s.pop_layer();
                                signout_dialog(s);
                            }));
                    } else {
                        // note: this code will break if the user time travels before the Epoch
                        sess::mk_sess(&id, time::get_time());
                        s.pop_layer();
                        // TODO: name
                        s.add_layer(Dialog::around(TextView::new(format!("Welcome, {}", "name")))
                            .title("Successfully signed in")
                            .button("Ok", |s| {
                                s.pop_layer();
                            }));
                    }
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
            .on_submit(|s, id| {
                if user::is_actionable(&id) {
                    if sess::is_signed_in(&id) {
                        let completed = sess::Session {
                            id: id.to_owned(),
                            start: sess::rm_and_get_sess(id),
                            end: time::get_time(),
                        };
                        completed.cache(); // creates and caches a completed session
                        // TODO: name
                        s.add_layer(Dialog::around(TextView::new(format!("Goodbye, {}\n\n{}", "name", (completed.end - completed.start))))
                            .title("Successfully signed out")
                            .button("Ok", |s| {
                                s.pop_layer();
                            }));
                    } else {
                        s.pop_layer();
                        s.add_layer(Dialog::around(TextView::new(format!("Users cannot sign out before signing in. Sign in?")))
                            .title("Net signed in")
                            .button("No", |s| {
                                s.pop_layer();
                            })
                            .button("Yes", |s| {
                                s.pop_layer();
                                signin_dialog(s);
                            }));
                    }
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
