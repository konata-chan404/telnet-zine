use handlebars::{Handlebars, Helper, HelperResult, Context, RenderContext, Output, RenderError};
use crossterm::style::{Color, Stylize};

pub fn color_helper(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    // Get the color and text parameters from the helper
    let fg_color_param = h.param(0).ok_or(RenderError::new("Missing foreground color parameter"))?;
    let text_param = h.param(1).ok_or(RenderError::new("Missing text parameter"))?;

    // Convert the parameters to strings
    let fg_color = fg_color_param.value().as_str().ok_or(RenderError::new("Invalid foreground color parameter"))?;
    let text = text_param.value().as_str().ok_or(RenderError::new("Invalid text parameter"))?;

    // Convert the color strings to Color enum values
    let fg_color_enum = match fg_color.to_lowercase().as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "purple" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        _ => return Err(RenderError::new("Invalid foreground color parameter")),
    };
    
    let bg_color_enum = match h.param(2) {
        Some(bg_color_param) => {
            let bg_color = bg_color_param.value().as_str().ok_or(RenderError::new("Invalid background color parameter"))?;
            match bg_color.to_lowercase().as_str() {
                "black" => Color::Black,
                "red" => Color::Red,
                "green" => Color::Green,
                "yellow" => Color::Yellow,
                "blue" => Color::Blue,
                "purple" => Color::Magenta,
                "cyan" => Color::Cyan,
                "white" => Color::White,
                _ => return Err(RenderError::new("Invalid background color parameter")),
            }
        }
        None => Color::Reset,
    };

    // Create the colored text string with both background and foreground colors
    let colored_text = format!("{}", text.with(fg_color_enum).on(bg_color_enum));

    // Write the colored text to the template
    out.write(&colored_text)?;

    Ok(())
}

pub fn italic_helper(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    // Get the color and text parameters from the helper
    let text_param = h.param(0).ok_or(RenderError::new("Missing text parameter"))?;

    // Convert the parameters to strings
    let text = text_param.value().as_str().ok_or(RenderError::new("Invalid text parameter"))?;

    let stylized_text = format!("{}", text.italic());
    // Write the colored text to the template
    out.write(&stylized_text)?;

    Ok(())
}

pub fn bold_helper(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    // Get the color and text parameters from the helper
    let text_param = h.param(0).ok_or(RenderError::new("Missing text parameter"))?;

    // Convert the parameters to strings
    let text = text_param.value().as_str().ok_or(RenderError::new("Invalid text parameter"))?;

    let stylized_text = format!("{}", text.bold());
    // Write the colored text to the template
    out.write(&stylized_text)?;

    Ok(())
}

pub fn underline_helper(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    // Get the color and text parameters from the helper
    let text_param = h.param(0).ok_or(RenderError::new("Missing text parameter"))?;

    // Convert the parameters to strings
    let text = text_param.value().as_str().ok_or(RenderError::new("Invalid text parameter"))?;

    let color_enum = match h.param(2) {
        Some(color_enum) => {
            let color = color_enum.value().as_str().ok_or(RenderError::new("Invalid underline color parameter"))?;
            match color.to_lowercase().as_str() {
                "black" => Color::Black,
                "red" => Color::Red,
                "green" => Color::Green,
                "yellow" => Color::Yellow,
                "blue" => Color::Blue,
                "purple" => Color::Magenta,
                "cyan" => Color::Cyan,
                "white" => Color::White,
                _ => return Err(RenderError::new("Invalid underline color parameter")),
            }
        }
        None => Color::Reset,
    };

    // Create the colored text string with both background and foreground colors
    let colored_text = format!("{}", text.underline(color_enum));

    // Write the colored text to the template
    out.write(&colored_text)?;

    Ok(())
}

pub fn rainbow_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let text_param = h.param(0).ok_or(RenderError::new("Missing text parameter"))?;
    let text = text_param.value().as_str().ok_or(RenderError::new("Invalid text parameter"))?;

    let freq = h.param(1).and_then(|v| v.value().as_f64());
    let spread = h.param(2).and_then(|v| v.value().as_u64()).map(|u| u as usize);

    let mut colored_text = String::new();
    for (i, c) in text.chars().enumerate() {
        let color = lol::rainbow(freq, spread, i);
        colored_text.push_str(&format!("{}", c.to_string().with(color)));
    }

    out.write(&colored_text)?;
    Ok(())
}

mod lol {
    use crossterm::style::Color;

    pub fn rainbow(freq: Option<f64>, spread: Option<usize>, i: usize) -> Color {
        let freq = freq.unwrap_or(0.2);
        let _spread = spread.unwrap_or(1);
        let red = (freq * i as f64 + 0.0).sin() * 127.0 + 128.0;
        let green = (freq * i as f64 + 2.0 * std::f64::consts::PI / 3.0).sin() * 127.0 + 128.0;
        let blue = (freq * i as f64 + 4.0 * std::f64::consts::PI / 3.0).sin() * 127.0 + 128.0;
        Color::Rgb { r: red as u8, g: green as u8, b: blue as u8 }
    }
}