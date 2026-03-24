use dioxus::prelude::*;

#[cfg(all(target_os = "android", feature = "mobile"))]
mod android {
    use super::handle_deep_link;
    use jni::EnvUnowned;
    use jni::objects::{JClass, JString};

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_dioxus_main_MainActivity_nativeHandleDeepLink<'caller>(
        mut unowned_env: EnvUnowned<'caller>,
        _class: JClass<'caller>,
        url: JString<'caller>,
    ) {
        let outcome = unowned_env.with_env(|env| -> Result<_, jni::errors::Error> {
            let url: String = url.to_string();

            println!("Deep link received: {}", url);

            handle_deep_link(url);
            Ok(())
        });

        outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
    }
}

fn handle_deep_link(url: String) {
    let url = url.strip_prefix("roommates://").unwrap_or(&url);
    let nav = navigator();
    nav.push(url);
}
