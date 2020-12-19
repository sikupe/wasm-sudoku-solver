import("../pkg/index.js").catch(console.error);

(async function () {
    const solver = await import("../pkg");

    let sudoku;

    const solve = () => {
        let solution = solver.solve({"sudoku": sudoku});
        updateTable("output", solution.sudoku);
    };

    const generate = async () => {
        let res = await fetch('https://sugoku.herokuapp.com/board?difficulty=hard')
        let fetched_sudoku = await res.json();
        sudoku = fetched_sudoku.board;
        updateTable("input", sudoku);
        updateTable("output", sudoku);
    };

    const updateTable = (tableId, values) => {
        let table = document.getElementById(tableId);
        let rows = table.getElementsByTagName("tr");
        for (let i = 0; i < 9; i++) {
            let row = rows[i].getElementsByTagName("td");
            let value_row = values[i];
            for (let j = 0; j < 9; j++) {
                let field = row[j];
                field.innerHTML = value_row[j];
            }
        }
    };


    document.getElementById("generate").addEventListener("click", generate);
    document.getElementById("solve").addEventListener("click", solve);

})();
