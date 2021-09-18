use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};

#[derive(Debug, FromForm)]
struct UserForm<'r> {
    #[field(validate = len(1..))]
    first_name: &'r str,

    #[field(validate = len(1..))]
    second_name: &'r str,

    #[field(validate = len(1..))]
    level: &'r str,

    #[field(validate = len(1..))]
    dept: &'r str,
}