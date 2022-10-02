function fetch_pasted(editor) {
    var data = `<h1> empty </h1>`

    function handle_paste(event) {
        event.preventDefault();
        let clipboardData = event.clipboardData || window.clipboardData;
        data = clipboardData.getData('text/html');
    }

    editor.addEventListener("paste", handle_paste)

    return data
}