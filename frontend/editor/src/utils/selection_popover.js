//const selectableTextArea = document.querySelectorAll(".selectable-text-area");
//const twitterShareBtn = document.querySelector("#twitter-share-btn");

//selectableTextArea.forEach(elem => {
    //elem.addEventListener("mouseup", selectableTextAreaMouseUp);
//});

//twitterShareBtn.addEventListener("click", twitterShareBtnClick);

//document.addEventListener("mousedown", documentMouseDown);

//function selectableTextAreaMouseUp(event) {
    //setTimeout(() => { // In order to avoid some weird behavior...
        //const selectedText = window.getSelection().toString().trim();
        //if (selectedText.length) {
            //const x = event.pageX;
            //const y = event.pageY;
            //const twitterShareBtnWidth = Number(getComputedStyle(twitterShareBtn).width.slice(0, -2));
            //const twitterShareBtnHeight = Number(getComputedStyle(twitterShareBtn).height.slice(0, -2));

            //if (document.activeElement !== twitterShareBtn) {
                //twitterShareBtn.style.left = `${x - twitterShareBtnWidth * 0.5}px`;
                //twitterShareBtn.style.top = `${y - twitterShareBtnHeight * 1.25}px`;
                //twitterShareBtn.style.display = "block";
                //twitterShareBtn.classList.add("btnEntrance");
            //} else {
                //twitterShareBtn.style.left = `${x - twitterShareBtnWidth * 0.5}px`;
                //twitterShareBtn.style.top = `${y - twitterShareBtnHeight * 0.5}px`;
            //}
        //}
    //}, 0);
//}

//function documentMouseDown(event) {
    //if (event.target.id !== "twitter-share-btn" && getComputedStyle(twitterShareBtn).display === "block") {
        //twitterShareBtn.style.display = "none";
        //twitterShareBtn.classList.remove("btnEntrance");
        //window.getSelection().empty();
    //}
//}

//function twitterShareBtnClick(event) {
    //const selectedText = window.getSelection().toString().trim();
    //if (selectedText.length) {
        //const twitterShareUrl = "https://twitter.com/intent/tweet";
        //const text = `${encodeURIComponent(selectedText)}`;
        //const currentUrl = encodeURIComponent(window.location.href);
        //const hashtags = "helloworld, test, testing";
        //const via = "CodingJrney";
        //window.open(`${twitterShareUrl}?text="${text}"&url=${currentUrl}&hashtags=${hashtags}&via=${via}`);

    //}
//}

export function my_function() {
    return "dummy data"
}
