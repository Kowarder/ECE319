// Code your testbench here
// or browse Examples
module task1_tb();
  reg clk, reset, dack;
  wire [7:0] data;
  wire dready, wr;
  
  task1 uut(.clk(clk), .reset(reset), .dack(dack), .data(data), .dready(dready));
  
  always #5 clk = ~clk;
  
  initial begin
    reset = 1;
    clk = 0;
    #5
    reset = 0;
    dack = 1'b0;
    #5
    dack = 1'b1;
    #15
    dack = 1'b0;
    #5
    dack = 1'b1;
    #10
    dack = 1'b0;
    #5
    dack = 1'b1;
    #120
    $stop;
  end
//  initial begin
 // $dumpfile("dump.vcd"); $dumpvars;
 // end
  initial begin

    while(1) begin
      #5

      $display("At time %t,data_out = %b, dready = %b", $time, data, dready);
    end
  end
endmodule

the result:
At time                    5,data_out = xxxxxxxx, dready = 1
At time                   10,data_out = xxxxxxxx, dready = 1
At time                   15,data_out = xxxxxxxx, dready = 1
At time                   20,data_out = 00000011, dready = 1
At time                   25,data_out = 00000011, dready = 1
At time                   30,data_out = 00000011, dready = 1
At time                   35,data_out = 00000011, dready = 1
At time                   40,data_out = 00000000, dready = 1
At time                   45,data_out = 00000000, dready = 1
At time                   50,data_out = 00000000, dready = 1
At time                   55,data_out = 00000000, dready = 1
At time                   60,data_out = 00000100, dready = 1
At time                   65,data_out = 00000100, dready = 1
At time                   70,data_out = 00000100, dready = 1
At time                   75,data_out = 00000100, dready = 1
At time                   80,data_out = 00000010, dready = 1
At time                   85,data_out = 00000010, dready = 1
At time                   90,data_out = 00000010, dready = 1
At time                   95,data_out = 00000010, dready = 1
At time                  100,data_out = 00000011, dready = 1
At time                  105,data_out = 00000011, dready = 1
At time                  110,data_out = 00000011, dready = 1
At time                  115,data_out = 00000011, dready = 1
At time                  120,data_out = 00000100, dready = 1
At time                  125,data_out = 00000100, dready = 1
At time                  130,data_out = 00000100, dready = 1
At time                  135,data_out = 00000100, dready = 1
At time                  140,data_out = 00000100, dready = 1
At time                  145,data_out = 00000100, dready = 1
At time                  150,data_out = 00000100, dready = 1
At time                  155,data_out = 00000100, dready = 1
At time                  160,data_out = 00000110, dready = 1
