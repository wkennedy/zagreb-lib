import * as wasm from "./zagreb_lib.js";
import * as d3 from "https://cdn.jsdelivr.net/npm/d3@7/+esm";

await wasm.default();

// Graph visualization variables
let simulation = null;
let svg = null;
let graphData = { nodes: [], links: [] };

// Initialize the visualization
function initVisualization() {
    const width = document.getElementById('graphVisualization').clientWidth;
    const height = 400;

    // Clear any existing SVG
    d3.select('#graphVisualization').selectAll('*').remove();

    // Create new SVG
    svg = d3.select('#graphVisualization')
        .append('svg')
        .attr('width', width)
        .attr('height', height)
        .attr('viewBox', [0, 0, width, height]);

    // Add a group for zoom/pan functionality
    const g = svg.append('g');

    // Add zoom behavior
    svg.call(d3.zoom()
        .extent([[0, 0], [width, height]])
        .scaleExtent([0.5, 5])
        .on('zoom', (event) => {
            g.attr('transform', event.transform);
        }));

    // Initialize force simulation
    simulation = d3.forceSimulation()
        .force('link', d3.forceLink().id(d => d.id).distance(80))
        .force('charge', d3.forceManyBody().strength(-300))
        .force('center', d3.forceCenter(width / 2, height / 2))
        .force('collision', d3.forceCollide().radius(30));

    return g;
}

// Update the visualization with new graph data
function updateVisualization() {
    if (!graphData.nodes.length) return;

    const g = initVisualization();

    // Create links
    const link = g.append('g')
        .attr('stroke', '#999')
        .attr('stroke-opacity', 0.6)
        .selectAll('line')
        .data(graphData.links)
        .join('line')
        .attr('stroke-width', 1.5);

    // Create nodes
    const node = g.append('g')
        .attr('stroke', '#fff')
        .attr('stroke-width', 1.5)
        .selectAll('circle')
        .data(graphData.nodes)
        .join('circle')
        .attr('r', 15)
        .attr('fill', d => {
            // Color nodes based on degree
            const degreeColors = d3.scaleLinear()
                .domain([0, d3.max(graphData.nodes, d => d.degree)])
                .range(['#4575b4', '#d73027']);
            return degreeColors(d.degree);
        })
        .call(drag(simulation));

    // Add vertex labels
    const labels = g.append('g')
        .selectAll('text')
        .data(graphData.nodes)
        .join('text')
        .text(d => d.id)
        .attr('font-size', '10px')
        .attr('text-anchor', 'middle')
        .attr('dy', '0.35em')
        .attr('fill', '#fff');

    // Add title/tooltip for nodes
    node.append('title')
        .text(d => `Vertex ${d.id}\nDegree: ${d.degree}`);

    // Set up force simulation update
    simulation.nodes(graphData.nodes)
        .on('tick', () => {
            link
                .attr('x1', d => d.source.x)
                .attr('y1', d => d.source.y)
                .attr('x2', d => d.target.x)
                .attr('y2', d => d.target.y);

            node
                .attr('cx', d => d.x)
                .attr('cy', d => d.y);

            labels
                .attr('x', d => d.x)
                .attr('y', d => d.y);
        });

    simulation.force('link').links(graphData.links);
    simulation.alpha(1).restart();

    // Add legend for degree colors
    const legendData = [
        { text: 'Low Degree', color: '#4575b4' },
        { text: 'High Degree', color: '#d73027' }
    ];

    const legend = svg.append('g')
        .attr('transform', 'translate(20, 20)');

    legend.selectAll('rect')
        .data(legendData)
        .join('rect')
        .attr('x', 0)
        .attr('y', (d, i) => i * 20)
        .attr('width', 15)
        .attr('height', 15)
        .attr('fill', d => d.color);

    legend.selectAll('text')
        .data(legendData)
        .join('text')
        .attr('x', 20)
        .attr('y', (d, i) => i * 20 + 12)
        .text(d => d.text)
        .attr('font-size', '12px');
}

// Implement drag functionality for nodes
function drag(simulation) {
    function dragstarted(event) {
        if (!event.active) simulation.alphaTarget(0.3).restart();
        event.subject.fx = event.subject.x;
        event.subject.fy = event.subject.y;
    }

    function dragged(event) {
        event.subject.fx = event.x;
        event.subject.fy = event.y;
    }

    function dragended(event) {
        if (!event.active) simulation.alphaTarget(0);
        event.subject.fx = null;
        event.subject.fy = null;
    }

    return d3.drag()
        .on('start', dragstarted)
        .on('drag', dragged)
        .on('end', dragended);
}

// Convert the WASM graph to a D3-friendly format
function convertGraphToD3Format(graph) {
    const nodes = [];
    const links = [];

    // Create nodes
    for (let i = 0; i < graph.vertex_count(); i++) {
        nodes.push({
            id: i,
            degree: graph.degree(i)
        });
    }

    // Create links (we need to check each possible edge)
    for (let i = 0; i < graph.vertex_count(); i++) {
        for (let j = i + 1; j < graph.vertex_count(); j++) {
            try {
                // This is inefficient but works for demo purposes
                // In a real app, we'd have a method to get all edges
                graph.degree(j); // This will throw if the vertex doesn't exist

                // Check if there's an edge between i and j
                const degreeBefore = graph.degree(i);
                const tempGraph = graph.clone ? graph.clone() : new wasm.WasmGraph(graph.vertex_count());
                if (!tempGraph.clone) {
                    // If clone isn't available, create a similar graph
                    for (let a = 0; a < graph.vertex_count(); a++) {
                        for (let b = a + 1; b < graph.vertex_count(); b++) {
                            try {
                                if (a !== i || b !== j) { // Skip the edge we're checking
                                    tempGraph.add_edge(a, b);
                                }
                            } catch (e) {
                                // Edge may not exist
                            }
                        }
                    }
                }

                try {
                    tempGraph.add_edge(i, j);
                    if (graph.degree(i) !== degreeBefore) {
                        // If adding the edge changed the degree, it didn't exist before
                        links.push({
                            source: i,
                            target: j
                        });
                    }
                } catch (e) {
                    // Edge already exists
                    links.push({
                        source: i,
                        target: j
                    });
                }
            } catch (e) {
                // Vertex doesn't exist, skip
            }
        }
    }

    return { nodes, links };
}

// Simple example to demonstrate the Zagreb Graph Library in a web context

// Create a new graph instance
function createGraph() {
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

        // Update the graph visualization data
        graphData = createGraphDataFromScratch(graph);
        updateVisualization();

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

// Create graph data manually for better performance
function createGraphDataFromScratch(graph) {
    const nodes = [];
    const links = [];

    // Create nodes
    for (let i = 0; i < graph.vertex_count(); i++) {
        try {
            nodes.push({
                id: i,
                degree: graph.degree(i)
            });
        } catch (e) {
            console.error(`Error getting degree for vertex ${i}:`, e);
        }
    }

    // For specific graph types, we can directly create the links
    const graphType = document.getElementById('graphType').value;

    switch(graphType) {
        case 'complete':
            for (let i = 0; i < nodes.length; i++) {
                for (let j = i + 1; j < nodes.length; j++) {
                    links.push({ source: i, target: j });
                }
            }
            break;

        case 'cycle':
            for (let i = 0; i < nodes.length; i++) {
                links.push({
                    source: i,
                    target: (i + 1) % nodes.length
                });
            }
            break;

        case 'star':
            for (let i = 1; i < nodes.length; i++) {
                links.push({ source: 0, target: i });
            }
            break;

        case 'petersen':
            // Outer pentagon
            for (let i = 0; i < 5; i++) {
                links.push({
                    source: i,
                    target: (i + 1) % 5
                });
            }

            // Spokes
            for (let i = 0; i < 5; i++) {
                links.push({ source: i, target: i + 5 });
            }

            // Inner pentagram
            links.push({ source: 5, target: 7 });
            links.push({ source: 7, target: 9 });
            links.push({ source: 9, target: 6 });
            links.push({ source: 6, target: 8 });
            links.push({ source: 8, target: 5 });
            break;

        default:
            // For custom graphs, we'll add links as they're added
            // This will be empty initially
            break;
    }

    return { nodes, links };
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

        // Update graph visualization
        // First, update node degrees
        for (const node of graphData.nodes) {
            try {
                node.degree = window.currentGraph.degree(node.id);
            } catch (e) {
                console.error(`Error updating degree for node ${node.id}:`, e);
            }
        }

        // Then add the new link if it doesn't exist yet
        const linkExists = graphData.links.some(link =>
            (link.source.id === vertex1 && link.target.id === vertex2) ||
            (link.source.id === vertex2 && link.target.id === vertex1)
        );

        if (!linkExists) {
            graphData.links.push({
                source: vertex1,
                target: vertex2
            });
        }

        // Update the visualization
        updateVisualization();
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

        // Highlight nodes based on analysis results
        highlightNodes(result);
    } catch (error) {
        console.error("Failed to analyze graph:", error);
        document.getElementById('analysisResults').textContent = `Error: ${error.message || error}`;
    }
}

// Highlight nodes based on analysis results
function highlightNodes(result) {
    if (!svg) return;

    // First, reset all nodes to their degree-based coloring
    svg.selectAll('circle')
        .transition()
        .duration(500)
        .attr('stroke', '#fff')
        .attr('stroke-width', 1.5)
        .attr('r', 15);

    // Find nodes with minimum degree (potential bottlenecks)
    const minDegreeNodes = graphData.nodes
        .filter(node => node.degree === result.min_degree)
        .map(node => node.id);

    // Find nodes with maximum degree (potential hubs)
    const maxDegreeNodes = graphData.nodes
        .filter(node => node.degree === result.max_degree)
        .map(node => node.id);

    // Highlight min degree nodes with yellow stroke
    svg.selectAll('circle')
        .filter(d => minDegreeNodes.includes(d.id))
        .transition()
        .duration(500)
        .attr('stroke', '#ffc107')
        .attr('stroke-width', 3);

    // Highlight max degree nodes with green stroke
    svg.selectAll('circle')
        .filter(d => maxDegreeNodes.includes(d.id))
        .transition()
        .duration(500)
        .attr('stroke', '#28a745')
        .attr('stroke-width', 3);
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
    const zagrebPercent = (result.zagreb_index / result.zagreb_upper_bound * 100).toFixed(1);
    interpretation += `<p>The Zagreb Index (${result.zagreb_index}) is ${zagrebPercent}% of its theoretical upper bound of ${result.zagreb_upper_bound.toFixed(2)}. ${
        zagrebPercent > 80
            ? 'This suggests that the graph\'s degree distribution is highly optimized for connectivity.'
            : 'This suggests the graph\'s degree distribution could be further optimized for connectivity.'
    }</p>`;

    // Add info about degree distribution
    interpretation += `<p>The degree distribution ranges from ${result.min_degree} to ${result.max_degree}. ${
        result.min_degree === result.max_degree
            ? 'This is a regular graph, with all vertices having the same degree.'
            : 'This is not a regular graph, as vertices have different degrees.'
    }</p>`;

    // Add blockchain-specific insights if applicable
    interpretation += `<p>In a blockchain network context, this topology ${
        result.is_likely_hamiltonian
            ? 'allows for efficient message propagation and leader rotation sequences.'
            : 'may require more complex gossip protocols to ensure timely message propagation.'
    }</p>`;

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

    // Initialize the visualization
    initVisualization();
}

// Set up the application
window.addEventListener('load', initApp);