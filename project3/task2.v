module task2(clk,reset, dack, data, dready, wr);
  input clk, reset;
  input dack;
  output reg [7:0] data;
  output reg dready, wr;
  
  parameter A=0, B=1, C=2;
  reg PS, NS;
  //read the data from the file and store in a memory
  reg [7:0] task1_memory[0:11];
  initial
    $readmemh("task1.dat", task1_memory);
  //reset the first state and the dready signal
  always@(posedge clk or posedge reset) begin
    if(reset) begin
      PS <= A;
    dready <= 0;
    end
    else
      PS <= NS;
  end
  // based on the mealy FSM
  always@(PS) begin
    case(PS)
      //state A means ready to write the data out
      A: begin
        if(~dack) begin
          NS = A;
        end
        else begin
          NS = B;
        end
        dready = 1'b1;
        wr = 1'b0;
      end
      //state B means receive the output require and write the data out
      B: begin
        NS = C;
        dready = 1'b1;
        wr = 1'b1;
      end
      //state C means already write the data out and wait to restart the cycle
      C: begin
        if(dack) begin
          NS = C;
        end
        else begin
          NS = A;
        end
        dready = 1'b0;
        wr = 1'b0;
      end
    endcase
  end
  
  integer i = 0; //index of memory
  always@(posedge clk or posedge reset) begin
    if(reset)
      i <= 0;
    else if(wr) begin // only when wr signal is 1 can write the data out and move to next index
      data <= task1_memory[i];
      i <= i + 1;
    end
  end
endmodule
