# This folder contains testbench file for five different Trojans.
## Testbench file contains random states and keys generation function, as well as outputting state and key values for different results using the same state and key to compare the results of the Trojan-implanted version with the creditable version (no Trojan implant).
Note: For some test, you need to reverse the reset signal. Cause in some Trojan's logic, it use ~rst to reset the counter. 
