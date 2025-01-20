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

let playSvg = '<svg viewBox="0 0 512 512" xmlns="http://www.w3.org/2000/svg" fill="#ffffff"><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"></g><g id="SVGRepo_iconCarrier"><path fill="#ffffff" d="M106.854 106.002a26.003 26.003 0 0 0-25.64 29.326c16 124 16 117.344 0 241.344a26.003 26.003 0 0 0 35.776 27.332l298-124a26.003 26.003 0 0 0 0-48.008l-298-124a26.003 26.003 0 0 0-10.136-1.994z"></path></g></svg>';


function build_path() {

    let advancement = [4, 1, 5];


    let height = 120 + document.querySelector('.content').offsetTop;
    for (let i = 0; i < 3; i++) {
        seed = i + 1;
        let pixel = 30 + document.querySelector('.content').offsetLeft;
        for (let j = 0; j < 20; j++) {
            let button = document.createElement("button");
            if (advancement[i] < j) {
                button.className = "round-button-grey";
            } else if (advancement[i] === j){
                button.className = "round-button-gold";
                button.onclick = () => {
                    document.location.href = "../lecture/lecture.html";
                }
            } else {
                button.className = "round-button";
                button.onclick = () => {
                    document.location.href = "../lecture/lecture.html";
                }
            }
            button.style.position = "absolute";
            button.style.left = `${pixel}px`;
            button.style.top = `${height + random_int(-40, 40)}px`;
            button.innerHTML = playSvg;


            document.querySelector('.content').appendChild(button);
            pixel += 180;
        }
        height += document.querySelector('.content').offsetHeight / 3;
    }

}


function toggleSidebar() {
    const sidebar = document.querySelector('.sidebar');
    if (sidebar.style.width === "300px") {
        closeNav();
    } else {
        sidebar.style.width = "300px";   
    }
}

function closeNav() {
    const sidebar = document.querySelector('.sidebar');
    sidebar.style.width = "0";
}

build_path();
toggleSidebar();