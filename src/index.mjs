import {h, Component, render} from "https://unpkg.com/preact?module";
import htm from "https://unpkg.com/htm?module";

const html = htm.bind(h);

function App (props){

    return html `
    <div class="container">
    ${props.cpus.map((cpu)=>{
        return html`
        <div class="bar">
        <div class="bar-inner" style="width:${cpu}vw">
        ${cpu.toFixed(2)}% usage
        </div>
        </div>
        `;
    })}
    </div>
    `;
}

setInterval(async() => {

    let response = await fetch("/api/cpus");

    if(response.status!=200){
        throw new Error("HTTP error: Could not fetch cpus usage");
    }

    let json = await response.json();
    let content = JSON.stringify(json, null, 2);
    // const app = h('pre', null, content);

    render(html`<${App} cpus=${json}></${App}>`, document.body)
}, 1000);