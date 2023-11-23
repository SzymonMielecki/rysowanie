use plotters::prelude::*;
fn main() {
    let delta_t = 1.0;
    let t_max = 10.0;
    let x_0 = 0.0;
    let v_0 = 5.0;
    let a_0 = 1.0;
    let k = 0.1;
    fn get_a(a: f32, k: f32, v: f32) -> f32 {
        a - k * v
    }

    fn get_v(v: f32, a: f32, delta_t: f32) -> f32 {
        v + a * delta_t
    }

    fn get_x(x: f32, v: f32, delta_t: f32) -> f32 {
        x + v * delta_t
    }

    draw(get_v, get_a, get_x, delta_t, x_0, v_0, t_max, a_0, k)
}

fn draw(
    fn_v: fn(f32, f32, f32) -> f32,
    fn_a: fn(f32, f32, f32) -> f32,
    fn_x: fn(f32, f32, f32) -> f32,
    delta_t: f32,
    x_0: f32,
    v_0: f32,
    t_max: f32,
    a_0: f32,
    k: f32,
) {
    let mut vec_t = vec![0.0];
    let mut vec_a = vec![a_0];
    let mut vec_v = vec![v_0];
    let mut vec_x = vec![x_0];
    while *vec_t.last().unwrap() <= t_max {
        vec_a.push(fn_a(*vec_a.last().unwrap(), k, *vec_v.last().unwrap()));
        vec_v.push(fn_v(
            *vec_v.last().unwrap(),
            *vec_a.last().unwrap(),
            delta_t,
        ));
        vec_x.push(fn_x(
            *vec_x.last().unwrap(),
            *vec_v.last().unwrap(),
            delta_t,
        ));
        vec_t.push(*vec_t.last().unwrap() + delta_t);
    }
    let max_w = t_max;
    let min_w = 0.;
    let max_h = get_max_value(&vec_a, &vec_v, &vec_x);
    let min_h = get_min_value(&vec_a, &vec_v, &vec_x);
    let root_area = BitMapBackend::new("images/2.5.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let drawing_area =
        SVGBackend::new("line_series_point_size.svg", (300, 200)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Wykres", ("sans-serif", 40))
        .build_cartesian_2d(min_w..max_w, min_h..max_h)
        .unwrap();
    ctx.configure_mesh().draw().unwrap();
    ctx.draw_series(LineSeries::new(vec_to_points(&vec_a, &vec_t), &BLACK))
        .unwrap();
    ctx.draw_series(LineSeries::new(vec_to_points(&vec_v, &vec_t), &BLUE))
        .unwrap();
    ctx.draw_series(LineSeries::new(vec_to_points(&vec_x, &vec_t), &RED))
        .unwrap();
}
fn vec_to_points(vec: &[f32], t: &[f32]) -> Vec<(f32, f32)> {
    vec.iter()
        .cloned()
        .enumerate()
        .map(|(i, x)| (t[i], x))
        .collect()
}

fn get_max_value(vec1: &[f32], vec2: &[f32], vec3: &[f32]) -> f32 {
    vec1.iter()
        .chain(vec2)
        .chain(vec3)
        .cloned()
        .fold(f32::NEG_INFINITY, f32::max)
}

fn get_min_value(vec1: &[f32], vec2: &[f32], vec3: &[f32]) -> f32 {
    vec1.iter()
        .chain(vec2)
        .chain(vec3)
        .cloned()
        .fold(f32::INFINITY, f32::min)
}
