//topcoat was loading the dom after the js making drawContext null
document.addEventListener("DOMContentLoaded", () => {

    //canvas
    const draw = document.getElementById("draw");
    const drawContext = draw.getContext("2d", { willReadFrequently: true });
    //preview canvas
    const preview = document.getElementById("preview");
    const previewContext = preview.getContext("2d", { willReadFrequently: true });
    //the grid with pixels and its greyscale
    const out = document.getElementById("out");
    const brush = document.getElementById("brush");
    const predict = document.getElementById("predict");
    const prediction = document.getElementById("prediction");
    drawContext.strokeStyle = "#000";
    drawContext.lineCap = "round";
    drawContext.lineJoin = "round";
    
    function reset() {
        drawContext.fillStyle = '#fff';
        drawContext.fillRect(0, 0, draw.width, draw.height);
    }

    function getPixelGrid(n = 28) {
        const grid = [];
        const { data } = drawContext.getImageData(0, 0, draw.width, draw.height);
        const width = draw.width, height = draw.height;
        for (let gridY = 0; gridY < n; gridY++) {
            const row = [];
            const y0 = Math.floor(gridY * height / n), y1 = Math.floor((gridY + 1) * height / n);
            for (let gridX = 0; gridX < n; gridX++) {
                const x0 = Math.floor(gridX * width / n), x1 = Math.floor((gridX + 1) * width / n);
                let sum = 0, count = 0;
                for (let y = y0; y < y1; y++) {
                    for (let x = x0; x < x1; x++) {
                        sum += 255 - data[(y * width + x) * 4];
                        count++;
                    }
                }
                //pixel doesnt need to be normalized here
                row.push(Math.round(sum / count));
            }
            grid.push(row);
        }
        return grid;
    }
    window.getPixelGrid = getPixelGrid;

    const n = 28;
    preview.width = n;
    preview.height = n;
    //for preview grid 
    function refresh() {
        const grid = getPixelGrid();
        const img = previewContext.createImageData(n, n);
        grid.forEach((row, y) => row.forEach((v, x) => {
            const i = (y * n + x) * 4, c = Math.round(255 - v);
            //rgb
            img.data[i] = img.data[i + 1] = img.data[i + 2] = c;
            img.data[i + 3] = 255;
        }));
        previewContext.putImageData(img, 0, 0);
        out.value = JSON.stringify(grid);
    }

    let drawing = false, last = null;

    function pos(e) {
        const r = draw.getBoundingClientRect();
        return { x: (e.clientX - r.left) * draw.width / r.width, y: (e.clientY - r.top) * draw.height / r.height };
    }

    draw.addEventListener("pointerdown", e => {
        drawing = true; draw.setPointerCapture(e.pointerId);
        last = pos(e)
        drawContext.lineWidth = Number(brush.value);
        drawContext.beginPath(); drawContext.moveTo(last.x, last.y); drawContext.lineTo(last.x + 0.01, last.y); drawContext.stroke();
        refresh();
    });

    draw.addEventListener("pointermove", e => {
        if (!drawing) return;
        const p = pos(e);
        drawContext.beginPath(); drawContext.moveTo(last.x, last.y); drawContext.lineTo(p.x, p.y); drawContext.stroke();
        last = p;
        refresh();
    });

    ['pointerup', 'pointercancel'].forEach(t =>
        draw.addEventListener(t, () => { if (drawing) { drawing = false; } })
    );

    //invoke post whenever predict is pressed
    predict.addEventListener("click", async e => {
        const response = await fetch("/draw/api", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(getPixelGrid().flat())
    });

    const result = await response.json();

    prediction.textContent = result.prediction;
    });

    document.getElementById('clear').addEventListener('click', () => { reset(); refresh(); });

    reset();
    refresh();
});



