// Code your testbench here
// or browse Examples
module eqn_evaluator_tb();
  
    reg clk;
    reg reset, wr_en, rd_en;
  reg [7:0] data_in;
    // Output
  wire [7:0] data_out;
  wire full;
  wire empty;
  

    // Instantiate the Unit Under Test (UUT)
    fifo_buffer uut ( 
        .clk(clk), 
        .reset(reset), 
      .wr_en(wr_en),
      .rd_en(rd_en),
      .data_in(data_in),
      .data_out(data_out),
      .full(full),
      .empty(empty)
    );

    initial begin
        // Initialize Inputs
        clk = 0;
        reset = 1;

        // Wait 100 ns for global reset to finish
        #10;
        reset = 0;      // Release reset
		rd_en = 0;
      wr_en = 1;
      data_in = 8'd1;
      #10
      rd_en = 0;
      wr_en = 1;
      data_in = 8'd2;
      #10
      rd_en = 0;
      wr_en = 1;
      data_in = 8'd3;
      #10
      rd_en = 1;
      wr_en = 1;
      data_in = 8'd4;
      #10
      rd_en = 1;
      wr_en = 1;
      data_in = 8'd5;
      


      
      #1000
        $finish;        // Terminate the simulation
    end


    always #5 clk = ~clk; 
  
  initial begin
  //  wait(~reset);
    while(1) begin
      #10
      //read the simulation result
      $display("At time %t,data_in=%d,data_out=%d", $time,data_in, data_out);
    end
  end

  
endmodule
    
    At time                   10,data_in=  1,data_out=  x
At time                   20,data_in=  2,data_out=  x
At time                   30,data_in=  3,data_out=  x
At time                   40,data_in=  4,data_out=  x
At time                   50,data_in=  5,data_out=  1
At time                   60,data_in=  5,data_out=  2
At time                   70,data_in=  5,data_out=  3
At time                   80,data_in=  5,data_out=  4
At time                   90,data_in=  5,data_out=  5
At time                  100,data_in=  5,data_out=  5
