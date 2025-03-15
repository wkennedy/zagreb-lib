import * as wasm from "./zagreb_lib.js";

await wasm.default();

// Simple example to demonstrate the Zagreb Graph Library in a web context

// Create a new graph instance
function createGraph() {
    console.log("createGraph");
    // Get the graph type from UI
    const graphType = document.getElementById('graphType').value;
    const vertices = parseInt(document.getElementById('numVertices').value);

    let graph;

    try {
        // Create the selected graph type
        switch(graphType) {
            case 'complete':
                graph = wasm.WasmGraph.create_complete(vertices);
                break;
            case 'cycle':
                graph = wasm.WasmGraph.create_cycle(vertices);
                break;
            case 'star':
                graph = wasm.WasmGraph.create_star(vertices);
                break;
            case 'petersen':
                graph = wasm.WasmGraph.create_petersen();
                break;
            default:
                // Create custom graph
                graph = new wasm.WasmGraph(vertices);
        }

        // Store the graph in a global variable for later use
        window.currentGraph = graph;

        // Update the UI
        document.getElementById('graphStatus').textContent = `Created ${graphType} graph with ${graph.vertex_count()} vertices`;
        document.getElementById('analyzeButton').disabled = false;

        // Update the edge inputs dropdown options
        updateEdgeDropdowns();

    } catch (error) {
        console.error("Failed to create graph:", error);
        document.getElementById('graphStatus').textContent = `Error: ${error.message || error}`;
    }
}

// Add edge to the current graph
function addEdge() {
    const vertex1 = parseInt(document.getElementById('vertex1').value);
    const vertex2 = parseInt(document.getElementById('vertex2').value);

    if (!window.currentGraph) {
        document.getElementById('edgeStatus').textContent = "Error: No graph created yet";
        return;
    }

    try {
        window.currentGraph.add_edge(vertex1, vertex2);
        document.getElementById('edgeStatus').textContent = `Added edge (${vertex1}, ${vertex2})`;
        document.getElementById('analyzeButton').disabled = false;
    } catch (error) {
        console.error("Failed to add edge:", error);
        document.getElementById('edgeStatus').textContent = `Error: ${error.message || error}`;
    }
}

// Analyze the current graph
function analyzeGraph() {
    if (!window.currentGraph) {
        document.getElementById('analysisResults').textContent = "Error: No graph created yet";
        return;
    }

    try {
        // Get analysis results
        const result = window.currentGraph.analyze();

        // Format the results
        const analysisHtml = `
      <h3>Graph Analysis Results</h3>
      <table class="analysis-table">
        <tr><td>Vertices:</td><td>${result.vertex_count}</td></tr>
        <tr><td>Edges:</td><td>${result.edge_count}</td></tr>
        <tr><td>Zagreb Index:</td><td>${result.zagreb_index}</td></tr>
        <tr><td>Minimum Degree:</td><td>${result.min_degree}</td></tr>
        <tr><td>Maximum Degree:</td><td>${result.max_degree}</td></tr>
        <tr><td>Likely Hamiltonian:</td><td>${result.is_likely_hamiltonian ? 'Yes' : 'No'}</td></tr>
        <tr><td>Likely Traceable:</td><td>${result.is_likely_traceable ? 'Yes' : 'No'}</td></tr>
        <tr><td>Independence Number (approx):</td><td>${result.independence_number}</td></tr>
        <tr><td>Zagreb Upper Bound:</td><td>${result.zagreb_upper_bound.toFixed(2)}</td></tr>
      </table>
      
      <h3>Interpretation</h3>
      <div class="interpretation">
        ${getInterpretation(result)}
      </div>
    `;

        document.getElementById('analysisResults').innerHTML = analysisHtml;
    } catch (error) {
        console.error("Failed to analyze graph:", error);
        document.getElementById('analysisResults').textContent = `Error: ${error.message || error}`;
    }
}

// Generate interpretation based on analysis results
function getInterpretation(result) {
    let interpretation = "";

    if (result.is_likely_hamiltonian) {
        interpretation += `<p>This graph likely contains a Hamiltonian cycle, meaning a path that visits every vertex exactly once and returns to the starting point. This property is useful for optimization problems and efficient routing.</p>`;
    } else if (result.is_likely_traceable) {
        interpretation += `<p>While this graph may not have a Hamiltonian cycle, it likely contains a Hamiltonian path - a path that visits every vertex exactly once without returning to the start. This is still useful for many network applications.</p>`;
    } else {
        interpretation += `<p>This graph is likely neither Hamiltonian nor traceable. This indicates a less connected structure that may have bottlenecks or inefficiencies for network traversal.</p>`;
    }

    // Add insights on Zagreb index
    interpretation += `<p>The Zagreb Index (${result.zagreb_index}) is ${result.zagreb_index / result.zagreb_upper_bound * 100 > 80 ? 'close to' : 'well below'} its theoretical upper bound of ${result.zagreb_upper_bound.toFixed(2)}. This suggests that the graph's degree distribution is ${result.zagreb_index / result.zagreb_upper_bound * 100 > 80 ? 'highly optimized' : 'not optimized'} for connectivity.</p>`;

    return interpretation;
}

// Update the edge dropdowns with the correct number of vertices
function updateEdgeDropdowns() {
    if (!window.currentGraph) {
        return;
    }

    const vertices = window.currentGraph.vertex_count();
    const vertex1Select = document.getElementById('vertex1');
    const vertex2Select = document.getElementById('vertex2');

    // Clear current options
    vertex1Select.innerHTML = '';
    vertex2Select.innerHTML = '';

    // Add options for each vertex
    for (let i = 0; i < vertices; i++) {
        const option1 = document.createElement('option');
        option1.value = i;
        option1.textContent = `Vertex ${i}`;
        vertex1Select.appendChild(option1);

        const option2 = document.createElement('option');
        option2.value = i;
        option2.textContent = `Vertex ${i}`;
        vertex2Select.appendChild(option2);
    }

    // Set defaults
    if (vertices > 1) {
        vertex1Select.value = 0;
        vertex2Select.value = 1;
    }
}

// Initialize the application when the page loads
export function initApp() {
    console.log("initApp");
    document.getElementById('createButton').addEventListener('click', createGraph);
    document.getElementById('addEdgeButton').addEventListener('click', addEdge);
    document.getElementById('analyzeButton').addEventListener('click', analyzeGraph);

    // Update graph type options display
    document.getElementById('graphType').addEventListener('change', function() {
        const graphType = this.value;
        const verticesInput = document.getElementById('numVertices');

        if (graphType === 'petersen') {
            // Petersen graph always has 10 vertices
            verticesInput.value = 10;
            verticesInput.disabled = true;
        } else {
            verticesInput.disabled = false;
        }
    });
}

// Set up the application
window.addEventListener('load', initApp);