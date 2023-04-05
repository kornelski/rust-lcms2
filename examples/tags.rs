
use lcms2::{InfoType, Locale, Profile, Tag};

use std::env;

fn main() {
    let profile = if let Some(path) = env::args().nth(1) {
        Profile::new_file(path).unwrap()
    } else {
        Profile::new_srgb()
    };

    for &info in &[InfoType::Description, InfoType::Manufacturer, InfoType::Model, InfoType::Copyright] {
        if let Some(data) = profile.info(info, Locale::none()) {
            println!("{info:?} = {data:?}");
        }
    }

    for sig in profile.tag_signatures() {
        let tag = profile.read_tag(sig);
        println!("{sig:?} = {tag:?}");
        match tag {
            Tag::Pipeline(pipeline) => {
                for stage in pipeline.stages() {
                    println!(" └─ {stage:?}");
                }
            },
            _ => {},
        }
    }
}
