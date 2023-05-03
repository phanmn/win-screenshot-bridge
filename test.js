const { windowList,  getWindowThreadProcessId } = require("./index");

const windows = windowList();

console.log(windows);
console.log(getWindowThreadProcessId(windows[0].hwnd));
console.log(getWindowThreadProcessId("123"));
