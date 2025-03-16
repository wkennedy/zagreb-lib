let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_3.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

let cachedUint32ArrayMemory0 = null;

function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}
/**
 * Make a JS-compatible string list of low connectivity validators
 * @param {WasmGraph} graph
 * @returns {Uint32Array}
 */
export function get_low_connectivity_validators(graph) {
    _assertClass(graph, WasmGraph);
    const ret = wasm.get_low_connectivity_validators(graph.__wbg_ptr);
    var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

const GraphAnalysisResultFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_graphanalysisresult_free(ptr >>> 0, 1));
/**
 * Graph analysis result to be returned to JavaScript
 */
export class GraphAnalysisResult {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GraphAnalysisResult.prototype);
        obj.__wbg_ptr = ptr;
        GraphAnalysisResultFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GraphAnalysisResultFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_graphanalysisresult_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get vertex_count() {
        const ret = wasm.graphanalysisresult_vertex_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get edge_count() {
        const ret = wasm.graphanalysisresult_edge_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get zagreb_index() {
        const ret = wasm.graphanalysisresult_zagreb_index(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get min_degree() {
        const ret = wasm.graphanalysisresult_min_degree(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get max_degree() {
        const ret = wasm.graphanalysisresult_max_degree(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {boolean}
     */
    get is_likely_hamiltonian() {
        const ret = wasm.graphanalysisresult_is_likely_hamiltonian(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get is_likely_traceable() {
        const ret = wasm.graphanalysisresult_is_likely_traceable(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {number}
     */
    get independence_number() {
        const ret = wasm.graphanalysisresult_independence_number(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get zagreb_upper_bound() {
        const ret = wasm.graphanalysisresult_zagreb_upper_bound(this.__wbg_ptr);
        return ret;
    }
}

const WasmErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmerror_free(ptr >>> 0, 1));
/**
 * A simple error type for WASM interfaces
 */
export class WasmError {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmError.prototype);
        obj.__wbg_ptr = ptr;
        WasmErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmErrorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmerror_free(ptr, 0);
    }
    /**
     * @param {string} message
     */
    constructor(message) {
        const ptr0 = passStringToWasm0(message, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmerror_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        WasmErrorFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {string}
     */
    get message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmerror_message(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const WasmGraphFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmgraph_free(ptr >>> 0, 1));
/**
 * WASM bindings for creating and manipulating graphs
 */
export class WasmGraph {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmGraph.prototype);
        obj.__wbg_ptr = ptr;
        WasmGraphFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmGraphFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmgraph_free(ptr, 0);
    }
    /**
     * Create a new empty graph with n vertices
     * @param {number} n
     */
    constructor(n) {
        const ret = wasm.wasmgraph_new(n);
        this.__wbg_ptr = ret >>> 0;
        WasmGraphFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Add an edge between vertices u and v
     * @param {number} u
     * @param {number} v
     */
    add_edge(u, v) {
        const ret = wasm.wasmgraph_add_edge(this.__wbg_ptr, u, v);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * Get the degree of a vertex
     * @param {number} v
     * @returns {number}
     */
    degree(v) {
        const ret = wasm.wasmgraph_degree(this.__wbg_ptr, v);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * Calculate the first Zagreb index of the graph
     * @returns {number}
     */
    first_zagreb_index() {
        const ret = wasm.wasmgraph_first_zagreb_index(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get the minimum degree of the graph
     * @returns {number}
     */
    min_degree() {
        const ret = wasm.wasmgraph_min_degree(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get the maximum degree of the graph
     * @returns {number}
     */
    max_degree() {
        const ret = wasm.wasmgraph_max_degree(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Check if the graph is k-connected
     * @param {number} k
     * @param {boolean} use_exact
     * @returns {boolean}
     */
    is_k_connected(k, use_exact) {
        const ret = wasm.wasmgraph_is_k_connected(this.__wbg_ptr, k, use_exact);
        return ret !== 0;
    }
    /**
     * Check if the graph is likely Hamiltonian
     * @param {boolean} use_exact_connectivity
     * @returns {boolean}
     */
    is_likely_hamiltonian(use_exact_connectivity) {
        const ret = wasm.wasmgraph_is_likely_hamiltonian(this.__wbg_ptr, use_exact_connectivity);
        return ret !== 0;
    }
    /**
     * Check if the graph is likely traceable
     * @param {boolean} use_exact_connectivity
     * @returns {boolean}
     */
    is_likely_traceable(use_exact_connectivity) {
        const ret = wasm.wasmgraph_is_likely_traceable(this.__wbg_ptr, use_exact_connectivity);
        return ret !== 0;
    }
    /**
     * Calculate independence number (approximate)
     * @returns {number}
     */
    independence_number_approx() {
        const ret = wasm.wasmgraph_independence_number_approx(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Calculate upper bound on Zagreb index
     * @returns {number}
     */
    zagreb_upper_bound() {
        const ret = wasm.wasmgraph_zagreb_upper_bound(this.__wbg_ptr);
        return ret;
    }
    /**
     * Get the number of vertices
     * @returns {number}
     */
    vertex_count() {
        const ret = wasm.wasmgraph_vertex_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get the number of edges
     * @returns {number}
     */
    edge_count() {
        const ret = wasm.wasmgraph_edge_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Analyze the graph and return a comprehensive result object
     * @returns {GraphAnalysisResult}
     */
    analyze() {
        const ret = wasm.wasmgraph_analyze(this.__wbg_ptr);
        return GraphAnalysisResult.__wrap(ret);
    }
    /**
     * Create a complete graph with n vertices
     * @param {number} n
     * @returns {WasmGraph}
     */
    static create_complete(n) {
        const ret = wasm.wasmgraph_create_complete(n);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmGraph.__wrap(ret[0]);
    }
    /**
     * Create a cycle graph with n vertices
     * @param {number} n
     * @returns {WasmGraph}
     */
    static create_cycle(n) {
        const ret = wasm.wasmgraph_create_cycle(n);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmGraph.__wrap(ret[0]);
    }
    /**
     * Create a star graph with n vertices
     * @param {number} n
     * @returns {WasmGraph}
     */
    static create_star(n) {
        const ret = wasm.wasmgraph_create_star(n);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmGraph.__wrap(ret[0]);
    }
    /**
     * Create the Petersen graph
     * @returns {WasmGraph}
     */
    static create_petersen() {
        const ret = wasm.wasmgraph_create_petersen();
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmGraph.__wrap(ret[0]);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_wasmerror_new = function(arg0) {
        const ret = WasmError.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_3;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('zagreb_lib_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
