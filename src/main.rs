mod divinum_officium;

fn main() -> Result<(), String> {
    let translations = divinum_officium::DivinumOfficium::new();

    for line in translations.psalm(1)? {
        println!("{line}");
    }

    Ok(())
}
