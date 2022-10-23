// #[init]
// fn init() {
//   unsafe {
//     STATE = Some(InfoStorage::new())
//   }
// }
//
// #[update]
// fn set_info(info: String) {
//   let state = unsafe { STATE.as_mut().unwrap() };
//   state.set_info(info);
// }
//
// #[query]
// fn get_info() -> String {
//   unsafe { STATE.as_ref().unwrap().get_info() }
// }
