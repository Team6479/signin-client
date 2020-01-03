#![allow(unused_must_use)]

use cursive::Cursive;
use cursive::views::{Dialog, TextView, EditView, Button, LinearLayout};

mod util;
use util::{time, sess, user, remote, traits::*};

fn main() {
    util::init();

    let mut tui = Cursive::default();

    tui.add_global_callback('.', admin_login);

    tui.add_layer(Dialog::around(LinearLayout::vertical()
            .child(Button::new("Sign In", signin_dialog))
            .child(Button::new("Sign Out", signout_dialog)))
        .title("Options"));
    
    tui.run();
}

fn signin_dialog(s: &mut Cursive) {
    s.add_layer(Dialog::around(EditView::new()
            .on_submit(|s, id| {
                if let Some(user) = user::get_user(&id) {
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
                        s.add_layer(Dialog::around(TextView::new(format!("Welcome, {}", user.name)))
                            .title("Successfully signed in")
                            .button("Ok", |s| {
                                s.pop_layer();
                            }));
                    }
                } else {
                    s.add_layer(Dialog::around(TextView::new("This user has not been created. Would you like to create one?"))
                        .title("User does not exist")
                        .button("No", |s| {
                            s.pop_layer();
                        })
                        .button("Yes", |s| {
                            s.pop_layer();
                            newuser_dialog(s);
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
                if let Some(user) = user::get_user(&id) {
                    if sess::is_signed_in(&id) {
                        let completed = sess::Session {
                            id: id.to_owned(),
                            start: sess::rm_and_get_sess(id),
                            end: time::get_time(),
                        };
                        completed.cache(); // creates and caches a completed session
                        s.pop_layer();
                        s.add_layer(Dialog::around(TextView::new(format!("Goodbye, {}\n\nTime elapsed: {}", user.name, time::format_time(completed.end - completed.start))))
                            .title("Successfully signed out")
                            .button("Ok", |s| {
                                s.pop_layer();
                            }));
                    } else {
                        s.pop_layer();
                        s.add_layer(Dialog::around(TextView::new(format!("Users cannot sign out before signing in. Sign in?")))
                            .title("Not signed in")
                            .button("No", |s| {
                                s.pop_layer();
                            })
                            .button("Yes", |s| {
                                s.pop_layer();
                                s.pop_layer();
                                signin_dialog(s);
                            }));
                    }
                } else {
                    s.add_layer(Dialog::around(TextView::new("This user has not been created. Would you like to create one?"))
                        .title("User does not exist")
                        .button("No", |s| {
                            s.pop_layer();
                        })
                        .button("Yes", |s| {
                            s.pop_layer();
                            newuser_dialog(s);
                            signin_dialog(s);
                        }));
                }
            }))
        .title("Enter or scan your ID to sign out")
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn newuser_dialog(s: &mut Cursive) {
    s.add_layer(Dialog::around(EditView::new() // id
            .on_submit(|s, id| {
                s.pop_layer();
                pt2_newuser_dialog(s, &id);
            }))
        .title("Enter or scan your ID.")
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}
fn pt2_newuser_dialog(s: &mut Cursive, id: &str) {
    let id = id.clone().to_owned();
    s.add_layer(Dialog::around(EditView::new() // name
            .on_submit(move |s, name| {
                let req = user::User {
                    id: id.to_string(),
                    name: name.to_string(),
                    lvl: 1, // all users are lvl 1 for now
                };
            
                match user::is_creatable(&req) {
                    user::Creatability::Unobstructed => {
                        s.pop_layer();
                        req.cache();
                        s.add_layer(Dialog::around(TextView::new(format!("User \"{}\" with id {} has been created.\nDon't forget to sign in again if desired.", name, id)))
                                .title("Successfully created user")
                                .button("Ok", |s| {
                                    s.pop_layer();
                                }));
                    },
                    user::Creatability::Privileged => {
                        s.add_layer(Dialog::around(TextView::new("This doesn't look like a valid CdS ID.\nIf this is your ID, talk to an officer for help."))
                                .title("Invalid format")
                                .button("Ok", |s| {
                                    s.pop_layer();
                                }));
                    },
                    user::Creatability::Impossible => {
                        s.add_layer(Dialog::around(TextView::new("The format of your ID and/or name is not allowed.\nGenerally, this is due to commas.\nPlease fix and re-enter, or talk to an officer for help."))
                                .title("Disallowed data")
                                .button("Ok", |s| {
                                    s.pop_layer();
                                }));
                    },
                }
            }))
        .title("Enter your name.")
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn portal_login(s: &mut Cursive) {
    // TODO: implement this
    s.add_layer(Dialog::around(TextView::new("This feature is not currenly implemented, but is coming soon.\nSorry for any inconvenience."))
        .title("Not currently implemented")
        .button("Ok", |s| {
            s.pop_layer();
        }));
}

fn admin_login(s: &mut Cursive) {
    let mut status = remote::get_status();
    match status {
        remote::InternetStatus::Portal => {
            s.add_layer(Dialog::around(TextView::new("The internet connection is blocked by a portal."))
                .title("Portal")
                .button("Continue offline", |s| {
                    s.pop_layer();
                })
                .button("Login", |s| {
                    s.pop_layer();
                    portal_login(s);
                }));
                status = remote::get_status();
        },
        _ => (),
    }
    s.add_layer(Dialog::around(EditView::new()
            .on_submit(move |s, passwd| {
                let auth = remote::auth("default", passwd, &status);
                if let Ok(is_correct) = auth {
                    if is_correct {
                        admin_zone(s, "default");
                    } else {
                        s.add_layer(Dialog::around(TextView::new("Please try again")) // TODO: login attempts
                            .title("Incorrect password")
                            .button("Ok", |s| {
                                s.pop_layer();
                            }));
                    }
                } else {
                    s.add_layer(Dialog::around(TextView::new("You must login at least once with an internet connection."))
                        .title("No login cache")
                        .button("Ok", |s| {
                            s.pop_layer();
                            s.pop_layer();
                        }));
                }
            }))
        .title("Enter your administrator password")
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn admin_zone(s: &mut Cursive, usr: &str) {
    // TODO: do something useful
}