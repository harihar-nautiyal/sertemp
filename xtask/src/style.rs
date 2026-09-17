use clap::builder::styling::{Effects, RgbColor, Style, Styles};

pub fn style() -> Styles {
    let bat_blue = Style::new()
        .fg_color(Some(RgbColor(65, 145, 255).into()))
        .effects(Effects::BOLD);

    let steel_blue = Style::new()
        .fg_color(Some(RgbColor(140, 185, 235).into()))
        .effects(Effects::BOLD);

    let slate_blue = Style::new()
        .fg_color(Some(RgbColor(170, 205, 240).into()))
        .effects(Effects::BOLD | Effects::UNDERLINE);

    let gotham_red = Style::new()
        .fg_color(Some(RgbColor(255, 75, 75).into()))
        .effects(Effects::BOLD);

    let arctic_mint = Style::new()
        .fg_color(Some(RgbColor(90, 230, 200).into()))
        .effects(Effects::BOLD);

    Styles::styled()
        .header(bat_blue)
        .usage(bat_blue)
        .literal(steel_blue)
        .placeholder(slate_blue)
        .error(gotham_red)
        .valid(arctic_mint)
        .invalid(gotham_red)
}
