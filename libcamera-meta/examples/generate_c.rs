use libcamera_meta::{control_ids, property_ids, Control};

/// Converts `ExampleName` to `example_name`
fn to_type_name(str: &str) -> String {
    let mut out = String::new();
    let chars = str.chars().collect::<Vec<_>>();

    for i in 0..chars.len() {
        // Do not split first char
        if i > 0 {
            let mut split = false;

            // Split if characters is uppercase and previous char is lowercase
            if chars[i].is_uppercase() && chars[i - 1].is_lowercase() {
                split = true;
            }

            // Split if characters is uppercase and following char is lowercase
            if chars[i].is_uppercase() && chars.get(i + 1).copied().map(char::is_lowercase).unwrap_or(false) {
                split = true;
            }

            // Ignore splitting if previous character is number
            if chars[i - 1].is_numeric() {
                split = false;
            }

            // Split if character is numeric and preivous is not
            if chars[i].is_numeric() && !chars[i - 1].is_numeric() {
                split = true;
            }

            if split {
                out.push('_');
            }
        }

        out.push(chars[i].to_ascii_lowercase());
    }

    out
}

/// Converts `ExampleName` to `EXAMPLE_NAME`
fn to_const_name(str: &str) -> String {
    to_type_name(str).to_uppercase()
}

fn to_enum_name(str: &str) -> String {
    format!("LIBCAMERA_CONTROL_ID_{}", to_const_name(str))
}

fn format_docstring(desc: &str, indent: usize) -> String {
    let mut parts = desc.split("\n").map(str::to_string).collect::<Vec<_>>();
    parts[0] = format!("\\brief {}", &parts[0]);

    // Remove last newline
    if parts.last().map(|s| s.is_empty()).unwrap_or(false) {
        parts.pop();
    }

    let mut out = Vec::new();

    out.push("/**".to_string());
    for part in parts {
        out.push(format!(" * {}", part));
    }
    out.push(" */".to_string());

    out.iter()
        .map(|line| format!("{}{}\n", " ".repeat(indent), line))
        .collect()
}

fn generate_controls(controls: &Vec<Control>) {
    let mut i = 1;
    println!("enum libcamera_control_id {{");
    for ctrl in controls.iter() {
        print!("{}", format_docstring(&ctrl.description, 4));
        println!("    {} = {},", to_enum_name(&ctrl.name), i);
        i += 1;
    }
    println!("}};\n");

    for ctrl in controls.iter() {
        if let Some(enumeration) = &ctrl.enumeration {
            println!("/**");
            println!(" * \\brief Supported values for {}", to_enum_name(&ctrl.name));
            println!(" */");
            println!("enum libcamera_{} {{", to_type_name(&ctrl.name));
            for val in enumeration {
                print!("{}", format_docstring(&val.description, 4));
                println!("    LIBCAMERA_{} = {},", to_const_name(&val.name), val.value);
            }
            println!("}};\n");
        }
    }
}

fn main() {
    let controls = control_ids();
    generate_controls(&controls);

    let properties = property_ids();
    generate_controls(&properties);
}