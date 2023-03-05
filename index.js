export function output(msg) {
    var term = document.getElementById("term");
    var out = document.getElementById("out");
    var prompt = document.getElementById("prompt");
    var input = document.getElementById("input");

    var oldInput = document.createElement("div");
    oldInput.textContent = `${prompt.textContent} ${input.value}`;
    var newOutput = document.createElement("div");
    newOutput.textContent = msg;

    out.appendChild(oldInput);
    out.appendChild(newOutput);
    input.value = "";
    term.scrollTop = term.scrollHeight;
}