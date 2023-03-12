export function editor_toolbar() {
    const selectableTextArea = document.querySelectorAll(".text_editor_container");
    if (!selectableTextArea.length) {
        return "None"
    }
    const toolbar = document.querySelector("#selection-popper");

    selectableTextArea.forEach(elem => {
        elem.addEventListener("mouseup", selectableTextAreaMouseUp);
    });

    toolbar.addEventListener("click", toolbarClick);

    document.addEventListener("mousedown", documentMouseDown);

    function selectableTextAreaMouseUp(event) {
        setTimeout(() => { // In order to avoid some weird behavior...
            const selectedText = window.getSelection().toString().trim();
            if (selectedText.length) {
                const x = event.pageX;
                const y = event.pageY;
                const rect = toolbar.getBoundingClientRect();
                const toolbar_width = 100;
                const toolbar_height = 44;

                if (document.activeElement !== toolbar) {
                    if (x + 120 > window.innerWidth) {
                        toolbar.style.left = `${x - toolbar_width}px`;
                    } else {
                        toolbar.style.left = `${x}px`;
                    }
                    toolbar.style.top = `${y - 33}px`;
                    toolbar.style.display = "block";
                    toolbar.classList.add("btnEntrance");
                } else {
                    toolbar.style.left = `${x - toolbar_width * 0.5}px`;
                    toolbar.style.top = `${y - toolbar_height * 0.5}px`;
                }
            }
        }, 0);
    }

    function documentMouseDown(event) {
        if (event.target.id !== "selection-popper" && getComputedStyle(toolbar).display === "block") {
            if (event.srcElement.nodeName != "INPUT") {
                toolbar.style.display = "none"
            }
            ;
            toolbar.classList.remove("btnEntrance");
            window.getSelection().empty();
        }
    }

    function toolbarClick(event) {
        const selectedText = window.getSelection().toString().trim();
        if (selectedText.length) {
            // General Twitter Share URL: https://twitter.com/intent/tweet?text={title}&url={url}&hashtags={hash_tags}&via={user_id}

            // Alternatively, we could include everything in the "text" field -> more room to customize the tweet:
            // window.open(`${twitterShareUrl}?text="${text}" by @${via} ${hashtags.split(",").map(h => "%23"+h.trim()).join(" ")} ${currentUrl}`);

            // We could also specify new window features:
            // const newWindowOptions = "height=400,width=550,top=0,left=0,resizable=yes,scrollbars=yes";
            // window.open(`${twitterShareUrl}?text="${text}"&url=${currentUrl}&hashtags=${hashtags}&via=${via}`, "ShareOnTwitter", newWindowOptions);
        }
    }

    console.log("editor_toolbars 3 ")

    return "bold"
}
