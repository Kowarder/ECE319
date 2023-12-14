module fifo_buffer (
    input clk,
    input reset,
    input wr_en,  // Write enable
    input rd_en,  // Read enable
    input [7:0] data_in,  // Data input
    output reg [7:0] data_out,  // Data output
    output full,  // Indicates FIFO is full
    output empty  // Indicates FIFO is empty
);

    // FIFO buffer parameters
    parameter FIFO_DEPTH = 16;
    reg [7:0] fifo_mem[FIFO_DEPTH-1:0];
    reg [4:0] write_ptr, read_ptr, count;

    // Write logic
    always @(posedge clk or posedge reset) begin
        if (reset) begin
            write_ptr <= 0;
            count <= 0;
        end else if (wr_en && !full) begin
            fifo_mem[write_ptr] <= data_in;
            write_ptr <= write_ptr + 1;
            count <= count + 1;
        end
    end

    // Read logic
    always @(posedge clk or posedge reset) begin
        if (reset) begin
            read_ptr <= 0;
        end else if (rd_en && !empty) begin
            data_out <= fifo_mem[read_ptr];
            read_ptr <= read_ptr + 1;
            count <= count - 1;
        end
    end

    // Full and empty flags
    assign full = (count == FIFO_DEPTH);
    assign empty = (count == 0);

endmodule
