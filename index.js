export function clear() {
    document.getElementById("out").innerHTML = "";
    document.getElementById("input").textContent = "";
}

export function output(msg) {
    var out = document.getElementById("out");
    var input = document.getElementById("input");
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

export function play(id) {
    document.getElementById("floyd").pause();
    document.getElementById("ysaye").pause();
    document.getElementById(id).play();
    output("");
}

export function redirect(url) {
    window.location = url;
    output("");
}

export function set_input(cmd) {
    document.getElementById("input").textContent = cmd;
}