use libcamera::{
    camera_manager::CameraManager,
    control::ControlList,
    controls::{Contrast, ControlId},
    logging::LoggingLevel,
    stream::StreamRole,
};
use libcamera_sys::libcamera_control_value;

fn main() {
    let mgr = CameraManager::new().unwrap();

    mgr.log_set_level("Camera", LoggingLevel::Error);

    let cameras = mgr.cameras();

    for i in 0..cameras.len() {
        let cam = cameras.get(i).unwrap();
        println!("Camera {}", i);
        println!("ID: {}", cam.id());

        println!("Properties: {:#?}", cam.properties());

        let config = cam.generate_configuration(&[StreamRole::ViewFinder]).unwrap();
        let view_finder_cfg = config.get(0).unwrap();
        // println!("Available formats: {:#?}", view_finder_cfg.formats());

        let controls = cam.controls();

        let mut new_controls = ControlList::new();

        for (num, info) in controls.into_iter() {
            let id = ControlId::try_from(num).unwrap();
            println!("{:?} {:?}", id, info.def());
            //println!("{:?} {:?}", num, info.get());
            println!("{:?} {:?}", num, info.values());

            if id == ControlId::Contrast {
                let mut contrast = Contrast::try_from(info.def()).unwrap();
                contrast.0 = 0.5;
                new_controls.set(contrast);
            }
        }
        let cam = cameras.get(0).expect("No cameras found");
        let mut cam = cam.acquire().expect("Unable to acquire camera");
        let mut cfgs = cam.generate_configuration(&[StreamRole::ViewFinder]).unwrap();
        cam.configure(&mut cfgs).expect("Unable to configure camera");
        cam.start(Some(&new_controls));
    }
}
