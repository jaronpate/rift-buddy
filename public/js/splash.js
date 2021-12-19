const invoke = window.__TAURI__.invoke
setTimeout(() => {
    invoke('close_splash')
}, 3000)

console.log(window.__TAURI__.window.getAll())
window.__TAURI__.window.getAll().forEach(win => {
    win.center();
});