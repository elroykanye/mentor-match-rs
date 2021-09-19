use rocket::form::{Form, Contextual, Submit};

#[post("/mentor/new", data = "<newMentorForm>")]
pub fn new_mentor(newMentorForm: Form<Contextual<Submit>>) -> bool {

    true
}

#[get("/mentor/all")]
pub fn get_all_mentors() {

}