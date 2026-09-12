use ash::vk::{ApplicationInfo, make_api_version};

fn main() {
    let entry = ash::Entry::linked();
    let app_info = ApplicationInfo {
        api_version: make_api_version(0, 1, 1, 0),
        ..Default::default()
    };

    let _instance = lz_vk::init::init(true, &entry, &app_info).unwrap();
}
