use ::tera::Tera;

pub fn init(glob: &str) -> Tera {
    let tera = match Tera::new(glob) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    //tera.autoescape_on(vec!["html", ".sql"]);
    //tera.register_filter("do_nothing", do_nothing_filter);
    tera
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;
    use crate::templating::vars::Vars;
        
    #[test]
    fn test_tera() {
        let glob: &str =&*format!("{}/tests/templates/**/*", env!("CARGO_MANIFEST_DIR"));
        let tera: Tera = super::init(glob);
        let vars = Vars {
            title: "Example".into(),
        };
        let actual = tera.render(
            "example.html", 
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        let expect = indoc! {"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Example</title>
                </head>
                <body>
                    <h1>Example</h1>
                </body>
            </html>
        "};
        assert_eq!(actual, expect);
    }

}

    