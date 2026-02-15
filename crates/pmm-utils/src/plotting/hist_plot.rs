// ./pmm-utils/src/plotting/hist_plot.rs

use plotly::common::{Mode, Title};
use plotly::layout::{Axis, DragMode, Layout, Legend, Margin};
use plotly::{Candlestick, Configuration, Plot, Scatter};
use polars::prelude::*;

/// Plot a candlestick chart from a Polars DataFrame.
/// The DataFrame must have columns: "time" (Utf8), "open", "high", "low", "close" (all Float64).
pub fn plot_candlestick(df: &DataFrame, title: &str) -> Result<(), PolarsError> {
    let time = df
        .column("date")?
        .str()?
        .into_no_null_iter()
        .map(|s| s.to_string()) // <-- Clone into String
        .collect::<Vec<_>>();
    let open = df
        .column("open")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();
    let high = df
        .column("high")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();
    let low = df
        .column("low")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();
    let close = df
        .column("close")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();

    let trace = Candlestick::new(time, open, high, low, close);

    let layout = Layout::new()
        .title(Title::with_text(title)) // <--- No .text(), se usa la struct
        .drag_mode(DragMode::Zoom)
        .paper_background_color("#f0f0f0")
        .plot_background_color("#ffffff")
        .x_axis(
            Axis::new()
                .title(Title::with_text("Date")) // Igual
                .grid_color("#cccccc")
                .line_color("#000000")
                .tick_color("#000000")
                .show_line(true)
                .zero_line(false),
        )
        .y_axis(
            Axis::new()
                .title(Title::with_text("Valor"))
                .grid_color("#cccccc")
                .line_color("#000000")
                .tick_color("#000000")
                .show_line(true)
                .zero_line(false),
        )
        .legend(
            Legend::new()
                .background_color("#e0e0e0")
                .border_color("#000000")
                .border_width(1),
        )
        .margin(Margin::new().top(50).bottom(50).left(50).right(50));

    let mut plot = Plot::new();
    plot.set_configuration(Configuration::new().scroll_zoom(true));
    plot.add_trace(trace);
    plot.set_layout(layout);

    plot.show();
    // plot.to_html();

    Ok(())
}

/// Plot a scatter plot from a Polars DataFrame.
/// The DataFrame must have columns: "x", "y" (both Float64).
pub fn plot_scatter(df: &DataFrame, title: &str) -> Result<(), PolarsError> {
    let x = df
        .column("x")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();
    let y = df
        .column("y")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();

    let trace = Scatter::new(x, y).mode(Mode::Markers);

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(Layout::new().title(Title::with_text(title)));

    plot.show();

    Ok(())
}

/// Plot a line plot from a Polars DataFrame.
/// The DataFrame must have columns: "x", "y" (both Float64).
pub fn plot_line(df: &DataFrame, title: &str) -> Result<(), PolarsError> {
    let x = df
        .column("x")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();
    let y = df
        .column("y")?
        .f64()?
        .into_no_null_iter()
        .collect::<Vec<_>>();

    let trace = Scatter::new(x, y).mode(Mode::Lines);

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(Layout::new().title(Title::with_text(title)));

    plot.show();

    Ok(())
}
