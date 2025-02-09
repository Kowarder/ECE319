// Code your testbench here
// or browse Examples
module trojan_testbench;
    reg clk, rst;
    reg [127:0] state, key;
    wire [127:0] trojan_output, golden_output;
    integer i, detect_count, total_tests;
    reg detected;

    // Instantiate Trojan-infected AES
    aes_128 aes_trojan (
        .clk(clk),
        .rst(rst),
        .state(state),
        .key(key),
        .out(trojan_output)
    );

    // Instantiate Trojan-free AES
    aes_128_golden aes_original (
        .clk(clk),
        .rst(rst),
        .state(state),
        .key(key),
        .out(golden_output)
    );

    always #5 clk = ~clk;

    initial begin
        $dumpfile("trojan_detection.vcd");
        $dumpvars(0, trojan_testbench);

        clk = 0;
        rst = 1;
        detected = 0;
        detect_count = 0;
        total_tests = 100; // Number of test cases to detect the Trojan

        #10 rst = 0;

        for (i = 0; i < total_tests; i = i + 1) begin

            state = {$random, $random, $random, $random};
   			key   = {$random, $random, $random, $random};
            
            #20; 

            // Compare outputs
            if (trojan_output !== golden_output) begin
                $display("Trojan detected at test #%d", i);
                $display("State: %h", state);
                $display("Key: %h", key);
                $display("Golden output: %h", golden_output);
                $display("Trojan output: %h", trojan_output);
                detected = 1;
                detect_count = detect_count + 1;
            end
        end

        // Print summary of detection results
        if (!detected)
            $display("No Trojan detected in %d tests", total_tests);
        else
            $display("Trojan detected in %d out of %d tests", detect_count, total_tests);

        $finish;
    end

endmodule
