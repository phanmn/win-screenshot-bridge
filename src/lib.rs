use neon::prelude::*;
use win_screenshot::prelude::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use windows::Win32::Foundation::HWND;

struct Window {
    pub hwnd: isize,
    pub name: String,
    pub pid: u32,
}

impl Window {
    fn to_object<'a>(&self, cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let hwnd = cx.string(self.hwnd.to_string());
        obj.set(cx, "hwnd", hwnd)?;

        let name = cx.string(&self.name);
        obj.set(cx, "name", name)?;

        let pid = cx.number(self.pid);
        obj.set(cx, "pid", pid)?;

        Ok(obj)
    }
}

fn js_window_list(mut cx: FunctionContext) -> JsResult<JsArray> {
    let window_vec: Vec<Window> = window_list()
        .unwrap()
        .iter()
        .map(|window| {
            let pid: u32 = unsafe { GetWindowThreadProcessId(HWND(window.hwnd), None) };

            Window {
                hwnd: window.hwnd,
                name: window.window_name.clone(),
                pid: pid,
            }
        })
        .collect();

    let window_array = JsArray::new(&mut cx, window_vec.len() as u32);
    for (i, window) in window_vec.iter().enumerate() {
        let v = window.to_object(&mut cx)?;
        window_array.set(&mut cx, i as u32, v)?;
    }

    Ok(window_array)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("windowList", js_window_list)?;
    Ok(())
}
