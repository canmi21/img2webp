use img2webp::*;

fn main() {
    let sample_data = vec![0; 100];
    let result = process_image_to_webp(sample_data);
    println!("{:?}", result);
}