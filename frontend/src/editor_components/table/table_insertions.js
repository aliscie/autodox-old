//https://codepen.io/Yash7/details/yozPxB


/*DELETES ROW*/

function deleteRow(id, row) {
    var table = document.getElementById(id);
    var i = row.parentNode.parentNode.rowIndex;
    var len = table.rows.length;
    if (len == 3) {
        alert('Cannot Delete');
        return;
    }
    table.deleteRow(i);
    for (a = i; a < table.rows.length; a++) {
        table.rows[a].cells[1].innerHTML = a - 1;
    }
}

/*DELETES COLUMN*/

function deleteCol(id,col) {
    var table = document.getElementById(id);
    var i = col.cellIndex;
    var len = table.rows.length;
    var length = table.rows[0].cells.length;
    if (length == 3) {
        alert('Cannot Delete');
        return;
    }
    for (var a = 0; a < len; a++) {
        table.rows[a].deleteCell(i);
    }
}

/*INSERTS ROWS*/

export function insRow(id) {
    var table = document.getElementById(id);

    var new_row = table.rows[2].cloneNode(true);
    var len = table.rows.length;
    new_row.cells[1].innerHTML = len - 1;

    var inp1 = new_row.cells[2].getElementsByTagName('input')[0];
    inp1.id += len;                                                                //Id gets incremented by len
    inp1.value = '';
    table.appendChild(new_row);
}

/*INSERTS COLUMNS*/

export function insCol(id) {
    var table = document.getElementById(id);
    var rows = table.getElementsByTagName('tr');
    i = 0;
    while (r = rows[i++]) {
        var c = r.getElementsByTagName('td');
        var clone = c[2].cloneNode(true);
        c[0].parentNode.appendChild(clone);
    }
}