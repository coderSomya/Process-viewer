import {h, Component, render} from "https://unpkg.com/preact?module";

setInterval(async() => {

    let response = await fetch("/api/cpus");

    if(response.status!=200){
        throw new Error("HTTP error: Could not fetch cpus usage");
    }

    let json = await response.json();

    const app = h('pre', null, JSON.stringify(json, null, 2));

    render(app, document.body)
}, 1000);