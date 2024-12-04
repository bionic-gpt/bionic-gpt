use std::fs::{self, File};
use std::io::Write;

use crate::pages::contact::ContactPage;
use crate::pages::pricing::Pricing;

use crate::pages::partners::PartnersPage;

pub async fn generate() {
    let html = crate::render(Pricing).await;

    fs::create_dir_all("dist/pricing").expect("Couyldn't create folder");
    let mut file = File::create("dist/pricing/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");

    let html = crate::render(PartnersPage).await;

    fs::create_dir_all("dist/partners").expect("Couyldn't create folder");
    let mut file = File::create("dist/partners/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");

    let html = crate::render(ContactPage).await;

    fs::create_dir_all("dist/contact").expect("Couyldn't create folder");
    let mut file = File::create("dist/contact/index.html").expect("Unable to create file");
    file.write_all(html.as_bytes())
        .expect("Unable to write to file");
}
