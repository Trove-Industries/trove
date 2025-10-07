use tera::{Tera, Context};
use once_cell::sync::Lazy;

static TERA: Lazy<Tera> = Lazy::new(|| {
    match Tera::new("theme/dev/templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Template parsing error: {}", e);
            std::process::exit(1);
        }
    }
});

pub fn render_template(template_name: &str, data: &impl serde::Serialize) -> tera::Result<String> {
    let context = Context::from_serialize(data)?;
    TERA.render(template_name, &context)
}
