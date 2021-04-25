use ::indoc::indoc;
use ::tera::Tera;
use crate::app::args::Args;
use crate::errors::*;

pub fn template_default_name() -> &'static str {
    "default.html"
}

pub fn template_default_html() -> &'static str {
    indoc!{r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <title>{{ title }}</title>
                <meta name="viewport" content="width=device-width, initial-scale=1">
            </head>
            <body>
        {{ content }}
            </body>
        </html>
    "#}
}

pub fn init(args: &Args) -> Result<Tera> {
    let mut tera: Tera = match &args.template_glob {
        Some(template_glob) => {
            Tera::new(&*template_glob)
            .chain_err(|| format!("create tera. template glob: {:?}", template_glob))?
        },
        _ => {
            Tera::default()
        }
    };
    tera.add_raw_template(
        template_default_name(),
        template_default_html(),
    )
    .chain_err(|| format!("add raw template. name: {:?}", template_default_name()))?;

    tera.autoescape_on(vec![]); // disable autoescaping completely
    //tera.autoescape_on(vec!["html", ".sql"]);
    //tera.register_filter("do_nothing", do_nothing_filter);
    Ok(tera)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::indoc::indoc;
    use crate::app::args::Args;
    use crate::templating::vars::Vars;

    fn fab_tera() -> Tera {
        let args = Args::default();
        super::init(&args).unwrap()
    }

    const FAB_OUTPUT_HTML: &str = indoc!{r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <title>my title</title>
                <meta name="viewport" content="width=device-width, initial-scale=1">
            </head>
            <body>
        my content
            </body>
        </html>
    "#};

    #[test]
    fn test_init() {
        //TODO
    }

    #[test]
    fn test_tera_x_vars() {
        let tera = fab_tera();
        let vars = Vars {
            title: Some("my title".into()),
            content: Some("my content".into()),
        };
        let actual = tera.render(
            template_default_name(),
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_tera_x_json() {
        let tera = fab_tera();
        let vars = indoc!{r#"
            {
                "title": "my title",
                "content": "my content"
            }
        "#};
        let vars: ::serde_json::Value = ::serde_json::from_str(vars).unwrap();
        let actual = tera.render(
            template_default_name(),
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_tera_x_toml() {
        let tera = fab_tera();
        let vars = indoc!{r#"
            title = "my title"
            content = "my content"
        "#};
        let vars: ::toml::Value = vars.parse::<::toml::Value>().unwrap();
        let actual = tera.render(
            template_default_name(),
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

    #[test]
    fn test_tera_x_yaml() {
        let tera = fab_tera();
        let vars = indoc!{r#"
            title: "my title"
            content: "my content"
        "#};
        let vars: ::serde_yaml::Value = ::serde_yaml::from_str(&vars).unwrap();
        let actual = tera.render(
            template_default_name(),
            &::tera::Context::from_serialize(&vars).unwrap()
        ).unwrap();
        assert_eq!(actual, FAB_OUTPUT_HTML);
    }

}
