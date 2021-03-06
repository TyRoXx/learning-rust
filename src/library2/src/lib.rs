use std::fs::read_dir;
use std::path::Path;
use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering;

pub fn add(first: i32, second: i32) -> i32 {
	first + second
}

#[test]
fn it_works() {
	assert!(5 == add(2, 3));
}

#[test]
fn list_files() {
	let mut listed = read_dir(Path::new("."));
	match listed {
		Ok(ref mut files) => {
			let first = files.next().unwrap();
			match first {
				Ok(ref entry) => {
					assert!(entry.path().as_path().is_relative());
				}
				Err(_) => {
					unreachable!();
				}
			}
		}
		Err(_) => {
			unreachable!();
		}
	}
}

#[test]
fn atomic_ptr() {
	let mut pointee = 3i32;
	let ptr = AtomicPtr::new(&mut pointee);
	ptr.store(ptr.load(Ordering::Relaxed), Ordering::Relaxed);
}
