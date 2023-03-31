use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use reqwest::blocking::get;

fn download_image(url: &str, media_queue: Arc<Mutex<VecDeque<Vec<u8>>>>, thread_counter: Arc<Mutex<u32>>) {
    let response = get(url);
    if let Ok(mut image) = response {
        let mut buffer = Vec::new();
        image.copy_to(&mut buffer).unwrap();
        media_queue.lock().unwrap().push_back(buffer);
    }
    let mut counter = thread_counter.lock().unwrap();
    *counter -= 1;
}

fn main() {
    let media_queue = Arc::new(Mutex::new(VecDeque::new()));
    let thread_counter = Arc::new(Mutex::new(0));
    let urls = vec![
        "https://example.com/image1.jpg",
        "https://example.com/image2.jpg",
        "https://example.com/image3.jpg",
        "https://example.com/image4.jpg",
        "https://example.com/image5.jpg",
        "https://example.com/image6.jpg",
        "https://example.com/image7.jpg",
        "https://example.com/image8.jpg",
        "https://example.com/image9.jpg",
        "https://example.com/image10.jpg",
    ];
    let mut i = 0;
    loop {
        if i >= urls.len() {
            break;
        }
        let url = urls[i];
        if *thread_counter.lock().unwrap() < 10 {
            let media_queue_clone = Arc::clone(&media_queue);
            let thread_counter_clone = Arc::clone(&thread_counter);
            let handle = thread::spawn(move || {
                download_image(url, media_queue_clone, thread_counter_clone);
            });
            i += 1;
            *thread_counter.lock().unwrap() += 1;
        }
        else {
            thread::sleep(std::time::Duration::from_millis(100));
        }
        let mut media = media_queue.lock().unwrap();
        while !media.is_empty() {
            let m = media.pop_front().unwrap();
            // do something with the media here
        }
    }
    while *thread_counter.lock().unwrap() > 0 {
        let mut media = media_queue.lock().unwrap();
        while !media.is_empty() {
            let m = media.pop_front().unwrap();
            // do something with the media here
        }
        thread::sleep(std::time::Duration::from_millis(100));
    }
}