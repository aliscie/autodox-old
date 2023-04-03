export function getCaretPosition() {
  if (window.getSelection && window.getSelection().getRangeAt) {
    var range = window.getSelection().getRangeAt(0);
    var selectedObj = window.getSelection();
    var rangeCount = 0;
    var childNodes = selectedObj.anchorNode.parentNode.childNodes;

    for (var i = 0; i <childNodes.length; i++) {
      if (childNodes[i] == selectedObj.anchorNode) {
        break;
      }

      if (childNodes[i].outerHTML)
        rangeCount += childNodes[i].outerHTML.length;
      else if (childNodes[i].nodeType == 3) {
        rangeCount += childNodes[i].textContent.length;
      }

    }
    return range.startOffset + rangeCount;
  }
  return -1;
}
