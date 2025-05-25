# FFmpeg Command Builder

This Rust library provides a robust and developer-friendly way to programmatically construct complex ffmpeg command strings. Instead of manually concatenating strings, which can be error-prone and difficult to manage, this library allows you to define ffmpeg operations in a structured, type-safe manner within your Rust code.

## Key Benefits of Using `ffmpeg-stringify`

Using `ffmpeg-stringify` to generate your ffmpeg commands offers several significant advantages over manual string construction:

*   **Type Safety**: By leveraging Rust's strong type system to define ffmpeg operations and their parameters, many potential errors are caught at compile-time. This is a stark contrast to manual string concatenation, where errors often only surface at runtime.

*   **Modularity and Readability**: The library encourages a structured, node-based approach to building commands. Each operation (input, filter, output) can be defined as a distinct module or component. This makes complex ffmpeg pipelines easier to construct, understand at a glance, and reason about, significantly improving the readability of your media processing code.

*   **Maintainability**: The structured nature of command construction greatly simplifies updates and modifications. Changes to filter parameters, stream routing, or pipeline stages can be made with more confidence, as the programmatic approach reduces the risk of inadvertently breaking other parts of the command.

*   **Reduced Risk of Syntax Errors**: Programmatic generation of the ffmpeg command string minimizes the chances of typos, incorrect quoting, improper escaping of special characters, or invalid filter syntax. The library handles the precise formatting details, letting you focus on the logic.

*   **Abstraction**: `ffmpeg-stringify` provides a clear API that abstracts away the often cumbersome and intricate details of direct ffmpeg command string manipulation. You interact with well-defined Rust structs and enums, rather than wrestling with the raw ffmpeg syntax. This makes the process less error-prone and more accessible.

## Core Functionality: Nodes and the `stringify` Function

The library's core revolves around the concept of **nodes**. Each node represents a distinct part of an ffmpeg command, such as:

*   **Input files/streams**: Specifying media sources.
*   **Filters**: Defining audio or video transformations (e.g., scaling, cropping, overlaying, audio mixing).
*   **Outputs files/streams**: Specifying where the processed media should go.
*   **Stream selection/mapping**: Explicitly defining how streams are connected between different processing stages.

Once you have constructed a graph or sequence of these nodes representing your desired ffmpeg workflow, the `stringify` function comes into play.

The **`stringify` function** is the engine that translates this structured, high-level representation of operations into a single, executable ffmpeg command string. Its role is crucial:

1.  **Graph Traversal**: It intelligently traverses the interconnected nodes that define your media pipeline.
2.  **Syntax Generation**: It correctly formats various ffmpeg options, flags, filter descriptions, and input/output specifiers.
3.  **Filter_complex Management**: For pipelines involving multiple filters, `stringify` automatically constructs the appropriate `-filter_complex` syntax, correctly labeling intermediate streams and linking them between filter stages.
4.  **Stream Mapping**: It ensures that input and output streams are correctly mapped throughout the command, preventing common "no such stream" errors.
5.  **Validation (Implicit)**: While not a full ffmpeg validator, the structured nature of node creation and the `stringify` process inherently prevent many common syntax errors.

In essence, you build your ffmpeg command conceptually using Rust objects (nodes), and `stringify` handles the meticulous task of converting that concept into the precise syntax ffmpeg requires. This separation of concerns makes working with ffmpeg in Rust significantly more manageable and less error-prone.

For example, you might define an input node for a video, connect it to a scale filter node, then to an overlay filter node (which takes another input), and finally to an output node. The `stringify` function would then generate the complete ffmpeg command, including the input files, the complex filter graph with correctly named intermediate streams, and the output file, all properly formatted.

## Node Types and Structures

The library defines several key structs and enums to represent the components of an ffmpeg command. These structures provide the building blocks for creating your media processing pipelines.

### `FNode` Struct

The `FNode` struct acts as a general wrapper or container for different types of operations within the ffmpeg command graph.

*   `name: String`: A unique identifier for this node within the graph. This is crucial for connecting nodes and for the `stringify` function to correctly map inputs and outputs.
*   `data: FNodeType`: The actual operation this node represents, which can be either a media stream (input/output) or a filter operation.

Think of `FNode` as a vertex in your processing graph, holding the specific details of the operation.

### `FNodeType` Enum

This is a central enum that distinguishes between the two primary types of operations you can define:

*   `Stream(Stream)`: Represents a media stream, typically an input file (e.g., `input.mp4`), an output file (e.g., `output.mkv`), or a network stream.
*   `FilterNode(FilterNode)`: Represents one or more ffmpeg filters applied to input streams, producing output streams. This is the workhorse for transformations like scaling, cropping, mixing, etc.

The `FNodeType` allows `FNode` to generically hold different kinds of ffmpeg operations.

### `Stream` Struct

The `Stream` struct defines an input or output point in your ffmpeg command.

*   `path: String`: The path to the file or the identifier for the stream (e.g., `"input.mp4"`, `"rtmp://server/live"`).
*   `name: String`: A label for this stream (e.g., `"input_video"`, `"main_audio_output"`). This is used internally for linking and can be helpful for clarity.
*   `stream_type: String`: Indicates the type of stream, typically `"input"` or `"output"`.
*   `inputs: Option<Vec<String>>`: For output streams, this field can optionally specify which nodes or streams feed into this output. This helps in explicitly defining connections in the command graph. For input streams, this is usually `None`.

### `FilterNode` Struct

A `FilterNode` represents a single ffmpeg filter operation or a chain of connected filters that are applied as part of a `-filter_complex` block.

*   `name: String`: A unique name for this filter node (e.g., `"scaling_filter"`, `"audio_effects"`). This name is used to label the output streams of this filter node, allowing them to be referenced by other filter nodes or output streams.
*   `inputs: Vec<String>`: A list of names identifying the input streams for this filter operation. These names typically refer to the `name` of `FNode`s (which could be `Stream`s or other `FilterNode`s' outputs).
*   `outputs: Vec<String>`: A list of names that will be used to label the output streams produced by this filter or filter chain. These labels can then be used as inputs for subsequent `FilterNode`s or mapped to final output files. For example, if a filter node is named "mysplit" and has `outputs: vec!["out1".to_string(), "out2".to_string()]`, its outputs can be referred to as `[mysplit:out1]` and `[mysplit:out2]` (syntax might vary depending on `stringify` implementation details).
*   `filters: Vec<Filter>`: A vector of `Filter` structs, defining the actual sequence of filter operations to be applied within this `FilterNode`.

### `Filter` Struct

The `Filter` struct defines a single ffmpeg filter with its specific parameters.

*   `name: String`: The name of the ffmpeg filter (e.g., `"scale"`, `"apad"`, `"overlay"`).
*   `options: FilterOptions`: The parameters or arguments for this filter.

For example, a scale filter might be defined as `Filter { name: "scale", options: FilterOptions::HashMap(...) }` where the HashMap contains width and height.

### `FilterOptions` Enum

This enum provides flexibility in how filter parameters are specified:

*   `HashMap(std::collections::HashMap<String, String>)`: Allows specifying filter options as key-value pairs. For example, `{"width": "1280", "height": "720"}` for a scale filter. This is generally the preferred, more structured way.
*   `String(String)`: Allows specifying filter options as a single pre-formatted string. This can be useful for filters with complex or non-standard parameter syntax, or when you want to pass a raw options string directly. For example, `"width=1280:height=720"`.

These structures work together to allow you to define a complex ffmpeg processing pipeline in a structured and manageable way before it's converted into a command string by the `stringify` function.

## FFmpeg Command Generation: The `stringify` Process

The `stringify` function is responsible for transforming the collection of `FNode`s (representing your desired ffmpeg operations) into a valid ffmpeg command string. The process generally follows these steps:

1.  **Initialization**:
    *   The function starts with the base `ffmpeg` command.
    *   It initializes empty lists or strings to accumulate input arguments, filter complex parts, and output arguments.

2.  **Processing Input Streams**:
    *   It iterates through all provided `FNode`s.
    *   If an `FNode` contains an `FNodeType::Stream` with `stream_type` indicating "input" (e.g., `StreamType::Input` or a similar convention):
        *   The `Stream.path` is used to form an input argument: `-i <path>`.
        *   This argument is added to the list of input arguments.
        *   The `Stream.name` (or a generated unique ID based on its FNode name or index) is recorded. This name acts as the initial label for streams originating from this input (e.g., `[0:v]`, `[0:a]` if the input is the first one, or a more descriptive label like `[input_video_0]`). The exact labeling convention might depend on the graph's complexity and whether explicit stream specifiers (like `0:v:0`) are derived from the `Stream.name` or managed internally by `stringify`.

3.  **Constructing the `-filter_complex` Argument**:
    *   This is typically the most complex part of the generation. The function identifies all `FNode`s that are of type `FNodeType::FilterNode`.
    *   For each `FilterNode`:
        *   **Input Pads**: The `FilterNode.inputs` field provides a list of strings. These strings are names that refer to the output of other nodes (either input `Stream`s or other `FilterNode`s). The `stringify` function resolves these names to the correct ffmpeg stream labels. For example, an input `FNode` named "my_video_input" might provide a video stream labeled `[my_video_input:v]`. If a `FilterNode.inputs` contains "my_video_input:v", this label is used directly. If it just contains "my_video_input", `stringify` might append a default stream specifier like `:v` or assume the primary stream from that node. Multiple inputs are formatted as separate pads, e.g., `[in1][in2]`.
        *   **Filter Chaining**: Inside a single `FilterNode`, the `FilterNode.filters` vector defines a sequence of individual ffmpeg filters. These are chained together.
            *   The output of the first filter in the chain implicitly takes the input pads generated above.
            *   If there are multiple filters within the `FilterNode`, intermediate, unnamed pads are typically used to link them (e.g., `[in]filter1[mid1]; [mid1]filter2[mid2]; [mid2]filter3[out]`). However, the library might simplify this by concatenating filters with commas if they form a linear chain without explicit intermediate output naming within that `FilterNode`.
        *   **Individual Filter Formatting**: For each `Filter` struct in `FilterNode.filters`:
            *   The `Filter.name` is written (e.g., `scale`, `overlay`).
            *   The `Filter.options` are formatted:
                *   If `FilterOptions::HashMap`, key-value pairs are converted to `key=value` strings, typically joined by colons (e.g., `width=1280:height=720`). Special characters in values might be escaped if necessary.
                *   If `FilterOptions::String`, the raw string is used directly (e.g., `1280:720`).
            *   The complete filter string becomes `filter_name=options_string` (e.g., `scale=width=1280:height=720`).
        *   **Output Pads**: The `FilterNode.outputs` field provides a list of strings that will serve as labels for the streams produced by this `FilterNode`. For example, if `FilterNode.name` is "scaler" and `FilterNode.outputs` is `["scaled_vid"]`, the final output of the filter chain within this `FilterNode` will be labeled `[scaled_vid]`. If there are multiple outputs (e.g., from a `split` filter), they are listed like `[out_label1][out_label2]`. These labels are crucial for mapping these processed streams to output files or to subsequent filter nodes.
        *   The complete string for a single `FilterNode` (representing one segment of the `-filter_complex` graph) looks something like: `[in_pad1][in_pad2]filter1=opts,filter2=opts[out_pad1][out_pad2]`.
    *   All such strings generated from individual `FilterNode`s are joined together with semicolons `;` to form the final `-filter_complex` argument value.

4.  **Processing Output Streams**:
    *   The function iterates through all `FNode`s again.
    *   If an `FNode` contains an `FNodeType::Stream` with `stream_type` indicating "output" (e.g., `StreamType::Output`):
        *   **Stream Mapping (`-map`)**: The `Stream.inputs` field (which is `Option<Vec<String>>`) is crucial here. It should contain a list of labels that refer to the output pads of `FilterNode`s or input streams that are to be included in this output file.
            *   For each label in `Stream.inputs` (e.g., `"scaled_vid"`), a `-map '[label]'` argument is generated (e.g., `-map '[scaled_vid]'`).
            *   If `Stream.inputs` is `None` or empty, the library might have a default mapping behavior (e.g., mapping the "last" processed stream or an error), or it might require explicit mapping.
        *   These `-map` arguments are added to the list of output arguments.
        *   **Output Path**: The `Stream.path` is added as the final part of this output segment (e.g., `output.mp4`).
        *   Any other output-specific options (e.g., codecs, bitrates, if supported by `Stream` or an associated options structure) would also be formatted and added here.

5.  **Assembling the Final Command**:
    *   The `stringify` function assembles the full command string in the typical ffmpeg order:
        1.  `ffmpeg`
        2.  Global options (if any are supported and defined)
        3.  All input arguments (`-i input1.mp4 -i input2.mov ...`)
        4.  The `-filter_complex "..."` argument, if any `FilterNode`s were processed.
        5.  All output arguments, which include stream mappings and output paths (`-map '[label1]' outputA.mkv -map '[label2]' outputB.mp4 ...`).
    *   The resulting string is returned.

This structured approach ensures that all inputs are declared, filters are correctly chained and labeled, and outputs are explicitly mapped from the desired processed streams, producing a coherent and often complex ffmpeg command.

## Future Enhancements

While `ffmpeg-stringify` currently provides a solid foundation for building ffmpeg commands, there are several areas where its functionality could be expanded and improved in the future:

*   **Advanced Graph Validation**:
    *   Implement more sophisticated checks for the validity of node connections and filter chains *before* generating the command. This could include ensuring stream types match between connected nodes (e.g., video to video), verifying the correct number of inputs and outputs for specific, known filters, and checking for dangling or disconnected nodes in the graph.

*   **Broader FFmpeg Feature Support**:
    *   Systematically expand the library to cover a wider range of FFmpeg options, such as global flags (e.g., `-y` for overwrite, `-loglevel`), input/output options (e.g., `-c:v`, `-b:a`, `-f`), and a more comprehensive list of available audio, video, and subtitle filters with typed parameters.

*   **Helper Functions/Macros for Common Operations**:
    *   Introduce convenient high-level functions, builders, or macros for frequently used filter chains or complex setups. For example, a function `add_watermark(input_stream, watermark_image_path, position)` could abstract a common overlay operation.

*   **Preset System**:
    *   Allow users to define, save, and reuse common filter configurations or entire node setups as presets. This would enable quick application of standard effects or output settings.

*   **Enhanced Error Reporting**:
    *   Improve error messages when invalid configurations or node structures are provided. Errors could pinpoint the problematic node or filter more precisely and offer more context or suggestions for correction, making debugging easier.

*   **Asynchronous Support**:
    *   Explore options for better integration into asynchronous Rust applications. While ffmpeg execution itself is often a blocking process, providing async wrappers or ensuring the library plays well within async runtimes could be beneficial for certain use cases, such as managing multiple ffmpeg processes.

*   **Auto-detection of Stream Types**:
    *   Investigate the feasibility of inferring basic stream types (e.g., video, audio) for input files if not explicitly specified by the user. This could simplify input node creation for common media files but would need to handle ambiguity and potential inaccuracies gracefully. This is a complex feature and would require careful consideration.

*   **Integration with FFmpeg Tooling**:
    *   Consider ways to integrate with ffmpeg's own capabilities, such as using `ffprobe` to gather media information for validation or to inform default settings.

Contributions and ideas in these areas, or others, would be welcome to make the library even more powerful and user-friendly.
