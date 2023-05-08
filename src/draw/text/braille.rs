use crate::config::PlotType;
use crate::draw::PlotInfo;
use crate::Config;
use crate::DataSet;

pub fn braille_plot(config: &Config, dataset: &DataSet, plot_info: &mut PlotInfo) {
    let mut rows = get_rows(plot_info);
    if config.axis {
        plot_info.calc_axis_pos();
        draw_axes(config, plot_info, &mut rows);
    }

    print_header(
        config,
        plot_info,
        config.mode == PlotType::Count,
        dataset.columns,
    );

    plot_points(config, dataset, plot_info, &mut rows);

    for row in rows {
        println!("{}", row.join(""));
    }
}

fn get_rows(plot_info: &PlotInfo) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    for _ in 0..plot_info.height {
        let mut row = Vec::new();
        for _ in 0..plot_info.width {
            row.push("⠀".to_string());
        }
        rows.push(row);
    }
    rows
}

fn draw_axes(config: &Config, plot_info: &PlotInfo, rows: &mut [Vec<String>]) {
    todo!()
}

fn print_header(config: &Config, plot_info: &PlotInfo, count_mode: bool, columns: usize) {
    todo!()
}

fn plot_points(config: &Config, dataset: &DataSet, plot_info: &PlotInfo, rows: &mut [Vec<String>]) {
    todo!()
}
