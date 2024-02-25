document.getElementById('encrypt-form').addEventListener('submit', function(event) {
    event.preventDefault();
    let fileInput = document.getElementById('encrypt-file');
    let file = fileInput.files[0];
    let formData = new FormData();
    formData.append('file', file);

    fetch('/encrypt', {
        method: 'POST',
        body: formData,
    })
    .then(response => response.json())
    .then(data => console.log(data))
    .catch(error => console.error('Error:', error));
});
