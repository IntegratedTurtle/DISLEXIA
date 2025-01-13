

function send_to_homescreen() {
    document.location.href = "../homescreen/homescreen.html";
}

function setProgress(percentage) {
    const progressElement = document.querySelector('.progress');
    progressElement.style.height = `${percentage}%`;
}
