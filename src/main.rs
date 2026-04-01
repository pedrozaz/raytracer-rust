fn main() {
    let width: i16 = 200;
    let height: i16 = 100;

    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let r = i as f64 / width as f64;
            let g = j as f64 / height as f64;
            let b = 0.2;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
