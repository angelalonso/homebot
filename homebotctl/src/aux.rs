use handlebars::Handlebars;
use std::collections::HashMap;

fn main() {
    // Create a Handlebars registry
    let mut handlebars = Handlebars::new();

    // Register a template
    let template = "Hello, {{name}}! Welcome to {{city}}.";
    handlebars
        .register_template_string("template", template)
        .expect("Failed to register template");

    // Create a context with variables
    let mut context = HashMap::new();
    context.insert("name", "Alice");
    context.insert("city", "New York");

    // Render the template
    let output = handlebars.render("template", &context).expect("Failed to render template");

    println!("{}", output);
}
