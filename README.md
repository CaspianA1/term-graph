## Graph Rust closures in the terminal.
- Note: the closures used are floating-point.

### Here's an example of how to use term-graph:
```rust
fn main() {
  let screen = init_screen();
  let graph_spot = make_plot_area(&screen, GraphPlacement::_TopMiddle);
  let graph_spot_2 = make_plot_area(&screen, GraphPlacement::_TopRight);

  let small_wave = |theta: f32| 8.0 * theta.sin();
  let big_wave = |theta: f32| 17.0 * theta.tan() + 17.0 * theta.atan();

  draw_function(graph_spot, small_wave);
  draw_function(graph_spot_2, big_wave);

  screen.deinit();
}
```
### Here's what that looks like:
![ASCII-graph](https://i.imgur.com/MjxyvPq.png)
