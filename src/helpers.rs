use std::fmt::format;

use handlebars::{Handlebars, HelperDef, RenderContext, Helper, Context, JsonRender, HelperResult, Output, RenderError};
use crossterm::{style::{Color, SetForegroundColor, ResetColor, Stylize}, execute};

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