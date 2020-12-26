Control the GH-CJDS66 60MHz Signal Generator  
--------------------------------------------  
  
Copyright © 2020 Jeremy Carter <jeremy@jeremycarter.ca>  
  
By using this software, you agree to the LICENSE TERMS 
outlined in the file titled LICENSE.md contained in the 
top-level directory of this project. If you don't agree
to the LICENSE TERMS, you aren't allowed to use this
software.  
  
  
Usage: ./signal-gen-cjds66 -h  
  
  
Purpose:  
-------  
  Control one or many GH-CJDS66 60MHz Signal Generators.  
  
USB Interface:  
-------------  
  ID 1a86:7523 QinHeng Electronics CH340 serial converter  
  
  
Tutorial - Design a Wave:  
------------------------  
1. Download Waveform Manager Plus (this has been tested with v4.13):  
     https://www.aimtti.com/resources/waveform-manager-plus-v413  
  
2. Unzip it and install it with wine (it's Windows-only but works well in wine):  
     wine start waveman.msi  
  
3. Run the program with wine:  
     cd ~/".wine/drive_c/Program Files (x86)/Waveman"  
     wine waveman.exe  
  
4. Design a new waveform of amplitude 4096 and length 2048, and save it as format "WaveCAD *.wav".  
  