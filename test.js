const { windowList,  getWindowProcessId } = require("./index");

const windows = windowList();

console.log(windows);
console.log(getWindowProcessId(windows[0].hwnd));
console.log(getWindowProcessId("123"));
