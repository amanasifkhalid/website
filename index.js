export function output(msg) {
    var out = document.getElementById("out");
    var input = document.getElementById("input");
    
    if (msg === "clear") {
        out.innerHTML = "";
        input.textContent = "";
        return;
    }
    
    var prompt = document.getElementById("prompt");
    var inputValue = document.createElement("p");
    inputValue.innerText = input.textContent;
    
    var oldInput = document.createElement("div");
    oldInput.innerHTML = `${prompt.innerHTML} ${inputValue.innerHTML}`;
    var newOutput = document.createElement("div");
    newOutput.innerHTML = msg;
    
    out.appendChild(oldInput);
    out.appendChild(newOutput);
    input.textContent = "";

    document.body.scrollTop = document.body.scrollHeight;
}

export function pause() {
    document.getElementById("floyd").pause();
    document.getElementById("ysaye").pause();
    output("");
}

export function play() {
    document.getElementById("ysaye").play();
    output("");
}

export function redirect(url) {
    window.location = url;
    output("");
}