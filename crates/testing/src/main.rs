use dom::{traits::*, DOMResult, Document, HTMLDivElement};

fn main() {
    if let Err(exception) = instance() {
        print!("{}", exception)
    }
}

fn instance() -> DOMResult {
    let time = std::time::Instant::now();
    let document = Document::new();
    let mut div: HTMLDivElement = document.create_element("div").try_into()?;
    for _ in 0..10 {
        let mut span = document.create_element("span");
        div.append(&mut span)?;
    }
    println!("{}", div.child_element_count());
    div.get_elements_by_tag_name("span").iter().for_each(|_| {
        //
    });
    println!("{:?}", time.elapsed());
    Ok(())
}
