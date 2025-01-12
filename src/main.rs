// See https://docs.opencv.org/4.x/ for the modules to import beside prelude
// Crate Documentation: https://docs.rs/crate/opencv/latest
use opencv::prelude::*;
use opencv::{
	core::{KeyPoint, Vector},
	videoio, highgui, features2d
};

fn main() {
	let mut video = videoio::VideoCapture::new_def(0);
	if video.as_ref().is_ok_and(|v| v.is_opened().is_ok()) {
		loop {
			let mut frame = Mat::default();
			let read = video.as_mut().unwrap().read(&mut frame);
			if read.is_ok() {
				// Extract KeyPoints
				let sift = features2d::SIFT::create_def();
				if sift.is_ok() {
					let mask = Mat::default();
					let mut keypoints: Vector<KeyPoint> = Vector::default();
					let mut descriptors = Mat::default();
					let sr = sift.unwrap().detect_and_compute(&frame, &mask, &mut keypoints, &mut descriptors, false);
					if sr.is_ok() {
						let mut kframe = Mat::default();
						let _ = features2d::draw_keypoints_def(&frame, &keypoints, &mut kframe);
						let _ = highgui::imshow("capture", &kframe);
					}
				} else {
					let _ = highgui::imshow("capture", &frame);
				}

				// Show the image
				let key = highgui::wait_key(10);
				if key.is_ok_and(|k| k >= 0) {
					let _ = highgui::destroy_all_windows();
					break;
				}
			} else {
				println!("No frame...");
			}
		}
	} else {
		println!("No Video");
	}
}
