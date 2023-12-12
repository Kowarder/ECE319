module task3(clk, reset, dready, data, dreq, rd);
    input clk, reset, dready;
    input [7:0] data;
    output reg dreq, rd;
    parameter A=2'b00, B=2'b01, C=2'b10;
    reg [1:0] PS, NS; // 2-bit registers for states

    always @(posedge clk or posedge reset) begin
        if (reset) begin
            PS <= A; //reset state
            dreq <= 1; //reset signal
            rd <= 0;
        end else begin
            PS <= NS;
        end
    end

    always @(PS or dready) begin
        case (PS)
          //base on the FSM diagram
          //state A if dready=1 start this cycle
            A: NS = (dready) ? B : A;
          //state B means ready for the data
            B: NS = (dready) ? B : C;
          //state C means read the data from sender
            C: NS = A;
        endcase
    end

    always @(PS) begin
        case (PS)
          //each state's output
            A: begin
                dreq = 1'b1;
                rd = 1'b0;
            end
            B: begin
                dreq = 1'b0;
                rd = 1'b0;
            end
            C: begin
                dreq = 1'b0;
                rd = 1'b1;
            end
        endcase
    end
	//store the data which need to be stored in task3
    reg [7:0] write_data;
    always @(posedge clk) begin
        if (rd) write_data <= data;
    end
	//open the file
    integer file;
    initial begin
        file = $fopen("task3.dat", "w");
    end
	//write the file
    always @(posedge clk) begin
        if (rd) begin
            $fwrite(file, "%d\n", write_data);
        end
    end
endmodule
