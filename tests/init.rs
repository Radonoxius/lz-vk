use ash::vk::{ApplicationInfo, make_api_version};

#[test]
fn init_test() {
    let entry = ash::Entry::linked();
    let app_info = ApplicationInfo {
        api_version: make_api_version(0, 1, 1, 0),
        ..Default::default()
    };

    let instance = lz_vk::init::init(true, &entry, &app_info);
    if let Err(_) = instance {
        instance.unwrap();
    }
}