use editpe::Image;
use jni::JNIEnv;
use jni::objects::{JClass, JString};

#[no_mangle]
pub extern "system" fn Java_io_github_fourlastor_construo_editpe_EditPE_replaceIcon<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    exe_path: JString<'local>,
    icon_path: JString<'local>,
    destination_path: JString<'local>,
) {
    let exe_path: String =
    env.get_string(&exe_path).expect("Couldn't get exe path").into();
    let icon_path: String =
    env.get_string(&icon_path).expect("Couldn't get icon path").into();
    let destination_path: String =
    env.get_string(&destination_path).expect("Couldn't get destination path").into();

    let mut image = Image::parse_file(exe_path.clone()).expect("Could not parse exe file");
    let mut resources = image.resource_directory().cloned().unwrap_or_default();
    resources.set_main_icon_file(&icon_path).expect("Failed to set icon file");
    image.set_resource_directory(resources).expect("Failed to set resources");
    image.write_file(destination_path).expect("Failed to write exe file");
}
