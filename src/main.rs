mod lib;

#[tokio::main]
async fn main() {
    dbg!(lib::get_course_status("20849", "202520").await.unwrap());
}
