const invoke = window.__TAURI__.invoke

// Find and center windows
console.log(window.__TAURI__.window.getAll())
window.__TAURI__.window.getAll().forEach(win => {
    win.center();
});

// Check if LCU is running
const interval = setInterval(connect, 5000)

function connect(){
    invoke('get_credentials')
        .then((connData) => {
            console.log("Found league client!")
            clearInterval(interval)
            setTimeout(invoke('close_splash'), 5000)
        })
        .catch((e) => console.log("No league client found..."));
}
