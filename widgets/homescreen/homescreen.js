let seed = 1;
function random() {
    const a = 1103515245;
    const c = 12345;
    const m = 1e9 + 7;
    seed = (a * seed + c) % m;
    return seed / m;
}

function random_int(min, max) {
    return Math.floor(random() * (max - min + 1) + min);
}




function build_path() {
    let data = {
        line1: [
            {
                height: 0,
            },
            {
                height: -1,
            },
            {
                height: 0.4,
            },
            {
                height: 0.6,
            },
            {
                height: 0.8,
            },
            {
                height: 1,
            },
            {
                height: 0.8,
            },
            {
                height: 0.6,
            },
            {
                height: 0.4,
            },
            {
                height: 0,
            },
            {
                height: -1,
            },
            {
                height: 0,
            },
            {
                height: 0.4,
            },
            {
                height: 0.6,
            },
            {
                height: 0.8,
            },
            {
                height: 1,
            },
            {
                height: 0.8,
            },
            {
                height: 0.6,
            },
            {
                height: 0.4,
            },
            {
                height: 0,
            },
            {
                height: -1,
            },
            {
                height: 0,
            },
            {
                height: 0.4,
            },
            {
                height: 0.6,
            },
            {
                height: 0.8,
            },
            {
                height: 1,
            },
            {
                height: 0.8,
            },
            {
                height: 0.6,
            },
            {
                height: 0.4,
            },
            {
                height: 0,
            },
            {
                height: -1,
            },
            {
                height: 0,
            },
            {
                height: 0.4,
            },
            {
                height: 0.6,
            },
            {
                height: 0.8,
            },
            {
                height: 1,
            },
            {
                height: 0.8,
            },
            {
                height: 0.6,
            },
            {
                height: 0.4,
            },
            {
                height: 0,
            },
            {
                height: -1,
            },
            {
                height: 0,
            },
            {
                height: 0.4,
            },
            {
                height: 0.6,
            },
            {
                height: 0.8,
            },
            {
                height: 1,
            },
            {
                height: 0.8,
            },
        ],
        line2: 3,
        line3: 3
    };

    let pixel = 30 + document.querySelector('.content').offsetLeft;
    let height = 70 + document.querySelector('.content').offsetTop;
    data.line1.forEach(element => {
        let button = document.createElement("button");
        button.className = "round-button";
        button.style.position = "absolute";
        button.style.left = `${pixel}px`;
        button.style.top = `${height + random_int(-40, 40)}px`;
        document.querySelector('.content').appendChild(button);
        pixel += 70;
    });
}



build_path();