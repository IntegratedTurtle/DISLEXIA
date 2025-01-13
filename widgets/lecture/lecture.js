function setProgressBar(value) {
    const progressBar = document.getElementById('progress-bar');
    progressBar.style.height = `${value}%`;
}

// Example usage: set the progress bar to 50%
setProgressBar(50);

function send_to_homescreen() {
    document.location.href = "../homescreen/homescreen.html";
}