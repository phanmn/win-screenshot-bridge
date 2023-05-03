const { windowList,  getWindowThreadProcessId } = require("./index");

const windows = windowList();

console.log(getWindowThreadProcessId(windows[0].hwnd));