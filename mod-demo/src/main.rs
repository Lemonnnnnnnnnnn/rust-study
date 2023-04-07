mod entity;
mod student;

use entity::farmer;

fn main() {
    student::say();
    farmer::say();
    entity::teacher::call_student();
}