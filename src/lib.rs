
mod term_graph {
  extern crate pancurses;
  extern crate line_drawing;
  use pancurses::{initscr, endwin, noecho};
  use line_drawing::Bresenham;
  use std::vec::Vec;

  struct Screen {
    max_y: i32,
    max_x: i32,
    _w: pancurses::Window
  }

  impl Screen {
    fn deinit(self) {endwin();}
  }

  enum GraphPlacement {
    _TopLeft, _TopMiddle, _TopRight,
    _BottomLeft, _BottomMiddle, _BottomRight
  }

  fn init_screen() -> Screen {
    let win = initscr();
    win.keypad(true);
    noecho();
    Screen {max_y: win.get_max_y(), max_x: win.get_max_x(), _w: win}
  }

  fn make_plot_area(screen: &Screen, plot_pos: GraphPlacement) -> pancurses::Window {
    let corner_origin = match plot_pos { // y, x
        GraphPlacement::_TopLeft => [0, 0],
        GraphPlacement::_TopMiddle => [0, screen.max_x / 3],
        GraphPlacement::_TopRight => [0, 2 * (screen.max_x / 3)],
        GraphPlacement::_BottomLeft => [screen.max_y / 2, 0],
        GraphPlacement::_BottomMiddle => [screen.max_y / 2, screen.max_x / 3],
        GraphPlacement::_BottomRight => [screen.max_y / 2, 2 * (screen.max_x / 3)]
      };

    pancurses::newwin(corner_origin[0] + screen.max_y / 2, corner_origin[1] + screen.max_x / 3, corner_origin[0], corner_origin[1])
  }

  fn draw_border_and_crosshatch(sub_win: &pancurses::Window, panel_height: i32, panel_width: i32) {
    for across_y in 0..panel_height {
      for across_x in 0..panel_width {
        if across_y == 0 || across_y == panel_height - 1 {sub_win.mvprintw(across_y, across_x, "-");}
        else if across_x == 0 || across_x == panel_width - 1 {sub_win.mvprintw(across_y, across_x, "'");}
        else if across_y == panel_height / 2 {sub_win.mvprintw(across_y, across_x, "-");}
        else if across_x == panel_width / 2 {sub_win.mvprintw(across_y, across_x, "'");}
      }
    }
  }

  fn plot_points(sub_win: &pancurses::Window, point_slice: Vec<[i32; 2]>) {
    let mut t_previous = (point_slice[0][0], point_slice[0][1]);
    let mut a_previous = point_slice[0];

    for a_current in point_slice.iter() {

      let t_current = (a_current[0], a_current[1]);

      let rise = a_previous[1] - a_current[1];
      let run = a_current[0] - a_previous[0];

      let mut slope_segment = "";
      if rise == 0 || run == 0 {slope_segment = "_";}
      else {
        let slope = rise / run;
        // log(format!("Slope: {}", slope));
        if i32::abs(slope) >= 10 {slope_segment = "|";}
        else if i32::abs(slope) as f32 > 0.3 {
          if slope > 0 {slope_segment = "/";}
          else {slope_segment = "\\";}
      }
    }
      for (x, y) in Bresenham::new(t_previous, t_current) {
        sub_win.mvprintw(y, x, slope_segment);
      }
      t_previous = t_current;
      a_previous = *a_current;
    }
  }

  fn draw_function<F: Fn(f32) -> f32>(sub_win: pancurses::Window, function: F) {
    let (begin_y, begin_x) = sub_win.get_beg_yx();
    let (end_y, end_x) = sub_win.get_max_yx();

    draw_border_and_crosshatch(&sub_win, end_y - begin_y, end_x - begin_x);

    let center_x = end_x - begin_x;

    let graph_half_width = center_x - center_x / 2;
    let scale_vert = (end_y - begin_y) / 2;
    let scale_hori = (end_x - begin_x) / 2;

    let mut points_to_plot = Vec::new(); // x, y
    
    for graph_x in -graph_half_width..graph_half_width {
      let x = scale_hori + graph_x;
      let y = scale_vert - function(graph_x as f32) as i32;
      points_to_plot.push([x, y]);
    }

    plot_points(&sub_win, points_to_plot);
    sub_win.refresh();
    sub_win.getch();
  }
}
