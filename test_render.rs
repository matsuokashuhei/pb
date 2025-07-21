use pb::progress_bar::render_progress_bar;

fn main() {
    println!("50.5%: {}", render_progress_bar(50.5));
    println!("2.5%: {}", render_progress_bar(2.5));
    println!("97.5%: {}", render_progress_bar(97.5));
    println!("2.4%: {}", render_progress_bar(2.4));
    println!("2.6%: {}", render_progress_bar(2.6));
    println!("97.8%: {}", render_progress_bar(97.8));
}
